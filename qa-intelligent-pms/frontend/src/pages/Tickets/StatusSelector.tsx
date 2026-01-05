import { useState, useEffect, useCallback } from "react";
import * as Select from "@radix-ui/react-select";
import { useToast } from "@/hooks/useToast";

interface TransitionInfo {
  id: string;
  name: string;
  toStatus: string;
  toStatusColor: string;
}

interface StatusSelectorProps {
  /** Jira ticket key */
  ticketKey: string;
  /** Current status name */
  currentStatus: string;
  /** Current status color category */
  currentStatusColor: string;
  /** Callback when status changes successfully */
  onStatusChange?: (newStatus: string, newStatusColor: string) => void;
}

const STATUS_COLORS: Record<string, string> = {
  blue: "bg-primary-100 text-primary-700 border-primary-300",
  yellow: "bg-warning-100 text-warning-700 border-warning-300",
  green: "bg-success-100 text-success-700 border-success-300",
  default: "bg-neutral-100 text-neutral-600 border-neutral-300",
};

function getStatusColorClass(colorName: string): string {
  return STATUS_COLORS[colorName.toLowerCase()] || STATUS_COLORS.default;
}

/** Fetch available transitions from API */
async function fetchTransitions(ticketKey: string): Promise<TransitionInfo[]> {
  const res = await fetch(`/api/v1/tickets/${encodeURIComponent(ticketKey)}/transitions`);
  if (!res.ok) {
    throw new Error("Failed to fetch transitions");
  }
  return res.json();
}

/** Perform transition via API */
async function performTransition(ticketKey: string, transitionId: string): Promise<void> {
  const res = await fetch(`/api/v1/tickets/${encodeURIComponent(ticketKey)}/transition`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ transitionId }),
  });
  if (!res.ok) {
    const error = await res.json().catch(() => ({ error: "Unknown error" }));
    throw new Error(error.error || "Transition failed");
  }
}

export function StatusSelector({
  ticketKey,
  currentStatus,
  currentStatusColor,
  onStatusChange,
}: StatusSelectorProps) {
  const [transitions, setTransitions] = useState<TransitionInfo[]>([]);
  const [displayStatus, setDisplayStatus] = useState(currentStatus);
  const [displayColor, setDisplayColor] = useState(currentStatusColor);
  const [isLoading, setIsLoading] = useState(false);
  const [isLoadingTransitions, setIsLoadingTransitions] = useState(true);
  const { toast, dismiss } = useToast();

  // Load available transitions on mount
  useEffect(() => {
    setIsLoadingTransitions(true);
    fetchTransitions(ticketKey)
      .then(setTransitions)
      .catch((err) => {
        console.error("Failed to load transitions:", err);
      })
      .finally(() => setIsLoadingTransitions(false));
  }, [ticketKey]);

  // Update display when props change
  useEffect(() => {
    setDisplayStatus(currentStatus);
    setDisplayColor(currentStatusColor);
  }, [currentStatus, currentStatusColor]);

  const handleTransition = useCallback(
    async (transition: TransitionInfo) => {
      const previousStatus = displayStatus;
      const previousColor = displayColor;

      // Optimistic update
      setDisplayStatus(transition.toStatus);
      setDisplayColor(transition.toStatusColor);
      setIsLoading(true);

      try {
        await performTransition(ticketKey, transition.id);

        toast({
          title: "Status updated",
          description: `Ticket moved to "${transition.toStatus}"`,
          variant: "success",
        });

        onStatusChange?.(transition.toStatus, transition.toStatusColor);

        // Reload transitions for new status
        fetchTransitions(ticketKey)
          .then(setTransitions)
          .catch(() => {});
      } catch (error) {
        // Revert optimistic update
        setDisplayStatus(previousStatus);
        setDisplayColor(previousColor);

        const errorMessage = error instanceof Error ? error.message : "Unknown error";

        toast({
          title: "Failed to update status",
          description: errorMessage,
          variant: "error",
          action: {
            label: "Retry",
            onClick: () => handleTransition(transition),
          },
        });
      } finally {
        setIsLoading(false);
      }
    },
    [ticketKey, displayStatus, displayColor, toast, dismiss, onStatusChange]
  );

  const handleValueChange = useCallback(
    (value: string) => {
      const transition = transitions.find((t) => t.id === value);
      if (transition) {
        handleTransition(transition);
      }
    },
    [transitions, handleTransition]
  );

  // If no transitions available, just show the current status as a badge
  if (!isLoadingTransitions && transitions.length === 0) {
    return (
      <span
        className={`inline-flex px-2.5 py-1 rounded-md text-sm font-medium border ${getStatusColorClass(
          displayColor
        )}`}
      >
        {displayStatus}
      </span>
    );
  }

  return (
    <Select.Root value="" onValueChange={handleValueChange} disabled={isLoading}>
      <Select.Trigger
        className={`
          inline-flex items-center gap-2 px-2.5 py-1 rounded-md text-sm font-medium border
          transition-all outline-none
          ${getStatusColorClass(displayColor)}
          ${isLoading ? "opacity-60 cursor-wait" : "cursor-pointer hover:shadow-sm"}
          focus:ring-2 focus:ring-primary-500 focus:ring-offset-1
        `}
        aria-label="Change status"
      >
        <Select.Value>
          <span className="flex items-center gap-2">
            {isLoading && (
              <svg
                className="w-3 h-3 animate-spin"
                fill="none"
                viewBox="0 0 24 24"
              >
                <circle
                  className="opacity-25"
                  cx="12"
                  cy="12"
                  r="10"
                  stroke="currentColor"
                  strokeWidth="4"
                />
                <path
                  className="opacity-75"
                  fill="currentColor"
                  d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                />
              </svg>
            )}
            {displayStatus}
          </span>
        </Select.Value>
        <Select.Icon>
          <svg
            className="w-4 h-4 opacity-60"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M19 9l-7 7-7-7"
            />
          </svg>
        </Select.Icon>
      </Select.Trigger>

      <Select.Portal>
        <Select.Content
          className="bg-white rounded-lg shadow-lg border border-neutral-200 py-1 min-w-[200px] z-50 overflow-hidden"
          position="popper"
          sideOffset={4}
          align="start"
        >
          <Select.Viewport>
            {/* Current status (disabled) */}
            <div className="px-3 py-2 text-sm flex items-center gap-2 bg-neutral-50 border-b border-neutral-100">
              <svg
                className="w-4 h-4 text-primary-600"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M5 13l4 4L19 7"
                />
              </svg>
              <span className="font-medium">{displayStatus}</span>
              <span className="text-xs text-neutral-400 ml-auto">Current</span>
            </div>

            {/* Available transitions */}
            {isLoadingTransitions ? (
              <div className="px-3 py-4 text-sm text-neutral-500 text-center">
                <svg
                  className="w-4 h-4 animate-spin mx-auto mb-2"
                  fill="none"
                  viewBox="0 0 24 24"
                >
                  <circle
                    className="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    strokeWidth="4"
                  />
                  <path
                    className="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                  />
                </svg>
                Loading transitions...
              </div>
            ) : transitions.length > 0 ? (
              <>
                <div className="px-3 py-1.5 text-xs font-medium text-neutral-400 uppercase tracking-wider">
                  Move to
                </div>
                {transitions.map((transition) => (
                  <Select.Item
                    key={transition.id}
                    value={transition.id}
                    className="px-3 py-2 text-sm flex items-center gap-2 cursor-pointer 
                             hover:bg-neutral-100 outline-none focus:bg-neutral-100
                             data-[highlighted]:bg-neutral-100"
                  >
                    <Select.ItemText>
                      <span className="flex items-center gap-2">
                        <svg
                          className="w-4 h-4 text-neutral-400"
                          fill="none"
                          viewBox="0 0 24 24"
                          stroke="currentColor"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={2}
                            d="M13 7l5 5m0 0l-5 5m5-5H6"
                          />
                        </svg>
                        <span className="font-medium">{transition.name}</span>
                        <span className="text-neutral-400">→</span>
                        <span
                          className={`px-1.5 py-0.5 rounded text-xs font-medium ${getStatusColorClass(
                            transition.toStatusColor
                          )}`}
                        >
                          {transition.toStatus}
                        </span>
                      </span>
                    </Select.ItemText>
                  </Select.Item>
                ))}
              </>
            ) : (
              <div className="px-3 py-4 text-sm text-neutral-500 text-center">
                No transitions available
              </div>
            )}
          </Select.Viewport>
        </Select.Content>
      </Select.Portal>
    </Select.Root>
  );
}

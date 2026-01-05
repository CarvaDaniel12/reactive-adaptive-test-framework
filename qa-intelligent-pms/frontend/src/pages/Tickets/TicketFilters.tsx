import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import { useWizardStore } from "../../stores/wizardStore";

interface TicketFiltersProps {
  currentStatus: string | undefined;
  onStatusChange: (status: string | undefined) => void;
}

// Default statuses if none configured
const DEFAULT_STATUSES = [
  "Backlog",
  "Ready for QA",
  "QA In Progress",
  "UAT",
  "Done",
];

export function TicketFilters({
  currentStatus,
  onStatusChange,
}: TicketFiltersProps) {
  const formData = useWizardStore((state) => state.formData);

  // Use configured ticket states or defaults
  const availableStatuses = formData.profile?.ticketStates?.length
    ? formData.profile.ticketStates
    : DEFAULT_STATUSES;

  return (
    <div className="flex items-center gap-3">
      {/* Status Filter Dropdown */}
      <DropdownMenu.Root>
        <DropdownMenu.Trigger asChild>
          <button
            className="inline-flex items-center gap-2 px-3 py-2 text-sm font-medium 
                       border border-neutral-300 rounded-lg hover:bg-neutral-50 
                       transition-colors focus:outline-none focus:ring-2 
                       focus:ring-primary-500 focus:ring-offset-2"
          >
            <svg
              className="w-4 h-4 text-neutral-500"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z"
              />
            </svg>
            <span>
              {currentStatus || "All Statuses"}
            </span>
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
                d="M19 9l-7 7-7-7"
              />
            </svg>
          </button>
        </DropdownMenu.Trigger>

        <DropdownMenu.Portal>
          <DropdownMenu.Content
            className="min-w-[180px] bg-white rounded-lg shadow-lg border border-neutral-200 
                       py-1 z-50 animate-in fade-in-0 zoom-in-95"
            sideOffset={5}
            align="end"
          >
            {/* All option */}
            <DropdownMenu.Item
              className={`px-3 py-2 text-sm cursor-pointer outline-none
                         hover:bg-neutral-50 transition-colors
                         ${!currentStatus ? "text-primary-600 font-medium bg-primary-50" : "text-neutral-700"}`}
              onSelect={() => onStatusChange(undefined)}
            >
              All Statuses
            </DropdownMenu.Item>

            <DropdownMenu.Separator className="h-px bg-neutral-200 my-1" />

            {/* Status options */}
            {availableStatuses.map((status: string) => (
              <DropdownMenu.Item
                key={status}
                className={`px-3 py-2 text-sm cursor-pointer outline-none
                           hover:bg-neutral-50 transition-colors
                           ${currentStatus === status ? "text-primary-600 font-medium bg-primary-50" : "text-neutral-700"}`}
                onSelect={() => onStatusChange(status)}
              >
                {status}
              </DropdownMenu.Item>
            ))}
          </DropdownMenu.Content>
        </DropdownMenu.Portal>
      </DropdownMenu.Root>

      {/* Clear filter button (shown when filter is active) */}
      {currentStatus && (
        <button
          onClick={() => onStatusChange(undefined)}
          className="p-2 text-neutral-400 hover:text-neutral-600 
                     hover:bg-neutral-100 rounded-lg transition-colors"
          title="Clear filter"
        >
          <svg
            className="w-4 h-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      )}
    </div>
  );
}

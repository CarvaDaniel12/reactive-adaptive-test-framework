import { formatDuration } from "@/hooks/useWorkflow";

// SVG Icons
function PlayIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="currentColor">
      <path fillRule="evenodd" d="M4.5 5.653c0-1.427 1.529-2.33 2.779-1.643l11.54 6.347c1.295.712 1.295 2.573 0 3.286L7.28 19.99c-1.25.687-2.779-.217-2.779-1.643V5.653Z" clipRule="evenodd" />
    </svg>
  );
}

function PauseIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="currentColor">
      <path fillRule="evenodd" d="M6.75 5.25a.75.75 0 0 1 .75-.75H9a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H7.5a.75.75 0 0 1-.75-.75V5.25Zm7.5 0A.75.75 0 0 1 15 4.5h1.5a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H15a.75.75 0 0 1-.75-.75V5.25Z" clipRule="evenodd" />
    </svg>
  );
}

function XMarkIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
    </svg>
  );
}

interface WorkflowHeaderProps {
  ticketKey: string;
  ticketTitle: string;
  templateName: string;
  currentStep: number;
  totalSteps: number;
  estimatedMinutes: number;
  status: string;
  onPause?: () => void;
  onResume?: () => void;
  onExit?: () => void;
}

/**
 * Header component for workflow mode.
 * Shows ticket context and workflow controls.
 */
export function WorkflowHeader({
  ticketKey,
  ticketTitle,
  templateName,
  currentStep,
  totalSteps,
  estimatedMinutes,
  status,
  onPause,
  onResume,
  onExit,
}: WorkflowHeaderProps) {
  const progressPercent = totalSteps > 0 ? ((currentStep + 1) / totalSteps) * 100 : 0;
  const isPaused = status === "paused";

  return (
    <div className="bg-white border-b border-neutral-200 px-6 py-4">
      {/* Top Row: Ticket Info + Controls */}
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-3">
          <span className="px-2 py-1 bg-blue-100 text-blue-700 text-sm font-mono rounded">
            {ticketKey}
          </span>
          <h1 className="text-lg font-medium text-neutral-900 truncate max-w-md">
            {ticketTitle}
          </h1>
        </div>

        <div className="flex items-center gap-2">
          {isPaused ? (
            <button
              onClick={onResume}
              className="flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium
                         text-green-700 bg-green-50 rounded-lg hover:bg-green-100 transition-colors"
            >
              <PlayIcon className="w-4 h-4" />
              Resume
            </button>
          ) : (
            <button
              onClick={onPause}
              className="flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium
                         text-amber-700 bg-amber-50 rounded-lg hover:bg-amber-100 transition-colors"
            >
              <PauseIcon className="w-4 h-4" />
              Pause
            </button>
          )}
          <button
            onClick={onExit}
            className="flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium
                       text-neutral-600 bg-neutral-100 rounded-lg hover:bg-neutral-200 transition-colors"
          >
            <XMarkIcon className="w-4 h-4" />
            Exit
          </button>
        </div>
      </div>

      {/* Bottom Row: Progress */}
      <div className="flex items-center gap-4">
        <span className="text-sm text-neutral-500">{templateName}</span>
        <div className="flex-1 h-2 bg-neutral-100 rounded-full overflow-hidden">
          <div
            className="h-full bg-blue-500 transition-all duration-300"
            style={{ width: `${progressPercent}%` }}
          />
        </div>
        <span className="text-sm font-medium text-neutral-700">
          Step {currentStep + 1} of {totalSteps}
        </span>
        <span className="text-sm text-neutral-400">
          ~{formatDuration(estimatedMinutes)} total
        </span>
      </div>
    </div>
  );
}

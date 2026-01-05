import { formatTime } from "@/hooks/useTimer";

interface TimerDisplayProps {
  stepSeconds: number;
  totalSeconds: number;
  isRunning: boolean;
  isPaused: boolean;
  onPause?: () => void;
  onResume?: () => void;
}

function ClockIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z" />
    </svg>
  );
}

function PauseIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="currentColor">
      <path fillRule="evenodd" d="M6.75 5.25a.75.75 0 01.75-.75H9a.75.75 0 01.75.75v13.5a.75.75 0 01-.75.75H7.5a.75.75 0 01-.75-.75V5.25zm7.5 0A.75.75 0 0115 4.5h1.5a.75.75 0 01.75.75v13.5a.75.75 0 01-.75.75H15a.75.75 0 01-.75-.75V5.25z" clipRule="evenodd" />
    </svg>
  );
}

function PlayIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="currentColor">
      <path fillRule="evenodd" d="M4.5 5.653c0-1.427 1.529-2.33 2.779-1.643l11.54 6.347c1.295.712 1.295 2.573 0 3.286L7.28 19.99c-1.25.687-2.779-.217-2.779-1.643V5.653z" clipRule="evenodd" />
    </svg>
  );
}

/**
 * Timer display component for workflow header.
 */
export function TimerDisplay({
  stepSeconds,
  totalSeconds,
  isRunning,
  isPaused,
  onPause,
  onResume,
}: TimerDisplayProps) {
  return (
    <div className="flex items-center gap-4">
      {/* Step Timer */}
      <div className="flex items-center gap-2">
        <div className="flex items-center gap-1.5">
          {isRunning && !isPaused && (
            <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse" />
          )}
          {isPaused && (
            <div className="w-2 h-2 bg-amber-500 rounded-full" />
          )}
          {!isRunning && !isPaused && (
            <div className="w-2 h-2 bg-neutral-300 rounded-full" />
          )}
          <span className="text-xs text-neutral-500 uppercase">Step</span>
        </div>
        <span className="font-mono text-lg font-semibold text-neutral-900">
          {formatTime(stepSeconds)}
        </span>
      </div>

      {/* Divider */}
      <div className="w-px h-6 bg-neutral-200" />

      {/* Total Timer */}
      <div className="flex items-center gap-2">
        <ClockIcon className="w-4 h-4 text-neutral-400" />
        <span className="text-xs text-neutral-500 uppercase">Total</span>
        <span className="font-mono text-sm text-neutral-600">
          {formatTime(totalSeconds + stepSeconds)}
        </span>
      </div>

      {/* Pause/Resume Button */}
      {(onPause || onResume) && isRunning && (
        <button
          onClick={isPaused ? onResume : onPause}
          className={`p-1.5 rounded-lg transition-colors ${
            isPaused
              ? "bg-green-100 text-green-600 hover:bg-green-200"
              : "bg-neutral-100 text-neutral-600 hover:bg-neutral-200"
          }`}
          title={isPaused ? "Resume timer" : "Pause timer"}
        >
          {isPaused ? (
            <PlayIcon className="w-4 h-4" />
          ) : (
            <PauseIcon className="w-4 h-4" />
          )}
        </button>
      )}
    </div>
  );
}

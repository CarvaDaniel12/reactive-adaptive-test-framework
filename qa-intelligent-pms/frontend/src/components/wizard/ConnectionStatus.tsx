/**
 * Connection status indicator component.
 *
 * Shows the result of an integration connection test with appropriate styling.
 */

type ConnectionState = "idle" | "testing" | "success" | "error";

interface ConnectionStatusProps {
  /** Current connection state */
  state: ConnectionState;
  /** Error message to display when state is "error" */
  errorMessage?: string;
  /** Success message to display when state is "success" */
  successMessage?: string;
  /** Additional info to show on success (e.g., project count) */
  successDetails?: string;
}

export function ConnectionStatus({
  state,
  errorMessage,
  successMessage = "Connection successful",
  successDetails,
}: ConnectionStatusProps) {
  if (state === "idle") return null;

  if (state === "testing") {
    return (
      <div className="flex items-center gap-2 text-neutral-600 py-3">
        <LoadingSpinner className="w-4 h-4" />
        <span>Testing connection...</span>
      </div>
    );
  }

  if (state === "success") {
    return (
      <div className="flex items-center gap-2 text-success-600 bg-success-50 px-4 py-3 rounded-lg">
        <CheckCircleIcon className="w-5 h-5 flex-shrink-0" />
        <div>
          <span className="font-medium">{successMessage}</span>
          {successDetails && (
            <span className="text-success-500 ml-2">{successDetails}</span>
          )}
        </div>
      </div>
    );
  }

  if (state === "error") {
    return (
      <div className="flex items-center gap-2 text-error-600 bg-error-50 px-4 py-3 rounded-lg">
        <CrossCircleIcon className="w-5 h-5 flex-shrink-0" />
        <span>{errorMessage || "Connection failed"}</span>
      </div>
    );
  }

  return null;
}

function LoadingSpinner({ className }: { className?: string }) {
  return (
    <svg
      className={`animate-spin ${className}`}
      viewBox="0 0 24 24"
      fill="none"
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
  );
}

function CheckCircleIcon({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
      strokeWidth={2}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
      />
    </svg>
  );
}

function CrossCircleIcon({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
      strokeWidth={2}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
      />
    </svg>
  );
}

export type { ConnectionState };

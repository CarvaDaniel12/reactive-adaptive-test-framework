/**
 * Critical error screen for blocking integration failures.
 *
 * Shown when a critical integration (like Jira) fails validation.
 * Provides options to fix settings or retry.
 */
import { Link } from "react-router-dom";

interface CriticalErrorScreenProps {
  integration: string;
  errorMessage: string;
}

export function CriticalErrorScreen({
  integration,
  errorMessage,
}: CriticalErrorScreenProps) {
  const integrationLabel = capitalize(integration);

  return (
    <div className="min-h-screen flex items-center justify-center bg-neutral-50 p-4">
      <div className="max-w-md w-full bg-white rounded-xl shadow-lg p-8 text-center">
        {/* Error Icon */}
        <div className="w-16 h-16 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-6">
          <ErrorIcon className="w-8 h-8 text-red-500" />
        </div>

        {/* Title */}
        <h1 className="text-xl font-semibold text-neutral-900 mb-2">
          Connection Error
        </h1>

        {/* Message */}
        <p className="text-neutral-600 mb-4">
          Unable to connect to{" "}
          <span className="font-medium">{integrationLabel}</span>
        </p>

        {/* Error details */}
        <div className="bg-red-50 border border-red-200 rounded-lg p-4 mb-6 text-left">
          <p className="text-sm text-red-700">{errorMessage}</p>
        </div>

        {/* Explanation */}
        <p className="text-sm text-neutral-500 mb-6">
          {integration === "jira"
            ? "Jira is required for the application to work. Please check your credentials and try again."
            : "This integration is required. Please verify your settings."}
        </p>

        {/* Actions */}
        <div className="space-y-3">
          <Link
            to={`/setup/${integration}`}
            className="flex items-center justify-center gap-2 w-full px-4 py-3 
                       bg-primary-500 text-white rounded-lg hover:bg-primary-600 
                       transition-colors font-medium"
          >
            <SettingsIcon className="w-5 h-5" />
            Fix Settings
          </Link>

          <button
            type="button"
            onClick={() => window.location.reload()}
            className="w-full px-4 py-3 border border-neutral-300 rounded-lg 
                       text-neutral-700 hover:bg-neutral-50 transition-colors"
          >
            Try Again
          </button>
        </div>

        {/* Help text */}
        <p className="mt-6 text-xs text-neutral-400">
          If the problem persists, check your network connection or contact
          support.
        </p>
      </div>
    </div>
  );
}

function capitalize(s: string): string {
  return s.charAt(0).toUpperCase() + s.slice(1);
}

function ErrorIcon({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={2}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z"
      />
    </svg>
  );
}

function SettingsIcon({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={2}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 010 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z"
      />
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
      />
    </svg>
  );
}

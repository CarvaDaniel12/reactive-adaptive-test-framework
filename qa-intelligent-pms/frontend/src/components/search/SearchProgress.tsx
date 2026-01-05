import type { SearchStatus } from "@/hooks/useContextualSearch";

interface SearchProgressProps {
  postman: SearchStatus;
  testmo: SearchStatus;
}

/**
 * Progress indicator for contextual search.
 * 
 * Shows the status of each integration search.
 */
export function SearchProgress({ postman, testmo }: SearchProgressProps) {
  return (
    <div className="space-y-2" role="status" aria-label="Search progress">
      <SearchProgressItem label="Postman" status={postman} />
      <SearchProgressItem label="Testmo" status={testmo} />
    </div>
  );
}

interface SearchProgressItemProps {
  label: string;
  status: SearchStatus;
}

function SearchProgressItem({ label, status }: SearchProgressItemProps) {
  const isSearching = status === "searching";
  const isComplete = status === "complete";
  const isError = status === "error";
  const isSkipped = status === "skipped";

  return (
    <div className="flex items-center gap-2 text-sm">
      {/* Status Icon */}
      {isSearching && (
        <div className="w-4 h-4 border-2 border-blue-200 border-t-blue-500 rounded-full animate-spin" />
      )}
      {isComplete && (
        <div className="w-4 h-4 bg-green-500 rounded-full flex items-center justify-center">
          <CheckIcon className="w-2.5 h-2.5 text-white" />
        </div>
      )}
      {isError && (
        <div className="w-4 h-4 bg-red-500 rounded-full flex items-center justify-center">
          <XIcon className="w-2.5 h-2.5 text-white" />
        </div>
      )}
      {isSkipped && (
        <div className="w-4 h-4 bg-neutral-300 rounded-full flex items-center justify-center">
          <MinusIcon className="w-2.5 h-2.5 text-white" />
        </div>
      )}
      {status === "pending" && (
        <div className="w-4 h-4 bg-neutral-200 rounded-full" />
      )}

      {/* Label */}
      <span className={isSearching ? "text-neutral-700" : "text-neutral-500"}>
        {isSearching ? `Searching ${label}...` : label}
      </span>

      {/* Status text */}
      {isError && <span className="text-xs text-red-500">Failed</span>}
      {isSkipped && <span className="text-xs text-neutral-400">Skipped</span>}
    </div>
  );
}

// Simple SVG icons
function CheckIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
    </svg>
  );
}

function XIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M6 18L18 6M6 6l12 12" />
    </svg>
  );
}

function MinusIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M20 12H4" />
    </svg>
  );
}

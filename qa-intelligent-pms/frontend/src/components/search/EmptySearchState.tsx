// SVG Icons
function MagnifyingGlassIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M10 6.5C10 8.433 8.433 10 6.5 10C4.567 10 3 8.433 3 6.5C3 4.567 4.567 3 6.5 3C8.433 3 10 4.567 10 6.5ZM9.30884 10.0159C8.53901 10.6318 7.56251 11 6.5 11C4.01472 11 2 8.98528 2 6.5C2 4.01472 4.01472 2 6.5 2C8.98528 2 11 4.01472 11 6.5C11 7.56251 10.6318 8.53901 10.0159 9.30884L12.8536 12.1464C13.0488 12.3417 13.0488 12.6583 12.8536 12.8536C12.6583 13.0488 12.3417 13.0488 12.1464 12.8536L9.30884 10.0159Z"
        fill="currentColor"
        fillRule="evenodd"
        clipRule="evenodd"
      />
    </svg>
  );
}

function LightBulbIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
    </svg>
  );
}

interface EmptySearchStateProps {
  keywords?: string[];
  showTips?: boolean;
}

/**
 * Empty state component shown when no search results are found.
 * 
 * Includes helpful tips for improving search results.
 */
export function EmptySearchState({ keywords = [], showTips = true }: EmptySearchStateProps) {
  return (
    <div className="text-center py-8">
      <div className="w-12 h-12 bg-neutral-100 rounded-full flex items-center justify-center mx-auto mb-4">
        <MagnifyingGlassIcon className="w-6 h-6 text-neutral-400" />
      </div>

      <h3 className="text-lg font-medium text-neutral-700 mb-2">
        No related tests found
      </h3>

      {keywords.length > 0 && (
        <p className="text-sm text-neutral-500 mb-4">
          Searched for: <span className="font-medium">{keywords.join(", ")}</span>
        </p>
      )}

      {showTips && (
        <div className="max-w-sm mx-auto text-left bg-neutral-50 rounded-lg p-4 mt-4">
          <div className="flex items-center gap-2 text-sm font-medium text-neutral-700 mb-2">
            <LightBulbIcon className="w-4 h-4" />
            Search Tips
          </div>
          <ul className="text-sm text-neutral-600 space-y-1.5 list-disc list-inside">
            <li>Check that Postman/Testmo integrations are configured</li>
            <li>Try using more specific terms in ticket title</li>
            <li>Ensure test collections use searchable naming conventions</li>
            <li>Verify the workspace/project is accessible</li>
          </ul>
        </div>
      )}
    </div>
  );
}

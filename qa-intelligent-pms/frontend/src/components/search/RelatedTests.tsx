import { useContextualSearch } from "@/hooks/useContextualSearch";
import { SelectableSearchResults } from "./SelectableSearchResults";
import { SearchResultsSkeleton } from "./SearchResultsSkeleton";
import { EmptySearchState } from "./EmptySearchState";
import { SearchProgress } from "./SearchProgress";

// SVG Icon
function ReloadIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M1.84998 7.49998C1.84998 4.66458 4.05979 1.84998 7.49998 1.84998C10.2783 1.84998 11.6515 3.9064 12.2367 5H10.5C10.2239 5 10 5.22386 10 5.5C10 5.77614 10.2239 6 10.5 6H13.5C13.7761 6 14 5.77614 14 5.5V2.5C14 2.22386 13.7761 2 13.5 2C13.2239 2 13 2.22386 13 2.5V4.31318C12.2955 3.07126 10.6659 0.849976 7.49998 0.849976C3.43716 0.849976 0.849976 4.18537 0.849976 7.49998C0.849976 10.8146 3.43716 14.15 7.49998 14.15C9.44382 14.15 11.0622 13.3808 12.2145 12.2084C12.8315 11.5806 13.3133 10.839 13.6418 10.0407C13.7469 9.78536 13.6251 9.49315 13.3698 9.38806C13.1144 9.28296 12.8222 9.40478 12.7171 9.66014C12.4363 10.3425 12.0251 10.9745 11.5013 11.5074C10.5295 12.4963 9.16504 13.15 7.49998 13.15C4.05979 13.15 1.84998 10.3354 1.84998 7.49998Z"
        fill="currentColor"
        fillRule="evenodd"
        clipRule="evenodd"
      />
    </svg>
  );
}

interface RelatedTestsProps {
  ticketKey: string;
  title: string;
  description?: string | null;
}

/**
 * Component that displays related tests from Postman and Testmo.
 *
 * Automatically searches when mounted based on ticket title/description.
 * Results are grouped by source with visual badges.
 */
export function RelatedTests({ ticketKey, title, description }: RelatedTestsProps) {
  const {
    results,
    keywordsUsed,
    searchTimeMs,
    isLoading,
    isFetching,
    progress,
    error,
    refetch,
  } = useContextualSearch({ ticketKey, title, description });

  return (
    <div className="bg-white border border-neutral-200 rounded-lg">
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b border-neutral-100">
        <h2 className="text-lg font-medium text-neutral-900">Related Tests</h2>
        <div className="flex items-center gap-3">
          {searchTimeMs !== undefined && !isLoading && (
            <span className="text-xs text-neutral-400">{searchTimeMs}ms</span>
          )}
          <button
            onClick={() => refetch()}
            disabled={isFetching}
            className="p-1.5 text-neutral-500 hover:text-neutral-700 
                       hover:bg-neutral-100 rounded transition-colors
                       disabled:opacity-50"
            title="Refresh search"
            aria-label="Refresh search results"
          >
            <ReloadIcon className={`w-4 h-4 ${isFetching ? "animate-spin" : ""}`} />
          </button>
        </div>
      </div>

      {/* Content */}
      <div className="p-4">
        {/* Loading State */}
        {isLoading && (
          <>
            <SearchProgress postman={progress.postman} testmo={progress.testmo} />
            <div className="mt-4">
              <SearchResultsSkeleton count={3} />
            </div>
          </>
        )}

        {/* Error State */}
        {error && !isLoading && (
          <div className="text-center py-4">
            <p className="text-red-500 mb-2">Failed to search for related tests</p>
            <button
              onClick={() => refetch()}
              className="text-sm text-blue-600 hover:underline"
            >
              Try again
            </button>
          </div>
        )}

        {/* Empty State */}
        {!isLoading && !error && results.length === 0 && (
          <EmptySearchState keywords={keywordsUsed} />
        )}

        {/* Results */}
        {!isLoading && !error && results.length > 0 && (
          <>
            {/* Keywords */}
            {keywordsUsed.length > 0 && (
              <div className="flex flex-wrap gap-1 mb-4">
                <span className="text-xs text-neutral-500">Keywords:</span>
                {keywordsUsed.map((keyword) => (
                  <span
                    key={keyword}
                    className="text-xs px-2 py-0.5 bg-neutral-100 text-neutral-600 rounded"
                  >
                    {keyword}
                  </span>
                ))}
              </div>
            )}

            {/* Selectable Results with Create Test Run */}
            <SelectableSearchResults results={results} ticketKey={ticketKey} />
          </>
        )}
      </div>
    </div>
  );
}

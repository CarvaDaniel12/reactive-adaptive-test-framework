import { SearchResultCard } from "./SearchResultCard";
import { SourceBadge } from "./SourceBadge";
import type { SearchResult } from "@/hooks/useContextualSearch";

interface SearchResultsGroupedProps {
  results: SearchResult[];
  showGroupHeaders?: boolean;
}

/**
 * Component that displays search results grouped by source.
 * 
 * Shows group headers with counts when results come from multiple sources.
 */
export function SearchResultsGrouped({
  results,
  showGroupHeaders = true,
}: SearchResultsGroupedProps) {
  // Group results by source
  const grouped = results.reduce<Record<string, SearchResult[]>>((acc, result) => {
    if (!acc[result.source]) {
      acc[result.source] = [];
    }
    acc[result.source].push(result);
    return acc;
  }, {});

  const sources = Object.keys(grouped);

  if (sources.length === 0) {
    return null;
  }

  // If only one source and headers not forced, show flat list
  if (sources.length === 1 && !showGroupHeaders) {
    return (
      <div className="space-y-3">
        {results.map((result) => (
          <SearchResultCard key={`${result.source}-${result.id}`} result={result} />
        ))}
      </div>
    );
  }

  // Order sources: postman first, then testmo, then others alphabetically
  const orderedSources = sources.sort((a, b) => {
    const order = { postman: 0, testmo: 1 };
    const aOrder = order[a as keyof typeof order] ?? 2;
    const bOrder = order[b as keyof typeof order] ?? 2;
    if (aOrder !== bOrder) return aOrder - bOrder;
    return a.localeCompare(b);
  });

  return (
    <div className="space-y-6">
      {orderedSources.map((source) => (
        <div key={source} role="region" aria-label={`${source} results`}>
          {/* Group Header */}
          <div className="flex items-center gap-2 mb-3">
            <SourceBadge source={source} />
            <span className="text-sm text-neutral-500">
              {grouped[source].length} result{grouped[source].length !== 1 ? "s" : ""}
            </span>
          </div>

          {/* Results */}
          <div className="space-y-3">
            {grouped[source].map((result) => (
              <SearchResultCard
                key={`${result.source}-${result.id}`}
                result={result}
                showScore={true}
              />
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}

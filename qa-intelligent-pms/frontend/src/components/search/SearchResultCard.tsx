import { SourceBadge } from "./SourceBadge";
import type { SearchResult } from "@/hooks/useContextualSearch";

// SVG Icons
function ExternalLinkIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M3 2C2.44772 2 2 2.44772 2 3V12C2 12.5523 2.44772 13 3 13H12C12.5523 13 13 12.5523 13 12V8.5C13 8.22386 12.7761 8 12.5 8C12.2239 8 12 8.22386 12 8.5V12H3V3L6.5 3C6.77614 3 7 2.77614 7 2.5C7 2.22386 6.77614 2 6.5 2H3ZM12.8536 2.14645C12.9015 2.19439 12.9377 2.24964 12.9621 2.30861C12.9861 2.36669 12.9996 2.4303 13 2.497L13 2.5V2.50049V5.5C13 5.77614 12.7761 6 12.5 6C12.2239 6 12 5.77614 12 5.5V3.70711L6.85355 8.85355C6.65829 9.04882 6.34171 9.04882 6.14645 8.85355C5.95118 8.65829 5.95118 8.34171 6.14645 8.14645L11.2929 3H9.5C9.22386 3 9 2.77614 9 2.5C9 2.22386 9.22386 2 9.5 2H12.4999H12.5C12.5678 2 12.6324 2.01349 12.6914 2.03794C12.7504 2.06234 12.8056 2.09851 12.8536 2.14645Z"
        fill="currentColor"
        fillRule="evenodd"
        clipRule="evenodd"
      />
    </svg>
  );
}

function StarIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 15 15" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
      <path d="M7.22303 0.665992C7.32551 0.419604 7.67454 0.419604 7.77702 0.665992L9.41343 4.60039C9.45663 4.70426 9.55432 4.77523 9.66645 4.78422L13.914 5.12475C14.18 5.14607 14.2878 5.47802 14.0852 5.65162L10.849 8.42374C10.7636 8.49692 10.7263 8.61176 10.7524 8.72118L11.7411 12.866C11.803 13.1256 11.5206 13.3308 11.2929 13.1917L7.6564 10.9705C7.5604 10.9119 7.43965 10.9119 7.34365 10.9705L3.70718 13.1917C3.47945 13.3308 3.19708 13.1256 3.25899 12.866L4.24769 8.72118C4.2738 8.61176 4.23648 8.49692 4.15105 8.42374L0.914889 5.65162C0.712228 5.47802 0.820086 5.14607 1.08608 5.12475L5.3336 4.78422C5.44573 4.77523 5.54342 4.70426 5.58662 4.60039L7.22303 0.665992Z" />
    </svg>
  );
}

interface SearchResultCardProps {
  result: SearchResult;
  showScore?: boolean;
}

/**
 * Card component for displaying a single search result.
 * 
 * Features:
 * - Source badge (Postman/Testmo)
 * - Relevance indicator
 * - Truncated description
 * - External link to source system
 * - Keyboard accessible
 */
export function SearchResultCard({ result, showScore = true }: SearchResultCardProps) {
  const relevanceLevel = getRelevanceLevel(result.score);

  return (
    <a
      href={result.url}
      target="_blank"
      rel="noopener noreferrer"
      className="group block p-4 bg-white border border-neutral-200 rounded-lg 
                 hover:border-blue-300 hover:shadow-md transition-all
                 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
      aria-label={`Open ${result.name} in ${result.source}`}
    >
      {/* Header */}
      <div className="flex items-start justify-between gap-3 mb-2">
        <div className="flex items-center gap-2 flex-wrap">
          <SourceBadge source={result.source} size="sm" />
          {showScore && (
            <RelevanceIndicator level={relevanceLevel} score={result.score} />
          )}
        </div>
        <ExternalLinkIcon
          className="w-4 h-4 text-neutral-400 group-hover:text-blue-500 
                     transition-colors flex-shrink-0"
        />
      </div>

      {/* Title */}
      <h4 className="font-medium text-neutral-900 group-hover:text-blue-600 
                     transition-colors line-clamp-1 mb-1">
        {result.name}
      </h4>

      {/* Description */}
      {result.description && (
        <p className="text-sm text-neutral-500 line-clamp-2">
          {result.description}
        </p>
      )}

      {/* Matches */}
      {result.matches.length > 0 && result.matches.length <= 5 && (
        <div className="mt-2 flex flex-wrap gap-1">
          {result.matches.slice(0, 3).map((match, i) => (
            <span
              key={i}
              className="text-xs px-1.5 py-0.5 bg-blue-50 text-blue-600 rounded"
            >
              {match}
            </span>
          ))}
          {result.matches.length > 3 && (
            <span className="text-xs text-neutral-400">
              +{result.matches.length - 3} more
            </span>
          )}
        </div>
      )}
    </a>
  );
}

function getRelevanceLevel(score: number): "high" | "medium" | "low" {
  if (score >= 3) return "high";
  if (score >= 1.5) return "medium";
  return "low";
}

interface RelevanceIndicatorProps {
  level: "high" | "medium" | "low";
  score: number;
}

function RelevanceIndicator({ level, score }: RelevanceIndicatorProps) {
  const colors = {
    high: "text-green-500",
    medium: "text-amber-500",
    low: "text-neutral-400",
  };

  const labels = {
    high: "High match",
    medium: "Medium match",
    low: "Low match",
  };

  return (
    <span
      className={`flex items-center gap-1 text-xs ${colors[level]}`}
      title={`Relevance: ${labels[level]} (${Math.round(score * 10) / 10})`}
    >
      <StarIcon className="w-3 h-3" />
      <span className="sr-only">{labels[level]}</span>
    </span>
  );
}

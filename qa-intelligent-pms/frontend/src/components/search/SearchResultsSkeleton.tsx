interface SearchResultsSkeletonProps {
  count?: number;
}

/**
 * Skeleton loading state for search results.
 * 
 * Matches the dimensions of SearchResultCard for smooth transitions.
 */
export function SearchResultsSkeleton({ count = 3 }: SearchResultsSkeletonProps) {
  return (
    <div className="space-y-3" role="status" aria-label="Loading search results">
      {Array.from({ length: count }).map((_, i) => (
        <SearchResultCardSkeleton key={i} />
      ))}
      <span className="sr-only">Loading...</span>
    </div>
  );
}

function SearchResultCardSkeleton() {
  return (
    <div className="p-4 bg-white border border-neutral-200 rounded-lg animate-pulse">
      {/* Header */}
      <div className="flex items-center gap-2 mb-3">
        <div className="h-5 w-20 bg-neutral-200 rounded-md" />
        <div className="h-4 w-4 bg-neutral-200 rounded" />
      </div>

      {/* Title */}
      <div className="h-5 w-3/4 bg-neutral-200 rounded mb-2" />

      {/* Description */}
      <div className="space-y-1.5">
        <div className="h-4 w-full bg-neutral-100 rounded" />
        <div className="h-4 w-2/3 bg-neutral-100 rounded" />
      </div>

      {/* Tags */}
      <div className="flex gap-1 mt-3">
        <div className="h-5 w-12 bg-neutral-100 rounded" />
        <div className="h-5 w-16 bg-neutral-100 rounded" />
      </div>
    </div>
  );
}

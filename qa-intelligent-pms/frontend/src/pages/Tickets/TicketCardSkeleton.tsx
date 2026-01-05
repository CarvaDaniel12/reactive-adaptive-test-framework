export function TicketCardSkeleton() {
  return (
    <div className="p-4 bg-white border border-neutral-200 rounded-lg animate-pulse">
      {/* Header skeleton */}
      <div className="flex items-start justify-between mb-2">
        <div className="h-4 w-20 bg-neutral-200 rounded" />
        <div className="h-4 w-14 bg-neutral-200 rounded" />
      </div>

      {/* Title skeleton - two lines */}
      <div className="space-y-2 mb-3">
        <div className="h-5 w-full bg-neutral-200 rounded" />
        <div className="h-5 w-3/4 bg-neutral-200 rounded" />
      </div>

      {/* Footer skeleton */}
      <div className="flex items-center justify-between">
        <div className="h-6 w-24 bg-neutral-200 rounded" />
        <div className="flex items-center gap-2">
          <div className="h-5 w-5 bg-neutral-200 rounded-full" />
          <div className="h-4 w-16 bg-neutral-200 rounded" />
        </div>
      </div>
    </div>
  );
}

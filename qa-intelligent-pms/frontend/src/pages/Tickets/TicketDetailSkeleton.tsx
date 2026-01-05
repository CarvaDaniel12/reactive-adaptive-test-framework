/** Loading skeleton for ticket detail page */
export function TicketDetailSkeleton() {
  return (
    <div className="max-w-4xl mx-auto space-y-6 animate-pulse">
      {/* Back button skeleton */}
      <div className="h-5 w-28 bg-neutral-200 rounded" />

      {/* Header skeleton */}
      <div className="flex items-start justify-between">
        <div className="space-y-2">
          <div className="h-4 w-20 bg-neutral-200 rounded" />
          <div className="h-8 w-96 bg-neutral-200 rounded" />
        </div>
        <div className="h-12 w-36 bg-neutral-200 rounded-lg" />
      </div>

      {/* Metadata skeleton */}
      <div className="bg-neutral-100 rounded-xl p-5">
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-5">
          {Array.from({ length: 6 }).map((_, i) => (
            <div key={i} className="space-y-2">
              <div className="h-3 w-12 bg-neutral-200 rounded" />
              <div className="h-6 w-20 bg-neutral-200 rounded" />
            </div>
          ))}
        </div>
      </div>

      {/* Labels skeleton */}
      <div className="flex gap-2">
        <div className="h-6 w-16 bg-neutral-200 rounded" />
        <div className="h-6 w-20 bg-neutral-200 rounded" />
        <div className="h-6 w-14 bg-neutral-200 rounded" />
      </div>

      {/* Description skeleton */}
      <div className="bg-white border border-neutral-200 rounded-xl p-6 space-y-4">
        <div className="h-6 w-28 bg-neutral-200 rounded" />
        <div className="space-y-2">
          <div className="h-4 w-full bg-neutral-200 rounded" />
          <div className="h-4 w-full bg-neutral-200 rounded" />
          <div className="h-4 w-3/4 bg-neutral-200 rounded" />
          <div className="h-4 w-5/6 bg-neutral-200 rounded" />
        </div>
      </div>

      {/* Comments skeleton */}
      <div className="bg-white border border-neutral-200 rounded-xl overflow-hidden">
        <div className="px-6 py-4 border-b border-neutral-200 bg-neutral-50">
          <div className="h-6 w-36 bg-neutral-200 rounded" />
        </div>
        <div className="p-6 space-y-6">
          {Array.from({ length: 2 }).map((_, i) => (
            <div key={i} className="space-y-3">
              <div className="flex items-center gap-3">
                <div className="w-8 h-8 bg-neutral-200 rounded-full" />
                <div className="h-4 w-32 bg-neutral-200 rounded" />
              </div>
              <div className="pl-11 space-y-2">
                <div className="h-4 w-full bg-neutral-200 rounded" />
                <div className="h-4 w-2/3 bg-neutral-200 rounded" />
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

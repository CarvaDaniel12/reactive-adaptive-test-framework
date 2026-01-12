/**
 * Skeleton component for Integration Health Card loading state.
 */
export function IntegrationHealthSkeleton() {
  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      {[1, 2, 3, 4].map((i) => (
        <div
          key={i}
          className="w-full p-4 bg-white border border-neutral-200 rounded-lg"
        >
          <div className="flex items-center justify-between mb-3">
            <div className="w-24 h-4 bg-neutral-100 rounded animate-pulse" />
            <div className="w-6 h-6 bg-neutral-100 rounded-full animate-pulse" />
          </div>
          <div className="space-y-1.5">
            <div className="w-32 h-3 bg-neutral-100 rounded animate-pulse" />
            <div className="w-28 h-3 bg-neutral-100 rounded animate-pulse" />
            <div className="w-36 h-3 bg-neutral-100 rounded animate-pulse" />
          </div>
        </div>
      ))}
    </div>
  );
}

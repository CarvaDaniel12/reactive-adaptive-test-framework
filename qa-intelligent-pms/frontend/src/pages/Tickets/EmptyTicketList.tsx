interface EmptyTicketListProps {
  onClearFilters?: () => void;
}

export function EmptyTicketList({ onClearFilters }: EmptyTicketListProps) {
  return (
    <div className="text-center py-12">
      {/* Icon */}
      <div className="inline-flex items-center justify-center w-16 h-16 rounded-full bg-neutral-100 mb-4">
        <svg
          className="w-8 h-8 text-neutral-400"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={1.5}
            d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
          />
        </svg>
      </div>

      {/* Text */}
      <h3 className="text-lg font-medium text-neutral-700 mb-2">
        No tickets found
      </h3>
      <p className="text-neutral-500 max-w-md mx-auto mb-6">
        There are no tickets matching your current filters. Try adjusting your
        filter settings or check back later for new tickets.
      </p>

      {/* Actions */}
      {onClearFilters && (
        <button
          onClick={onClearFilters}
          className="px-4 py-2 text-sm font-medium text-primary-600 bg-primary-50 
                     rounded-lg hover:bg-primary-100 transition-colors"
        >
          Clear Filters
        </button>
      )}
    </div>
  );
}

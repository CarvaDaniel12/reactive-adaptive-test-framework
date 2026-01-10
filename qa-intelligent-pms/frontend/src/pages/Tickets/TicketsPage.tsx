import { useEffect, useState, useCallback } from "react";
import { useSearchParams } from "react-router-dom";
import { TicketCard } from "./TicketCard";
import { TicketCardSkeleton } from "./TicketCardSkeleton";
import { TicketFilters } from "./TicketFilters";
import { EmptyTicketList } from "./EmptyTicketList";
import type { TicketListResponse } from "./types";

/** Fetch tickets from API */
async function fetchTickets(params: {
  status?: string;
  sprint?: string;
  page: number;
}): Promise<TicketListResponse> {
  const url = new URL("/api/v1/tickets", window.location.origin);
  if (params.status) url.searchParams.set("status", params.status);
  if (params.sprint) url.searchParams.set("sprint", params.sprint);
  url.searchParams.set("page", String(params.page));

  const res = await fetch(url);
  if (!res.ok) {
    const error = await res.json().catch(() => ({ error: "Unknown error" }));
    throw new Error(error.error || `Failed to fetch tickets: ${res.status}`);
  }
  return res.json();
}

export function TicketsPage() {
  const [searchParams, setSearchParams] = useSearchParams();
  const [data, setData] = useState<TicketListResponse | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const status = searchParams.get("status") || undefined;
  const sprint = searchParams.get("sprint") || undefined;
  const page = parseInt(searchParams.get("page") || "1", 10);

  const loadTickets = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const result = await fetchTickets({ status, sprint, page });
      setData(result);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load tickets");
    } finally {
      setIsLoading(false);
    }
  }, [status, sprint, page]);

  useEffect(() => {
    loadTickets();
  }, [loadTickets]);

  const handleStatusChange = (newStatus: string | undefined) => {
    const params = new URLSearchParams();
    if (newStatus) params.set("status", newStatus);
    if (sprint) params.set("sprint", sprint);
    params.set("page", "1");
    setSearchParams(params);
  };

  const handleSprintChange = (newSprint: string | undefined) => {
    const params = new URLSearchParams();
    if (status) params.set("status", status);
    if (newSprint) params.set("sprint", newSprint);
    params.set("page", "1");
    setSearchParams(params);
  };

  const handlePageChange = (newPage: number) => {
    const params = new URLSearchParams(searchParams);
    params.set("page", String(newPage));
    setSearchParams(params);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-semibold text-neutral-900">My Tickets</h1>
          {data && !isLoading && (
            <p className="text-sm text-neutral-500 mt-1">
              {data.total} ticket{data.total !== 1 ? "s" : ""} found
              {data.loadTimeMs && (
                <span className="ml-2 text-neutral-400">
                  ({data.loadTimeMs}ms)
                </span>
              )}
            </p>
          )}
        </div>
        <TicketFilters
          currentStatus={status}
          currentSprint={sprint}
          onStatusChange={handleStatusChange}
          onSprintChange={handleSprintChange}
        />
      </div>

      {/* Content */}
      {isLoading ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {Array.from({ length: 6 }).map((_, i) => (
            <TicketCardSkeleton key={i} />
          ))}
        </div>
      ) : error ? (
        <div className="text-center py-12">
          <div className="inline-flex items-center justify-center w-12 h-12 rounded-full bg-error-100 mb-4">
            <svg
              className="w-6 h-6 text-error-600"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
              />
            </svg>
          </div>
          <h3 className="text-lg font-medium text-neutral-900 mb-2">
            Failed to load tickets
          </h3>
          <p className="text-neutral-500 mb-4">{error}</p>
          <button
            onClick={loadTickets}
            className="px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors"
          >
            Try Again
          </button>
        </div>
      ) : data?.tickets.length === 0 ? (
        <EmptyTicketList onClearFilters={() => handleStatusChange(undefined)} />
      ) : (
        <>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {data?.tickets.map((ticket) => (
              <TicketCard key={ticket.key} ticket={ticket} />
            ))}
          </div>

          {/* Pagination */}
          {data && (data.hasMore || page > 1) && (
            <div className="flex items-center justify-center gap-4 pt-4">
              <button
                disabled={page === 1}
                onClick={() => handlePageChange(page - 1)}
                className="px-4 py-2 text-sm font-medium border border-neutral-300 rounded-lg 
                         hover:bg-neutral-50 disabled:opacity-50 disabled:cursor-not-allowed 
                         transition-colors"
              >
                Previous
              </button>
              <span className="text-sm text-neutral-600">
                Page {page}
                {data.total > 0 && ` of ${Math.ceil(data.total / data.pageSize)}`}
              </span>
              <button
                disabled={!data.hasMore}
                onClick={() => handlePageChange(page + 1)}
                className="px-4 py-2 text-sm font-medium border border-neutral-300 rounded-lg 
                         hover:bg-neutral-50 disabled:opacity-50 disabled:cursor-not-allowed 
                         transition-colors"
              >
                Next
              </button>
            </div>
          )}
        </>
      )}
    </div>
  );
}

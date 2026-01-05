/**
 * Integration status panel component.
 *
 * Container for all integration status cards with:
 * - Grid layout for cards
 * - Polling every 60 seconds
 * - Manual refresh button
 * - Loading and error states
 */
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import {
  IntegrationStatusCard,
  type IntegrationHealth,
} from "./IntegrationStatusCard";

const API_BASE = import.meta.env.VITE_API_URL || "";

export function IntegrationStatusPanel() {
  const queryClient = useQueryClient();

  const {
    data: integrations,
    isLoading,
    error,
    isFetching,
  } = useQuery({
    queryKey: ["integrationHealth"],
    queryFn: fetchIntegrationHealth,
    refetchInterval: 60000, // Poll every 60 seconds
    staleTime: 30000, // Consider data stale after 30s
  });

  const refreshMutation = useMutation({
    mutationFn: triggerHealthCheck,
    onSuccess: () => {
      // Invalidate and refetch after manual refresh
      queryClient.invalidateQueries({ queryKey: ["integrationHealth"] });
    },
  });

  if (isLoading) {
    return <IntegrationStatusSkeleton />;
  }

  if (error) {
    return (
      <div className="p-6 bg-red-50 border border-red-200 rounded-xl text-center">
        <p className="text-red-700 font-medium">
          Failed to load integration status
        </p>
        <p className="text-red-600 text-sm mt-1">
          {error instanceof Error ? error.message : "Unknown error"}
        </p>
        <button
          type="button"
          onClick={() =>
            queryClient.invalidateQueries({ queryKey: ["integrationHealth"] })
          }
          className="mt-3 px-4 py-2 bg-red-100 text-red-700 rounded-lg hover:bg-red-200 transition-colors text-sm font-medium"
        >
          Try Again
        </button>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <h3 className="text-lg font-semibold text-neutral-900">
            Integration Status
          </h3>
          {isFetching && !isLoading && (
            <span className="flex items-center gap-1.5 text-xs text-neutral-500">
              <span className="w-2 h-2 bg-primary-500 rounded-full animate-pulse" />
              Updating...
            </span>
          )}
        </div>
        <button
          type="button"
          onClick={() => refreshMutation.mutate()}
          disabled={refreshMutation.isPending}
          className="flex items-center gap-2 px-3 py-1.5 text-sm text-neutral-600
                     hover:text-neutral-900 hover:bg-neutral-100 rounded-lg transition-colors
                     disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <RefreshIcon
            className={`w-4 h-4 ${refreshMutation.isPending ? "animate-spin" : ""}`}
          />
          Refresh
        </button>
      </div>

      {integrations && integrations.length > 0 ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {integrations.map((integration: IntegrationHealth) => (
            <IntegrationStatusCard
              key={integration.integration}
              integration={integration}
            />
          ))}
        </div>
      ) : (
        <div className="text-center py-12 bg-neutral-50 rounded-xl border border-neutral-200">
          <div className="text-4xl mb-3">🔌</div>
          <p className="text-neutral-600 font-medium">
            No integrations configured
          </p>
          <p className="text-neutral-500 text-sm mt-1">
            Set up your integrations in the Setup Wizard
          </p>
        </div>
      )}
    </div>
  );
}

function IntegrationStatusSkeleton() {
  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div className="h-7 w-40 bg-neutral-200 rounded animate-pulse" />
        <div className="h-8 w-20 bg-neutral-200 rounded animate-pulse" />
      </div>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {[1, 2, 3].map((i) => (
          <div
            key={i}
            className="p-4 bg-white border border-neutral-200 rounded-xl animate-pulse"
          >
            <div className="flex items-start justify-between">
              <div className="flex items-center gap-3">
                <div className="w-8 h-8 bg-neutral-200 rounded" />
                <div>
                  <div className="h-5 w-20 bg-neutral-200 rounded mb-2" />
                  <div className="h-4 w-16 bg-neutral-200 rounded" />
                </div>
              </div>
              <div className="h-5 w-12 bg-neutral-200 rounded" />
            </div>
            <div className="h-3 w-24 bg-neutral-200 rounded mt-3" />
          </div>
        ))}
      </div>
    </div>
  );
}

async function fetchIntegrationHealth(): Promise<IntegrationHealth[]> {
  const res = await fetch(`${API_BASE}/api/v1/health/integrations`);
  if (!res.ok) {
    throw new Error(`Failed to fetch: ${res.status} ${res.statusText}`);
  }
  return res.json();
}

async function triggerHealthCheck(): Promise<void> {
  const res = await fetch(`${API_BASE}/api/v1/health/integrations/refresh`, {
    method: "POST",
  });
  if (!res.ok) {
    throw new Error(`Failed to refresh: ${res.status} ${res.statusText}`);
  }
}

function RefreshIcon({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={2}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99"
      />
    </svg>
  );
}

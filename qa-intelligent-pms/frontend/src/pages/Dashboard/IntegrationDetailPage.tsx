/**
 * Integration Detail Page.
 * Displays detailed integration health metrics and timeline.
 */
import { useParams, useNavigate, Link } from "react-router-dom";
import { useIntegrationHealthDetail } from "@/hooks/useIntegrationHealth";
import { IntegrationMetrics } from "./IntegrationMetrics";
import { IntegrationTimeline } from "./IntegrationTimeline";

const getIntegrationName = (id: string): string => {
  const names: Record<string, string> = {
    "booking-com": "Booking.com",
    "airbnb": "Airbnb",
    "vrbo": "Vrbo",
    "hmbn": "HMBN",
  };
  return names[id] || id;
};

export function IntegrationDetailPage() {
  const { integrationId } = useParams<{ integrationId: string }>();
  const navigate = useNavigate();
  const { data, isLoading, error } = useIntegrationHealthDetail(integrationId);

  if (isLoading) {
    return (
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <Link
              to="/"
              className="text-sm text-indigo-600 hover:text-indigo-700 hover:underline mb-2 inline-block"
            >
              ← Back to Dashboard
            </Link>
            <h1 className="text-2xl font-bold text-neutral-900">Loading...</h1>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          {[1, 2, 3].map((i) => (
            <div key={i} className="p-4 bg-white border border-neutral-200 rounded-lg animate-pulse">
              <div className="h-4 bg-neutral-100 rounded mb-2" />
              <div className="h-8 bg-neutral-100 rounded" />
            </div>
          ))}
        </div>
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <Link
              to="/"
              className="text-sm text-indigo-600 hover:text-indigo-700 hover:underline mb-2 inline-block"
            >
              ← Back to Dashboard
            </Link>
            <h1 className="text-2xl font-bold text-neutral-900">Integration Health</h1>
          </div>
        </div>
        <div className="p-4 bg-red-50 border border-red-200 rounded-lg">
          <p className="text-sm text-red-600">
            {error instanceof Error ? error.message : "Failed to load integration health"}
          </p>
        </div>
      </div>
    );
  }

  const integrationName = integrationId ? getIntegrationName(integrationId) : "Unknown";

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <Link
            to="/"
            className="text-sm text-indigo-600 hover:text-indigo-700 hover:underline mb-2 inline-block"
          >
            ← Back to Dashboard
          </Link>
          <h1 className="text-2xl font-bold text-neutral-900">{integrationName} Integration Health</h1>
          <p className="text-sm text-neutral-500">Detailed metrics and event timeline</p>
        </div>
      </div>

      {/* Metrics */}
      <IntegrationMetrics health={data.health} />

      {/* Timeline */}
      <IntegrationTimeline events={data.events} />
    </div>
  );
}

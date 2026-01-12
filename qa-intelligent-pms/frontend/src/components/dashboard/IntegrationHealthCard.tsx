/**
 * Integration Health Card component.
 * Displays integration status with indicators and sub-statuses.
 */
import type { IntegrationHealth, HealthStatus } from "@/hooks/useIntegrationHealth";

interface IntegrationHealthCardProps {
  integration: IntegrationHealth;
  onClick: () => void;
}

export function IntegrationHealthCard({ integration, onClick }: IntegrationHealthCardProps) {
  const statusConfig: Record<HealthStatus, { icon: string; color: string; label: string }> = {
    healthy: { icon: "🟢", color: "text-emerald-500", label: "Healthy" },
    warning: { icon: "🟡", color: "text-yellow-500", label: "Warning" },
    critical: { icon: "🔴", color: "text-red-500", label: "Critical" },
  };

  const status = statusConfig[integration.status] || statusConfig.healthy;

  const getIntegrationName = (id: string): string => {
    const names: Record<string, string> = {
      "booking-com": "Booking.com",
      "airbnb": "Airbnb",
      "vrbo": "Vrbo",
      "hmbn": "HMBN",
    };
    return names[id] || id;
  };

  const getStatusIcon = (status?: HealthStatus): string => {
    if (!status) return "";
    return status === "healthy" ? "✅" : status === "warning" ? "⚠️" : "❌";
  };

  return (
    <button
      onClick={onClick}
      className="w-full p-4 bg-white border border-neutral-200 rounded-lg hover:shadow-md hover:border-indigo-300 transition-shadow text-left"
      aria-label={`${getIntegrationName(integration.integrationId)} integration status: ${status.label}`}
    >
      <div className="flex items-center justify-between mb-3">
        <h3 className="text-sm font-medium text-neutral-900">
          {getIntegrationName(integration.integrationId)}
        </h3>
        <span className="text-lg" role="img" aria-label={status.label}>
          {status.icon}
        </span>
      </div>

      <div className="space-y-1.5">
        {integration.pricingSyncStatus && (
          <div className="flex items-center gap-2 text-xs text-neutral-600">
            <span>Pricing Sync:</span>
            <span className={status.color}>
              {getStatusIcon(integration.pricingSyncStatus)}
            </span>
          </div>
        )}

        {integration.feesSyncStatus && (
          <div className="flex items-center gap-2 text-xs text-neutral-600">
            <span>Fees Sync:</span>
            <span className={status.color}>
              {getStatusIcon(integration.feesSyncStatus)}
            </span>
          </div>
        )}

        {integration.bookingLossRate !== undefined && integration.bookingLossRate !== null && (
          <div className="flex items-center gap-2 text-xs text-neutral-600">
            <span>Booking Loss:</span>
            <span className={status.color}>
              {(integration.bookingLossRate * 100).toFixed(1)}%
            </span>
          </div>
        )}

        {integration.errorRate !== undefined && integration.errorRate !== null && (
          <div className="flex items-center gap-2 text-xs text-neutral-600">
            <span>Error Rate:</span>
            <span className={status.color}>
              {(integration.errorRate * 100).toFixed(1)}%
            </span>
          </div>
        )}
      </div>
    </button>
  );
}

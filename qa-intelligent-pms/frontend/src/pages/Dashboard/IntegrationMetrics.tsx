/**
 * Integration Metrics component.
 * Displays detailed metrics for an integration.
 */
import type { IntegrationHealth, HealthStatus } from "@/hooks/useIntegrationHealth";

interface IntegrationMetricsProps {
  health: IntegrationHealth;
}

export function IntegrationMetrics({ health }: IntegrationMetricsProps) {
  const getStatusIcon = (status?: HealthStatus): string => {
    if (!status) return "⚪";
    return status === "healthy" ? "🟢" : status === "warning" ? "🟡" : "🔴";
  };

  const getStatusColor = (status?: HealthStatus): string => {
    if (!status) return "text-neutral-500";
    return status === "healthy"
      ? "text-emerald-500"
      : status === "warning"
        ? "text-yellow-500"
        : "text-red-500";
  };

  const getStatusLabel = (status?: HealthStatus): string => {
    if (!status) return "N/A";
    return status === "healthy" ? "Healthy" : status === "warning" ? "Warning" : "Critical";
  };

  return (
    <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
      {/* Pricing Sync Status */}
      <div className="p-4 bg-white border border-neutral-200 rounded-lg">
        <h3 className="text-sm font-medium text-neutral-500 mb-2">Pricing Sync</h3>
        <div className="flex items-center gap-2">
          <span className="text-lg">{getStatusIcon(health.pricingSyncStatus)}</span>
          <span className={`text-lg font-semibold ${getStatusColor(health.pricingSyncStatus)}`}>
            {getStatusLabel(health.pricingSyncStatus)}
          </span>
        </div>
      </div>

      {/* Fees Sync Status */}
      <div className="p-4 bg-white border border-neutral-200 rounded-lg">
        <h3 className="text-sm font-medium text-neutral-500 mb-2">Fees Sync</h3>
        <div className="flex items-center gap-2">
          <span className="text-lg">{getStatusIcon(health.feesSyncStatus)}</span>
          <span className={`text-lg font-semibold ${getStatusColor(health.feesSyncStatus)}`}>
            {getStatusLabel(health.feesSyncStatus)}
          </span>
        </div>
      </div>

      {/* Booking Loss Rate */}
      <div className="p-4 bg-white border border-neutral-200 rounded-lg">
        <h3 className="text-sm font-medium text-neutral-500 mb-2">Booking Loss Rate</h3>
        <div className="flex items-center gap-2">
          <span className="text-2xl font-semibold text-neutral-900">
            {health.bookingLossRate !== undefined && health.bookingLossRate !== null
              ? `${(health.bookingLossRate * 100).toFixed(2)}%`
              : "N/A"}
          </span>
        </div>
        {health.errorRate !== undefined && health.errorRate !== null && (
          <div className="mt-2 pt-2 border-t border-neutral-100">
            <p className="text-xs text-neutral-500">Error Rate</p>
            <p className="text-sm font-medium text-neutral-900">
              {(health.errorRate * 100).toFixed(2)}%
            </p>
          </div>
        )}
      </div>
    </div>
  );
}

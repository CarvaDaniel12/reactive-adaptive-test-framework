/**
 * Integration Health Widget component.
 * Displays status cards for all integrations.
 */
import { useNavigate } from "react-router-dom";
import { useIntegrationHealth } from "@/hooks/useIntegrationHealth";
import { IntegrationHealthCard } from "./IntegrationHealthCard";
import { IntegrationHealthSkeleton } from "./IntegrationHealthSkeleton";

export function IntegrationHealthWidget() {
  const navigate = useNavigate();
  const { data, isLoading, error } = useIntegrationHealth();

  if (isLoading) {
    return (
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <h2 className="text-lg font-semibold text-neutral-900">Integration Health</h2>
        </div>
        <IntegrationHealthSkeleton />
      </div>
    );
  }

  if (error) {
    return (
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <h2 className="text-lg font-semibold text-neutral-900">Integration Health</h2>
        </div>
        <div className="p-4 bg-red-50 border border-red-200 rounded-lg">
          <p className="text-sm text-red-600">Failed to load integration health</p>
        </div>
      </div>
    );
  }

  const integrations = data?.integrations || [];

  if (integrations.length === 0) {
    return (
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <h2 className="text-lg font-semibold text-neutral-900">Integration Health</h2>
        </div>
        <div className="p-4 bg-neutral-50 border border-neutral-200 rounded-lg">
          <p className="text-sm text-neutral-600">No integration health data available</p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-semibold text-neutral-900">Integration Health</h2>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {integrations.map((integration) => (
          <IntegrationHealthCard
            key={integration.integrationId}
            integration={integration}
            onClick={() => navigate(`/dashboard/integrations/${integration.integrationId}`)}
          />
        ))}
      </div>
    </div>
  );
}

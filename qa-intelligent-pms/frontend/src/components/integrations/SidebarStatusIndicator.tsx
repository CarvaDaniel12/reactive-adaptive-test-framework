/**
 * Compact status indicator for the sidebar.
 *
 * Shows overall integration health at a glance with:
 * - Single dot representing worst status
 * - Text summary (e.g., "2 offline", "All online")
 * - Tooltip with per-integration breakdown
 */
import { useQuery } from "@tanstack/react-query";
import * as Tooltip from "@radix-ui/react-tooltip";
import { StatusIndicator, type HealthStatus } from "./StatusIndicator";
import type { IntegrationHealth } from "./IntegrationStatusCard";

const API_BASE = import.meta.env.VITE_API_URL || "";

interface SidebarStatusIndicatorProps {
  collapsed?: boolean;
}

export function SidebarStatusIndicator({
  collapsed = false,
}: SidebarStatusIndicatorProps) {
  const { data: integrations } = useQuery({
    queryKey: ["integrationHealth"],
    queryFn: fetchIntegrationHealth,
    refetchInterval: 60000,
    staleTime: 30000,
  });

  if (!integrations || integrations.length === 0) {
    return null;
  }

  const offlineCount = integrations.filter(
    (i: IntegrationHealth) => i.status === "offline"
  ).length;
  const degradedCount = integrations.filter(
    (i: IntegrationHealth) => i.status === "degraded"
  ).length;
  const onlineCount = integrations.filter(
    (i: IntegrationHealth) => i.status === "online"
  ).length;

  // Determine overall status (worst wins)
  let overallStatus: HealthStatus = "online";
  if (offlineCount > 0) overallStatus = "offline";
  else if (degradedCount > 0) overallStatus = "degraded";

  const statusText =
    offlineCount > 0
      ? `${offlineCount} offline`
      : degradedCount > 0
        ? `${degradedCount} degraded`
        : "All online";

  const content = (
    <div
      className={`
        flex items-center gap-2 px-3 py-2 cursor-default
        ${collapsed ? "justify-center" : ""}
      `}
    >
      <StatusIndicator status={overallStatus} size="sm" showPulse={false} />
      {!collapsed && (
        <span className="text-xs text-neutral-500">{statusText}</span>
      )}
    </div>
  );

  return (
    <Tooltip.Provider delayDuration={300}>
      <Tooltip.Root>
        <Tooltip.Trigger asChild>{content}</Tooltip.Trigger>
        <Tooltip.Portal>
          <Tooltip.Content
            side="right"
            className="bg-neutral-900 text-white text-xs px-3 py-2 rounded-lg shadow-lg max-w-[200px] z-50"
            sideOffset={8}
          >
            <p className="font-medium mb-2 text-neutral-300">
              Integration Status
            </p>
            <div className="space-y-1.5">
              {integrations.map((i: IntegrationHealth) => (
                <div key={i.integration} className="flex items-center gap-2">
                  <StatusIndicator
                    status={i.status}
                    size="sm"
                    showPulse={false}
                  />
                  <span className="capitalize">{i.integration}</span>
                  {i.responseTimeMs !== null && i.status === "online" && (
                    <span className="text-neutral-400 ml-auto">
                      {i.responseTimeMs}ms
                    </span>
                  )}
                </div>
              ))}
            </div>
            <div className="mt-2 pt-2 border-t border-neutral-700 text-neutral-400">
              {onlineCount}/{integrations.length} online
            </div>
            <Tooltip.Arrow className="fill-neutral-900" />
          </Tooltip.Content>
        </Tooltip.Portal>
      </Tooltip.Root>
    </Tooltip.Provider>
  );
}

async function fetchIntegrationHealth(): Promise<IntegrationHealth[]> {
  const res = await fetch(`${API_BASE}/api/v1/health/integrations`);
  if (!res.ok) {
    throw new Error(`Failed to fetch: ${res.status} ${res.statusText}`);
  }
  return res.json();
}

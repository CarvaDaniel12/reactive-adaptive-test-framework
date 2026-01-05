/**
 * Integration status card component.
 *
 * Displays a single integration's health status with:
 * - Status indicator (colored dot)
 * - Integration name
 * - Response time (when online)
 * - Last check timestamp
 * - Error message (when offline)
 * - Clickable to open detail modal
 */
import { useState } from "react";
import { formatDistanceToNow } from "date-fns";
import * as Dialog from "@radix-ui/react-dialog";
import { StatusIndicator, type HealthStatus } from "./StatusIndicator";

export interface IntegrationHealth {
  integration: string;
  status: HealthStatus;
  lastSuccessfulCheck: string | null;
  lastCheck: string;
  responseTimeMs: number | null;
  errorMessage: string | null;
  consecutiveFailures: number;
  downtimeStart: string | null;
}

interface IntegrationStatusCardProps {
  integration: IntegrationHealth;
}

const INTEGRATION_LABELS: Record<string, string> = {
  jira: "Jira",
  postman: "Postman",
  testmo: "Testmo",
  splunk: "Splunk",
};

const INTEGRATION_ICONS: Record<string, string> = {
  jira: "🎫",
  postman: "📮",
  testmo: "🧪",
  splunk: "📊",
};

export function IntegrationStatusCard({
  integration,
}: IntegrationStatusCardProps) {
  const [isDetailOpen, setIsDetailOpen] = useState(false);

  const label =
    INTEGRATION_LABELS[integration.integration] || integration.integration;
  const icon = INTEGRATION_ICONS[integration.integration] || "🔌";

  return (
    <>
      <button
        type="button"
        onClick={() => setIsDetailOpen(true)}
        className="w-full p-4 bg-white border border-neutral-200 rounded-xl
                   hover:border-primary-300 hover:shadow-md transition-all text-left
                   focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2"
      >
        <div className="flex items-start justify-between">
          <div className="flex items-center gap-3">
            <span className="text-2xl" role="img" aria-hidden="true">
              {icon}
            </span>
            <div>
              <h4 className="font-semibold text-neutral-900">{label}</h4>
              <div className="flex items-center gap-2 mt-1">
                <StatusIndicator status={integration.status} size="sm" />
                <span className="text-sm text-neutral-600 capitalize">
                  {integration.status}
                </span>
              </div>
            </div>
          </div>

          {integration.responseTimeMs !== null &&
            integration.status !== "offline" && (
              <span className="text-sm font-mono text-neutral-500 bg-neutral-100 px-2 py-0.5 rounded">
                {integration.responseTimeMs}ms
              </span>
            )}
        </div>

        {/* Last check */}
        <p className="text-xs text-neutral-400 mt-3">
          Checked{" "}
          {formatDistanceToNow(new Date(integration.lastCheck), {
            addSuffix: true,
          })}
        </p>

        {/* Error message */}
        {integration.errorMessage && integration.status === "offline" && (
          <div className="mt-3 p-2 bg-red-50 border border-red-200 rounded-lg text-xs text-red-700">
            {integration.errorMessage}
          </div>
        )}
      </button>

      {/* Detail Modal */}
      <Dialog.Root open={isDetailOpen} onOpenChange={setIsDetailOpen}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/50 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-white rounded-xl shadow-2xl p-6 w-full max-w-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95">
            <div className="flex items-center justify-between mb-4">
              <Dialog.Title className="text-lg font-semibold flex items-center gap-2">
                <span role="img" aria-hidden="true">
                  {icon}
                </span>
                {label} Integration
              </Dialog.Title>
              <Dialog.Close asChild>
                <button
                  type="button"
                  className="text-neutral-400 hover:text-neutral-600 p-1 rounded-lg hover:bg-neutral-100 transition-colors"
                  aria-label="Close"
                >
                  <CloseIcon className="w-5 h-5" />
                </button>
              </Dialog.Close>
            </div>

            <div className="space-y-4">
              {/* Status */}
              <div className="flex items-center gap-3 p-3 bg-neutral-50 rounded-lg">
                <StatusIndicator status={integration.status} size="lg" />
                <div>
                  <span className="font-medium capitalize">
                    {integration.status}
                  </span>
                  {integration.consecutiveFailures > 0 && (
                    <p className="text-sm text-neutral-500">
                      {integration.consecutiveFailures} consecutive failure
                      {integration.consecutiveFailures > 1 ? "s" : ""}
                    </p>
                  )}
                </div>
              </div>

              {/* Details */}
              <dl className="divide-y divide-neutral-100">
                <div className="py-3 flex justify-between">
                  <dt className="text-neutral-500">Last Check</dt>
                  <dd className="text-neutral-900 text-sm">
                    {new Date(integration.lastCheck).toLocaleString()}
                  </dd>
                </div>
                {integration.lastSuccessfulCheck && (
                  <div className="py-3 flex justify-between">
                    <dt className="text-neutral-500">Last Success</dt>
                    <dd className="text-neutral-900 text-sm">
                      {new Date(
                        integration.lastSuccessfulCheck
                      ).toLocaleString()}
                    </dd>
                  </div>
                )}
                {integration.responseTimeMs !== null && (
                  <div className="py-3 flex justify-between">
                    <dt className="text-neutral-500">Response Time</dt>
                    <dd className="text-neutral-900 font-mono text-sm">
                      {integration.responseTimeMs}ms
                    </dd>
                  </div>
                )}
                {integration.downtimeStart && (
                  <div className="py-3 flex justify-between">
                    <dt className="text-neutral-500">Down Since</dt>
                    <dd className="text-red-600 text-sm">
                      {new Date(integration.downtimeStart).toLocaleString()}
                    </dd>
                  </div>
                )}
              </dl>

              {/* Error details */}
              {integration.errorMessage && (
                <div className="p-3 bg-red-50 border border-red-200 rounded-lg">
                  <p className="text-sm font-medium text-red-700 mb-1">Error</p>
                  <p className="text-sm text-red-600">
                    {integration.errorMessage}
                  </p>
                </div>
              )}

              {/* Troubleshooting tips */}
              {integration.status === "offline" && (
                <div className="p-3 bg-amber-50 border border-amber-200 rounded-lg">
                  <p className="text-sm font-medium text-amber-800 mb-2">
                    💡 Troubleshooting
                  </p>
                  <ul className="text-sm text-amber-700 space-y-1 list-disc list-inside">
                    <li>Check your network connection</li>
                    <li>Verify your credentials in Settings</li>
                    <li>Try re-authenticating</li>
                  </ul>
                </div>
              )}
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </>
  );
}

function CloseIcon({ className }: { className?: string }) {
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
        d="M6 18L18 6M6 6l12 12"
      />
    </svg>
  );
}

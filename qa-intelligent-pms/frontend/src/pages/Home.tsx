import { useEffect, useState } from 'react';
import { healthApi } from '../lib/api';
import type { HealthResponse } from '../lib/api';

/**
 * Home page showing health status and quick actions.
 */
export function Home() {
  const [health, setHealth] = useState<HealthResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchHealth = async () => {
      try {
        const data = await healthApi.check();
        setHealth(data);
        setError(null);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to fetch health');
      } finally {
        setLoading(false);
      }
    };

    fetchHealth();
  }, []);

  return (
    <div className="space-y-6">
      {/* Welcome section */}
      <div>
        <h2 className="text-2xl font-bold text-neutral-900">
          Welcome to QA Intelligent PMS
        </h2>
        <p className="mt-1 text-neutral-500">
          Your companion framework for structured QA testing workflows.
        </p>
      </div>

      {/* Health status card */}
      <div className="bg-white rounded-xl border border-neutral-200 p-6">
        <h3 className="text-lg font-semibold text-neutral-900 mb-4">
          System Status
        </h3>

        {loading && (
          <div className="flex items-center gap-2 text-neutral-500">
            <span className="animate-pulse">⏳</span>
            <span>Checking system health...</span>
          </div>
        )}

        {error && (
          <div className="flex items-center gap-2 text-error-600 bg-error-50 rounded-lg p-4">
            <span>❌</span>
            <span>{error}</span>
          </div>
        )}

        {health && (
          <div className="space-y-4">
            <div className="flex items-center gap-4">
              <StatusBadge
                status={health.status === 'healthy' ? 'online' : 'offline'}
                label={`API: ${health.status}`}
              />
              <StatusBadge
                status={health.database.connected ? 'online' : 'offline'}
                label={`Database: ${health.database.connected ? 'Connected' : 'Disconnected'}`}
              />
            </div>
            <div className="text-sm text-neutral-500">
              <p>Version: {health.version}</p>
              {health.database.responseTimeMs && (
                <p>Database response: {health.database.responseTimeMs}ms</p>
              )}
            </div>
          </div>
        )}
      </div>

      {/* Quick actions */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <QuickActionCard
          icon="📋"
          title="Start Workflow"
          description="Begin a new testing workflow for a Jira ticket"
          href="/workflows/new"
        />
        <QuickActionCard
          icon="📊"
          title="View Dashboard"
          description="See your performance metrics and trends"
          href="/dashboard"
        />
        <QuickActionCard
          icon="⚙️"
          title="Configure Integrations"
          description="Set up Jira, Postman, and Testmo connections"
          href="/settings/integrations"
        />
      </div>
    </div>
  );
}

interface StatusBadgeProps {
  status: 'online' | 'offline' | 'degraded';
  label: string;
}

function StatusBadge({ status, label }: StatusBadgeProps) {
  const colors = {
    online: 'bg-success-100 text-success-700',
    offline: 'bg-error-100 text-error-700',
    degraded: 'bg-warning-100 text-warning-700',
  };

  const dots = {
    online: 'bg-success-500',
    offline: 'bg-error-500',
    degraded: 'bg-warning-500',
  };

  return (
    <span
      className={`inline-flex items-center gap-2 px-3 py-1 rounded-full text-sm font-medium ${colors[status]}`}
    >
      <span className={`w-2 h-2 rounded-full ${dots[status]}`} />
      {label}
    </span>
  );
}

interface QuickActionCardProps {
  icon: string;
  title: string;
  description: string;
  href: string;
}

function QuickActionCard({
  icon,
  title,
  description,
  href,
}: QuickActionCardProps) {
  return (
    <a
      href={href}
      className="block bg-white rounded-xl border border-neutral-200 p-6 hover:border-primary-300 hover:shadow-sm transition-all"
    >
      <span className="text-3xl">{icon}</span>
      <h3 className="mt-4 text-lg font-semibold text-neutral-900">{title}</h3>
      <p className="mt-1 text-sm text-neutral-500">{description}</p>
    </a>
  );
}

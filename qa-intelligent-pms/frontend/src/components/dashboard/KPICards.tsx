/**
 * KPI Cards grid component.
 * Displays 4 KPI cards: Tickets Completed, Avg Time, Efficiency, Total Hours.
 * 
 * Stories implemented:
 * - 8.2: Tickets Completed KPI with trend
 * - 8.3: Time Metrics KPIs with color coding for efficiency
 */
import { KPICard, KPICardSkeleton, type Trend } from "./KPICard";

export interface DashboardKPIs {
  ticketsCompleted: { value: number; change: number; trend: Trend };
  avgTimePerTicket: { value: number; change: number; trend: Trend }; // seconds
  efficiency: { value: number; change: number; trend: Trend }; // ratio (actual/estimated)
  totalHours: { value: number; change: number; trend: Trend };
}

interface KPICardsProps {
  data?: DashboardKPIs;
  isLoading: boolean;
}

export function KPICards({ data, isLoading }: KPICardsProps) {
  if (isLoading) {
    return (
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {[1, 2, 3, 4].map((i) => (
          <KPICardSkeleton key={i} />
        ))}
      </div>
    );
  }

  const efficiencyValue = data?.efficiency.value ?? 1;
  const efficiencyColor = getEfficiencyColor(efficiencyValue);

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      {/* Story 8.2: Tickets Completed */}
      <KPICard
        title="Tickets Completed"
        value={data?.ticketsCompleted.value ?? 0}
        change={data?.ticketsCompleted.change ?? 0}
        trend={data?.ticketsCompleted.trend ?? "neutral"}
        icon={<CheckCircleIcon className="w-5 h-5" />}
      />
      
      {/* Story 8.3: Average Time per Ticket */}
      <KPICard
        title="Avg. Time per Ticket"
        value={formatDuration(data?.avgTimePerTicket.value ?? 0)}
        change={data?.avgTimePerTicket.change ?? 0}
        trend={data?.avgTimePerTicket.trend ?? "neutral"}
        icon={<ClockIcon className="w-5 h-5" />}
        invertTrend // Lower is better
      />
      
      {/* Story 8.3: Efficiency with color coding */}
      <KPICard
        title="Efficiency"
        value={`${(efficiencyValue * 100).toFixed(0)}%`}
        change={data?.efficiency.change ?? 0}
        trend={data?.efficiency.trend ?? "neutral"}
        icon={<RocketIcon className="w-5 h-5" />}
        description={getEfficiencyDescription(efficiencyValue)}
        valueColor={efficiencyColor}
      />
      
      {/* Story 8.3: Total Hours */}
      <KPICard
        title="Total Hours"
        value={(data?.totalHours.value ?? 0).toFixed(1)}
        change={data?.totalHours.change ?? 0}
        trend={data?.totalHours.trend ?? "neutral"}
        icon={<TimerIcon className="w-5 h-5" />}
      />
    </div>
  );
}

/**
 * Get color class based on efficiency ratio.
 * Story 8.3: Color coding - 🟢 ≤1.0, 🟡 1.0-1.2, 🔴 >1.2
 * Note: Higher efficiency % is better (100% = on target)
 */
function getEfficiencyColor(ratio: number): string {
  if (ratio >= 1.0) return "text-emerald-600"; // 🟢 On or ahead of schedule
  if (ratio >= 0.8) return "text-amber-600";   // 🟡 Slightly behind
  return "text-red-600";                        // 🔴 Significantly behind
}

function getEfficiencyDescription(ratio: number): string {
  if (ratio >= 1.0) return "On or ahead of schedule";
  if (ratio >= 0.8) return "Slightly behind estimate";
  return "Behind schedule";
}

function formatDuration(seconds: number): string {
  if (seconds < 60) return `${seconds}s`;
  if (seconds < 3600) return `${Math.round(seconds / 60)}m`;
  const hours = Math.floor(seconds / 3600);
  const mins = Math.round((seconds % 3600) / 60);
  return mins > 0 ? `${hours}h ${mins}m` : `${hours}h`;
}

// Icons
function CheckCircleIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
    </svg>
  );
}

function ClockIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z" />
    </svg>
  );
}

function RocketIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M15.59 14.37a6 6 0 01-5.84 7.38v-4.8m5.84-2.58a14.98 14.98 0 006.16-12.12A14.98 14.98 0 009.631 8.41m5.96 5.96a14.926 14.926 0 01-5.841 2.58m-.119-8.54a6 6 0 00-7.381 5.84h4.8m2.581-5.84a14.927 14.927 0 00-2.58 5.84m2.699 2.7c-.103.021-.207.041-.311.06a15.09 15.09 0 01-2.448-2.448 14.9 14.9 0 01.06-.312m-2.24 2.39a4.493 4.493 0 00-1.757 4.306 4.493 4.493 0 004.306-1.758M16.5 9a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0z" />
    </svg>
  );
}

function TimerIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M12 6v6l4 2m6-2a9 9 0 11-18 0 9 9 0 0118 0z" />
    </svg>
  );
}

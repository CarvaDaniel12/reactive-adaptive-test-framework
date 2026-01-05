/**
 * Hook for fetching dashboard data with React Query.
 * Supports period-based filtering and automatic refresh.
 */
import { useQuery } from "@tanstack/react-query";
import type { Period } from "@/components/dashboard";
import type { DashboardKPIs, TrendDataPoint, ActivityItem } from "@/components/dashboard";

export interface DashboardData {
  kpis: DashboardKPIs;
  trend: TrendDataPoint[];
  recentActivity: ActivityItem[];
}

interface DashboardResponse {
  kpis: {
    tickets_completed: { value: number; change: number; trend: string };
    avg_time_per_ticket: { value: number; change: number; trend: string };
    efficiency: { value: number; change: number; trend: string };
    total_hours: { value: number; change: number; trend: string };
  };
  trend: Array<{ date: string; tickets: number; hours: number }>;
  recent_activity: Array<{
    id: string;
    type: string;
    title: string;
    ticket_key?: string;
    timestamp: string;
    duration?: number;
  }>;
}

async function fetchDashboard(period: Period): Promise<DashboardData> {
  const response = await fetch(`/api/v1/dashboard?period=${period}`);

  if (!response.ok) {
    throw new Error("Failed to fetch dashboard data");
  }

  const data: DashboardResponse = await response.json();

  // Transform snake_case to camelCase
  return {
    kpis: {
      ticketsCompleted: {
        value: data.kpis.tickets_completed.value,
        change: data.kpis.tickets_completed.change,
        trend: data.kpis.tickets_completed.trend as "up" | "down" | "neutral",
      },
      avgTimePerTicket: {
        value: data.kpis.avg_time_per_ticket.value,
        change: data.kpis.avg_time_per_ticket.change,
        trend: data.kpis.avg_time_per_ticket.trend as "up" | "down" | "neutral",
      },
      efficiency: {
        value: data.kpis.efficiency.value,
        change: data.kpis.efficiency.change,
        trend: data.kpis.efficiency.trend as "up" | "down" | "neutral",
      },
      totalHours: {
        value: data.kpis.total_hours.value,
        change: data.kpis.total_hours.change,
        trend: data.kpis.total_hours.trend as "up" | "down" | "neutral",
      },
    },
    trend: data.trend,
    recentActivity: data.recent_activity.map((item) => ({
      id: item.id,
      type: item.type as "workflow_completed" | "ticket_closed" | "time_logged",
      title: item.title,
      ticketKey: item.ticket_key,
      timestamp: item.timestamp,
      duration: item.duration,
    })),
  };
}

export function useDashboardData(period: Period) {
  return useQuery({
    queryKey: ["dashboard", period],
    queryFn: () => fetchDashboard(period),
    staleTime: 60_000, // 1 minute
    refetchInterval: 60_000, // Auto-refresh every minute
    refetchIntervalInBackground: false,
  });
}

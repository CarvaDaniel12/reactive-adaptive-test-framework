/**
 * Shared types for dashboard components.
 * Centralized to avoid duplication and circular dependencies.
 */

export type Trend = "up" | "down" | "neutral";

export interface TicketBreakdown {
  ticket_type: string;
  count: number;
  percentage: number;
}

export interface DashboardKPIs {
  ticketsCompleted: { value: number; change: number; trend: Trend };
  ticketsBreakdownByType?: TicketBreakdown[];
  avgTimePerTicket: { value: number; change: number; trend: Trend }; // seconds
  efficiency: { value: number; change: number; trend: Trend }; // ratio (actual/estimated)
  totalHours: { value: number; change: number; trend: Trend };
}

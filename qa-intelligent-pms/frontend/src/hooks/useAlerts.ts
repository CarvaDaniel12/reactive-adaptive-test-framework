/**
 * Hooks for alert management.
 * Story 9.3, 9.4: Alert fetching and management.
 */
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";

export interface Alert {
  id: string;
  patternId: string | null;
  alertType: "time_excess" | "consecutive_problem" | "spike";
  severity: "info" | "warning" | "critical";
  title: string;
  message: string | null;
  affectedTickets: string[];
  suggestedActions: string[];
  isRead: boolean;
  isDismissed: boolean;
  createdAt: string;
}

interface AlertsResponse {
  alerts: Alert[];
  total: number;
}

interface UnreadCountResponse {
  count: number;
}

async function fetchAlerts(): Promise<AlertsResponse> {
  const response = await fetch("/api/v1/alerts");
  if (!response.ok) throw new Error("Failed to fetch alerts");
  return response.json();
}

async function fetchUnreadCount(): Promise<number> {
  const response = await fetch("/api/v1/alerts/count");
  if (!response.ok) throw new Error("Failed to fetch count");
  const data: UnreadCountResponse = await response.json();
  return data.count;
}

async function markAlertRead(id: string): Promise<void> {
  const response = await fetch(`/api/v1/alerts/${id}/read`, { method: "POST" });
  if (!response.ok) throw new Error("Failed to mark alert read");
}

async function dismissAlert(id: string): Promise<void> {
  const response = await fetch(`/api/v1/alerts/${id}/dismiss`, { method: "POST" });
  if (!response.ok) throw new Error("Failed to dismiss alert");
}

/**
 * Hook to fetch all alerts.
 */
export function useAlerts() {
  return useQuery({
    queryKey: ["alerts"],
    queryFn: fetchAlerts,
    staleTime: 30_000,
    refetchInterval: 60_000,
  });
}

/**
 * Hook to get unread alert count for badge.
 */
export function useUnreadAlertCount() {
  return useQuery({
    queryKey: ["alerts", "count"],
    queryFn: fetchUnreadCount,
    staleTime: 30_000,
    refetchInterval: 30_000, // More frequent for badge
  });
}

/**
 * Hook to mark an alert as read.
 */
export function useMarkAlertRead() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: markAlertRead,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["alerts"] });
    },
  });
}

/**
 * Hook to dismiss an alert.
 */
export function useDismissAlert() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: dismissAlert,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["alerts"] });
    },
  });
}

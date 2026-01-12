/**
 * Hook for fetching integration health data with React Query.
 */
import { useQuery } from "@tanstack/react-query";

export type IntegrationId = "booking-com" | "airbnb" | "vrbo" | "hmbn";
export type HealthStatus = "healthy" | "warning" | "critical";

export interface IntegrationHealth {
  integrationId: IntegrationId;
  status: HealthStatus;
  pricingSyncStatus?: HealthStatus;
  feesSyncStatus?: HealthStatus;
  bookingLossRate?: number;
  errorRate?: number;
  lastChecked: string;
  trend: "up" | "down" | "neutral";
}

export interface IntegrationHealthResponse {
  integrations: IntegrationHealth[];
}

async function fetchIntegrationHealth(): Promise<IntegrationHealthResponse> {
  const response = await fetch("/api/v1/integrations/health");

  if (!response.ok) {
    throw new Error("Failed to fetch integration health");
  }

  const data: IntegrationHealthResponse = await response.json();
  return data;
}

export function useIntegrationHealth() {
  return useQuery<IntegrationHealthResponse>({
    queryKey: ["integration-health"],
    queryFn: fetchIntegrationHealth,
    refetchInterval: 60_000, // Refetch every 60 seconds
    staleTime: 60_000, // Consider data stale after 60 seconds
  });
}

export interface IntegrationEvent {
  id: string;
  integrationId: IntegrationId;
  eventType: string;
  severity: string;
  message?: string;
  metadata?: Record<string, unknown>;
  occurredAt: string;
  createdAt: string;
}

export interface IntegrationEventsResponse {
  events: IntegrationEvent[];
}

async function fetchIntegrationHealthById(integrationId: string): Promise<IntegrationHealth> {
  const response = await fetch(`/api/v1/integrations/health/${integrationId}`);

  if (!response.ok) {
    if (response.status === 404) {
      throw new Error(`Integration not found: ${integrationId}`);
    }
    throw new Error("Failed to fetch integration health");
  }

  const data: IntegrationHealth = await response.json();
  return data;
}

async function fetchIntegrationEvents(integrationId: string): Promise<IntegrationEventsResponse> {
  const response = await fetch(`/api/v1/integrations/health/${integrationId}/events`);

  if (!response.ok) {
    if (response.status === 404) {
      throw new Error(`Integration not found: ${integrationId}`);
    }
    throw new Error("Failed to fetch integration events");
  }

  const data: IntegrationEventsResponse = await response.json();
  return data;
}

export interface IntegrationHealthDetailData {
  health: IntegrationHealth;
  events: IntegrationEvent[];
}

export function useIntegrationHealthDetail(integrationId: string | undefined) {
  return useQuery<IntegrationHealthDetailData>({
    queryKey: ["integration-health", integrationId],
    queryFn: async () => {
      if (!integrationId) {
        throw new Error("Integration ID is required");
      }

      const [health, eventsResponse] = await Promise.all([
        fetchIntegrationHealthById(integrationId),
        fetchIntegrationEvents(integrationId),
      ]);

      return {
        health,
        events: eventsResponse.events || [],
      };
    },
    enabled: !!integrationId,
    refetchInterval: 60_000, // Refetch every 60 seconds
    staleTime: 60_000, // Consider data stale after 60 seconds
  });
}

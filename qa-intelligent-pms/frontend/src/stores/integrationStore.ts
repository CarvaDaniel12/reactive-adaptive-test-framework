/**
 * Integration store using Zustand.
 * Manages integration health status.
 */
import { create } from 'zustand';

type IntegrationStatus = 'online' | 'degraded' | 'offline' | 'unknown';

interface IntegrationHealth {
  name: string;
  status: IntegrationStatus;
  lastCheck?: string;
  responseTimeMs?: number;
  error?: string;
}

interface IntegrationState {
  integrations: IntegrationHealth[];
  isLoading: boolean;
  error: string | null;

  // Actions
  setIntegrations: (integrations: IntegrationHealth[]) => void;
  updateIntegration: (name: string, health: Partial<IntegrationHealth>) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}

export const useIntegrationStore = create<IntegrationState>((set, get) => ({
  integrations: [
    { name: 'Jira', status: 'unknown' },
    { name: 'Postman', status: 'unknown' },
    { name: 'Testmo', status: 'unknown' },
    { name: 'Splunk', status: 'unknown' },
  ],
  isLoading: false,
  error: null,

  setIntegrations: (integrations) => set({ integrations, error: null }),

  updateIntegration: (name, health) => {
    const { integrations } = get();
    set({
      integrations: integrations.map((int) =>
        int.name === name ? { ...int, ...health } : int
      ),
    });
  },

  setLoading: (isLoading) => set({ isLoading }),

  setError: (error) => set({ error, isLoading: false }),
}));

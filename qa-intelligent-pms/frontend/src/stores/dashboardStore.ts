/**
 * Dashboard store using Zustand.
 * Manages dashboard metrics and data.
 */
import { create } from 'zustand';

interface DashboardMetrics {
  ticketsCompleted: number;
  ticketsCompletedChange: number;
  averageTimeMinutes: number;
  timeEfficiencyRatio: number;
  totalHours: number;
}

interface DashboardState {
  metrics: DashboardMetrics | null;
  period: '7d' | '30d' | '90d' | 'year';
  isLoading: boolean;
  error: string | null;

  // Actions
  setMetrics: (metrics: DashboardMetrics) => void;
  setPeriod: (period: '7d' | '30d' | '90d' | 'year') => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}

export const useDashboardStore = create<DashboardState>((set) => ({
  metrics: null,
  period: '30d',
  isLoading: false,
  error: null,

  setMetrics: (metrics) => set({ metrics, error: null }),

  setPeriod: (period) => set({ period }),

  setLoading: (isLoading) => set({ isLoading }),

  setError: (error) => set({ error, isLoading: false }),
}));

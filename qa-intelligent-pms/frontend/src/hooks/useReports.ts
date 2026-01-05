/**
 * Hook for fetching report data.
 */
import { useQuery } from '@tanstack/react-query';
import { reportsApi } from '@/lib/api';
import type { ReportResponse, ReportListResponse } from '@/types';

/**
 * Hook to fetch a single report by ID.
 */
export function useReport(reportId: string) {
  return useQuery({
    queryKey: ['report', reportId],
    queryFn: () => reportsApi.get(reportId),
    enabled: !!reportId,
    staleTime: 60_000,
  });
}

/**
 * Hook to fetch list of reports with optional filters.
 */
export function useReports(options: {
  page?: number;
  perPage?: number;
  ticketKey?: string;
  startDate?: string;
  endDate?: string;
  templateName?: string;
} = {}) {
  return useQuery({
    queryKey: ['reports', options],
    queryFn: () => reportsApi.list(options),
    staleTime: 30_000,
  });
}
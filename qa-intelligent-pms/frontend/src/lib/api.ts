/**
 * API client for backend communication.
 */

const API_BASE = '/api/v1';

interface ApiError {
  error: string;
  code: string;
  details?: Record<string, unknown>;
}

class ApiClient {
  private baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> {
    const url = `${this.baseUrl}${endpoint}`;

    const response = await fetch(url, {
      ...options,
      headers: {
        'Content-Type': 'application/json',
        ...options.headers,
      },
    });

    if (!response.ok) {
      const error: ApiError = await response.json().catch(() => ({
        error: 'Unknown error',
        code: 'UNKNOWN_ERROR',
      }));
      throw new Error(error.error);
    }

    return response.json();
  }

  async get<T>(endpoint: string): Promise<T> {
    return this.request<T>(endpoint, { method: 'GET' });
  }

  async post<T>(endpoint: string, data?: unknown): Promise<T> {
    return this.request<T>(endpoint, {
      method: 'POST',
      body: data ? JSON.stringify(data) : undefined,
    });
  }

  async put<T>(endpoint: string, data?: unknown): Promise<T> {
    return this.request<T>(endpoint, {
      method: 'PUT',
      body: data ? JSON.stringify(data) : undefined,
    });
  }

  async delete<T>(endpoint: string): Promise<T> {
    return this.request<T>(endpoint, { method: 'DELETE' });
  }
}

export const api = new ApiClient(API_BASE);

// API types
export interface HealthResponse {
  status: string;
  version: string;
  timestamp: string;
  database: {
    connected: boolean;
    responseTimeMs?: number;
    error?: string;
  };
}

// API functions
export const healthApi = {
  check: () => api.get<HealthResponse>('/health'),
};

// Report API
import type {
  ReportResponse,
  ReportListResponse,
  AIStatusResponse,
  AIConfigResponse,
  ProvidersResponse,
  ConfigureAIRequest,
  ConnectionTestResult,
  SuccessResponse,
} from '@/types';

export const reportsApi = {
  get: (id: string) => api.get<ReportResponse>(`/reports/${id}`),
  list: (params?: {
    page?: number;
    perPage?: number;
    ticketKey?: string;
    startDate?: string;
    endDate?: string;
    templateName?: string;
  }) => {
    const searchParams = new URLSearchParams();
    if (params?.page) searchParams.set('page', params.page.toString());
    if (params?.perPage) searchParams.set('per_page', params.perPage.toString());
    if (params?.ticketKey) searchParams.set('ticket_key', params.ticketKey);
    if (params?.startDate) searchParams.set('start_date', params.startDate);
    if (params?.endDate) searchParams.set('end_date', params.endDate);
    if (params?.templateName) searchParams.set('template_name', params.templateName);
    const query = searchParams.toString();
    return api.get<ReportListResponse>(`/reports${query ? `?${query}` : ''}`);
  },
};

// AI API (Story 13.1)
export const aiApi = {
  getStatus: () => api.get<AIStatusResponse>('/ai/status'),
  getConfig: async (): Promise<AIConfigResponse | null> => {
    try {
      return await api.get<AIConfigResponse>('/ai/config');
    } catch (error) {
      // Check if it's a 404 (not configured) - return null instead of throwing
      if (error instanceof Error && error.message.toLowerCase().includes('not found')) {
        return null;
      }
      throw error;
    }
  },
  getProviders: () => api.get<ProvidersResponse>('/ai/providers'),
  configure: (data: ConfigureAIRequest) => api.post<SuccessResponse>('/ai/configure', data),
  testConnection: (data: ConfigureAIRequest) => api.post<ConnectionTestResult>('/ai/test', data),
  disable: () => api.post<SuccessResponse>('/ai/disable'),
};
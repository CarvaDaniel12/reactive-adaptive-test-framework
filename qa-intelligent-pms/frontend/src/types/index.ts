/**
 * Shared TypeScript types.
 */

// User types
export interface User {
  id: string;
  name: string;
  email: string;
}

// Workflow types
export interface WorkflowStep {
  id: string;
  name: string;
  description: string;
  estimatedMinutes: number;
  isCompleted: boolean;
  notes?: string;
}

export interface Workflow {
  id: string;
  templateId: string;
  ticketId: string;
  currentStepIndex: number;
  steps: WorkflowStep[];
  status: 'pending' | 'in_progress' | 'paused' | 'completed';
  startedAt?: string;
  completedAt?: string;
}

// Integration types
export type IntegrationStatus = 'online' | 'degraded' | 'offline' | 'unknown';

export interface IntegrationHealth {
  name: string;
  status: IntegrationStatus;
  lastCheck?: string;
  responseTimeMs?: number;
  error?: string;
}

// Dashboard types
export interface DashboardMetrics {
  ticketsCompleted: number;
  ticketsCompletedChange: number;
  averageTimeMinutes: number;
  timeEfficiencyRatio: number;
  totalHours: number;
}

// API response types
export interface PageInfo {
  page: number;
  pageSize: number;
  totalItems: number;
  totalPages: number;
}

export interface Paginated<T> {
  data: T[];
  pagination: PageInfo;
}

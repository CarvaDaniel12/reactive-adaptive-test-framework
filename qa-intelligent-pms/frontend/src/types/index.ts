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

// Report types
export interface ReportStep {
  index: number;
  name: string;
  status: string;
  notes?: string | null;
  timeSeconds: number;
}

export interface ReportContent {
  steps: ReportStep[];
  notes: string[];
  testsCovered: string[];
  strategies: string[];
  timeSummary?: {
    totalActiveSeconds: number;
    totalEstimatedSeconds: number;
  };
}

export interface ReportSummary {
  id: string;
  ticketKey: string;
  ticketTitle?: string | null;
  templateName: string;
  totalTimeSeconds?: number | null;
  status: string;
  createdAt: string;
}

export interface ReportResponse {
  id: string;
  workflowInstanceId: string;
  ticketId: string;
  ticketTitle?: string | null;
  templateName: string;
  content: ReportContent;
  totalTimeSeconds: number;
  generatedAt: string;
}

export interface ReportListResponse {
  reports: ReportSummary[];
  total: number;
  page: number;
  perPage: number;
  totalPages: number;
}

// Test Case types (Story 31.1)
export interface TestCase {
  id: string;
  title: string;
  description: string;
  preconditions: string[];
  priority: string;
  type: string;
  steps: string[];
  expectedResult: string;
  automatizable: boolean;
  component: string;
  endpoint?: string | null;
  method?: string | null;
  tags: string[];
  status: string;
}

export interface GenerateTestsRequest {
  ticketKey: string;
  includeRegression?: boolean;
  includeSecurity?: boolean;
  includePerformance?: boolean;
  force?: boolean;
}

export interface GenerateTestsResponse {
  testCases: TestCase[];
  count: number;
  ticketKey: string;
}

// AI Configuration types (Story 13.1)
// Note: Backend uses snake_case serialization, so OpenAi becomes "open_ai"
export type ProviderType = "anthropic" | "open_ai" | "deepseek" | "zai" | "custom";

export interface ModelInfo {
  id: string;
  name: string;
  contextWindow: number;
  supportsStreaming: boolean;
}

export interface ProviderModels {
  provider: ProviderType;
  models: ModelInfo[];
}

export interface AIStatusResponse {
  available: boolean;
  provider: string | null;
  model: string | null;
  message: string;
}

export interface AIConfigResponse {
  enabled: boolean;
  provider: string;
  modelId: string;
  customBaseUrl?: string | null;
  validatedAt?: string | null;
}

export interface ProvidersResponse {
  providers: ProviderModels[];
}

export interface ConfigureAIRequest {
  provider: string;
  apiKey: string;
  modelId: string;
  customBaseUrl?: string | null;
}

export interface ConnectionTestResult {
  success: boolean;
  message?: string | null;
}

export interface SuccessResponse {
  message: string;
}
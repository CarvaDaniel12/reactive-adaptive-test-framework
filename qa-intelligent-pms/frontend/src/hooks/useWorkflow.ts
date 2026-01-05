import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";

// ============================================================================
// Types
// ============================================================================

export interface WorkflowTemplate {
  id: string;
  name: string;
  description: string | null;
  ticketType: string;
  stepCount: number;
  estimatedMinutes: number;
  isDefault: boolean;
}

export interface WorkflowStep {
  index: number;
  name: string;
  description: string;
  estimatedMinutes: number;
}

export interface WorkflowStepWithStatus extends WorkflowStep {
  status: "pending" | "in_progress" | "completed" | "skipped";
  notes: string | null;
}

export interface WorkflowSummary {
  id: string;
  templateName: string;
  status: string;
  currentStep: number;
  totalSteps: number;
  startedAt: string;
}

export interface WorkflowDetail {
  id: string;
  templateId: string;
  templateName: string;
  ticketId: string;
  status: string;
  currentStep: number;
  steps: WorkflowStepWithStatus[];
  estimatedMinutes: number;
  startedAt: string;
}

export interface CreateWorkflowResponse {
  id: string;
  templateName: string;
  currentStep: WorkflowStep;
  totalSteps: number;
}

// ============================================================================
// Hooks
// ============================================================================

/**
 * Fetch available workflow templates.
 */
export function useWorkflowTemplates() {
  return useQuery({
    queryKey: ["workflow-templates"],
    queryFn: async (): Promise<WorkflowTemplate[]> => {
      const response = await fetch("/api/v1/workflows/templates");
      if (!response.ok) {
        throw new Error("Failed to fetch templates");
      }
      const data = await response.json();
      return data.templates;
    },
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

/**
 * Check for active workflow on a ticket.
 */
export function useActiveWorkflow(ticketId: string | undefined) {
  return useQuery({
    queryKey: ["active-workflow", ticketId],
    queryFn: async (): Promise<{ exists: boolean; workflow: WorkflowSummary | null }> => {
      if (!ticketId) {
        return { exists: false, workflow: null };
      }
      const response = await fetch(`/api/v1/workflows/active/${encodeURIComponent(ticketId)}`);
      if (!response.ok) {
        throw new Error("Failed to check active workflow");
      }
      return response.json();
    },
    enabled: !!ticketId,
  });
}

/**
 * Get workflow details by ID.
 */
export function useWorkflowDetail(workflowId: string | undefined) {
  return useQuery({
    queryKey: ["workflow", workflowId],
    queryFn: async (): Promise<WorkflowDetail> => {
      if (!workflowId) {
        throw new Error("Workflow ID required");
      }
      const response = await fetch(`/api/v1/workflows/${workflowId}`);
      if (!response.ok) {
        throw new Error("Failed to fetch workflow");
      }
      return response.json();
    },
    enabled: !!workflowId,
    refetchInterval: 30000, // Refresh every 30s
  });
}

/**
 * Create a new workflow instance.
 */
export function useCreateWorkflow() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (params: {
      templateId: string;
      ticketId: string;
      ticketTitle: string;
      userId: string;
    }): Promise<CreateWorkflowResponse> => {
      const response = await fetch("/api/v1/workflows", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(params),
      });

      if (!response.ok) {
        const error = await response.json().catch(() => ({ error: "Unknown error" }));
        throw new Error(error.error || "Failed to create workflow");
      }

      return response.json();
    },
    onSuccess: (_, variables) => {
      // Invalidate active workflow query for this ticket
      queryClient.invalidateQueries({ queryKey: ["active-workflow", variables.ticketId] });
    },
  });
}

/**
 * Format minutes to human-readable duration.
 */
export function formatDuration(minutes: number): string {
  if (minutes < 60) {
    return `${minutes}m`;
  }
  const hours = Math.floor(minutes / 60);
  const mins = minutes % 60;
  return mins > 0 ? `${hours}h ${mins}m` : `${hours}h`;
}

// ============================================================================
// Step Action Types
// ============================================================================

export interface StepLink {
  label: string;
  url: string;
}

export interface CompleteStepParams {
  workflowId: string;
  stepIndex: number;
  notes?: string;
  links?: StepLink[];
}

export interface SkipStepParams {
  workflowId: string;
  stepIndex: number;
}

export interface StepActionResponse {
  workflowCompleted: boolean;
  nextStep: WorkflowStep | null;
  currentStepIndex: number;
}

// ============================================================================
// Step Action Hooks
// ============================================================================

/**
 * Complete a workflow step.
 */
export function useCompleteStep() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      workflowId,
      stepIndex,
      notes,
      links,
    }: CompleteStepParams): Promise<StepActionResponse> => {
      const response = await fetch(
        `/api/v1/workflows/${workflowId}/steps/${stepIndex}/complete`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ notes, links: links || [] }),
        }
      );

      if (!response.ok) {
        const error = await response.json().catch(() => ({ error: "Unknown error" }));
        throw new Error(error.error || "Failed to complete step");
      }

      return response.json();
    },
    onSuccess: (_, variables) => {
      // Invalidate workflow detail query
      queryClient.invalidateQueries({ queryKey: ["workflow", variables.workflowId] });
    },
  });
}

/**
 * Skip a workflow step.
 */
export function useSkipStep() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({ workflowId, stepIndex }: SkipStepParams): Promise<StepActionResponse> => {
      const response = await fetch(
        `/api/v1/workflows/${workflowId}/steps/${stepIndex}/skip`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
        }
      );

      if (!response.ok) {
        const error = await response.json().catch(() => ({ error: "Unknown error" }));
        throw new Error(error.error || "Failed to skip step");
      }

      return response.json();
    },
    onSuccess: (_, variables) => {
      // Invalidate workflow detail query
      queryClient.invalidateQueries({ queryKey: ["workflow", variables.workflowId] });
    },
  });
}

// ============================================================================
// Pause/Resume Hooks
// ============================================================================

export interface WorkflowStatusResponse {
  status: string;
  message: string;
}

/**
 * Pause a workflow.
 */
export function usePauseWorkflow() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (workflowId: string): Promise<WorkflowStatusResponse> => {
      const response = await fetch(`/api/v1/workflows/${workflowId}/pause`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
      });

      if (!response.ok) {
        const error = await response.json().catch(() => ({ error: "Unknown error" }));
        throw new Error(error.error || "Failed to pause workflow");
      }

      return response.json();
    },
    onSuccess: (_, workflowId) => {
      queryClient.invalidateQueries({ queryKey: ["workflow", workflowId] });
    },
  });
}

/**
 * Resume a paused workflow.
 */
export function useResumeWorkflow() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (workflowId: string): Promise<WorkflowStatusResponse> => {
      const response = await fetch(`/api/v1/workflows/${workflowId}/resume`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
      });

      if (!response.ok) {
        const error = await response.json().catch(() => ({ error: "Unknown error" }));
        throw new Error(error.error || "Failed to resume workflow");
      }

      return response.json();
    },
    onSuccess: (_, workflowId) => {
      queryClient.invalidateQueries({ queryKey: ["workflow", workflowId] });
    },
  });
}

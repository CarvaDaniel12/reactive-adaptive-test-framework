/**
 * Workflow store using Zustand.
 * Manages workflow execution state.
 */
import { create } from 'zustand';

interface WorkflowStep {
  id: string;
  name: string;
  description: string;
  estimatedMinutes: number;
  isCompleted: boolean;
  notes?: string;
}

interface Workflow {
  id: string;
  templateId: string;
  ticketId: string;
  currentStepIndex: number;
  steps: WorkflowStep[];
  status: 'pending' | 'in_progress' | 'paused' | 'completed';
  startedAt?: string;
  completedAt?: string;
}

interface WorkflowState {
  currentWorkflow: Workflow | null;
  workflows: Workflow[];
  isLoading: boolean;
  error: string | null;

  // Actions
  setCurrentWorkflow: (workflow: Workflow | null) => void;
  setWorkflows: (workflows: Workflow[]) => void;
  completeStep: (stepId: string, notes?: string) => void;
  pauseWorkflow: () => void;
  resumeWorkflow: () => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}

export const useWorkflowStore = create<WorkflowState>((set, get) => ({
  currentWorkflow: null,
  workflows: [],
  isLoading: false,
  error: null,

  setCurrentWorkflow: (currentWorkflow) => set({ currentWorkflow }),

  setWorkflows: (workflows) => set({ workflows }),

  completeStep: (stepId, notes) => {
    const { currentWorkflow } = get();
    if (!currentWorkflow) return;

    const updatedSteps = currentWorkflow.steps.map((step) =>
      step.id === stepId ? { ...step, isCompleted: true, notes } : step
    );

    const nextStepIndex = updatedSteps.findIndex((s) => !s.isCompleted);
    const isComplete = nextStepIndex === -1;

    set({
      currentWorkflow: {
        ...currentWorkflow,
        steps: updatedSteps,
        currentStepIndex: isComplete
          ? currentWorkflow.steps.length - 1
          : nextStepIndex,
        status: isComplete ? 'completed' : 'in_progress',
        completedAt: isComplete ? new Date().toISOString() : undefined,
      },
    });
  },

  pauseWorkflow: () => {
    const { currentWorkflow } = get();
    if (!currentWorkflow) return;

    set({
      currentWorkflow: {
        ...currentWorkflow,
        status: 'paused',
      },
    });
  },

  resumeWorkflow: () => {
    const { currentWorkflow } = get();
    if (!currentWorkflow) return;

    set({
      currentWorkflow: {
        ...currentWorkflow,
        status: 'in_progress',
      },
    });
  },

  setLoading: (isLoading) => set({ isLoading }),

  setError: (error) => set({ error, isLoading: false }),
}));

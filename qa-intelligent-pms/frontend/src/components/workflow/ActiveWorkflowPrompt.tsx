import { useNavigate } from "react-router-dom";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { useToast } from "@/hooks/useToast";

interface WorkflowSummary {
  id: string;
  templateName: string;
  status: string;
  currentStep: number;
  totalSteps: number;
  startedAt: string;
}

interface UserActiveWorkflowsResponse {
  workflows: WorkflowSummary[];
}

function PlayIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="currentColor">
      <path fillRule="evenodd" d="M4.5 5.653c0-1.427 1.529-2.33 2.779-1.643l11.54 6.347c1.295.712 1.295 2.573 0 3.286L7.28 19.99c-1.25.687-2.779-.217-2.779-1.643V5.653Z" clipRule="evenodd" />
    </svg>
  );
}

function XMarkIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
    </svg>
  );
}

/**
 * Prompt shown when user has active workflows.
 */
export function ActiveWorkflowPrompt() {
  const navigate = useNavigate();
  const { toast } = useToast();
  const queryClient = useQueryClient();

  const { data, isLoading } = useQuery({
    queryKey: ["user-active-workflows"],
    queryFn: async (): Promise<UserActiveWorkflowsResponse> => {
      const response = await fetch("/api/v1/workflows/user/active");
      if (!response.ok) throw new Error("Failed to fetch");
      return response.json();
    },
    staleTime: 60000, // 1 minute
  });

  const cancelMutation = useMutation({
    mutationFn: async (workflowId: string) => {
      const response = await fetch(`/api/v1/workflows/${workflowId}/cancel`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
      });
      if (!response.ok) throw new Error("Failed to cancel");
      return response.json();
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["user-active-workflows"] });
      toast({
        title: "Workflow Abandoned",
        description: "The workflow has been cancelled",
        variant: "info",
      });
    },
  });

  if (isLoading || !data?.workflows?.length) {
    return null;
  }

  const workflow = data.workflows[0]; // Show first active workflow

  return (
    <div className="fixed bottom-4 right-4 z-50 animate-slide-up">
      <div className="bg-white rounded-lg shadow-xl border border-neutral-200 p-4 max-w-sm">
        <div className="flex items-start gap-3">
          <div className="w-10 h-10 bg-amber-100 rounded-full flex items-center justify-center flex-shrink-0">
            <PlayIcon className="w-5 h-5 text-amber-600" />
          </div>
          <div className="flex-1 min-w-0">
            <h4 className="font-semibold text-neutral-900">Active Workflow</h4>
            <p className="text-sm text-neutral-500 mt-0.5">
              {workflow.templateName} - Step {workflow.currentStep + 1}/{workflow.totalSteps}
            </p>
            <div className="flex gap-2 mt-3">
              <button
                onClick={() => navigate(`/workflows/${workflow.id}`)}
                className="flex-1 px-3 py-1.5 bg-blue-500 text-white text-sm font-medium rounded-lg hover:bg-blue-600"
              >
                Resume
              </button>
              <button
                onClick={() => cancelMutation.mutate(workflow.id)}
                disabled={cancelMutation.isPending}
                className="px-3 py-1.5 text-neutral-600 text-sm font-medium rounded-lg border border-neutral-300 hover:bg-neutral-50"
              >
                Abandon
              </button>
            </div>
          </div>
          <button
            onClick={() => cancelMutation.mutate(workflow.id)}
            className="text-neutral-400 hover:text-neutral-600"
          >
            <XMarkIcon className="w-5 h-5" />
          </button>
        </div>
      </div>
    </div>
  );
}

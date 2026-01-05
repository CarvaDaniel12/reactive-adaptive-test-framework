import { useParams, useNavigate } from "react-router-dom";
import {
  useWorkflowDetail,
  useCompleteStep,
  useSkipStep,
  usePauseWorkflow,
  useResumeWorkflow,
} from "@/hooks/useWorkflow";
import { useToast } from "@/hooks/useToast";
import { WorkflowHeader, CurrentStepCard, StepsSidebar } from "@/components/workflow";

/**
 * Main workflow execution page.
 * Shows the current step and allows progression through the workflow.
 */
export function WorkflowPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { toast } = useToast();
  const { data: workflow, isLoading, error, refetch } = useWorkflowDetail(id);
  const completeStep = useCompleteStep();
  const skipStep = useSkipStep();
  const pauseWorkflow = usePauseWorkflow();
  const resumeWorkflow = useResumeWorkflow();

  // Loading state
  if (isLoading) {
    return (
      <div className="min-h-screen bg-neutral-50 flex items-center justify-center">
        <div className="text-center">
          <div className="w-8 h-8 border-4 border-blue-500 border-t-transparent rounded-full animate-spin mx-auto mb-4" />
          <p className="text-neutral-500">Loading workflow...</p>
        </div>
      </div>
    );
  }

  // Error state
  if (error || !workflow) {
    return (
      <div className="min-h-screen bg-neutral-50 flex items-center justify-center">
        <div className="text-center max-w-md">
          <div className="w-16 h-16 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-4">
            <svg className="w-8 h-8 text-red-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
            </svg>
          </div>
          <h2 className="text-xl font-semibold text-neutral-900 mb-2">Workflow Not Found</h2>
          <p className="text-neutral-500 mb-6">
            {error instanceof Error ? error.message : "Unable to load workflow"}
          </p>
          <button
            onClick={() => navigate("/tickets")}
            className="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600"
          >
            Back to Tickets
          </button>
        </div>
      </div>
    );
  }

  const currentStepData = workflow.steps[workflow.currentStep];
  const isLastStep = workflow.currentStep === workflow.steps.length - 1;

  const handleComplete = async (notes: string) => {
    if (!id) return;

    try {
      const result = await completeStep.mutateAsync({
        workflowId: id,
        stepIndex: workflow.currentStep,
        notes: notes || undefined,
      });

      if (result.workflowCompleted) {
        toast({
          title: "Workflow Completed! 🎉",
          description: "All steps have been completed successfully.",
          variant: "success",
        });
        // Navigate back to ticket
        navigate(`/tickets/${workflow.ticketId}`);
      } else {
        toast({
          title: "Step Completed",
          description: result.nextStep ? `Moving to: ${result.nextStep.name}` : "Step completed",
          variant: "success",
        });
        // Refetch to get updated workflow state
        refetch();
      }
    } catch (err) {
      toast({
        title: "Failed to Complete Step",
        description: err instanceof Error ? err.message : "Unknown error",
        variant: "error",
      });
    }
  };

  const handleSkip = async () => {
    if (!id) return;

    try {
      const result = await skipStep.mutateAsync({
        workflowId: id,
        stepIndex: workflow.currentStep,
      });

      if (result.workflowCompleted) {
        toast({
          title: "Workflow Completed",
          description: "All steps have been processed.",
          variant: "success",
        });
        navigate(`/tickets/${workflow.ticketId}`);
      } else {
        toast({
          title: "Step Skipped",
          description: result.nextStep ? `Moving to: ${result.nextStep.name}` : "Step skipped",
          variant: "info",
        });
        refetch();
      }
    } catch (err) {
      toast({
        title: "Failed to Skip Step",
        description: err instanceof Error ? err.message : "Unknown error",
        variant: "error",
      });
    }
  };

  const handlePause = async () => {
    if (!id) return;
    try {
      await pauseWorkflow.mutateAsync(id);
      toast({
        title: "Workflow Paused",
        description: "You can resume anytime from the ticket page",
        variant: "info",
      });
      refetch();
    } catch (err) {
      toast({
        title: "Failed to Pause",
        description: err instanceof Error ? err.message : "Unknown error",
        variant: "error",
      });
    }
  };

  const handleResume = async () => {
    if (!id) return;
    try {
      await resumeWorkflow.mutateAsync(id);
      toast({
        title: "Workflow Resumed",
        description: "Continue where you left off",
        variant: "success",
      });
      refetch();
    } catch (err) {
      toast({
        title: "Failed to Resume",
        description: err instanceof Error ? err.message : "Unknown error",
        variant: "error",
      });
    }
  };

  const handleExit = () => {
    navigate(`/tickets/${workflow.ticketId}`);
  };

  const isProcessing =
    completeStep.isPending ||
    skipStep.isPending ||
    pauseWorkflow.isPending ||
    resumeWorkflow.isPending;

  return (
    <div className="min-h-screen bg-neutral-50">
      {/* Header */}
      <WorkflowHeader
        ticketKey={workflow.ticketId}
        ticketTitle={workflow.ticketId} // TODO: Store ticket title in workflow
        templateName={workflow.templateName}
        currentStep={workflow.currentStep}
        totalSteps={workflow.steps.length}
        estimatedMinutes={workflow.estimatedMinutes}
        status={workflow.status}
        onPause={handlePause}
        onResume={handleResume}
        onExit={handleExit}
      />

      {/* Main Content */}
      <div className="max-w-6xl mx-auto px-6 py-8">
        <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
          {/* Steps Sidebar */}
          <div className="lg:col-span-1">
            <StepsSidebar steps={workflow.steps} currentStep={workflow.currentStep} />
          </div>

          {/* Current Step */}
          <div className="lg:col-span-3">
            {currentStepData && (
              <CurrentStepCard
                step={currentStepData}
                isLast={isLastStep}
                onComplete={handleComplete}
                onSkip={handleSkip}
                loading={isProcessing}
              />
            )}

            {/* Workflow completed state */}
            {workflow.status === "completed" && (
              <div className="bg-green-50 border border-green-200 rounded-xl p-8 text-center">
                <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
                  <svg className="w-8 h-8 text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                  </svg>
                </div>
                <h3 className="text-xl font-semibold text-green-800 mb-2">Workflow Completed!</h3>
                <p className="text-green-600 mb-4">
                  All steps have been completed for this ticket.
                </p>
                <button
                  onClick={() => navigate(`/tickets/${workflow.ticketId}`)}
                  className="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700"
                >
                  Return to Ticket
                </button>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

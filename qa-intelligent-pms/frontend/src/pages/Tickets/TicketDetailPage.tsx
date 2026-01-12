import { useEffect, useState, useCallback } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { MarkdownRenderer } from "@/components/MarkdownRenderer";
import { RelatedTests } from "@/components/search";
import { TemplateSelectionDialog } from "@/components/workflow";
import { TestGenerator, BugRiskScore } from "@/components/ai";
import { useActiveWorkflow, useCreateWorkflow, useToast } from "@/hooks";
import type { WorkflowTemplate } from "@/hooks/useWorkflow";
import { TicketMetadata } from "./TicketMetadata";
import { TicketComments } from "./TicketComments";
import { TicketAttachments } from "./TicketAttachments";
import { TicketDetailSkeleton } from "./TicketDetailSkeleton";
import type { TicketDetail } from "./types";

/** Fetch ticket details from API */
async function fetchTicket(key: string): Promise<TicketDetail> {
  const res = await fetch(`/api/v1/tickets/${encodeURIComponent(key)}`);
  if (!res.ok) {
    if (res.status === 404) {
      throw new Error(`Ticket not found: ${key}`);
    }
    const error = await res.json().catch(() => ({ error: "Unknown error" }));
    throw new Error(error.error || `Failed to fetch ticket: ${res.status}`);
  }
  return res.json();
}

export function TicketDetailPage() {
  const { key } = useParams<{ key: string }>();
  const navigate = useNavigate();
  const { toast } = useToast();
  const [ticket, setTicket] = useState<TicketDetail | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showTemplateDialog, setShowTemplateDialog] = useState(false);

  // Check for active workflow
  const { data: activeWorkflowData } = useActiveWorkflow(key);
  const createWorkflow = useCreateWorkflow();

  const loadTicket = useCallback(async () => {
    if (!key) return;

    setIsLoading(true);
    setError(null);

    try {
      const data = await fetchTicket(key);
      setTicket(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load ticket");
    } finally {
      setIsLoading(false);
    }
  }, [key]);

  useEffect(() => {
    loadTicket();
  }, [loadTicket]);

  // Handle status change from StatusSelector
  const handleStatusChange = useCallback(
    (newStatus: string, newStatusColor: string) => {
      if (ticket) {
        setTicket({
          ...ticket,
          status: newStatus,
          statusColor: newStatusColor,
        });
      }
    },
    [ticket]
  );

  // Handle workflow template selection
  const handleTemplateSelect = useCallback(
    async (template: WorkflowTemplate) => {
      if (!ticket) return;

      try {
        const result = await createWorkflow.mutateAsync({
          templateId: template.id,
          ticketId: ticket.key,
          ticketTitle: ticket.title,
          userId: "current-user@example.com", // TODO: Get from auth context
        });

        setShowTemplateDialog(false);
        toast({
          title: "Workflow Started",
          description: `Started "${result.templateName}" workflow`,
          variant: "success",
        });

        // Navigate to workflow view
        navigate(`/workflows/${result.id}`);
      } catch (err) {
        toast({
          title: "Failed to Start Workflow",
          description: err instanceof Error ? err.message : "Unknown error",
          variant: "error",
        });
      }
    },
    [ticket, createWorkflow, toast, navigate]
  );

  // Handle start workflow button click
  const handleStartWorkflow = useCallback(() => {
    if (activeWorkflowData?.exists && activeWorkflowData.workflow) {
      // Active workflow exists - navigate to it
      navigate(`/workflows/${activeWorkflowData.workflow.id}`);
    } else {
      // No active workflow - show template selection
      setShowTemplateDialog(true);
    }
  }, [activeWorkflowData, navigate]);

  // Loading state
  if (isLoading) {
    return <TicketDetailSkeleton />;
  }

  // Error state
  if (error || !ticket) {
    return (
      <div className="max-w-4xl mx-auto">
        <div className="text-center py-16">
          <div className="inline-flex items-center justify-center w-16 h-16 rounded-full bg-error-100 mb-6">
            <svg
              className="w-8 h-8 text-error-600"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
              />
            </svg>
          </div>
          <h2 className="text-xl font-semibold text-neutral-900 mb-2">
            Failed to load ticket
          </h2>
          <p className="text-neutral-500 mb-6 max-w-md mx-auto">{error}</p>
          <div className="flex items-center justify-center gap-4">
            <button
              onClick={() => navigate("/tickets")}
              className="px-4 py-2 text-neutral-700 border border-neutral-300 rounded-lg 
                       hover:bg-neutral-50 transition-colors"
            >
              Back to tickets
            </button>
            <button
              onClick={loadTicket}
              className="px-4 py-2 bg-primary-600 text-white rounded-lg 
                       hover:bg-primary-700 transition-colors"
            >
              Try again
            </button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto space-y-6 animate-fade-in">
      {/* Back Navigation */}
      <button
        onClick={() => navigate("/tickets")}
        className="inline-flex items-center gap-2 text-sm text-neutral-600 
                 hover:text-neutral-900 transition-colors group"
      >
        <svg
          className="w-4 h-4 transition-transform group-hover:-translate-x-0.5"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M10 19l-7-7m0 0l7-7m-7 7h18"
          />
        </svg>
        Back to tickets
      </button>

      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-4">
        <div className="space-y-1">
          <span className="inline-block text-sm font-mono font-semibold text-primary-600 bg-primary-50 px-2 py-0.5 rounded">
            {ticket.key}
          </span>
          <h1 className="text-2xl font-bold text-neutral-900 leading-tight">
            {ticket.title}
          </h1>
        </div>

        {/* Action Buttons */}
        <div className="flex items-center gap-3 flex-wrap">
          {/* Generate Tests Button (Task 5.2) */}
          <TestGenerator ticketKey={ticket.key} ticketTitle={ticket.title} />
          
          {/* Analyze Risk Button (Task 8.7) */}
          <BugRiskScore ticketKey={ticket.key} ticketTitle={ticket.title} />
          
          {/* Start/Resume Workflow Button */}
          <button
            onClick={handleStartWorkflow}
            className={`inline-flex items-center gap-2 px-5 py-2.5 text-white 
                     rounded-lg transition-colors font-medium shadow-sm
                     hover:shadow-md shrink-0 ${
                       activeWorkflowData?.exists
                         ? "bg-amber-500 hover:bg-amber-600"
                         : "bg-primary-600 hover:bg-primary-700"
                     }`}
          >
            <svg
              className="w-5 h-5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"
              />
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            {activeWorkflowData?.exists ? "Resume Workflow" : "Start Workflow"}
          </button>
        </div>
      </div>

      {/* Metadata */}
      <TicketMetadata ticket={ticket} onStatusChange={handleStatusChange} />

      {/* Labels */}
      {ticket.labels.length > 0 && (
        <div className="flex flex-wrap gap-2">
          {ticket.labels.map((label) => (
            <span
              key={label}
              className="px-2.5 py-1 bg-primary-50 text-primary-700 text-sm font-medium rounded-md
                       border border-primary-100"
            >
              {label}
            </span>
          ))}
        </div>
      )}

      {/* Description */}
      {ticket.descriptionHtml ? (
        <div className="bg-white border border-neutral-200 rounded-xl p-6">
          <h2 className="text-lg font-semibold text-neutral-900 mb-4 flex items-center gap-2">
            <svg
              className="w-5 h-5 text-neutral-500"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M4 6h16M4 12h16M4 18h7"
              />
            </svg>
            Description
            {ticket.hasGherkin && (
              <span className="text-xs font-medium text-success-600 bg-success-50 px-2 py-0.5 rounded">
                Gherkin
              </span>
            )}
          </h2>
          <MarkdownRenderer
            html={ticket.descriptionHtml}
            highlightGherkin={ticket.hasGherkin}
          />
        </div>
      ) : (
        <div className="bg-neutral-50 border border-neutral-200 rounded-xl p-6 text-center">
          <p className="text-neutral-500">No description provided</p>
        </div>
      )}

      {/* Related Tests */}
      <RelatedTests
        ticketKey={ticket.key}
        title={ticket.title}
        description={ticket.descriptionRaw}
      />

      {/* Attachments */}
      {ticket.attachments.length > 0 && (
        <TicketAttachments attachments={ticket.attachments} />
      )}

      {/* Comments */}
      {ticket.comments.length > 0 && (
        <TicketComments comments={ticket.comments} />
      )}

      {/* Performance indicator (dev mode) */}
      {ticket.loadTimeMs && (
        <div className="text-center text-xs text-neutral-400">
          Loaded in {ticket.loadTimeMs}ms
        </div>
      )}

      {/* Template Selection Dialog */}
      <TemplateSelectionDialog
        open={showTemplateDialog}
        onOpenChange={setShowTemplateDialog}
        onSelect={handleTemplateSelect}
        loading={createWorkflow.isPending}
      />
    </div>
  );
}

import { useNavigate } from "react-router-dom";

interface StepSummary {
  index: number;
  name: string;
  status: string;
  notes: string | null;
}

interface WorkflowSummaryData {
  id: string;
  templateName: string;
  ticketId: string;
  status: string;
  startedAt: string;
  completedAt: string | null;
  totalSteps: number;
  completedSteps: number;
  skippedSteps: number;
  steps: StepSummary[];
  allNotes: string[];
}

interface WorkflowSummaryProps {
  summary: WorkflowSummaryData;
  onGenerateReport?: () => void;
}

function CheckIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="currentColor">
      <path fillRule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12Zm13.36-1.814a.75.75 0 1 0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 1.14-.094l3.75-5.25Z" clipRule="evenodd" />
    </svg>
  );
}

function MinusIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="currentColor">
      <path fillRule="evenodd" d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25Zm3 10.5a.75.75 0 0 0 0-1.5H9a.75.75 0 0 0 0 1.5h6Z" clipRule="evenodd" />
    </svg>
  );
}

function DocumentIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z" />
    </svg>
  );
}

/**
 * Workflow completion summary component.
 */
export function WorkflowSummary({ summary, onGenerateReport }: WorkflowSummaryProps) {
  const navigate = useNavigate();
  const completionRate = summary.totalSteps > 0
    ? Math.round((summary.completedSteps / summary.totalSteps) * 100)
    : 0;

  return (
    <div className="bg-white rounded-xl shadow-lg border border-neutral-200 overflow-hidden">
      {/* Header */}
      <div className="bg-gradient-to-r from-green-500 to-emerald-600 px-6 py-8 text-white text-center">
        <div className="w-16 h-16 bg-white/20 rounded-full flex items-center justify-center mx-auto mb-4">
          <CheckIcon className="w-10 h-10 text-white" />
        </div>
        <h2 className="text-2xl font-bold mb-2">Workflow Complete!</h2>
        <p className="text-green-100">
          {summary.templateName} for {summary.ticketId}
        </p>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-3 gap-4 p-6 bg-neutral-50 border-b border-neutral-200">
        <div className="text-center">
          <div className="text-3xl font-bold text-green-600">{summary.completedSteps}</div>
          <div className="text-sm text-neutral-500">Completed</div>
        </div>
        <div className="text-center">
          <div className="text-3xl font-bold text-neutral-400">{summary.skippedSteps}</div>
          <div className="text-sm text-neutral-500">Skipped</div>
        </div>
        <div className="text-center">
          <div className="text-3xl font-bold text-blue-600">{completionRate}%</div>
          <div className="text-sm text-neutral-500">Completion</div>
        </div>
      </div>

      {/* Steps Summary */}
      <div className="p-6">
        <h3 className="text-lg font-semibold text-neutral-900 mb-4">Step Summary</h3>
        <div className="space-y-3">
          {summary.steps.map((step) => (
            <div
              key={step.index}
              className="flex items-start gap-3 p-3 bg-neutral-50 rounded-lg"
            >
              {step.status === "completed" ? (
                <CheckIcon className="w-5 h-5 text-green-500 flex-shrink-0 mt-0.5" />
              ) : step.status === "skipped" ? (
                <MinusIcon className="w-5 h-5 text-neutral-400 flex-shrink-0 mt-0.5" />
              ) : (
                <div className="w-5 h-5 rounded-full border-2 border-neutral-300 flex-shrink-0 mt-0.5" />
              )}
              <div className="flex-1 min-w-0">
                <p className="font-medium text-neutral-900">{step.name}</p>
                {step.notes && (
                  <p className="text-sm text-neutral-500 mt-1">{step.notes}</p>
                )}
              </div>
              <span
                className={`text-xs px-2 py-0.5 rounded-full ${
                  step.status === "completed"
                    ? "bg-green-100 text-green-700"
                    : step.status === "skipped"
                    ? "bg-neutral-200 text-neutral-600"
                    : "bg-neutral-100 text-neutral-500"
                }`}
              >
                {step.status}
              </span>
            </div>
          ))}
        </div>
      </div>

      {/* Notes Section */}
      {summary.allNotes.length > 0 && (
        <div className="p-6 border-t border-neutral-200">
          <h3 className="text-lg font-semibold text-neutral-900 mb-4">All Notes</h3>
          <div className="space-y-2">
            {summary.allNotes.map((note, i) => (
              <div key={i} className="p-3 bg-amber-50 border border-amber-100 rounded-lg">
                <p className="text-sm text-amber-800">{note}</p>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Actions */}
      <div className="p-6 bg-neutral-50 border-t border-neutral-200 flex justify-center gap-4">
        <button
          onClick={() => navigate(`/tickets/${summary.ticketId}`)}
          className="px-4 py-2 text-neutral-700 border border-neutral-300 rounded-lg hover:bg-neutral-100"
        >
          Back to Ticket
        </button>
        {onGenerateReport && (
          <button
            onClick={onGenerateReport}
            className="flex items-center gap-2 px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600"
          >
            <DocumentIcon className="w-5 h-5" />
            Generate Report
          </button>
        )}
      </div>
    </div>
  );
}

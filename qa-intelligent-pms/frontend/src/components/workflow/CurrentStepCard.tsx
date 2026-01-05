import { useState } from "react";
import { formatDuration, type WorkflowStepWithStatus } from "@/hooks/useWorkflow";

// SVG Icons
function CheckIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M4.5 12.75l6 6 9-13.5" />
    </svg>
  );
}

function ClockIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z" />
    </svg>
  );
}

function ChevronRightIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M8.25 4.5l7.5 7.5-7.5 7.5" />
    </svg>
  );
}

interface CurrentStepCardProps {
  step: WorkflowStepWithStatus;
  isLast: boolean;
  onComplete: (notes: string) => void;
  onSkip: () => void;
  loading?: boolean;
}

/**
 * Card displaying the current workflow step.
 */
export function CurrentStepCard({
  step,
  isLast,
  onComplete,
  onSkip,
  loading = false,
}: CurrentStepCardProps) {
  const [notes, setNotes] = useState("");
  const [showNotes, setShowNotes] = useState(false);

  const handleComplete = () => {
    onComplete(notes);
    setNotes("");
    setShowNotes(false);
  };

  return (
    <div className="bg-white rounded-xl shadow-lg border border-neutral-200 overflow-hidden">
      {/* Step Header */}
      <div className="bg-gradient-to-r from-blue-500 to-blue-600 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <span className="w-8 h-8 flex items-center justify-center bg-white/20 
                           text-white font-bold rounded-full">
              {step.index + 1}
            </span>
            <h2 className="text-xl font-semibold text-white">{step.name}</h2>
          </div>
          <div className="flex items-center gap-2 text-blue-100">
            <ClockIcon className="w-4 h-4" />
            <span className="text-sm">{formatDuration(step.estimatedMinutes)}</span>
          </div>
        </div>
      </div>

      {/* Step Content */}
      <div className="p-6">
        <p className="text-neutral-600 leading-relaxed">{step.description}</p>

        {/* Notes Section */}
        {showNotes && (
          <div className="mt-4">
            <label className="block text-sm font-medium text-neutral-700 mb-2">
              Notes (optional)
            </label>
            <textarea
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
              placeholder="Add any observations, issues found, or important details..."
              className="w-full px-3 py-2 border border-neutral-300 rounded-lg
                         focus:outline-none focus:ring-2 focus:ring-blue-500
                         focus:border-transparent resize-none"
              rows={3}
            />
          </div>
        )}

        {/* Actions */}
        <div className="flex items-center justify-between mt-6 pt-4 border-t border-neutral-100">
          <div className="flex items-center gap-2">
            <button
              onClick={() => setShowNotes(!showNotes)}
              className="text-sm text-neutral-500 hover:text-neutral-700"
            >
              {showNotes ? "Hide notes" : "Add notes"}
            </button>
            <span className="text-neutral-300">|</span>
            <button
              onClick={onSkip}
              disabled={loading}
              className="text-sm text-neutral-500 hover:text-neutral-700 disabled:opacity-50"
            >
              Skip step
            </button>
          </div>

          <button
            onClick={handleComplete}
            disabled={loading}
            className="flex items-center gap-2 px-5 py-2.5 bg-green-500 text-white 
                       font-medium rounded-lg hover:bg-green-600 transition-colors
                       disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading ? (
              <>
                <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                Saving...
              </>
            ) : (
              <>
                {isLast ? (
                  <>
                    <CheckIcon className="w-5 h-5" />
                    Complete Workflow
                  </>
                ) : (
                  <>
                    Next Step
                    <ChevronRightIcon className="w-5 h-5" />
                  </>
                )}
              </>
            )}
          </button>
        </div>
      </div>
    </div>
  );
}

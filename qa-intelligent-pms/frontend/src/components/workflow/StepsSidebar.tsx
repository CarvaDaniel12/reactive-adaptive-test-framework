import { useState } from "react";
import { formatDuration, type WorkflowStepWithStatus } from "@/hooks/useWorkflow";

// SVG Icons
function CheckCircleIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="currentColor">
      <path fillRule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12Zm13.36-1.814a.75.75 0 1 0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 1.14-.094l3.75-5.25Z" clipRule="evenodd" />
    </svg>
  );
}

function MinusCircleIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="currentColor">
      <path fillRule="evenodd" d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25Zm3 10.5a.75.75 0 0 0 0-1.5H9a.75.75 0 0 0 0 1.5h6Z" clipRule="evenodd" />
    </svg>
  );
}

function ChevronDownIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M19.5 8.25l-7.5 7.5-7.5-7.5" />
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

function ChatBubbleIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M7.5 8.25h9m-9 3H12m-9.75 1.51c0 1.6 1.123 2.994 2.707 3.227 1.129.166 2.27.293 3.423.379.35.026.67.21.865.501L12 21l2.755-4.133a1.14 1.14 0 01.865-.501 48.172 48.172 0 003.423-.379c1.584-.233 2.707-1.626 2.707-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0012 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018z" />
    </svg>
  );
}

interface StepsSidebarProps {
  steps: WorkflowStepWithStatus[];
  currentStep: number;
  onStepClick?: (index: number) => void;
}

/**
 * Sidebar showing all workflow steps with their status.
 * Completed steps can be expanded to show notes.
 */
export function StepsSidebar({ steps, currentStep, onStepClick }: StepsSidebarProps) {
  const [expandedStep, setExpandedStep] = useState<number | null>(null);

  const completedCount = steps.filter((s) => s.status === "completed").length;
  const progressPercent = steps.length > 0 ? (completedCount / steps.length) * 100 : 0;

  const handleStepClick = (index: number, step: WorkflowStepWithStatus) => {
    // Only allow expanding completed or skipped steps
    if (step.status === "completed" || step.status === "skipped") {
      setExpandedStep(expandedStep === index ? null : index);
    }
    onStepClick?.(index);
  };

  return (
    <div className="bg-white rounded-lg border border-neutral-200 overflow-hidden">
      {/* Header with Progress */}
      <div className="p-4 border-b border-neutral-100">
        <h3 className="text-sm font-semibold text-neutral-700 uppercase tracking-wide mb-3">
          Steps
        </h3>
        <div className="flex items-center gap-2">
          <div className="flex-1 h-2 bg-neutral-100 rounded-full overflow-hidden">
            <div
              className="h-full bg-green-500 transition-all duration-500"
              style={{ width: `${progressPercent}%` }}
            />
          </div>
          <span className="text-xs font-medium text-neutral-500">
            {completedCount}/{steps.length}
          </span>
        </div>
      </div>

      {/* Steps List */}
      <div className="p-2">
        {steps.map((step, index) => {
          const isActive = index === currentStep;
          const isCompleted = step.status === "completed";
          const isSkipped = step.status === "skipped";
          const isExpanded = expandedStep === index;
          const canExpand = (isCompleted || isSkipped) && (step.notes || step.description);

          return (
            <div key={index} className="mb-1">
              {/* Step Row */}
              <button
                onClick={() => handleStepClick(index, step)}
                disabled={!canExpand && !isActive}
                className={`w-full flex items-center gap-3 px-3 py-2.5 rounded-lg transition-all
                  ${isActive ? "bg-blue-50 border border-blue-200" : "hover:bg-neutral-50"}
                  ${canExpand ? "cursor-pointer" : isActive ? "cursor-default" : "cursor-default opacity-70"}
                  ${isExpanded ? "bg-neutral-50" : ""}`}
              >
                {/* Status Icon */}
                <div className="flex-shrink-0">
                  {isCompleted ? (
                    <CheckCircleIcon className="w-5 h-5 text-green-500" />
                  ) : isSkipped ? (
                    <MinusCircleIcon className="w-5 h-5 text-neutral-400" />
                  ) : isActive ? (
                    <div className="w-5 h-5 rounded-full border-2 border-blue-500 
                                  flex items-center justify-center">
                      <div className="w-2 h-2 bg-blue-500 rounded-full animate-pulse" />
                    </div>
                  ) : (
                    <div className="w-5 h-5 rounded-full border-2 border-neutral-300" />
                  )}
                </div>

                {/* Step Info */}
                <div className="flex-1 min-w-0 text-left">
                  <p
                    className={`text-sm font-medium truncate
                      ${isActive ? "text-blue-700" : isCompleted ? "text-green-700" : isSkipped ? "text-neutral-500" : "text-neutral-600"}`}
                  >
                    {step.name}
                  </p>
                  {isActive && (
                    <div className="flex items-center gap-1 mt-0.5 text-xs text-blue-500">
                      <ClockIcon className="w-3 h-3" />
                      <span>{formatDuration(step.estimatedMinutes)}</span>
                    </div>
                  )}
                </div>

                {/* Expand Indicator / Step Number */}
                <div className="flex items-center gap-2">
                  {step.notes && (isCompleted || isSkipped) && (
                    <ChatBubbleIcon className="w-4 h-4 text-neutral-400" />
                  )}
                  {canExpand ? (
                    <ChevronDownIcon
                      className={`w-4 h-4 text-neutral-400 transition-transform
                        ${isExpanded ? "rotate-180" : ""}`}
                    />
                  ) : (
                    <span className="text-xs text-neutral-400 w-4 text-center">{index + 1}</span>
                  )}
                </div>
              </button>

              {/* Expanded Content */}
              {isExpanded && canExpand && (
                <div className="ml-8 mr-3 mt-1 mb-2 p-3 bg-neutral-50 rounded-lg border border-neutral-100 animate-fade-in">
                  {/* Description */}
                  <p className="text-xs text-neutral-500 mb-2">{step.description}</p>

                  {/* Notes */}
                  {step.notes && (
                    <div className="mt-2 pt-2 border-t border-neutral-200">
                      <div className="flex items-center gap-1 text-xs font-medium text-neutral-600 mb-1">
                        <ChatBubbleIcon className="w-3 h-3" />
                        Notes
                      </div>
                      <p className="text-sm text-neutral-700 whitespace-pre-wrap">
                        {step.notes}
                      </p>
                    </div>
                  )}

                  {/* Status Badge */}
                  <div className="mt-2 flex items-center gap-2">
                    <span
                      className={`text-xs px-2 py-0.5 rounded-full font-medium
                        ${isCompleted ? "bg-green-100 text-green-700" : "bg-neutral-200 text-neutral-600"}`}
                    >
                      {isCompleted ? "Completed" : "Skipped"}
                    </span>
                    <span className="text-xs text-neutral-400">
                      Est. {formatDuration(step.estimatedMinutes)}
                    </span>
                  </div>
                </div>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}

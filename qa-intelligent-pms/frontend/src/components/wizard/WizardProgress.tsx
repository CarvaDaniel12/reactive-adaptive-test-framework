/**
 * Wizard progress indicator component.
 *
 * Shows step progress with visual indicators for completed, current, and pending steps.
 */
import { useWizardStore, WIZARD_STEPS } from "@/stores/wizardStore";

export function WizardProgress() {
  const { currentStep, completedSteps, skippedSteps } = useWizardStore();

  return (
    <div className="flex items-center justify-between">
      {WIZARD_STEPS.map((step, index) => {
        const isCompleted = completedSteps.includes(step.id);
        const isSkipped = skippedSteps.includes(step.id);
        const isCurrent = currentStep === step.id;

        return (
          <div key={step.id} className="flex items-center">
            {/* Step circle */}
            <div
              className={`
                w-10 h-10 rounded-full flex items-center justify-center font-medium
                transition-colors duration-300
                ${
                  isCompleted
                    ? "bg-success-500 text-white"
                    : isSkipped
                      ? "bg-warning-500 text-white"
                      : isCurrent
                        ? "bg-primary-500 text-white"
                        : "bg-neutral-200 text-neutral-600"
                }
              `}
              aria-current={isCurrent ? "step" : undefined}
            >
              {isCompleted ? (
                <CheckIcon className="w-5 h-5" />
              ) : isSkipped ? (
                <SkipIcon className="w-5 h-5" />
              ) : (
                step.id
              )}
            </div>

            {/* Connector line */}
            {index < WIZARD_STEPS.length - 1 && (
              <div
                className={`
                  h-1 w-12 sm:w-16 mx-2 rounded transition-colors duration-300
                  ${isCompleted || isSkipped ? "bg-success-500" : "bg-neutral-200"}
                `}
              />
            )}
          </div>
        );
      })}
    </div>
  );
}

/**
 * Compact progress indicator showing "Step X of Y".
 */
export function WizardProgressCompact() {
  const { currentStep, totalSteps } = useWizardStore();

  return (
    <div className="text-sm text-neutral-600">
      Step {currentStep} of {totalSteps}
    </div>
  );
}

function CheckIcon({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
      strokeWidth={2}
    >
      <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
    </svg>
  );
}

function SkipIcon({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
      strokeWidth={2}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M13 5l7 7-7 7M5 5l7 7-7 7"
      />
    </svg>
  );
}

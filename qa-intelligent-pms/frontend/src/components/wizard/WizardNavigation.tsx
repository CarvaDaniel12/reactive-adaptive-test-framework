/**
 * Wizard navigation component.
 *
 * Provides Back/Next buttons and optional Skip for optional steps.
 */
import { useNavigate } from "react-router-dom";
import { useWizardStore, WIZARD_STEPS } from "@/stores/wizardStore";
import { SkipButton } from "./SkipButton";
import { useCallback, useEffect } from "react";

interface WizardNavigationProps {
  /** Whether the current step passes validation */
  isValid?: boolean;
  /** Custom callback before moving to next step */
  onBeforeNext?: () => Promise<boolean> | boolean;
  /** Custom callback before moving to previous step */
  onBeforeBack?: () => Promise<boolean> | boolean;
}

export function WizardNavigation({
  isValid = true,
  onBeforeNext,
  onBeforeBack,
}: WizardNavigationProps) {
  const navigate = useNavigate();
  const {
    currentStep,
    totalSteps,
    nextStep,
    prevStep,
    markStepComplete,
    markStepSkipped,
  } = useWizardStore();

  const currentStepConfig = WIZARD_STEPS.find((s) => s.id === currentStep);
  const isFirstStep = currentStep === 1;
  const isLastStep = currentStep === totalSteps;
  const isOptional = currentStepConfig?.optional ?? false;

  const getNextPath = useCallback(() => {
    if (isLastStep) return "/setup/complete";
    const nextStepConfig = WIZARD_STEPS.find((s) => s.id === currentStep + 1);
    return `/setup/${nextStepConfig?.path ?? "profile"}`;
  }, [currentStep, isLastStep]);

  const getPrevPath = useCallback(() => {
    const prevStepConfig = WIZARD_STEPS.find((s) => s.id === currentStep - 1);
    return `/setup/${prevStepConfig?.path ?? "profile"}`;
  }, [currentStep]);

  const handleNext = useCallback(async () => {
    if (onBeforeNext) {
      const canProceed = await onBeforeNext();
      if (!canProceed) return;
    }

    markStepComplete(currentStep);
    nextStep();
    navigate(getNextPath());
  }, [
    onBeforeNext,
    markStepComplete,
    currentStep,
    nextStep,
    navigate,
    getNextPath,
  ]);

  const handleBack = useCallback(async () => {
    if (onBeforeBack) {
      const canProceed = await onBeforeBack();
      if (!canProceed) return;
    }

    prevStep();
    navigate(getPrevPath());
  }, [onBeforeBack, prevStep, navigate, getPrevPath]);

  const handleSkip = useCallback(() => {
    markStepSkipped(currentStep);
    nextStep();
    navigate(getNextPath());
  }, [markStepSkipped, currentStep, nextStep, navigate, getNextPath]);

  // Keyboard navigation
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Only handle if no input is focused
      const activeElement = document.activeElement;
      const isInputFocused =
        activeElement instanceof HTMLInputElement ||
        activeElement instanceof HTMLTextAreaElement ||
        activeElement instanceof HTMLSelectElement;

      if (isInputFocused) return;

      if (e.key === "Enter" && isValid && !isLastStep) {
        e.preventDefault();
        handleNext();
      } else if (e.key === "Escape" && !isFirstStep) {
        e.preventDefault();
        handleBack();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isValid, isLastStep, isFirstStep, handleNext, handleBack]);

  return (
    <div className="mt-8 flex items-center justify-between border-t border-neutral-200 pt-6">
      {/* Left side: Back button */}
      <div>
        {!isFirstStep && (
          <button
            type="button"
            onClick={handleBack}
            className="px-4 py-2 text-neutral-700 hover:text-neutral-900
              border border-neutral-300 rounded-lg
              hover:bg-neutral-50 transition-colors duration-150"
          >
            Back
          </button>
        )}
      </div>

      {/* Right side: Skip and Next buttons */}
      <div className="flex items-center gap-4">
        {isOptional && <SkipButton onSkip={handleSkip} />}

        <button
          type="button"
          onClick={handleNext}
          disabled={!isValid}
          className="px-6 py-2 bg-primary-500 text-white rounded-lg font-medium
            hover:bg-primary-600 disabled:opacity-50 disabled:cursor-not-allowed
            transition-colors duration-150"
        >
          {isLastStep ? "Complete Setup" : "Next"}
        </button>
      </div>
    </div>
  );
}

/**
 * Setup Wizard layout component.
 *
 * Provides the shell for the wizard with progress indicator and navigation.
 */
import { Outlet, useLocation } from "react-router-dom";
import { useEffect } from "react";
import {
  WizardProgress,
  WizardProgressCompact,
} from "@/components/wizard/WizardProgress";
import { useWizardStore, WIZARD_STEPS } from "@/stores/wizardStore";

export function SetupWizardLayout() {
  const location = useLocation();
  const { setCurrentStep } = useWizardStore();

  // Sync current step with URL
  useEffect(() => {
    const path = location.pathname.split("/").pop();
    const step = WIZARD_STEPS.find((s) => s.path === path);
    if (step) {
      setCurrentStep(step.id);
    }
  }, [location.pathname, setCurrentStep]);

  return (
    <div className="min-h-screen bg-neutral-50 flex items-center justify-center p-4">
      <div className="w-full max-w-2xl">
        {/* Header */}
        <div className="text-center mb-8">
          <h1 className="text-3xl font-bold text-neutral-900">
            QA Intelligent PMS
          </h1>
          <p className="mt-2 text-neutral-600">Let's get you set up</p>
        </div>

        {/* Card */}
        <div className="bg-white rounded-xl shadow-lg overflow-hidden">
          {/* Progress indicator */}
          <div className="px-8 pt-8 pb-4">
            <WizardProgress />
            <div className="mt-4 text-center">
              <WizardProgressCompact />
            </div>
          </div>

          {/* Step content */}
          <div className="px-8 pb-8">
            <Outlet />
          </div>
        </div>

        {/* Footer */}
        <div className="mt-4 text-center text-sm text-neutral-500">
          Need help?{" "}
          <a href="#" className="text-primary-500 hover:underline">
            View documentation
          </a>
        </div>
      </div>
    </div>
  );
}

/**
 * Setup completion screen.
 *
 * Shows summary of configured integrations, calls API to persist, and redirects to main app.
 */
import { useNavigate } from "react-router-dom";
import { useEffect, useState } from "react";
import { useWizardStore } from "@/stores/wizardStore";

type CompletionState = "idle" | "saving" | "success" | "error";

interface SetupValidationError {
  field: string;
  message: string;
  step: string;
  fixPath: string;
}

interface CompletionResponse {
  success: boolean;
  errors?: SetupValidationError[];
  configuredIntegrations?: string[];
  configPath?: string;
}

export function SetupComplete() {
  const navigate = useNavigate();
  const { completedSteps, skippedSteps, formData, completeWizard, isComplete } =
    useWizardStore();

  const [state, setState] = useState<CompletionState>("idle");
  const [errors, setErrors] = useState<SetupValidationError[]>([]);

  // If already complete, redirect to main app
  useEffect(() => {
    if (isComplete) {
      navigate("/", { replace: true });
    }
  }, [isComplete, navigate]);

  const handleComplete = async () => {
    setState("saving");
    setErrors([]);

    try {
      const response = await fetch("/api/v1/setup/complete", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          profile: formData.profile,
          jira: formData.jira,
          postman: formData.postman,
          testmo: formData.testmo,
          splunk: formData.splunk,
        }),
      });

      const result: CompletionResponse = await response.json();

      if (result.success) {
        setState("success");
        completeWizard();

        // Redirect after success animation
        setTimeout(() => {
          navigate("/", { replace: true });
        }, 2000);
      } else {
        setState("error");
        setErrors(
          result.errors ?? [
            {
              field: "setup",
              message: "Configuration validation failed",
              step: "general",
              fixPath: "/setup",
            },
          ]
        );
      }
    } catch {
      setState("error");
      setErrors([
        {
          field: "connection",
          message: "Could not connect to server. Please try again.",
          step: "general",
          fixPath: "/setup",
        },
      ]);
    }
  };

  // Success state
  if (state === "success") {
    return (
      <div className="text-center py-8">
        <div className="w-16 h-16 bg-success-100 rounded-full flex items-center justify-center mx-auto mb-6 animate-pulse">
          <CheckIcon className="w-8 h-8 text-success-500" />
        </div>

        <h2 className="text-2xl font-semibold text-neutral-900 mb-2">
          Setup Complete!
        </h2>
        <p className="text-neutral-600">
          Redirecting to your dashboard...
        </p>
      </div>
    );
  }

  return (
    <div className="py-4">
      <div className="text-center mb-6">
        <h2 className="text-2xl font-semibold text-neutral-900 mb-2">
          Review & Complete
        </h2>
        <p className="text-neutral-600">
          Review your configuration before completing setup
        </p>
      </div>

      {/* Summary */}
      <div className="bg-neutral-50 rounded-lg p-6 text-left mb-6">
        <h3 className="font-medium text-neutral-900 mb-4">Configuration Summary</h3>

        <div className="space-y-3">
          <div className="flex items-center justify-between">
            <span className="text-neutral-600">Profile</span>
            <span className="text-success-500 font-medium flex items-center gap-1">
              <CheckIcon className="w-4 h-4" />
              {formData.profile?.displayName || "Configured"}
            </span>
          </div>

          <div className="flex items-center justify-between">
            <span className="text-neutral-600">Jira</span>
            {formData.jira ? (
              <span className="text-success-500 font-medium flex items-center gap-1">
                <CheckIcon className="w-4 h-4" />
                Connected
              </span>
            ) : (
              <span className="text-error-500">Required</span>
            )}
          </div>

          {["postman", "testmo", "splunk"].map((key) => {
            const data = formData[key as keyof typeof formData];
            const name = key.charAt(0).toUpperCase() + key.slice(1);
            return (
              <div key={key} className="flex items-center justify-between">
                <span className="text-neutral-600">{name}</span>
                {data ? (
                  <span className="text-success-500 font-medium flex items-center gap-1">
                    <CheckIcon className="w-4 h-4" />
                    Connected
                  </span>
                ) : (
                  <span className="text-neutral-400">Skipped</span>
                )}
              </div>
            );
          })}

          <div className="border-t border-neutral-200 pt-3 mt-3">
            <div className="flex items-center justify-between text-sm">
              <span className="text-neutral-500">Steps completed</span>
              <span className="text-neutral-700">{completedSteps.length}</span>
            </div>
            {skippedSteps.length > 0 && (
              <div className="flex items-center justify-between text-sm mt-1">
                <span className="text-neutral-500">Steps skipped</span>
                <span className="text-warning-500">{skippedSteps.length}</span>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Errors */}
      {state === "error" && errors.length > 0 && (
        <div className="bg-error-50 border border-error-200 rounded-lg p-4 mb-6">
          <div className="flex items-center gap-2 mb-2">
            <CrossIcon className="w-5 h-5 text-error-500" />
            <span className="font-medium text-error-700">Setup Failed</span>
          </div>
          <ul className="space-y-2">
            {errors.map((error, index) => (
              <li
                key={index}
                className="text-sm text-error-700 flex items-start justify-between gap-3"
              >
                <span className="flex-1">
                  <span className="font-medium">{error.field}</span>: {error.message}
                </span>
                {error.fixPath ? (
                  <button
                    type="button"
                    onClick={() => navigate(error.fixPath)}
                    className="text-primary-600 hover:underline whitespace-nowrap"
                  >
                    Fix
                  </button>
                ) : null}
              </li>
            ))}
          </ul>
        </div>
      )}

      <button
        type="button"
        onClick={handleComplete}
        disabled={state === "saving" || !formData.jira}
        className="w-full px-8 py-3 bg-primary-500 text-white rounded-lg font-medium
          hover:bg-primary-600 disabled:opacity-50 disabled:cursor-not-allowed
          transition-colors"
      >
        {state === "saving" ? (
          <span className="flex items-center justify-center gap-2">
            <LoadingSpinner className="w-5 h-5" />
            Saving Configuration...
          </span>
        ) : (
          "Complete Setup"
        )}
      </button>
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

function CrossIcon({ className }: { className?: string }) {
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
        d="M6 18L18 6M6 6l12 12"
      />
    </svg>
  );
}

function LoadingSpinner({ className }: { className?: string }) {
  return (
    <svg
      className={`animate-spin ${className}`}
      viewBox="0 0 24 24"
      fill="none"
    >
      <circle
        className="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        strokeWidth="4"
      />
      <path
        className="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      />
    </svg>
  );
}

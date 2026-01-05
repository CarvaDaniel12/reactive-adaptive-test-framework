/**
 * Testmo Integration configuration step (Step 4).
 *
 * Collects Testmo credentials and tests connection.
 */
import { useState, useCallback } from "react";
import { WizardStepHeader } from "@/components/wizard/WizardStepHeader";
import { WizardNavigation } from "@/components/wizard/WizardNavigation";
import {
  ConnectionStatus,
  type ConnectionState,
} from "@/components/wizard/ConnectionStatus";
import { useWizardStore } from "@/stores/wizardStore";

interface TestmoTestResponse {
  success: boolean;
  message?: string;
  projectCount?: number;
}

export function TestmoStep() {
  const { formData, setStepData } = useWizardStore();
  const testmoData = formData.testmo;

  // Form state
  const [instanceUrl, setInstanceUrl] = useState(testmoData?.instanceUrl ?? "");
  const [apiKey, setApiKey] = useState(testmoData?.apiKey ?? "");
  const [showKey, setShowKey] = useState(false);

  // Connection test state
  const [connectionState, setConnectionState] = useState<ConnectionState>("idle");
  const [errorMessage, setErrorMessage] = useState("");
  const [projectCount, setProjectCount] = useState<number | null>(null);

  // Validation
  const isUrlValid =
    instanceUrl.trim().startsWith("https://") && instanceUrl.includes(".");
  const isFormValid = isUrlValid && apiKey.trim().length > 0;
  const hasTestedSuccessfully = connectionState === "success";

  // Reset connection state when form changes
  const handleFieldChange = useCallback(
    (setter: (value: string) => void) => (value: string) => {
      setter(value);
      if (connectionState !== "idle") {
        setConnectionState("idle");
        setErrorMessage("");
        setProjectCount(null);
      }
    },
    [connectionState]
  );

  const testConnection = async () => {
    if (!isFormValid) return;

    setConnectionState("testing");
    setErrorMessage("");
    setProjectCount(null);

    try {
      const response = await fetch("/api/v1/setup/integrations/testmo/test", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          instanceUrl: instanceUrl.trim(),
          apiKey: apiKey.trim(),
        }),
      });

      const data: TestmoTestResponse = await response.json();

      if (data.success) {
        setConnectionState("success");
        setProjectCount(data.projectCount ?? null);
        // Save to wizard store
        setStepData("testmo", {
          instanceUrl: instanceUrl.trim(),
          apiKey: apiKey.trim(),
        });
      } else {
        setConnectionState("error");
        setErrorMessage(data.message ?? "Connection failed");
      }
    } catch {
      setConnectionState("error");
      setErrorMessage("Cannot reach server. Check your network connection.");
    }
  };

  const handleBeforeNext = () => {
    // Save data even if not tested (optional step)
    if (instanceUrl.trim() && apiKey.trim() && !hasTestedSuccessfully) {
      setStepData("testmo", {
        instanceUrl: instanceUrl.trim(),
        apiKey: apiKey.trim(),
      });
    }
    return true;
  };

  return (
    <div>
      <WizardStepHeader
        title="Testmo Integration"
        description="Connect to Testmo for test management"
        optional
      />

      <div className="space-y-4">
        {/* Instance URL */}
        <div>
          <label
            htmlFor="instanceUrl"
            className="block text-sm font-medium text-neutral-700 mb-1"
          >
            Testmo Instance URL
          </label>
          <input
            id="instanceUrl"
            type="url"
            value={instanceUrl}
            onChange={(e) => handleFieldChange(setInstanceUrl)(e.target.value)}
            placeholder="https://your-company.testmo.net"
            className={`w-full px-3 py-2 border rounded-lg transition-colors
              focus:ring-2 focus:ring-primary-500 focus:border-primary-500
              ${
                instanceUrl && !isUrlValid
                  ? "border-error-300 bg-error-50"
                  : "border-neutral-300"
              }`}
          />
          {instanceUrl && !isUrlValid && (
            <p className="mt-1 text-sm text-error-500">
              Please enter a valid HTTPS URL
            </p>
          )}
        </div>

        {/* API Key */}
        <div>
          <label
            htmlFor="apiKey"
            className="block text-sm font-medium text-neutral-700 mb-1"
          >
            Testmo API Key
          </label>
          <div className="relative">
            <input
              id="apiKey"
              type={showKey ? "text" : "password"}
              value={apiKey}
              onChange={(e) => handleFieldChange(setApiKey)(e.target.value)}
              placeholder="Your Testmo API key"
              className="w-full px-3 py-2 pr-10 border border-neutral-300 rounded-lg
                focus:ring-2 focus:ring-primary-500 focus:border-primary-500
                transition-colors"
            />
            <button
              type="button"
              onClick={() => setShowKey(!showKey)}
              className="absolute right-3 top-1/2 -translate-y-1/2 text-neutral-500
                hover:text-neutral-700 transition-colors"
              aria-label={showKey ? "Hide API key" : "Show API key"}
            >
              {showKey ? (
                <EyeClosedIcon className="w-5 h-5" />
              ) : (
                <EyeOpenIcon className="w-5 h-5" />
              )}
            </button>
          </div>
        </div>

        {/* Test Connection */}
        {instanceUrl.trim().length > 0 && apiKey.trim().length > 0 && (
          <div className="flex items-center gap-4 pt-2">
            <button
              type="button"
              onClick={testConnection}
              disabled={!isFormValid || connectionState === "testing"}
              className="px-4 py-2 border border-primary-500 text-primary-500 rounded-lg
                hover:bg-primary-50 disabled:opacity-50 disabled:cursor-not-allowed
                transition-colors"
            >
              {connectionState === "testing" ? "Testing..." : "Test Connection"}
            </button>

            {connectionState === "error" && (
              <button
                type="button"
                onClick={testConnection}
                className="text-primary-500 hover:text-primary-600 underline text-sm"
              >
                Retry
              </button>
            )}
          </div>
        )}

        {/* Connection Status */}
        <ConnectionStatus
          state={connectionState}
          errorMessage={errorMessage}
          successMessage="Connected to Testmo"
          successDetails={
            projectCount ? `Found ${projectCount} projects` : undefined
          }
        />

        {/* Help Section */}
        <div className="p-4 bg-neutral-50 rounded-lg text-sm text-neutral-600">
          <p className="font-medium mb-2">Why connect Testmo?</p>
          <ul className="list-disc list-inside space-y-1">
            <li>Search test cases related to your tickets</li>
            <li>Create test runs directly from the app</li>
            <li>Track test execution progress</li>
          </ul>
        </div>
      </div>

      <WizardNavigation isValid onBeforeNext={handleBeforeNext} />
    </div>
  );
}

function EyeOpenIcon({ className }: { className?: string }) {
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
        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
      />
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
      />
    </svg>
  );
}

function EyeClosedIcon({ className }: { className?: string }) {
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
        d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21"
      />
    </svg>
  );
}

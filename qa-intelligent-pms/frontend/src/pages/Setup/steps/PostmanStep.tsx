/**
 * Postman Integration configuration step (Step 3).
 *
 * Collects Postman API key and tests connection.
 */
import { useState, useCallback, useEffect } from "react";
import { WizardStepHeader } from "@/components/wizard/WizardStepHeader";
import { WizardNavigation } from "@/components/wizard/WizardNavigation";
import {
  ConnectionStatus,
  type ConnectionState,
} from "@/components/wizard/ConnectionStatus";
import { useWizardStore } from "@/stores/wizardStore";

interface PostmanTestResponse {
  success: boolean;
  message?: string;
  workspaceCount?: number;
}

export function PostmanStep() {
  const { formData, setStepData } = useWizardStore();
  const postmanData = formData.postman;

  // Form state
  const [apiKey, setApiKey] = useState(postmanData?.apiKey ?? "");
  const [showKey, setShowKey] = useState(false);

  // Connection test state
  const [connectionState, setConnectionState] = useState<ConnectionState>("idle");
  const [errorMessage, setErrorMessage] = useState("");
  const [workspaceCount, setWorkspaceCount] = useState<number | null>(null);

  // Rehydrate from persisted store when available
  useEffect(() => {
    if (postmanData) {
      setApiKey(postmanData.apiKey ?? "");
    }
  }, [postmanData]);

  // Validation - Postman API keys are typically 64 chars starting with PMAK-
  const isKeyValid = apiKey.trim().length >= 32;
  const hasTestedSuccessfully = connectionState === "success";

  // Reset connection state when form changes
  const handleFieldChange = useCallback(
    (value: string) => {
      setApiKey(value);
      if (connectionState !== "idle") {
        setConnectionState("idle");
        setErrorMessage("");
        setWorkspaceCount(null);
      }
    },
    [connectionState]
  );

  const testConnection = async () => {
    if (!isKeyValid) return;

    setConnectionState("testing");
    setErrorMessage("");
    setWorkspaceCount(null);

    try {
      const response = await fetch("/api/v1/setup/integrations/postman/test", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          apiKey: apiKey.trim(),
        }),
      });

      const data: PostmanTestResponse = await response.json();

      if (data.success) {
        setConnectionState("success");
        setWorkspaceCount(data.workspaceCount ?? null);
        // Save to wizard store
        setStepData("postman", {
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
    if (apiKey.trim() && !hasTestedSuccessfully) {
      setStepData("postman", {
        apiKey: apiKey.trim(),
      });
    }
    return true;
  };

  return (
    <div>
      <WizardStepHeader
        title="Postman Integration"
        description="Connect to Postman for API test search"
        optional
      />

      <div className="space-y-4">
        {/* API Key */}
        <div>
          <label
            htmlFor="apiKey"
            className="block text-sm font-medium text-neutral-700 mb-1"
          >
            Postman API Key
          </label>
          <div className="relative">
            <input
              id="apiKey"
              type={showKey ? "text" : "password"}
              value={apiKey}
              onChange={(e) => handleFieldChange(e.target.value)}
              placeholder="PMAK-xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
              className="w-full px-3 py-2 pr-10 border border-neutral-300 rounded-lg
                text-neutral-900 placeholder:text-neutral-400
                focus:ring-2 focus:ring-primary-500 focus:border-primary-500
                transition-colors font-mono text-sm"
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
          <p className="mt-1 text-sm text-neutral-500">
            Found in Postman &gt; Settings &gt; API Keys
          </p>
        </div>

        {/* Test Connection */}
        {apiKey.trim().length > 0 && (
          <div className="flex items-center gap-4 pt-2">
            <button
              type="button"
              onClick={testConnection}
              disabled={!isKeyValid || connectionState === "testing"}
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
          successMessage="Connected to Postman"
          successDetails={
            workspaceCount ? `Found ${workspaceCount} workspaces` : undefined
          }
        />

        {/* Help Section */}
        <div className="p-4 bg-neutral-50 rounded-lg text-sm text-neutral-600">
          <p className="font-medium mb-2">Why connect Postman?</p>
          <ul className="list-disc list-inside space-y-1">
            <li>Search API tests related to your Jira tickets</li>
            <li>View test documentation alongside your work</li>
            <li>Quickly find relevant test collections</li>
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

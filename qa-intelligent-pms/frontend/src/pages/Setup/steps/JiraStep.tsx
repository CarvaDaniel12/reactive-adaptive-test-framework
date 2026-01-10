/**
 * Jira Integration configuration step (Step 2).
 *
 * Supports two auth methods:
 * 1. API Token (recommended) - Email + API Token
 * 2. OAuth 2.0 (advanced) - Client ID + Client Secret
 */
import { useState, useCallback, useEffect } from "react";
import { WizardStepHeader } from "@/components/wizard/WizardStepHeader";
import { WizardNavigation } from "@/components/wizard/WizardNavigation";
import {
  ConnectionStatus,
  type ConnectionState,
} from "@/components/wizard/ConnectionStatus";
import { useWizardStore } from "@/stores/wizardStore";

type AuthMethod = "api_token" | "oauth";

interface JiraTestResponse {
  success: boolean;
  message?: string;
  projectCount?: number;
}

export function JiraStep() {
  const { formData, setStepData } = useWizardStore();
  const jiraData = formData.jira;

  const initialAuthMethod: AuthMethod =
    jiraData?.authMethod ??
    (jiraData?.clientId || jiraData?.clientSecret ? "oauth" : "api_token");
  // Auth method selection
  const [authMethod, setAuthMethod] = useState<AuthMethod>(initialAuthMethod);

  // Form state - common
  const [instanceUrl, setInstanceUrl] = useState(jiraData?.instanceUrl ?? "");

  // Form state - API Token
  const [email, setEmail] = useState(jiraData?.email ?? "");
  const [apiToken, setApiToken] = useState(jiraData?.apiToken ?? "");

  // Form state - OAuth
  const [clientId, setClientId] = useState(jiraData?.clientId ?? "");
  const [clientSecret, setClientSecret] = useState(jiraData?.clientSecret ?? "");

  const [showSecret, setShowSecret] = useState(false);

  // Connection test state
  const [connectionState, setConnectionState] = useState<ConnectionState>("idle");
  const [errorMessage, setErrorMessage] = useState("");
  const [projectCount, setProjectCount] = useState<number | null>(null);

  // Rehydrate from persisted store when available
  useEffect(() => {
    if (jiraData) {
      setInstanceUrl(jiraData.instanceUrl ?? "");
      setEmail(jiraData.email ?? "");
      setApiToken(jiraData.apiToken ?? "");
      setClientId(jiraData.clientId ?? "");
      setClientSecret(jiraData.clientSecret ?? "");
      const inferredMethod: AuthMethod =
        jiraData.authMethod ??
        (jiraData.clientId || jiraData.clientSecret ? "oauth" : "api_token");
      setAuthMethod(inferredMethod);
    }
  }, [jiraData]);

  // Validation
  const isUrlValid =
    instanceUrl.trim().startsWith("https://") && instanceUrl.includes(".");

  const isApiTokenValid =
    isUrlValid &&
    email.trim().length > 0 &&
    email.includes("@") &&
    apiToken.trim().length > 0;

  const isOAuthValid =
    isUrlValid &&
    clientId.trim().length > 0 &&
    clientSecret.trim().length > 0;

  const isFormValid = authMethod === "api_token" ? isApiTokenValid : isOAuthValid;
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
      const body =
        authMethod === "api_token"
          ? {
              instanceUrl: instanceUrl.trim(),
              email: email.trim(),
              apiToken: apiToken.trim(),
            }
          : {
              instanceUrl: instanceUrl.trim(),
              clientId: clientId.trim(),
              clientSecret: clientSecret.trim(),
            };

      const response = await fetch("/api/v1/setup/integrations/jira/test", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body),
      });

      const data: JiraTestResponse = await response.json();

      if (data.success) {
        setConnectionState("success");
        setProjectCount(data.projectCount ?? null);
        // Save to wizard store
        setStepData("jira", {
          instanceUrl: instanceUrl.trim(),
          authMethod,
          ...(authMethod === "api_token"
            ? { email: email.trim(), apiToken: apiToken.trim() }
            : { clientId: clientId.trim(), clientSecret: clientSecret.trim() }),
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

  return (
    <div>
      <WizardStepHeader
        title="Jira Integration"
        description="Connect to your Jira Cloud instance"
      />

      <div className="space-y-4">
        {/* Instance URL */}
        <div>
          <label
            htmlFor="instanceUrl"
            className="block text-sm font-medium text-neutral-700 mb-1"
          >
            Jira Instance URL <span className="text-error-500">*</span>
          </label>
          <input
            id="instanceUrl"
            type="url"
            value={instanceUrl}
            onChange={(e) => handleFieldChange(setInstanceUrl)(e.target.value)}
            placeholder="https://your-company.atlassian.net"
            className={`w-full px-3 py-2 border rounded-lg transition-colors
              text-neutral-900 placeholder:text-neutral-400
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

        {/* Auth Method Selection */}
        <div>
          <label className="block text-sm font-medium text-neutral-700 mb-2">
            Authentication Method
          </label>
          <div className="flex gap-4">
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="radio"
                name="authMethod"
                value="api_token"
                checked={authMethod === "api_token"}
                onChange={() => {
                  setAuthMethod("api_token");
                  setConnectionState("idle");
                }}
                className="w-4 h-4 text-primary-500"
              />
              <span className="text-sm text-neutral-700">
                API Token <span className="text-success-600 text-xs">(Recommended)</span>
              </span>
            </label>
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="radio"
                name="authMethod"
                value="oauth"
                checked={authMethod === "oauth"}
                onChange={() => {
                  setAuthMethod("oauth");
                  setConnectionState("idle");
                }}
                className="w-4 h-4 text-primary-500"
              />
              <span className="text-sm text-neutral-700">OAuth 2.0</span>
            </label>
          </div>
        </div>

        {/* API Token Fields */}
        {authMethod === "api_token" && (
          <>
            <div>
              <label
                htmlFor="email"
                className="block text-sm font-medium text-neutral-700 mb-1"
              >
                Jira Email <span className="text-error-500">*</span>
              </label>
              <input
                id="email"
                type="email"
                value={email}
                onChange={(e) => handleFieldChange(setEmail)(e.target.value)}
                placeholder="your.email@company.com"
                className="w-full px-3 py-2 border border-neutral-300 rounded-lg
                  text-neutral-900 placeholder:text-neutral-400
                  focus:ring-2 focus:ring-primary-500 focus:border-primary-500
                  transition-colors"
              />
            </div>

            <div>
              <label
                htmlFor="apiToken"
                className="block text-sm font-medium text-neutral-700 mb-1"
              >
                API Token <span className="text-error-500">*</span>
              </label>
              <div className="relative">
                <input
                  id="apiToken"
                  type={showSecret ? "text" : "password"}
                  value={apiToken}
                  onChange={(e) => handleFieldChange(setApiToken)(e.target.value)}
                  placeholder="Your Jira API Token"
                  className="w-full px-3 py-2 pr-10 border border-neutral-300 rounded-lg
                    text-neutral-900 placeholder:text-neutral-400
                    focus:ring-2 focus:ring-primary-500 focus:border-primary-500
                    transition-colors"
                />
                <button
                  type="button"
                  onClick={() => setShowSecret(!showSecret)}
                  className="absolute right-3 top-1/2 -translate-y-1/2 text-neutral-500
                    hover:text-neutral-700 transition-colors"
                  aria-label={showSecret ? "Hide token" : "Show token"}
                >
                  {showSecret ? (
                    <EyeClosedIcon className="w-5 h-5" />
                  ) : (
                    <EyeOpenIcon className="w-5 h-5" />
                  )}
                </button>
              </div>
            </div>

            {/* API Token Help */}
            <div className="p-4 bg-neutral-50 rounded-lg text-sm text-neutral-600">
              <p className="font-medium mb-2">How to get your API Token:</p>
              <ol className="list-decimal list-inside space-y-1">
                <li>
                  Go to{" "}
                  <a
                    href="https://id.atlassian.com/manage-profile/security/api-tokens"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-primary-500 hover:underline"
                  >
                    Atlassian API Tokens
                  </a>
                </li>
                <li>Click "Create API token"</li>
                <li>Give it a name (e.g., "QA PMS")</li>
                <li>Copy and paste the token here</li>
              </ol>
            </div>
          </>
        )}

        {/* OAuth Fields */}
        {authMethod === "oauth" && (
          <>
            <div>
              <label
                htmlFor="clientId"
                className="block text-sm font-medium text-neutral-700 mb-1"
              >
                OAuth Client ID <span className="text-error-500">*</span>
              </label>
              <input
                id="clientId"
                type="text"
                value={clientId}
                onChange={(e) => handleFieldChange(setClientId)(e.target.value)}
                placeholder="Your OAuth 2.0 Client ID"
                className="w-full px-3 py-2 border border-neutral-300 rounded-lg
                  text-neutral-900 placeholder:text-neutral-400
                  focus:ring-2 focus:ring-primary-500 focus:border-primary-500
                  transition-colors"
              />
            </div>

            <div>
              <label
                htmlFor="clientSecret"
                className="block text-sm font-medium text-neutral-700 mb-1"
              >
                OAuth Client Secret <span className="text-error-500">*</span>
              </label>
              <div className="relative">
                <input
                  id="clientSecret"
                  type={showSecret ? "text" : "password"}
                  value={clientSecret}
                  onChange={(e) =>
                    handleFieldChange(setClientSecret)(e.target.value)
                  }
                  placeholder="Your OAuth 2.0 Client Secret"
                  className="w-full px-3 py-2 pr-10 border border-neutral-300 rounded-lg
                    text-neutral-900 placeholder:text-neutral-400
                    focus:ring-2 focus:ring-primary-500 focus:border-primary-500
                    transition-colors"
                />
                <button
                  type="button"
                  onClick={() => setShowSecret(!showSecret)}
                  className="absolute right-3 top-1/2 -translate-y-1/2 text-neutral-500
                    hover:text-neutral-700 transition-colors"
                  aria-label={showSecret ? "Hide secret" : "Show secret"}
                >
                  {showSecret ? (
                    <EyeClosedIcon className="w-5 h-5" />
                  ) : (
                    <EyeOpenIcon className="w-5 h-5" />
                  )}
                </button>
              </div>
            </div>

            {/* OAuth Help */}
            <div className="p-4 bg-neutral-50 rounded-lg text-sm text-neutral-600">
              <p className="font-medium mb-2">How to get OAuth credentials:</p>
              <ol className="list-decimal list-inside space-y-1">
                <li>
                  Go to{" "}
                  <a
                    href="https://developer.atlassian.com/console/myapps/"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-primary-500 hover:underline"
                  >
                    Atlassian Developer Console
                  </a>
                </li>
                <li>Create a new OAuth 2.0 integration</li>
                <li>Add the required scopes for Jira access</li>
                <li>Copy the Client ID and Client Secret</li>
              </ol>
            </div>
          </>
        )}

        {/* Test Connection */}
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

        {/* Connection Status */}
        <ConnectionStatus
          state={connectionState}
          errorMessage={errorMessage}
          successMessage="Connected to Jira"
          successDetails={
            projectCount ? `Found ${projectCount} projects` : undefined
          }
        />
      </div>

      <WizardNavigation isValid={hasTestedSuccessfully} />
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

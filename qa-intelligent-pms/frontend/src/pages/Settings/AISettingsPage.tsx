/**
 * AI Provider Configuration Settings Page (Story 13.1).
 * 
 * Allows users to configure their own AI provider (BYOK - Bring Your Own Key):
 * - Select provider (Anthropic, OpenAI, Deepseek, z.ai, Custom)
 * - Enter API key (masked)
 * - Select model
 * - Test connection
 * - Enable/disable AI features
 */
import { useState, useEffect } from "react";
import { useLayoutStore } from "@/stores/layoutStore";
import {
  useAIStatus,
  useAIConfig,
  useProviders,
  useConfigureAI,
  useTestConnection,
  useDisableAI,
} from "@/hooks/useAI";
import type { ProviderType, ModelInfo, ProviderModels } from "@/types";

function EyeOpenIcon({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        strokeWidth={2}
        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
      />
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        strokeWidth={2}
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
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        strokeWidth={2}
        d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21"
      />
    </svg>
  );
}

function CheckCircledIcon({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        strokeWidth={2}
        d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
      />
    </svg>
  );
}

function CrossCircledIcon({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        strokeWidth={2}
        d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
      />
    </svg>
  );
}

export function AISettingsPage() {
  const { setPageTitle } = useLayoutStore();

  // Hooks
  const { data: status } = useAIStatus();
  const { data: config, isLoading: isLoadingConfig } = useAIConfig();
  const { data: providersData } = useProviders();
  const configureMutation = useConfigureAI();
  const testMutation = useTestConnection();
  const disableMutation = useDisableAI();

  // Local state
  const [provider, setProvider] = useState<ProviderType | "">("");
  const [apiKey, setApiKey] = useState("");
  const [modelId, setModelId] = useState("");
  const [customBaseUrl, setCustomBaseUrl] = useState("");
  const [isEnabled, setIsEnabled] = useState(true);
  const [showApiKey, setShowApiKey] = useState(false);
  const [testResult, setTestResult] = useState<{
    success: boolean;
    message?: string | null;
  } | null>(null);

  // Get providers list
  const providers = providersData?.providers || [];
  const selectedProviderData = providers.find((p) => p.provider === provider);

  // Load existing config
  useEffect(() => {
    if (config) {
      setProvider(config.provider as ProviderType);
      setModelId(config.modelId);
      setCustomBaseUrl(config.customBaseUrl || "");
      setIsEnabled(config.enabled);
      // API key is never returned from backend for security
    }
  }, [config]);

  // Update page title
  useEffect(() => {
    setPageTitle("AI Configuration", "Configure your AI provider and API key");
    return () => setPageTitle("");
  }, [setPageTitle]);

  // Handle provider change
  const handleProviderChange = (newProvider: string) => {
    setProvider(newProvider as ProviderType);
    setModelId("");
    setApiKey("");
    setTestResult(null);
  };

  // Handle test connection
  const handleTest = async () => {
    if (!provider || (!apiKey && !config)) {
      return;
    }

    // For testing, we need the API key
    if (!apiKey && config) {
      // Can't test without API key - user needs to re-enter it
      setTestResult({
        success: false,
        message: "Please enter your API key to test the connection",
      });
      return;
    }

    try {
      const result = await testMutation.mutateAsync({
        provider,
        apiKey: apiKey || "",
        modelId: modelId || selectedProviderData?.models[0]?.id || "",
        customBaseUrl: provider === "custom" ? customBaseUrl : null,
      });
      setTestResult(result);
    } catch (error) {
      setTestResult({
        success: false,
        message: error instanceof Error ? error.message : "Connection test failed",
      });
    }
  };

  // Handle save configuration
  const handleSave = async () => {
    if (!provider) {
      return;
    }

    if (!apiKey && !config) {
      // Can't save without API key (unless updating other fields)
      return;
    }

    try {
      await configureMutation.mutateAsync({
        provider,
        apiKey: apiKey || "", // Backend will validate
        modelId: modelId || selectedProviderData?.models[0]?.id || "",
        customBaseUrl: provider === "custom" ? customBaseUrl : null,
      });
      setTestResult(null);
      setApiKey(""); // Clear API key after successful save (security)
    } catch (error) {
      // Error handled by hook
    }
  };

  // Handle disable AI
  const handleDisable = async () => {
    if (window.confirm("Are you sure you want to disable AI features? This will revert to basic mode.")) {
      await disableMutation.mutateAsync();
      setIsEnabled(false);
    }
  };

  // Validation
  const canTest =
    provider &&
    (apiKey || config) &&
    (modelId || selectedProviderData?.models.length === 0) &&
    (provider !== "custom" || customBaseUrl);

  const canSave =
    provider &&
    (apiKey || config) &&
    (modelId || selectedProviderData?.models.length === 0) &&
    (provider !== "custom" || customBaseUrl);

  // Get provider display name
  const getProviderName = (p: ProviderType): string => {
    const names: Record<ProviderType, string> = {
      anthropic: "Anthropic (Claude)",
      open_ai: "OpenAI",
      deepseek: "DeepSeek",
      zai: "z.ai",
      custom: "Custom (OpenAI Compatible)",
    };
    return names[p] || p;
  };

  return (
    <div className="max-w-3xl mx-auto p-6 space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-2xl font-bold text-neutral-900">AI Configuration</h1>
        <p className="text-neutral-500 mt-1">
          Configure your AI provider for enhanced features. Your API key is encrypted and stored securely.
        </p>
      </div>

      {/* Current Status */}
      {status && (
        <div
          className={`p-4 rounded-lg border ${
            status.available
              ? "bg-success-50 border-success-200 text-success-800"
              : "bg-neutral-50 border-neutral-200 text-neutral-700"
          }`}
        >
          <div className="flex items-center gap-2">
            {status.available ? (
              <CheckCircledIcon className="w-5 h-5" />
            ) : (
              <CrossCircledIcon className="w-5 h-5" />
            )}
            <div>
              <p className="font-medium">{status.message}</p>
              {status.provider && status.model && (
                <p className="text-sm mt-1">
                  Provider: {getProviderName(status.provider as ProviderType)} | Model: {status.model}
                </p>
              )}
            </div>
          </div>
        </div>
      )}

      {/* Enable/Disable Toggle */}
      <div className="bg-white border border-neutral-200 rounded-lg p-6">
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-lg font-semibold text-neutral-900">AI Features</h2>
            <p className="text-sm text-neutral-500 mt-1">
              {isEnabled ? "AI features are enabled" : "Using basic mode (no AI)"}
            </p>
          </div>
          <label className="relative inline-flex items-center cursor-pointer">
            <input
              type="checkbox"
              checked={isEnabled}
              onChange={(e) => {
                if (!e.target.checked) {
                  handleDisable();
                } else {
                  setIsEnabled(true);
                }
              }}
              disabled={disableMutation.isPending || !config}
              className="sr-only peer"
            />
            <div className="w-11 h-6 bg-neutral-300 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-primary-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-neutral-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary-600"></div>
          </label>
        </div>
      </div>

      {/* Configuration Form */}
      {isEnabled && (
        <div className="bg-white border border-neutral-200 rounded-lg p-6 space-y-6">
          {/* Provider Selection */}
          <div>
            <label className="block text-sm font-medium text-neutral-700 mb-2">
              AI Provider <span className="text-error-500">*</span>
            </label>
            <select
              value={provider}
              onChange={(e) => handleProviderChange(e.target.value)}
              className="w-full px-3 py-2 border border-neutral-300 rounded-lg
                text-neutral-900
                focus:ring-2 focus:ring-primary-500 focus:border-primary-500
                transition-colors"
            >
              <option value="">Select a provider...</option>
              {providers.map((p) => (
                <option key={p.provider} value={p.provider}>
                  {getProviderName(p.provider)}
                </option>
              ))}
            </select>
          </div>

          {/* API Key Input */}
          {provider && (
            <div>
              <label className="block text-sm font-medium text-neutral-700 mb-2">
                API Key <span className="text-error-500">*</span>
              </label>
              <div className="relative">
                <input
                  type={showApiKey ? "text" : "password"}
                  value={apiKey}
                  onChange={(e) => {
                    setApiKey(e.target.value);
                    setTestResult(null);
                  }}
                  placeholder={config?.provider === provider ? "Enter new API key or leave blank to keep current" : "Enter your API key"}
                  className="w-full px-3 py-2 pr-10 border border-neutral-300 rounded-lg
                    text-neutral-900 placeholder:text-neutral-400
                    focus:ring-2 focus:ring-primary-500 focus:border-primary-500
                    transition-colors"
                />
                <button
                  type="button"
                  onClick={() => setShowApiKey(!showApiKey)}
                  className="absolute right-3 top-1/2 -translate-y-1/2 text-neutral-500
                    hover:text-neutral-700 transition-colors"
                  aria-label={showApiKey ? "Hide API key" : "Show API key"}
                >
                  {showApiKey ? (
                    <EyeClosedIcon className="w-5 h-5" />
                  ) : (
                    <EyeOpenIcon className="w-5 h-5" />
                  )}
                </button>
              </div>
              <p className="text-xs text-neutral-500 mt-1">
                Your API key is encrypted before storage. {config?.provider === provider && "Leave blank to keep current key."}
              </p>
            </div>
          )}

          {/* Model Selection */}
          {selectedProviderData && selectedProviderData.models.length > 0 && (
            <div>
              <label className="block text-sm font-medium text-neutral-700 mb-2">
                Model <span className="text-error-500">*</span>
              </label>
              <select
                value={modelId}
                onChange={(e) => setModelId(e.target.value)}
                className="w-full px-3 py-2 border border-neutral-300 rounded-lg
                  text-neutral-900
                  focus:ring-2 focus:ring-primary-500 focus:border-primary-500
                  transition-colors"
              >
                <option value="">Select a model...</option>
                {selectedProviderData.models.map((model: ModelInfo) => (
                  <option key={model.id} value={model.id}>
                    {model.name} ({model.contextWindow.toLocaleString()} tokens)
                  </option>
                ))}
              </select>
            </div>
          )}

          {/* Custom Endpoint (for Custom provider) */}
          {provider === "custom" && (
            <div>
              <label className="block text-sm font-medium text-neutral-700 mb-2">
                Custom Base URL <span className="text-error-500">*</span>
              </label>
              <input
                type="url"
                value={customBaseUrl}
                onChange={(e) => setCustomBaseUrl(e.target.value)}
                placeholder="https://your-api.example.com/v1"
                className="w-full px-3 py-2 border border-neutral-300 rounded-lg
                  text-neutral-900 placeholder:text-neutral-400
                  focus:ring-2 focus:ring-primary-500 focus:border-primary-500
                  transition-colors"
              />
              <p className="text-xs text-neutral-500 mt-1">
                Base URL for your OpenAI-compatible API endpoint
              </p>
            </div>
          )}

          {/* Test Result */}
          {testResult && (
            <div
              className={`p-4 rounded-lg border ${
                testResult.success
                  ? "bg-success-50 border-success-200 text-success-800"
                  : "bg-error-50 border-error-200 text-error-800"
              }`}
            >
              <div className="flex items-center gap-2">
                {testResult.success ? (
                  <CheckCircledIcon className="w-5 h-5" />
                ) : (
                  <CrossCircledIcon className="w-5 h-5" />
                )}
                <div>
                  <p className="font-medium">
                    {testResult.success ? "Connection successful!" : "Connection failed"}
                  </p>
                  {testResult.message && (
                    <p className="text-sm mt-1">{testResult.message}</p>
                  )}
                </div>
              </div>
            </div>
          )}

          {/* Actions */}
          <div className="flex items-center gap-4 pt-4 border-t border-neutral-200">
            <button
              onClick={handleTest}
              disabled={!canTest || testMutation.isPending || isLoadingConfig}
              className="px-4 py-2 border border-neutral-300 rounded-lg
                hover:bg-neutral-50 disabled:opacity-50 disabled:cursor-not-allowed
                transition-colors"
            >
              {testMutation.isPending ? "Testing..." : "Test Connection"}
            </button>
            <button
              onClick={handleSave}
              disabled={!canSave || configureMutation.isPending || isLoadingConfig}
              className="px-4 py-2 bg-primary-500 text-white rounded-lg
                hover:bg-primary-600 disabled:opacity-50 disabled:cursor-not-allowed
                transition-colors"
            >
              {configureMutation.isPending ? "Saving..." : "Save Configuration"}
            </button>
          </div>
        </div>
      )}

      {/* Help Text */}
      <div className="bg-neutral-50 border border-neutral-200 rounded-lg p-4 text-sm text-neutral-600">
        <p className="font-medium mb-2">How to get API keys:</p>
        <ul className="list-disc list-inside space-y-1">
          <li>
            <strong>Anthropic:</strong>{" "}
            <a
              href="https://console.anthropic.com/"
              target="_blank"
              rel="noopener noreferrer"
              className="text-primary-600 hover:underline"
            >
              console.anthropic.com
            </a>
          </li>
          <li>
            <strong>OpenAI:</strong>{" "}
            <a
              href="https://platform.openai.com/api-keys"
              target="_blank"
              rel="noopener noreferrer"
              className="text-primary-600 hover:underline"
            >
              platform.openai.com/api-keys
            </a>
          </li>
          <li>
            <strong>DeepSeek:</strong>{" "}
            <a
              href="https://platform.deepseek.com/"
              target="_blank"
              rel="noopener noreferrer"
              className="text-primary-600 hover:underline"
            >
              platform.deepseek.com
            </a>
          </li>
        </ul>
      </div>
    </div>
  );
}

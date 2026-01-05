# Story 13.1: AI Provider Configuration (BYOK)

Status: ready-for-dev

## Story

As a user,
I want to configure my own AI provider,
So that I can use AI features with my API key.

## Acceptance Criteria

1. **Given** user opens AI settings
   **When** user configures AI provider
   **Then** provider selection is available (Anthropic, OpenAI, Deepseek, z.ai, Custom)

2. **Given** provider is selected
   **When** configuration shown
   **Then** API Key input is available (masked)

3. **Given** provider is selected
   **When** configuration shown
   **Then** model selection is available (per provider)

4. **Given** API key entered
   **When** testing
   **Then** "Test Connection" button validates key

5. **Given** API key is saved
   **When** stored
   **Then** key is encrypted before storage (NFR-SEC-01)

6. **Given** test is executed
   **When** completed
   **Then** success/failure is shown clearly

7. **Given** AI is configured
   **When** user wants to disable
   **Then** can disable AI anytime (reverts to basic mode)

## Tasks

- [ ] Task 1: Create AISettings page component
- [ ] Task 2: Create ProviderSelector component
- [ ] Task 3: Create APIKeyInput with masking
- [ ] Task 4: Create ModelSelector per provider
- [ ] Task 5: Implement test connection API
- [ ] Task 6: Encrypt API keys before storage
- [ ] Task 7: Create disable AI toggle

## Dev Notes

### Database Schema

```sql
-- AI configuration per user
CREATE TABLE ai_configurations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id VARCHAR(255) UNIQUE NOT NULL,
    provider VARCHAR(50) NOT NULL,
    api_key_encrypted BYTEA NOT NULL,
    model VARCHAR(100),
    custom_endpoint VARCHAR(500),
    is_enabled BOOLEAN DEFAULT true,
    last_tested_at TIMESTAMPTZ,
    test_status VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### Provider Configuration

```rust
// crates/qa-pms-ai/src/config.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIProviderConfig {
    pub id: String,
    pub name: String,
    pub models: Vec<ModelOption>,
    pub requires_custom_endpoint: bool,
    pub documentation_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelOption {
    pub id: String,
    pub name: String,
    pub description: String,
    pub max_tokens: i32,
}

pub fn get_supported_providers() -> Vec<AIProviderConfig> {
    vec![
        AIProviderConfig {
            id: "anthropic".into(),
            name: "Anthropic (Claude)".into(),
            models: vec![
                ModelOption {
                    id: "claude-3-5-sonnet-20241022".into(),
                    name: "Claude 3.5 Sonnet".into(),
                    description: "Best for complex tasks".into(),
                    max_tokens: 200_000,
                },
                ModelOption {
                    id: "claude-3-haiku-20240307".into(),
                    name: "Claude 3 Haiku".into(),
                    description: "Fast and efficient".into(),
                    max_tokens: 200_000,
                },
            ],
            requires_custom_endpoint: false,
            documentation_url: "https://docs.anthropic.com".into(),
        },
        AIProviderConfig {
            id: "openai".into(),
            name: "OpenAI".into(),
            models: vec![
                ModelOption {
                    id: "gpt-4o".into(),
                    name: "GPT-4o".into(),
                    description: "Most capable model".into(),
                    max_tokens: 128_000,
                },
                ModelOption {
                    id: "gpt-4o-mini".into(),
                    name: "GPT-4o Mini".into(),
                    description: "Fast and cost-effective".into(),
                    max_tokens: 128_000,
                },
            ],
            requires_custom_endpoint: false,
            documentation_url: "https://platform.openai.com/docs".into(),
        },
        AIProviderConfig {
            id: "deepseek".into(),
            name: "DeepSeek".into(),
            models: vec![
                ModelOption {
                    id: "deepseek-chat".into(),
                    name: "DeepSeek Chat".into(),
                    description: "General purpose".into(),
                    max_tokens: 64_000,
                },
            ],
            requires_custom_endpoint: false,
            documentation_url: "https://platform.deepseek.com".into(),
        },
        AIProviderConfig {
            id: "custom".into(),
            name: "Custom (OpenAI Compatible)".into(),
            models: vec![],
            requires_custom_endpoint: true,
            documentation_url: "".into(),
        },
    ]
}
```

### AI Settings Page

```tsx
// frontend/src/pages/settings/AISettings.tsx
import { useState, useEffect } from "react";
import { useAIConfig, useSaveAIConfig, useTestAIConnection } from "@/hooks/useAI";

export function AISettingsPage() {
  const { data: config, isLoading } = useAIConfig();
  const { mutate: saveConfig, isPending: isSaving } = useSaveAIConfig();
  const { mutate: testConnection, isPending: isTesting, data: testResult } = useTestAIConnection();

  const [provider, setProvider] = useState("");
  const [apiKey, setApiKey] = useState("");
  const [model, setModel] = useState("");
  const [customEndpoint, setCustomEndpoint] = useState("");
  const [isEnabled, setIsEnabled] = useState(true);

  const providers = useSupportedProviders();
  const selectedProvider = providers.find(p => p.id === provider);

  useEffect(() => {
    if (config) {
      setProvider(config.provider);
      setModel(config.model || "");
      setCustomEndpoint(config.customEndpoint || "");
      setIsEnabled(config.isEnabled);
      // API key is never returned from backend
    }
  }, [config]);

  const handleSave = () => {
    saveConfig({
      provider,
      apiKey: apiKey || undefined, // Only send if changed
      model,
      customEndpoint: provider === "custom" ? customEndpoint : undefined,
      isEnabled,
    });
  };

  const handleTest = () => {
    testConnection({
      provider,
      apiKey: apiKey || undefined,
      model,
      customEndpoint,
    });
  };

  return (
    <div className="max-w-2xl mx-auto p-6">
      <div className="mb-8">
        <h1 className="text-2xl font-bold">AI Configuration</h1>
        <p className="text-neutral-500 mt-1">
          Configure your AI provider for enhanced features. Your API key is encrypted and stored securely.
        </p>
      </div>

      {/* Enable Toggle */}
      <div className="flex items-center justify-between p-4 bg-neutral-50 rounded-lg mb-6">
        <div>
          <p className="font-medium">AI Features</p>
          <p className="text-sm text-neutral-500">
            {isEnabled ? "AI features are enabled" : "Using basic mode (no AI)"}
          </p>
        </div>
        <Switch checked={isEnabled} onCheckedChange={setIsEnabled} />
      </div>

      {isEnabled && (
        <div className="space-y-6">
          {/* Provider Selection */}
          <div>
            <label className="block text-sm font-medium text-neutral-700 mb-2">
              AI Provider
            </label>
            <select
              value={provider}
              onChange={(e) => {
                setProvider(e.target.value);
                setModel("");
                setApiKey("");
              }}
              className="w-full px-3 py-2 border border-neutral-300 rounded-lg"
            >
              <option value="">Select a provider...</option>
              {providers.map((p) => (
                <option key={p.id} value={p.id}>{p.name}</option>
              ))}
            </select>
          </div>

          {/* API Key */}
          {provider && (
            <div>
              <label className="block text-sm font-medium text-neutral-700 mb-2">
                API Key
              </label>
              <div className="relative">
                <input
                  type="password"
                  value={apiKey}
                  onChange={(e) => setApiKey(e.target.value)}
                  placeholder={config?.provider === provider ? "••••••••••••••••" : "Enter your API key"}
                  className="w-full px-3 py-2 border border-neutral-300 rounded-lg pr-24"
                />
                <button
                  type="button"
                  onClick={() => {/* toggle visibility */}}
                  className="absolute right-2 top-1/2 -translate-y-1/2 text-sm text-neutral-500"
                >
                  Show
                </button>
              </div>
              {selectedProvider?.documentationUrl && (
                <a
                  href={selectedProvider.documentationUrl}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-sm text-primary-600 hover:underline mt-1 inline-block"
                >
                  Get API key →
                </a>
              )}
            </div>
          )}

          {/* Model Selection */}
          {selectedProvider && selectedProvider.models.length > 0 && (
            <div>
              <label className="block text-sm font-medium text-neutral-700 mb-2">
                Model
              </label>
              <select
                value={model}
                onChange={(e) => setModel(e.target.value)}
                className="w-full px-3 py-2 border border-neutral-300 rounded-lg"
              >
                <option value="">Select a model...</option>
                {selectedProvider.models.map((m) => (
                  <option key={m.id} value={m.id}>
                    {m.name} - {m.description}
                  </option>
                ))}
              </select>
            </div>
          )}

          {/* Custom Endpoint */}
          {provider === "custom" && (
            <div>
              <label className="block text-sm font-medium text-neutral-700 mb-2">
                Custom Endpoint
              </label>
              <input
                type="url"
                value={customEndpoint}
                onChange={(e) => setCustomEndpoint(e.target.value)}
                placeholder="https://your-api.example.com/v1"
                className="w-full px-3 py-2 border border-neutral-300 rounded-lg"
              />
            </div>
          )}

          {/* Test Result */}
          {testResult && (
            <div className={cn(
              "p-4 rounded-lg",
              testResult.success ? "bg-success-50 text-success-700" : "bg-error-50 text-error-700"
            )}>
              <div className="flex items-center gap-2">
                {testResult.success ? <CheckCircledIcon /> : <CrossCircledIcon />}
                <span className="font-medium">
                  {testResult.success ? "Connection successful!" : "Connection failed"}
                </span>
              </div>
              {testResult.message && (
                <p className="text-sm mt-1">{testResult.message}</p>
              )}
            </div>
          )}

          {/* Actions */}
          <div className="flex items-center gap-4 pt-4 border-t">
            <button
              onClick={handleTest}
              disabled={!provider || (!apiKey && !config?.provider) || isTesting}
              className="px-4 py-2 border border-neutral-300 rounded-lg hover:bg-neutral-50 disabled:opacity-50"
            >
              {isTesting ? "Testing..." : "Test Connection"}
            </button>
            <button
              onClick={handleSave}
              disabled={!provider || isSaving}
              className="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 disabled:opacity-50"
            >
              {isSaving ? "Saving..." : "Save Configuration"}
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
```

### Test Connection API

```rust
// POST /api/v1/ai/test
pub async fn test_ai_connection(
    State(state): State<Arc<AppState>>,
    Json(request): Json<TestConnectionRequest>,
) -> Result<Json<TestConnectionResponse>, ApiError> {
    let client = create_ai_client(&request)?;
    
    // Send minimal test request
    let result = client.complete("Say 'Hello' in one word.").await;

    match result {
        Ok(_) => Ok(Json(TestConnectionResponse {
            success: true,
            message: Some("Successfully connected and received response".into()),
        })),
        Err(e) => Ok(Json(TestConnectionResponse {
            success: false,
            message: Some(format!("Connection failed: {}", e)),
        })),
    }
}
```

### References

- [Source: epics.md#Story 13.1]
- [NFR: NFR-SEC-01 - Encrypt API keys]

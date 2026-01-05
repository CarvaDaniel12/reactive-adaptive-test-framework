# Story 13.6: AI Graceful Fallback

Status: ready-for-dev

## Story

As a user,
I want the app to work without AI,
So that I'm not blocked if AI is unavailable.

## Acceptance Criteria

1. **Given** AI is not configured or API fails
   **When** semantic search is triggered
   **Then** system falls back to keyword search automatically

2. **Given** AI is unavailable
   **When** test suggestions feature is accessed
   **Then** "Enable AI for suggestions" message with setup link is shown

3. **Given** AI is unavailable
   **When** chatbot is opened
   **Then** "AI not configured" message with setup link is displayed

4. **Given** any AI feature fails
   **When** error occurs
   **Then** no error blocks user workflow

5. **Given** fallback occurs
   **When** system switches to non-AI mode
   **Then** transition is silent (no disruptive error popups)

6. **Given** any feature with AI enhancement
   **When** AI unavailable
   **Then** feature degrades gracefully to basic mode

7. **Given** any part of the app
   **When** AI is down or misconfigured
   **Then** user can always complete their work without AI

## Tasks

- [ ] Task 1: Create AI availability checker service
- [ ] Task 2: Implement search fallback logic
- [ ] Task 3: Add graceful fallback to test suggestions
- [ ] Task 4: Add graceful fallback to chatbot
- [ ] Task 5: Create fallback UI components
- [ ] Task 6: Add AI status indicator
- [ ] Task 7: Write fallback tests

## Dev Notes

### AI Availability Service (Backend)

```rust
// crates/qa-pms-ai/src/availability.rs
use std::time::Duration;
use tokio::time::timeout;

pub struct AIAvailabilityService {
    db_pool: PgPool,
    timeout_duration: Duration,
}

#[derive(Debug, Clone)]
pub struct AIStatus {
    pub is_available: bool,
    pub is_configured: bool,
    pub last_check: DateTime<Utc>,
    pub error_message: Option<String>,
}

impl AIAvailabilityService {
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            db_pool,
            timeout_duration: Duration::from_secs(5), // 5s timeout for AI calls
        }
    }

    /// Check if AI is available for a user
    pub async fn check_availability(&self, user_id: &str) -> AIStatus {
        // Check if user has configured AI
        let config = match self.get_user_config(user_id).await {
            Ok(Some(config)) if config.is_enabled => config,
            Ok(Some(_)) => return AIStatus {
                is_available: false,
                is_configured: true,
                last_check: Utc::now(),
                error_message: Some("AI is disabled in settings".to_string()),
            },
            Ok(None) => return AIStatus {
                is_available: false,
                is_configured: false,
                last_check: Utc::now(),
                error_message: None,
            },
            Err(e) => return AIStatus {
                is_available: false,
                is_configured: false,
                last_check: Utc::now(),
                error_message: Some(format!("Failed to check config: {}", e)),
            },
        };

        // Quick health check with timeout
        match timeout(self.timeout_duration, self.ping_provider(&config)).await {
            Ok(Ok(_)) => AIStatus {
                is_available: true,
                is_configured: true,
                last_check: Utc::now(),
                error_message: None,
            },
            Ok(Err(e)) => AIStatus {
                is_available: false,
                is_configured: true,
                last_check: Utc::now(),
                error_message: Some(format!("API error: {}", e)),
            },
            Err(_) => AIStatus {
                is_available: false,
                is_configured: true,
                last_check: Utc::now(),
                error_message: Some("Request timeout".to_string()),
            },
        }
    }

    async fn get_user_config(&self, user_id: &str) -> Result<Option<AIConfiguration>> {
        sqlx::query_as!(
            AIConfiguration,
            r#"
            SELECT id, user_id, provider, api_key_encrypted, model, 
                   custom_endpoint, is_enabled, last_tested_at, test_status,
                   created_at, updated_at
            FROM ai_configurations
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(Into::into)
    }

    async fn ping_provider(&self, config: &AIConfiguration) -> Result<()> {
        let client = create_ai_client(config)?;
        // Simple ping - just verify connection works
        client.ping().await
    }
}
```

### Search Fallback Logic

```rust
// crates/qa-pms-api/src/services/search.rs
use qa_pms_ai::availability::AIAvailabilityService;

pub struct SearchService {
    ai_availability: AIAvailabilityService,
    semantic_search: SemanticSearchService,
    keyword_search: KeywordSearchService,
}

impl SearchService {
    /// Execute search with automatic fallback
    pub async fn search(
        &self,
        user_id: &str,
        ticket: &JiraTicket,
    ) -> SearchResponse {
        let ai_status = self.ai_availability.check_availability(user_id).await;

        if ai_status.is_available {
            // Try semantic search
            match self.semantic_search.search(ticket).await {
                Ok(results) => {
                    return SearchResponse {
                        results: results.into(),
                        search_type: SearchType::Semantic,
                        fallback_reason: None,
                    };
                }
                Err(e) => {
                    tracing::warn!("Semantic search failed, falling back to keyword: {}", e);
                    // Fall through to keyword search
                }
            }
        }

        // Fallback to keyword search
        let results = self.keyword_search.search(ticket).await
            .unwrap_or_default();

        let fallback_reason = if !ai_status.is_configured {
            Some(FallbackReason::NotConfigured)
        } else if !ai_status.is_available {
            Some(FallbackReason::Unavailable(ai_status.error_message))
        } else {
            Some(FallbackReason::Error)
        };

        SearchResponse {
            results: results.into(),
            search_type: SearchType::Keyword,
            fallback_reason,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub search_type: SearchType,
    pub fallback_reason: Option<FallbackReason>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchType {
    Semantic,
    Keyword,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackReason {
    NotConfigured,
    Unavailable(Option<String>),
    Error,
}
```

### Frontend AI Status Hook

```tsx
// frontend/src/hooks/useAIStatus.ts
interface AIStatus {
  isAvailable: boolean;
  isConfigured: boolean;
  lastCheck: Date | null;
  errorMessage: string | null;
}

export function useAIStatus() {
  return useQuery<AIStatus>({
    queryKey: ["ai-status"],
    queryFn: async () => {
      const response = await fetch("/api/v1/ai/status");
      if (!response.ok) {
        // If status check fails, assume unavailable but don't throw
        return {
          isAvailable: false,
          isConfigured: false,
          lastCheck: new Date(),
          errorMessage: "Could not check AI status",
        };
      }
      return response.json();
    },
    staleTime: 60 * 1000, // Cache for 1 minute
    refetchOnWindowFocus: false,
    // Don't throw errors - always return a status
    retry: false,
  });
}

export function useAIFeature<T>(
  aiFeatureFn: () => Promise<T>,
  fallbackFn: () => Promise<T> | T,
  options?: {
    onFallback?: (reason: string) => void;
  }
) {
  const { data: aiStatus } = useAIStatus();

  return useMutation({
    mutationFn: async () => {
      if (!aiStatus?.isAvailable) {
        options?.onFallback?.("AI unavailable");
        return fallbackFn();
      }

      try {
        return await aiFeatureFn();
      } catch (error) {
        options?.onFallback?.("AI error");
        return fallbackFn();
      }
    },
  });
}
```

### AI Not Configured Component

```tsx
// frontend/src/components/ai/AINotConfigured.tsx
interface AINotConfiguredProps {
  feature: "search" | "suggestions" | "chatbot";
  compact?: boolean;
}

export function AINotConfigured({ feature, compact = false }: AINotConfiguredProps) {
  const navigate = useNavigate();

  const messages: Record<typeof feature, { title: string; description: string }> = {
    search: {
      title: "Using keyword search",
      description: "Enable AI for smarter semantic search",
    },
    suggestions: {
      title: "Enable AI for suggestions",
      description: "Get AI-powered test suggestions from acceptance criteria",
    },
    chatbot: {
      title: "AI not configured",
      description: "Set up your AI provider to use the assistant",
    },
  };

  const { title, description } = messages[feature];

  if (compact) {
    return (
      <div className="flex items-center gap-2 text-sm text-neutral-500">
        <InfoCircledIcon className="w-4 h-4" />
        <span>{title}</span>
        <button
          onClick={() => navigate("/settings/ai")}
          className="text-primary-600 hover:underline"
        >
          Configure
        </button>
      </div>
    );
  }

  return (
    <div className="p-6 text-center bg-neutral-50 rounded-lg border border-neutral-200">
      <SparklesIcon className="w-12 h-12 mx-auto text-neutral-300 mb-4" />
      <h3 className="font-medium text-neutral-900">{title}</h3>
      <p className="text-sm text-neutral-500 mt-1 mb-4">{description}</p>
      <button
        onClick={() => navigate("/settings/ai")}
        className="px-4 py-2 bg-primary-500 text-white text-sm font-medium rounded-lg hover:bg-primary-600"
      >
        Configure AI
      </button>
    </div>
  );
}
```

### Chatbot Fallback UI

```tsx
// frontend/src/components/ai/ChatbotPanel.tsx (updated)
export function ChatbotPanel() {
  const { data: aiStatus, isLoading: isCheckingAI } = useAIStatus();
  const {
    messages,
    isOpen,
    isMinimized,
    closeChat,
    toggleMinimize,
    sendMessage,
  } = useChatbot();

  if (!isOpen) return null;

  // Show fallback if AI not configured
  if (!isCheckingAI && !aiStatus?.isConfigured) {
    return (
      <div className={cn(
        "fixed bottom-6 right-6 bg-white rounded-xl shadow-2xl border border-neutral-200 z-50",
        "w-80 transition-all duration-200"
      )}>
        {/* Header */}
        <div className="flex items-center justify-between p-3 border-b border-neutral-200">
          <div className="flex items-center gap-2">
            <SparklesIcon className="w-5 h-5 text-neutral-400" />
            <span className="font-medium text-neutral-600">AI Assistant</span>
          </div>
          <button
            onClick={closeChat}
            className="p-1 text-neutral-400 hover:text-neutral-600 rounded"
          >
            <Cross2Icon className="w-4 h-4" />
          </button>
        </div>

        {/* Fallback Content */}
        <div className="p-6">
          <AINotConfigured feature="chatbot" />
        </div>
      </div>
    );
  }

  // Show degraded state if AI configured but unavailable
  if (!isCheckingAI && aiStatus?.isConfigured && !aiStatus?.isAvailable) {
    return (
      <div className={cn(
        "fixed bottom-6 right-6 bg-white rounded-xl shadow-2xl border border-neutral-200 z-50",
        "w-80 transition-all duration-200"
      )}>
        <div className="flex items-center justify-between p-3 border-b border-neutral-200">
          <div className="flex items-center gap-2">
            <SparklesIcon className="w-5 h-5 text-warning-500" />
            <span className="font-medium">AI Assistant</span>
            <span className="text-xs bg-warning-100 text-warning-700 px-2 py-0.5 rounded">
              Offline
            </span>
          </div>
          <button onClick={closeChat} className="p-1 text-neutral-400 hover:text-neutral-600 rounded">
            <Cross2Icon className="w-4 h-4" />
          </button>
        </div>

        <div className="p-6 text-center">
          <ExclamationTriangleIcon className="w-12 h-12 mx-auto text-warning-400 mb-4" />
          <h3 className="font-medium">AI Temporarily Unavailable</h3>
          <p className="text-sm text-neutral-500 mt-1">
            {aiStatus.errorMessage || "Unable to connect to AI provider"}
          </p>
          <p className="text-sm text-neutral-500 mt-4">
            Try again later or check your AI settings.
          </p>
          <button
            onClick={() => window.location.href = "/settings/ai"}
            className="mt-4 text-sm text-primary-600 hover:underline"
          >
            Check Settings
          </button>
        </div>
      </div>
    );
  }

  // Normal chatbot UI (existing code)
  return (
    // ... existing ChatbotPanel implementation
  );
}
```

### Test Suggestions Fallback

```tsx
// frontend/src/components/ai/TestSuggestionsPanel.tsx (updated)
export function TestSuggestionsPanel({ ticketKey, onAddToNotes }: TestSuggestionsPanelProps) {
  const { data: aiStatus } = useAIStatus();
  const { data: suggestions, isLoading, error } = useTestSuggestions(ticketKey, {
    enabled: aiStatus?.isAvailable,
  });

  // Not configured - show setup prompt
  if (!aiStatus?.isConfigured) {
    return (
      <div className="bg-white rounded-xl border border-neutral-200 p-4">
        <div className="flex items-center gap-2 mb-4">
          <LightningBoltIcon className="w-5 h-5 text-neutral-400" />
          <h3 className="font-semibold text-neutral-600">AI Test Suggestions</h3>
        </div>
        <AINotConfigured feature="suggestions" />
      </div>
    );
  }

  // Configured but unavailable - show degraded state
  if (!aiStatus?.isAvailable) {
    return (
      <div className="bg-white rounded-xl border border-neutral-200 p-4">
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center gap-2">
            <LightningBoltIcon className="w-5 h-5 text-warning-500" />
            <h3 className="font-semibold">AI Test Suggestions</h3>
            <span className="text-xs bg-warning-100 text-warning-700 px-2 py-0.5 rounded">
              Unavailable
            </span>
          </div>
        </div>
        <p className="text-sm text-neutral-500">
          AI suggestions are temporarily unavailable. You can still:
        </p>
        <ul className="mt-2 space-y-1 text-sm text-neutral-600">
          <li className="flex items-center gap-2">
            <CheckIcon className="w-4 h-4 text-success-500" />
            Search for existing tests manually
          </li>
          <li className="flex items-center gap-2">
            <CheckIcon className="w-4 h-4 text-success-500" />
            Add your own notes and test ideas
          </li>
          <li className="flex items-center gap-2">
            <CheckIcon className="w-4 h-4 text-success-500" />
            Complete your workflow normally
          </li>
        </ul>
      </div>
    );
  }

  // Loading state
  if (isLoading) {
    return (
      <div className="bg-white rounded-xl border border-neutral-200 p-4">
        <div className="flex items-center gap-2 mb-4">
          <LightningBoltIcon className="w-5 h-5 text-primary-500 animate-pulse" />
          <h3 className="font-semibold">Generating Test Suggestions...</h3>
        </div>
        <div className="space-y-2">
          {[1, 2, 3].map((i) => (
            <div key={i} className="h-16 bg-neutral-100 rounded-lg animate-pulse" />
          ))}
        </div>
      </div>
    );
  }

  // Error state (AI failed mid-request)
  if (error) {
    return (
      <div className="bg-white rounded-xl border border-neutral-200 p-4">
        <div className="flex items-center gap-2 mb-4">
          <LightningBoltIcon className="w-5 h-5 text-error-500" />
          <h3 className="font-semibold text-error-700">Suggestions Failed</h3>
        </div>
        <p className="text-sm text-neutral-500">
          Could not generate suggestions. This doesn't affect your workflow.
        </p>
      </div>
    );
  }

  // Normal suggestions UI
  return (
    // ... existing TestSuggestionsPanel implementation with suggestions
  );
}
```

### Search Results with Fallback Indicator

```tsx
// frontend/src/components/search/SearchResults.tsx (updated)
interface SearchResultsProps {
  results: SearchResult[];
  searchType: "semantic" | "keyword";
  fallbackReason?: FallbackReason;
  isLoading: boolean;
}

export function SearchResults({ 
  results, 
  searchType, 
  fallbackReason,
  isLoading 
}: SearchResultsProps) {
  return (
    <div className="bg-white rounded-xl border border-neutral-200">
      <div className="p-4 border-b border-neutral-200">
        <div className="flex items-center justify-between">
          <h3 className="font-semibold">Search Results</h3>
          <SearchTypeIndicator type={searchType} fallbackReason={fallbackReason} />
        </div>
      </div>

      <div className="p-4">
        {isLoading ? (
          <SearchSkeleton />
        ) : results.length === 0 ? (
          <EmptyResults searchType={searchType} />
        ) : (
          <ul className="space-y-3">
            {results.map((result) => (
              <SearchResultItem key={result.id} result={result} />
            ))}
          </ul>
        )}
      </div>
    </div>
  );
}

function SearchTypeIndicator({ 
  type, 
  fallbackReason 
}: { 
  type: "semantic" | "keyword";
  fallbackReason?: FallbackReason;
}) {
  if (type === "semantic") {
    return (
      <Tooltip content="AI-powered semantic search">
        <div className="flex items-center gap-1 text-xs text-primary-600 bg-primary-50 px-2 py-1 rounded-full">
          <SparklesIcon className="w-3 h-3" />
          <span>Semantic</span>
        </div>
      </Tooltip>
    );
  }

  const tooltipContent = fallbackReason === "not_configured"
    ? "Configure AI for smarter search"
    : fallbackReason === "unavailable"
    ? "AI temporarily unavailable, using keyword search"
    : "Using keyword-based search";

  return (
    <Tooltip content={tooltipContent}>
      <div className="flex items-center gap-1 text-xs text-neutral-500 bg-neutral-100 px-2 py-1 rounded-full">
        <MagnifyingGlassIcon className="w-3 h-3" />
        <span>Keyword</span>
        {fallbackReason === "not_configured" && (
          <Link to="/settings/ai" className="text-primary-600 hover:underline ml-1">
            Enable AI
          </Link>
        )}
      </div>
    </Tooltip>
  );
}
```

### AI Status API Endpoint

```rust
// crates/qa-pms-api/src/routes/ai.rs
// GET /api/v1/ai/status
pub async fn get_ai_status(
    State(state): State<Arc<AppState>>,
    // Extract user from auth middleware
) -> Result<Json<AIStatusResponse>, ApiError> {
    let ai_service = AIAvailabilityService::new(state.db_pool.clone());
    let status = ai_service.check_availability("current_user").await;

    Ok(Json(AIStatusResponse {
        is_available: status.is_available,
        is_configured: status.is_configured,
        last_check: status.last_check,
        error_message: status.error_message,
    }))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AIStatusResponse {
    pub is_available: bool,
    pub is_configured: bool,
    pub last_check: DateTime<Utc>,
    pub error_message: Option<String>,
}
```

### Global Error Boundary for AI Features

```tsx
// frontend/src/components/ai/AIErrorBoundary.tsx
interface AIErrorBoundaryProps {
  feature: "search" | "suggestions" | "chatbot";
  children: React.ReactNode;
  fallback?: React.ReactNode;
}

interface State {
  hasError: boolean;
}

export class AIErrorBoundary extends React.Component<AIErrorBoundaryProps, State> {
  constructor(props: AIErrorBoundaryProps) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(): State {
    return { hasError: true };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    // Log AI-specific errors without blocking
    console.error(`AI feature error (${this.props.feature}):`, error, errorInfo);
    
    // Don't report AI errors to error tracking as critical
    // They are expected to fail sometimes
  }

  render() {
    if (this.state.hasError) {
      return this.props.fallback || (
        <div className="p-4 bg-neutral-50 rounded-lg text-center">
          <p className="text-sm text-neutral-500">
            This AI feature encountered an error.
          </p>
          <button
            onClick={() => this.setState({ hasError: false })}
            className="mt-2 text-sm text-primary-600 hover:underline"
          >
            Try again
          </button>
        </div>
      );
    }

    return this.props.children;
  }
}
```

### Integration in App

```tsx
// frontend/src/App.tsx
function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ChatbotProvider>
        <Router>
          <Layout>
            <Routes>
              {/* ... routes ... */}
            </Routes>
          </Layout>
          
          {/* Chatbot always available, handles its own fallback */}
          <AIErrorBoundary feature="chatbot" fallback={null}>
            <ChatbotButton />
            <ChatbotPanel />
          </AIErrorBoundary>
        </Router>
      </ChatbotProvider>
    </QueryClientProvider>
  );
}
```

### Key Design Principles

1. **Silent Fallback**: AI failures should never show disruptive error dialogs. Log to console, use inline indicators.

2. **Feature Degradation**: Each AI feature has a clear non-AI alternative:
   - Semantic search → Keyword search (automatic)
   - Test suggestions → Manual test creation (user continues normally)
   - Chatbot → Setup prompt or offline message (no workflow impact)

3. **Status Caching**: AI status is cached for 1 minute to avoid repeated checks while still detecting recovery.

4. **User Workflow Priority**: Users can always complete their work. AI is enhancement, not requirement.

5. **Clear Communication**: When AI is unavailable, users see:
   - What's happening (AI offline/not configured)
   - What they can do (setup link or alternative actions)
   - That their workflow isn't blocked

### References

- [Source: epics.md#Story 13.6]
- Implements: FR-AI-07 (Graceful fallback when AI unavailable)

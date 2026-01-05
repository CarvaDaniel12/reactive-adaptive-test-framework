# Story 13.5: Chatbot Contextual Awareness

Status: ready-for-dev

## Story

As a user,
I want the chatbot to know where I am,
So that answers are relevant to my current context.

## Acceptance Criteria

1. **Given** chatbot is open
   **When** user asks a question
   **Then** AI receives current page/view context

2. **Given** context is sent
   **When** viewing a ticket
   **Then** AI receives current ticket info (if viewing one)

3. **Given** context is sent
   **When** in workflow
   **Then** AI receives current workflow step (if in workflow)

4. **Given** context is sent
   **When** question asked
   **Then** AI receives user's recent actions

5. **Given** response is generated
   **When** displayed
   **Then** responses reference current context

6. **Given** helpful question
   **When** asked
   **Then** can ask "What should I do next?" and get relevant answer

7. **Given** privacy concerns
   **When** session ends
   **Then** context is not stored after session (privacy)

## Tasks

- [ ] Task 1: Create context extraction hook
- [ ] Task 2: Implement page context detection
- [ ] Task 3: Add ticket context extraction
- [ ] Task 4: Add workflow context extraction
- [ ] Task 5: Create recent actions tracking
- [ ] Task 6: Update chat API with context
- [ ] Task 7: Ensure no context persistence

## Dev Notes

### Context Extraction Hook

```tsx
// frontend/src/hooks/usePageContext.ts
import { useLocation, useParams } from "react-router-dom";
import { useWorkflowStore } from "@/stores/workflowStore";

interface PageContext {
  pageName: string;
  pageType: string;
  ticketKey?: string;
  ticketTitle?: string;
  workflowStep?: WorkflowStepContext;
  recentActions: RecentAction[];
}

interface WorkflowStepContext {
  stepIndex: number;
  stepName: string;
  totalSteps: number;
  status: string;
}

interface RecentAction {
  action: string;
  target: string;
  timestamp: Date;
}

export function usePageContext(): PageContext {
  const location = useLocation();
  const params = useParams();
  const workflowStore = useWorkflowStore();
  const recentActions = useRecentActions();

  const getPageInfo = () => {
    const path = location.pathname;
    
    if (path.startsWith("/tickets/")) {
      return { pageName: "Ticket View", pageType: "ticket" };
    }
    if (path.startsWith("/dashboard")) {
      return { pageName: "Dashboard", pageType: "dashboard" };
    }
    if (path.startsWith("/pm-dashboard")) {
      return { pageName: "PM Dashboard", pageType: "pm_dashboard" };
    }
    if (path.startsWith("/reports")) {
      return { pageName: "Reports", pageType: "reports" };
    }
    if (path.startsWith("/settings")) {
      return { pageName: "Settings", pageType: "settings" };
    }
    if (path.startsWith("/admin")) {
      return { pageName: "Admin", pageType: "admin" };
    }
    
    return { pageName: "Home", pageType: "home" };
  };

  const pageInfo = getPageInfo();

  const context: PageContext = {
    ...pageInfo,
    recentActions: recentActions.slice(0, 5),
  };

  // Add ticket context if on ticket page
  if (pageInfo.pageType === "ticket" && params.ticketKey) {
    context.ticketKey = params.ticketKey;
    // Would get from a hook/store
    // context.ticketTitle = useTicket(params.ticketKey).data?.summary;
  }

  // Add workflow context if active
  if (workflowStore.isActive) {
    context.workflowStep = {
      stepIndex: workflowStore.currentStep,
      stepName: workflowStore.steps[workflowStore.currentStep]?.name || "",
      totalSteps: workflowStore.steps.length,
      status: workflowStore.status,
    };
  }

  return context;
}
```

### Recent Actions Tracking

```tsx
// frontend/src/hooks/useRecentActions.ts
const MAX_ACTIONS = 10;
const actionsStore: RecentAction[] = [];

export function trackAction(action: string, target: string) {
  actionsStore.unshift({
    action,
    target,
    timestamp: new Date(),
  });
  
  // Keep only last N actions
  if (actionsStore.length > MAX_ACTIONS) {
    actionsStore.pop();
  }
}

export function useRecentActions(): RecentAction[] {
  return [...actionsStore];
}

// Usage in components:
// trackAction("viewed", "ticket ABC-123");
// trackAction("completed", "step: Environment Setup");
// trackAction("generated", "report for ABC-123");
```

### Context-Aware Chat API

```rust
// POST /api/v1/ai/chat (updated)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatRequest {
    pub message: String,
    pub history: Vec<ChatMessage>,
    pub context: Option<PageContext>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageContext {
    pub page_name: String,
    pub page_type: String,
    pub ticket_key: Option<String>,
    pub ticket_title: Option<String>,
    pub workflow_step: Option<WorkflowStepContext>,
    pub recent_actions: Vec<RecentAction>,
}

pub async fn chat_with_context(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, ApiError> {
    let ai_config = get_user_ai_config(&state.db_pool, "current_user").await?;
    let client = create_ai_client(&ai_config)?;

    // Build context-aware system prompt
    let system_prompt = build_contextual_prompt(&request.context);
    
    let messages = build_messages(&system_prompt, &request.history, &request.message);
    let response = client.chat(&messages).await?;

    Ok(Json(ChatResponse { response }))
}

fn build_contextual_prompt(context: &Option<PageContext>) -> String {
    let mut prompt = String::from(
        r#"You are a helpful assistant for "QA Intelligent PMS", a QA workflow and testing framework.

You help users with:
- Navigating the interface and using features
- Understanding workflow processes
- Providing testing tips and best practices
- Answering questions about their current task

Be concise, helpful, and context-aware."#
    );

    if let Some(ctx) = context {
        prompt.push_str(&format!("\n\n## Current Context"));
        prompt.push_str(&format!("\nPage: {} ({})", ctx.page_name, ctx.page_type));

        if let Some(ticket) = &ctx.ticket_key {
            prompt.push_str(&format!("\nViewing ticket: {}", ticket));
            if let Some(title) = &ctx.ticket_title {
                prompt.push_str(&format!(" - {}", title));
            }
        }

        if let Some(step) = &ctx.workflow_step {
            prompt.push_str(&format!(
                "\nIn workflow: Step {} of {} - '{}' (Status: {})",
                step.step_index + 1,
                step.total_steps,
                step.step_name,
                step.status
            ));
        }

        if !ctx.recent_actions.is_empty() {
            prompt.push_str("\n\nRecent actions:");
            for action in ctx.recent_actions.iter().take(3) {
                prompt.push_str(&format!("\n- {} {}", action.action, action.target));
            }
        }
    }

    prompt.push_str("\n\nIf the user asks \"What should I do next?\", use the context above to give specific guidance for their current situation.");

    prompt
}
```

### Contextual Suggestions

```tsx
// frontend/src/components/ai/ContextualSuggestions.tsx
export function ContextualSuggestions({ onSelect }: { onSelect: (q: string) => void }) {
  const context = usePageContext();
  
  const suggestions = useMemo(() => {
    const base = ["What should I do next?"];
    
    if (context.pageType === "ticket") {
      return [
        "How do I start testing this ticket?",
        "What tests are relevant for this ticket?",
        ...base
      ];
    }
    
    if (context.workflowStep) {
      return [
        `What should I check in the "${context.workflowStep.stepName}" step?`,
        "What are common issues at this stage?",
        ...base
      ];
    }
    
    if (context.pageType === "dashboard") {
      return [
        "How do I improve my efficiency?",
        "What do these metrics mean?",
        ...base
      ];
    }
    
    return [
      "How do I use this framework?",
      "What features are available?",
      ...base
    ];
  }, [context]);

  return (
    <div className="space-y-2">
      {suggestions.map((q) => (
        <button
          key={q}
          onClick={() => onSelect(q)}
          className="w-full text-left px-3 py-2 text-sm bg-neutral-50 rounded-lg hover:bg-neutral-100"
        >
          {q}
        </button>
      ))}
    </div>
  );
}
```

### Privacy: No Persistence

- Context is only held in memory during session
- No context is stored in database
- No context is logged to server logs
- Chat history is cleared on page refresh

```tsx
// Context provider resets on unmount
useEffect(() => {
  return () => {
    // Clear all context and history on unmount
    setMessages([]);
  };
}, []);
```

### References

- [Source: epics.md#Story 13.5]

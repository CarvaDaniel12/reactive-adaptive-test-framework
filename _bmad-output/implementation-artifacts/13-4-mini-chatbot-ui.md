# Story 13.4: Mini-Chatbot UI

Status: ready-for-dev

## Story

As a user,
I want a chatbot for framework questions,
So that I can get help without leaving the app.

## Acceptance Criteria

1. **Given** user has AI configured
   **When** user opens chatbot (icon in corner)
   **Then** chat interface shows message history (session-based)

2. **Given** chat interface is open
   **When** rendered
   **Then** text input with send button is available

3. **Given** chat interface is open
   **When** context available
   **Then** "Ask about this page" context button is shown

4. **Given** chat interface is open
   **When** user interacts
   **Then** close/minimize button is available

5. **Given** chatbot is not open
   **When** UI renders
   **Then** chatbot icon is always visible (corner position)

6. **Given** keyboard interaction
   **When** shortcut used
   **Then** keyboard shortcut `Ctrl+K` opens chat

7. **Given** chat is open
   **When** navigating
   **Then** chat persists during session

## Tasks

- [ ] Task 1: Create ChatbotButton component
- [ ] Task 2: Create ChatbotPanel component
- [ ] Task 3: Create ChatMessage component
- [ ] Task 4: Implement message history state
- [ ] Task 5: Create chat API endpoint
- [ ] Task 6: Add keyboard shortcut
- [ ] Task 7: Implement minimize/maximize

## Dev Notes

### Chatbot Context Provider

```tsx
// frontend/src/contexts/ChatbotContext.tsx
interface ChatMessage {
  id: string;
  role: "user" | "assistant";
  content: string;
  timestamp: Date;
}

interface ChatbotContextType {
  messages: ChatMessage[];
  isOpen: boolean;
  isMinimized: boolean;
  openChat: () => void;
  closeChat: () => void;
  toggleMinimize: () => void;
  sendMessage: (content: string) => Promise<void>;
  clearHistory: () => void;
}

export const ChatbotContext = createContext<ChatbotContextType | null>(null);

export function ChatbotProvider({ children }: { children: React.ReactNode }) {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [isOpen, setIsOpen] = useState(false);
  const [isMinimized, setIsMinimized] = useState(false);
  const { data: aiConfig } = useAIConfig();

  const sendMessage = async (content: string) => {
    if (!aiConfig?.isEnabled) {
      toast.error("AI is not configured");
      return;
    }

    const userMessage: ChatMessage = {
      id: crypto.randomUUID(),
      role: "user",
      content,
      timestamp: new Date(),
    };

    setMessages(prev => [...prev, userMessage]);

    try {
      const response = await fetch("/api/v1/ai/chat", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          message: content,
          history: messages.slice(-10), // Last 10 messages for context
          pageContext: getPageContext(),
        }),
      });

      const data = await response.json();

      const assistantMessage: ChatMessage = {
        id: crypto.randomUUID(),
        role: "assistant",
        content: data.response,
        timestamp: new Date(),
      };

      setMessages(prev => [...prev, assistantMessage]);
    } catch (error) {
      toast.error("Failed to get response");
    }
  };

  // Keyboard shortcut
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === "k") {
        e.preventDefault();
        setIsOpen(prev => !prev);
        setIsMinimized(false);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  return (
    <ChatbotContext.Provider value={{
      messages,
      isOpen,
      isMinimized,
      openChat: () => { setIsOpen(true); setIsMinimized(false); },
      closeChat: () => setIsOpen(false),
      toggleMinimize: () => setIsMinimized(prev => !prev),
      sendMessage,
      clearHistory: () => setMessages([]),
    }}>
      {children}
    </ChatbotContext.Provider>
  );
}
```

### Chatbot Button

```tsx
// frontend/src/components/ai/ChatbotButton.tsx
export function ChatbotButton() {
  const { openChat, isOpen } = useChatbot();
  const { data: aiConfig } = useAIConfig();

  if (!aiConfig?.isEnabled) {
    return null;
  }

  if (isOpen) {
    return null; // Hidden when chat is open
  }

  return (
    <button
      onClick={openChat}
      className="fixed bottom-6 right-6 w-14 h-14 bg-primary-500 text-white rounded-full 
                 shadow-lg hover:bg-primary-600 hover:shadow-xl transition-all
                 flex items-center justify-center z-40"
      title="Open chat (Ctrl+K)"
    >
      <ChatBubbleIcon className="w-6 h-6" />
    </button>
  );
}
```

### Chatbot Panel

```tsx
// frontend/src/components/ai/ChatbotPanel.tsx
export function ChatbotPanel() {
  const {
    messages,
    isOpen,
    isMinimized,
    closeChat,
    toggleMinimize,
    sendMessage,
    clearHistory,
  } = useChatbot();

  const [input, setInput] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const handleSend = async () => {
    if (!input.trim() || isLoading) return;

    setIsLoading(true);
    const message = input;
    setInput("");

    try {
      await sendMessage(message);
    } finally {
      setIsLoading(false);
    }
  };

  // Auto-scroll to bottom
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  if (!isOpen) return null;

  return (
    <div className={cn(
      "fixed bottom-6 right-6 bg-white rounded-xl shadow-2xl border border-neutral-200 z-50",
      "transition-all duration-200",
      isMinimized ? "w-72 h-14" : "w-96 h-[500px]"
    )}>
      {/* Header */}
      <div className="flex items-center justify-between p-3 border-b border-neutral-200">
        <div className="flex items-center gap-2">
          <SparklesIcon className="w-5 h-5 text-primary-500" />
          <span className="font-medium">AI Assistant</span>
        </div>
        <div className="flex items-center gap-1">
          <button
            onClick={toggleMinimize}
            className="p-1 text-neutral-400 hover:text-neutral-600 rounded"
          >
            {isMinimized ? <MaximizeIcon /> : <MinimizeIcon />}
          </button>
          <button
            onClick={closeChat}
            className="p-1 text-neutral-400 hover:text-neutral-600 rounded"
          >
            <Cross2Icon className="w-4 h-4" />
          </button>
        </div>
      </div>

      {!isMinimized && (
        <>
          {/* Messages */}
          <div className="flex-1 overflow-y-auto p-4 space-y-4 h-[380px]">
            {messages.length === 0 ? (
              <div className="text-center text-neutral-500 mt-8">
                <SparklesIcon className="w-12 h-12 mx-auto text-neutral-300 mb-4" />
                <p className="font-medium">How can I help?</p>
                <p className="text-sm mt-1">Ask me about the QA framework</p>
                
                {/* Quick Actions */}
                <div className="mt-4 space-y-2">
                  <button
                    onClick={() => sendMessage("How do I start a workflow?")}
                    className="w-full text-left px-3 py-2 text-sm bg-neutral-50 rounded-lg hover:bg-neutral-100"
                  >
                    How do I start a workflow?
                  </button>
                  <button
                    onClick={() => sendMessage("What's on this page?")}
                    className="w-full text-left px-3 py-2 text-sm bg-neutral-50 rounded-lg hover:bg-neutral-100"
                  >
                    Ask about this page
                  </button>
                </div>
              </div>
            ) : (
              messages.map((message) => (
                <ChatMessage key={message.id} message={message} />
              ))
            )}
            <div ref={messagesEndRef} />
          </div>

          {/* Input */}
          <div className="p-3 border-t border-neutral-200">
            <div className="flex items-center gap-2">
              <input
                type="text"
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyDown={(e) => e.key === "Enter" && handleSend()}
                placeholder="Ask anything..."
                disabled={isLoading}
                className="flex-1 px-3 py-2 border border-neutral-300 rounded-lg text-sm
                           focus:outline-none focus:ring-2 focus:ring-primary-500"
              />
              <button
                onClick={handleSend}
                disabled={!input.trim() || isLoading}
                className="p-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 
                           disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isLoading ? <Spinner className="w-5 h-5" /> : <PaperPlaneIcon className="w-5 h-5" />}
              </button>
            </div>
          </div>
        </>
      )}
    </div>
  );
}
```

### Chat Message Component

```tsx
// frontend/src/components/ai/ChatMessage.tsx
interface ChatMessageProps {
  message: ChatMessage;
}

export function ChatMessage({ message }: ChatMessageProps) {
  const isUser = message.role === "user";

  return (
    <div className={cn("flex", isUser ? "justify-end" : "justify-start")}>
      <div className={cn(
        "max-w-[80%] rounded-lg px-4 py-2",
        isUser
          ? "bg-primary-500 text-white"
          : "bg-neutral-100 text-neutral-900"
      )}>
        <p className="text-sm whitespace-pre-wrap">{message.content}</p>
        <p className={cn(
          "text-xs mt-1",
          isUser ? "text-primary-200" : "text-neutral-400"
        )}>
          {formatTime(message.timestamp)}
        </p>
      </div>
    </div>
  );
}
```

### Chat API Endpoint

```rust
// POST /api/v1/ai/chat
pub async fn chat(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, ApiError> {
    let ai_config = get_user_ai_config(&state.db_pool, "current_user").await?;
    let client = create_ai_client(&ai_config)?;

    let system_prompt = format!(
        r#"You are a helpful assistant for a QA framework called "QA Intelligent PMS". 
        You help users with:
        - Navigating the interface
        - Understanding workflow features
        - Answering questions about testing
        - Providing tips for efficient testing

        Current page context: {}
        
        Be concise and helpful. If you don't know something, say so."#,
        request.page_context.unwrap_or_default()
    );

    let messages = build_messages(&system_prompt, &request.history, &request.message);
    let response = client.chat(&messages).await?;

    Ok(Json(ChatResponse { response }))
}
```

### References

- [Source: epics.md#Story 13.4]

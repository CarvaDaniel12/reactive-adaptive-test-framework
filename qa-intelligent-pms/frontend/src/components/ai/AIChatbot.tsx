import { useState, useEffect, useRef } from "react";
import { useQuery, useMutation } from "@tanstack/react-query";
import { useLocation } from "react-router-dom";

// Types
interface ChatMessage {
  id: string;
  role: "user" | "assistant" | "system";
  content: string;
  timestamp: string;
}

interface AIStatus {
  available: boolean;
  provider?: string;
  model?: string;
  message: string;
}

interface ChatContext {
  currentPage: string;
  currentTicket?: {
    key: string;
    title: string;
    description?: string;
    ticketType: string;
    status: string;
  };
  workflowStep?: {
    workflowName: string;
    stepName: string;
    stepNumber: number;
    totalSteps: number;
  };
  recentActions: string[];
}

// API functions
async function fetchAIStatus(): Promise<AIStatus> {
  const res = await fetch("/api/v1/ai/status");
  if (!res.ok) throw new Error("Failed to fetch AI status");
  return res.json();
}

async function fetchSuggestions(context: ChatContext | null): Promise<{ suggestions: string[] }> {
  const res = await fetch("/api/v1/ai/chat/suggestions", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ context }),
  });
  if (!res.ok) throw new Error("Failed to fetch suggestions");
  return res.json();
}

async function sendMessage(data: {
  message: string;
  history: ChatMessage[];
  context: ChatContext | null;
}): Promise<{ message: ChatMessage }> {
  const res = await fetch("/api/v1/ai/chat", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(data),
  });
  if (!res.ok) {
    const error = await res.json().catch(() => ({ message: "Chat failed" }));
    throw new Error(error.message || "Chat failed");
  }
  return res.json();
}

export function AIChatbot() {
  const [isOpen, setIsOpen] = useState(false);
  const [isMinimized, setIsMinimized] = useState(false);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState("");
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const location = useLocation();

  // Build context from current location
  const context: ChatContext = {
    currentPage: location.pathname,
    recentActions: [],
  };

  // Queries
  const statusQuery = useQuery({
    queryKey: ["ai-status"],
    queryFn: fetchAIStatus,
    staleTime: 60000,
  });

  const suggestionsQuery = useQuery({
    queryKey: ["ai-suggestions", location.pathname],
    queryFn: () => fetchSuggestions(context),
    enabled: isOpen && statusQuery.data?.available,
  });

  // Mutation for sending messages
  const sendMutation = useMutation({
    mutationFn: sendMessage,
    onSuccess: (data) => {
      setMessages((prev) => [...prev, data.message]);
    },
  });

  // Keyboard shortcut
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === "k") {
        e.preventDefault();
        setIsOpen((prev) => !prev);
      }
      if (e.key === "Escape" && isOpen) {
        setIsOpen(false);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isOpen]);

  // Auto-scroll to bottom
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  // Focus input when opened
  useEffect(() => {
    if (isOpen && !isMinimized) {
      inputRef.current?.focus();
    }
  }, [isOpen, isMinimized]);

  const handleSend = () => {
    if (!input.trim() || sendMutation.isPending) return;

    const userMessage: ChatMessage = {
      id: crypto.randomUUID(),
      role: "user",
      content: input.trim(),
      timestamp: new Date().toISOString(),
    };

    setMessages((prev) => [...prev, userMessage]);
    setInput("");

    sendMutation.mutate({
      message: userMessage.content,
      history: messages,
      context,
    });
  };

  const handleSuggestionClick = (suggestion: string) => {
    setInput(suggestion);
    inputRef.current?.focus();
  };

  const aiStatus = statusQuery.data;
  const isAvailable = aiStatus?.available ?? false;

  if (!isOpen) {
    return (
      <button
        onClick={() => setIsOpen(true)}
        className="fixed bottom-6 right-6 w-14 h-14 bg-primary-600 text-white rounded-full shadow-lg hover:bg-primary-700 transition-all hover:scale-105 flex items-center justify-center z-50"
        title="Open AI Assistant (Ctrl+K)"
      >
        <AIIcon className="w-7 h-7" />
      </button>
    );
  }

  if (isMinimized) {
    return (
      <div className="fixed bottom-6 right-6 bg-white rounded-lg shadow-xl border border-neutral-200 z-50">
        <div className="flex items-center gap-2 px-4 py-2">
          <AIIcon className="w-5 h-5 text-primary-600" />
          <span className="text-sm font-medium text-neutral-900">QA Assistant</span>
          <button
            onClick={() => setIsMinimized(false)}
            className="ml-2 text-neutral-500 hover:text-neutral-700"
          >
            <ExpandIcon className="w-4 h-4" />
          </button>
          <button
            onClick={() => setIsOpen(false)}
            className="text-neutral-500 hover:text-neutral-700"
          >
            <CloseIcon className="w-4 h-4" />
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="fixed bottom-6 right-6 w-96 h-[500px] bg-white rounded-lg shadow-xl border border-neutral-200 flex flex-col z-50">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-neutral-200 bg-primary-50 rounded-t-lg">
        <div className="flex items-center gap-2">
          <AIIcon className="w-5 h-5 text-primary-600" />
          <span className="font-semibold text-neutral-900">QA Assistant</span>
          {isAvailable && (
            <span className="px-2 py-0.5 text-xs bg-green-100 text-green-700 rounded">
              {aiStatus?.model}
            </span>
          )}
        </div>
        <div className="flex items-center gap-1">
          <button
            onClick={() => setIsMinimized(true)}
            className="p-1 text-neutral-500 hover:text-neutral-700 rounded"
            title="Minimize"
          >
            <MinimizeIcon className="w-4 h-4" />
          </button>
          <button
            onClick={() => setIsOpen(false)}
            className="p-1 text-neutral-500 hover:text-neutral-700 rounded"
            title="Close (Esc)"
          >
            <CloseIcon className="w-4 h-4" />
          </button>
        </div>
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {!isAvailable && (
          <div className="bg-amber-50 border border-amber-200 rounded-lg p-3 text-sm">
            <p className="text-amber-800 font-medium">AI Not Configured</p>
            <p className="text-amber-700 text-xs mt-1">
              Add your API key in Settings to enable AI features.
            </p>
          </div>
        )}

        {messages.length === 0 && isAvailable && (
          <div className="text-center text-neutral-500 text-sm py-8">
            <p>👋 Hi! I'm your QA Assistant.</p>
            <p className="mt-1">Ask me anything about testing or this page.</p>
          </div>
        )}

        {messages.map((msg) => (
          <div
            key={msg.id}
            className={`flex ${msg.role === "user" ? "justify-end" : "justify-start"}`}
          >
            <div
              className={`max-w-[80%] rounded-lg px-3 py-2 text-sm ${
                msg.role === "user"
                  ? "bg-primary-600 text-white"
                  : "bg-neutral-100 text-neutral-900"
              }`}
            >
              <p className="whitespace-pre-wrap">{msg.content}</p>
            </div>
          </div>
        ))}

        {sendMutation.isPending && (
          <div className="flex justify-start">
            <div className="bg-neutral-100 rounded-lg px-3 py-2 text-sm text-neutral-500">
              <span className="animate-pulse">Thinking...</span>
            </div>
          </div>
        )}

        <div ref={messagesEndRef} />
      </div>

      {/* Suggestions */}
      {messages.length === 0 && suggestionsQuery.data && isAvailable && (
        <div className="px-4 pb-2">
          <p className="text-xs text-neutral-500 mb-2">Suggested questions:</p>
          <div className="flex flex-wrap gap-1">
            {suggestionsQuery.data.suggestions.slice(0, 3).map((suggestion, i) => (
              <button
                key={i}
                onClick={() => handleSuggestionClick(suggestion)}
                className="px-2 py-1 text-xs bg-neutral-100 text-neutral-700 rounded hover:bg-neutral-200 transition-colors"
              >
                {suggestion}
              </button>
            ))}
          </div>
        </div>
      )}

      {/* Input */}
      <div className="p-3 border-t border-neutral-200">
        <div className="flex gap-2">
          <input
            ref={inputRef}
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleSend()}
            placeholder={isAvailable ? "Ask a question..." : "AI not configured"}
            disabled={!isAvailable || sendMutation.isPending}
            className="flex-1 px-3 py-2 border border-neutral-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:bg-neutral-100 disabled:cursor-not-allowed"
          />
          <button
            onClick={handleSend}
            disabled={!isAvailable || !input.trim() || sendMutation.isPending}
            className="px-3 py-2 bg-primary-600 text-white rounded-lg text-sm hover:bg-primary-700 disabled:bg-neutral-300 disabled:cursor-not-allowed transition-colors"
          >
            <SendIcon className="w-4 h-4" />
          </button>
        </div>
        <p className="text-xs text-neutral-400 mt-1 text-center">
          Press Ctrl+K to toggle
        </p>
      </div>
    </div>
  );
}

// Icons
function AIIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M9.813 15.904L9 18.75l-.813-2.846a4.5 4.5 0 00-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 003.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 003.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 00-3.09 3.09zM18.259 8.715L18 9.75l-.259-1.035a3.375 3.375 0 00-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 002.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 002.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 00-2.456 2.456zM16.894 20.567L16.5 21.75l-.394-1.183a2.25 2.25 0 00-1.423-1.423L13.5 18.75l1.183-.394a2.25 2.25 0 001.423-1.423l.394-1.183.394 1.183a2.25 2.25 0 001.423 1.423l1.183.394-1.183.394a2.25 2.25 0 00-1.423 1.423z" />
    </svg>
  );
}

function CloseIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
    </svg>
  );
}

function MinimizeIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M19 9l-7 7-7-7" />
    </svg>
  );
}

function ExpandIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M5 15l7-7 7 7" />
    </svg>
  );
}

function SendIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M6 12L3.269 3.126A59.768 59.768 0 0121.485 12 59.77 59.77 0 013.27 20.876L5.999 12zm0 0h7.5" />
    </svg>
  );
}

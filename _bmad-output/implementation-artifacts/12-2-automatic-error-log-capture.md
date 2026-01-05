# Story 12.2: Automatic Error Log Capture

Status: ready-for-dev

## Story

As a system,
I want to capture errors with context automatically,
So that support can diagnose issues.

## Acceptance Criteria

1. **Given** an error occurs in the application
   **When** error is caught
   **Then** error message and stack trace are logged

2. **Given** error is logged
   **When** context captured
   **Then** user ID and session info are included

3. **Given** error is logged
   **When** context captured
   **Then** current page/action is included

4. **Given** error is logged
   **When** context captured
   **Then** browser and device info are included

5. **Given** error is logged
   **When** context captured
   **Then** timestamp is included

6. **Given** error contains data
   **When** logged
   **Then** sensitive data is NOT logged (NFR-SEC-02)

7. **Given** data retention
   **When** time passes
   **Then** logs are retained 30 days (NFR-REL-04)

8. **Given** error is captured
   **When** stored
   **Then** logs are stored in database

## Tasks

- [ ] Task 1: Create frontend error boundary with capture
- [ ] Task 2: Create error reporting API endpoint
- [ ] Task 3: Implement sensitive data filtering
- [ ] Task 4: Create browser/device detection
- [ ] Task 5: Implement session context capture
- [ ] Task 6: Create log cleanup job

## Dev Notes

### Frontend Error Capture

```tsx
// frontend/src/lib/errorCapture.ts
interface ErrorContext {
  userId?: string;
  sessionId?: string;
  pageUrl: string;
  userAgent: string;
  timestamp: string;
  additionalContext?: Record<string, any>;
}

class ErrorCaptureService {
  private sessionId: string;

  constructor() {
    this.sessionId = this.generateSessionId();
    this.setupGlobalHandlers();
  }

  private generateSessionId(): string {
    return `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private setupGlobalHandlers() {
    // Uncaught errors
    window.onerror = (message, source, lineno, colno, error) => {
      this.captureError(error || new Error(String(message)), {
        source,
        lineno,
        colno,
      });
    };

    // Unhandled promise rejections
    window.onunhandledrejection = (event) => {
      this.captureError(
        event.reason instanceof Error
          ? event.reason
          : new Error(String(event.reason)),
        { type: "unhandledrejection" }
      );
    };
  }

  async captureError(error: Error, extra?: Record<string, any>) {
    const context = this.buildContext(extra);
    const sanitizedError = this.sanitizeError(error);

    try {
      await fetch("/api/v1/errors", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          errorType: error.name,
          message: sanitizedError.message,
          stackTrace: sanitizedError.stack,
          context,
        }),
      });
    } catch (e) {
      // Silently fail - don't cause more errors
      console.error("Failed to report error:", e);
    }
  }

  private buildContext(extra?: Record<string, any>): ErrorContext {
    const user = useAuthStore.getState().user;
    
    return {
      userId: user?.id,
      sessionId: this.sessionId,
      pageUrl: window.location.href,
      userAgent: navigator.userAgent,
      timestamp: new Date().toISOString(),
      additionalContext: {
        ...extra,
        screenSize: `${window.innerWidth}x${window.innerHeight}`,
        language: navigator.language,
      },
    };
  }

  private sanitizeError(error: Error): Error {
    // Remove sensitive data from error messages
    const sensitivePatterns = [
      /api[_-]?key[=:]\s*['"]?[\w-]+['"]?/gi,
      /password[=:]\s*['"]?[^'"&\s]+['"]?/gi,
      /token[=:]\s*['"]?[\w.-]+['"]?/gi,
      /bearer\s+[\w.-]+/gi,
      /authorization[=:]\s*['"]?[^'"&\s]+['"]?/gi,
      /secret[=:]\s*['"]?[^'"&\s]+['"]?/gi,
      // Credit card patterns
      /\b\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}\b/g,
      // Email addresses
      /\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b/g,
    ];

    let message = error.message;
    let stack = error.stack || "";

    for (const pattern of sensitivePatterns) {
      message = message.replace(pattern, "[REDACTED]");
      stack = stack.replace(pattern, "[REDACTED]");
    }

    const sanitized = new Error(message);
    sanitized.name = error.name;
    sanitized.stack = stack;
    return sanitized;
  }
}

export const errorCapture = new ErrorCaptureService();
```

### Error Boundary Component

```tsx
// frontend/src/components/ErrorBoundary.tsx
import { Component, ReactNode } from "react";
import { errorCapture } from "@/lib/errorCapture";

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error?: Error;
}

export class ErrorBoundary extends Component<Props, State> {
  state: State = { hasError: false };

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    errorCapture.captureError(error, {
      componentStack: errorInfo.componentStack,
    });
  }

  render() {
    if (this.state.hasError) {
      return this.props.fallback || (
        <div className="p-8 text-center">
          <h2 className="text-xl font-semibold mb-2">Something went wrong</h2>
          <p className="text-neutral-600 mb-4">
            We've been notified and are working on it.
          </p>
          <button
            onClick={() => window.location.reload()}
            className="px-4 py-2 bg-primary-500 text-white rounded-lg"
          >
            Reload Page
          </button>
        </div>
      );
    }

    return this.props.children;
  }
}
```

### Backend Error Capture API

```rust
// POST /api/v1/errors
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureErrorRequest {
    pub error_type: String,
    pub message: String,
    pub stack_trace: Option<String>,
    pub context: ErrorContext,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorContext {
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub page_url: String,
    pub user_agent: String,
    pub timestamp: String,
    pub additional_context: Option<serde_json::Value>,
}

pub async fn capture_error(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CaptureErrorRequest>,
) -> Result<StatusCode, ApiError> {
    // Parse browser/device from user agent
    let (browser, device) = parse_user_agent(&request.context.user_agent);

    // Determine severity based on error type
    let severity = determine_severity(&request.error_type, &request.message);

    sqlx::query(
        r#"
        INSERT INTO error_logs 
            (error_type, message, stack_trace, severity, user_id, session_id,
             page_url, browser, device, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#,
    )
    .bind(&request.error_type)
    .bind(&request.message)
    .bind(&request.stack_trace)
    .bind(severity)
    .bind(&request.context.user_id)
    .bind(&request.context.session_id)
    .bind(&request.context.page_url)
    .bind(&browser)
    .bind(&device)
    .bind(&request.context.additional_context)
    .execute(&state.db_pool)
    .await?;

    Ok(StatusCode::CREATED)
}

fn determine_severity(error_type: &str, message: &str) -> &'static str {
    let critical_patterns = ["TypeError", "ReferenceError", "crash", "fatal"];
    let high_patterns = ["NetworkError", "SyntaxError", "500"];
    
    let combined = format!("{} {}", error_type, message).to_lowercase();
    
    if critical_patterns.iter().any(|p| combined.contains(&p.to_lowercase())) {
        return "critical";
    }
    if high_patterns.iter().any(|p| combined.contains(&p.to_lowercase())) {
        return "high";
    }
    "medium"
}
```

### Log Cleanup Job

```rust
// crates/qa-pms-api/src/jobs/cleanup.rs
pub async fn cleanup_old_error_logs(pool: &PgPool) -> Result<u64> {
    // Keep logs for 30 days per NFR-REL-04
    let deleted = sqlx::query(
        "DELETE FROM error_logs WHERE created_at < NOW() - INTERVAL '30 days'"
    )
    .execute(pool)
    .await?
    .rows_affected();

    tracing::info!(deleted = deleted, "Cleaned up old error logs");
    Ok(deleted)
}
```

### References

- [Source: epics.md#Story 12.2]
- [NFR: NFR-SEC-02 - No sensitive data in logs]
- [NFR: NFR-REL-04 - 30 day retention]

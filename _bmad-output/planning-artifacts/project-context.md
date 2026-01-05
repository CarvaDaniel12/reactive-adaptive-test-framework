# Project Context for AI Agents

**Project:** QA Intelligent PMS - Companion Framework
**Updated:** 2026-01-04
**Purpose:** Concise, optimized guide for AI agents implementing this codebase

---

## Implementation Progress

| Epic | Status | Progress |
|------|--------|----------|
| Epic 1: Project Foundation | **DONE** | 10/10 (100%) |
| Epic 2: Setup Wizard | **DONE** | 9/9 (100%) |
| Epic 3: Jira Integration | **IN-PROGRESS** | 1/7 (14%) |
| Epics 4-13 | BACKLOG | 0/47 |
| **Total** | **27%** | **20/73 stories** |

### What's Implemented

**Backend (Rust):**
- Cargo workspace with 11 crates
- Core types, error handling (`qa-pms-core`)
- AES-256-GCM encryption (`qa-pms-config`)
- Axum API server with health endpoint (`qa-pms-api`)
- Neon PostgreSQL connection with SQLx
- Database migrations infrastructure
- Setup wizard API endpoints
- Jira OAuth 2.0 + PKCE authentication flow
- OpenAPI documentation with utoipa

**Frontend (React):**
- React 19 + Vite 7 + Tailwind CSS v4
- Zustand stores (6 stores)
- Setup wizard with 5 steps (profile, jira, postman, testmo, splunk)
- Main layout with collapsible sidebar
- React Router v7 routing

**DevOps:**
- GitHub Actions CI (Rust + Frontend)

### Next Priority: Epic 3 Stories 3.2-3.7

The Jira OAuth flow is complete (Story 3.1). Next stories:
- 3-2: Jira ticket listing with filters
- 3-3: Jira ticket detail view
- 3-4: Jira ticket status updates
- 3-5: Integration health check system
- 3-6: Integration status dashboard component
- 3-7: Credential validation on startup

---

## Quick Reference

| Aspect | Decision |
|--------|----------|
| **Backend Language** | Rust 1.80+ |
| **Async Runtime** | Tokio |
| **Web Framework** | Axum 0.7+ |
| **Database** | Neon PostgreSQL (cloud) + SQLx 0.7 |
| **Frontend** | React 18+ / Vite 5+ / Tailwind CSS v4 |
| **State Management** | Zustand |
| **Component Library** | Radix UI (headless primitives) |
| **Error Handling** | `anyhow` (internal) + `thiserror` (API boundaries) |
| **Logging** | `tracing` + `tracing-subscriber` |
| **Encryption** | `aes-gcm` 0.10 + `secrecy` 0.8 |

---

## Critical Rust Patterns

### 1. Error Handling Pattern (MANDATORY)

**Internal code uses `anyhow` for context:**
```rust
use anyhow::{Context, Result};

async fn internal_operation() -> Result<Data> {
    fetch_data()
        .await
        .context("Failed to fetch data from external API")?;
}
```

**API boundaries use `thiserror` for structured errors:**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("User not found: {0}")]
    NotFound(String),
    
    #[error("Validation failed: {0}")]
    Validation(String),
    
    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_response) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, ErrorResponse::new(msg, "NOT_FOUND")),
            ApiError::Validation(msg) => (StatusCode::BAD_REQUEST, ErrorResponse::new(msg, "VALIDATION_ERROR")),
            ApiError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse::new("Internal error", "INTERNAL_ERROR")),
        };
        (status, Json(error_response)).into_response()
    }
}
```

### 2. Async Pattern (MANDATORY)

**Always use tokio runtime with proper async handlers:**
```rust
#[tokio::main]
async fn main() {
    // Application setup
}

// Handler example
async fn handler(State(pool): State<PgPool>) -> Result<Json<Response>, ApiError> {
    // Async operation
}
```

### 3. Logging Pattern (MANDATORY - Never println!)

```rust
use tracing::{info, debug, warn, error, instrument};

#[tracing::instrument(skip(pool), fields(user_id = %user_id))]
async fn get_user(pool: &PgPool, user_id: Uuid) -> Result<User> {
    tracing::info!("Fetching user");
    // ...
    tracing::debug!(rows_affected = 1, "User retrieved successfully");
}
```

| Level | Usage |
|-------|-------|
| `ERROR` | Unrecoverable failures, requires attention |
| `WARN` | Recoverable issues, degraded functionality |
| `INFO` | Important business events (user created, workflow started) |
| `DEBUG` | Development debugging, detailed flow |
| `TRACE` | Very detailed tracing, performance profiling |

### 4. Serde Pattern (MANDATORY for API types)

**Always use camelCase for JSON, snake_case for Rust:**
```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,  // Serializes as "createdAt"
}

// For optional fields, skip if None
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
```

### 5. Retry Pattern (NFR-REL-03)

```rust
const RETRY_DELAYS: [Duration; 3] = [
    Duration::from_secs(1),
    Duration::from_secs(2),
    Duration::from_secs(4),
];

async fn with_retry<F, Fut, T, E>(mut operation: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut last_error = None;
    for delay in RETRY_DELAYS {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                tracing::warn!(error = ?e, "Operation failed, retrying in {:?}", delay);
                last_error = Some(e);
                tokio::time::sleep(delay).await;
            }
        }
    }
    Err(last_error.unwrap())
}
```

### 6. Token Encryption Pattern (NFR-SEC-01)

```rust
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use secrecy::{Secret, ExposeSecret};

// Wrap secrets to prevent accidental logging
let api_key: Secret<String> = Secret::new(raw_key);

// Access only when needed
fn use_key(key: &Secret<String>) {
    let exposed = key.expose_secret();
    // Use the key...
}
```

---

## Critical React Patterns

### 1. Component Naming (MANDATORY)

```typescript
// ✅ Correct: PascalCase for components
export function UserCard({ userId, onSelect }: UserCardProps) { }

// ✅ Correct: camelCase for hooks with 'use' prefix
export function useUser(id: string) { }
export function useWorkflowStore() { }

// ❌ Wrong
const user_card = () => { }  // Should be PascalCase
const GetUserData = () => { } // Hook should start with 'use'
```

### 2. Zustand Store Pattern (MANDATORY)

**One store per domain, actions inside store:**
```typescript
// stores/workflowStore.ts
interface WorkflowState {
  workflows: Workflow[];
  currentWorkflow: Workflow | null;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  fetchWorkflows: () => Promise<void>;
  setCurrentWorkflow: (id: string) => void;
  clearError: () => void;
}

export const useWorkflowStore = create<WorkflowState>((set, get) => ({
  workflows: [],
  currentWorkflow: null,
  isLoading: false,
  error: null,
  
  fetchWorkflows: async () => {
    set({ isLoading: true, error: null });
    try {
      const workflows = await api.getWorkflows();
      set({ workflows, isLoading: false });
    } catch (e) {
      set({ error: e.message, isLoading: false });
    }
  },
  
  setCurrentWorkflow: (id) => {
    const workflow = get().workflows.find(w => w.id === id);
    set({ currentWorkflow: workflow ?? null });
  },
  
  clearError: () => set({ error: null }),
}));
```

### 3. API Response Types

```typescript
// types/api.ts - Must match Rust API responses

// Direct payload (no wrapper)
interface UserResponse {
  id: string;
  email: string;
  createdAt: string;  // ISO 8601 UTC
}

// Error response
interface ApiError {
  error: string;
  code: string;
  details?: Record<string, unknown>;
}

// Paginated response
interface PaginatedResponse<T> {
  data: T[];
  pagination: {
    page: number;
    pageSize: number;
    totalItems: number;
    totalPages: number;
  };
}
```

### 4. Tailwind CSS v4 Pattern (CSS-First)

**Use CSS variables in `index.css`, not `tailwind.config.js`:**
```css
@import "tailwindcss";

@theme {
  /* Primary - Hostfully Blue */
  --color-primary-500: oklch(62.3% 0.17 236);
  --color-primary-600: oklch(54.6% 0.158 236);
  
  /* Success - Hostfully Green */
  --color-success-500: oklch(72.3% 0.17 155);
  --color-success-600: oklch(62.7% 0.145 155);
  
  /* Warning - Amber */
  --color-warning-500: oklch(76.9% 0.188 70);
  
  /* Error - Red */
  --color-error-500: oklch(63.7% 0.237 25);
  
  /* Fonts */
  --font-sans: "Inter", ui-sans-serif, system-ui, sans-serif;
  --font-mono: "JetBrains Mono", ui-monospace, monospace;
}
```

---

## Naming Conventions Summary

| Context | Convention | Example |
|---------|------------|---------|
| Rust modules | `snake_case` | `user_service`, `workflow_engine` |
| Rust functions | `snake_case` | `get_user_by_id`, `validate_token` |
| Rust types/structs | `PascalCase` | `UserResponse`, `WorkflowConfig` |
| Rust constants | `SCREAMING_SNAKE_CASE` | `MAX_RETRY_ATTEMPTS` |
| Database tables | `snake_case`, plural | `users`, `workflow_steps` |
| Database columns | `snake_case` | `user_id`, `created_at` |
| API endpoints | `/api/v1/{resource}` snake_case | `/api/v1/workflow_steps` |
| JSON fields | `camelCase` | `userId`, `createdAt` |
| React components | `PascalCase` | `UserCard.tsx`, `WorkflowTimer.tsx` |
| React hooks | `camelCase` with `use` | `useWorkflow`, `useAuth` |
| TypeScript interfaces | `PascalCase` | `UserData`, `ApiResponse` |

---

## Anti-Patterns to AVOID

### Rust Anti-Patterns

```rust
// ❌ NEVER use println! in production
println!("User created: {:?}", user);  // Use tracing::info!

// ❌ NEVER mix naming conventions
pub struct user_response { }  // Should be PascalCase
pub fn GetUserById() { }       // Should be snake_case

// ❌ NEVER validate everywhere (trust internal code)
fn internal_helper(data: &Data) {
    if data.id.is_empty() { panic!("Invalid!"); }  // Trust caller
}

// ❌ NEVER use unwrap() in production code without justification
let value = some_option.unwrap();  // Use ? or handle properly

// ❌ NEVER ignore Result types
let _ = file.write_all(data);  // Handle the Result!
```

### React Anti-Patterns

```typescript
// ❌ NEVER mix naming conventions
const user_card = () => { }  // Should be PascalCase
const fetchuserdata = () => { }  // Should be camelCase

// ❌ NEVER put API calls directly in components
function UserCard() {
  useEffect(() => {
    fetch('/api/users')...  // Use store or custom hook
  }, []);
}

// ❌ NEVER duplicate state management logic
// Use Zustand stores, not prop drilling or context everywhere
```

---

## Testing Requirements

### Rust Testing

- **Unit tests:** Co-located with source in same module (`src/users/tests.rs`)
- **Integration tests:** Separate directory per crate (`tests/integration/`)
- **Async tests:** Use `#[tokio::test]` attribute
- **Coverage target:** >80% for core functionality

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_user_success() {
        // Test implementation
    }
}
```

### React Testing

- **Unit tests:** `frontend/src/tests/components/`
- **Hook tests:** `frontend/src/tests/hooks/`
- **E2E (future):** `tests/e2e/` with Playwright

---

## Code Quality Commands

### Rust

```bash
# Format code (ALWAYS run before commit)
cargo fmt

# Lint with clippy (MUST have zero warnings)
cargo clippy -- -D warnings

# Run tests
cargo test

# Security audit
cargo audit
```

### Frontend

```bash
# Format code
npm run format  # Uses Prettier

# Lint
npm run lint    # Uses ESLint

# Type check
npm run typecheck
```

---

## Database Conventions

### Migration Files

Location: `migrations/YYYYMMDDHHMMSS_description.sql`

```sql
-- 20260103000001_create_users.sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
```

### SQLx Query Pattern

```rust
// Compile-time checked queries
let user = sqlx::query_as!(
    User,
    r#"
    SELECT id, email, created_at, updated_at
    FROM users
    WHERE id = $1
    "#,
    user_id
)
.fetch_optional(&pool)
.await
.context("Failed to fetch user")?;
```

---

## API Response Formats

### Success (direct payload)

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "createdAt": "2026-01-03T10:00:00Z"
}
```

### Error (standardized)

```json
{
  "error": "User not found",
  "code": "USER_NOT_FOUND",
  "details": {
    "userId": "requested-uuid"
  }
}
```

### Paginated

```json
{
  "data": [...],
  "pagination": {
    "page": 1,
    "pageSize": 20,
    "totalItems": 150,
    "totalPages": 8
  }
}
```

---

## Performance Requirements (NFRs)

| Requirement | Target | Context |
|-------------|--------|---------|
| API response | <2s for 95% | NFR-PERF-01 |
| Dashboard load | <5s for historical data | NFR-PERF-02 |
| Search operations | <3s for 90% | NFR-PERF-03 |
| Uptime | >99.5% | NFR-REL-01 |
| Health checks | Every 60s | NFR-REL-02 |
| Concurrent users | 100 QAs | NFR-SCAL-01 |

---

## Project Structure Overview

```
qa-intelligent-pms/
├── crates/                         # Rust workspace
│   ├── qa-pms-core/               # Shared types, traits
│   ├── qa-pms-config/             # YAML + encryption
│   ├── qa-pms-api/                # Axum API
│   ├── qa-pms-workflow/           # Workflow engine
│   ├── qa-pms-tracking/           # Time tracking
│   ├── qa-pms-dashboard/          # Dashboard logic
│   ├── qa-pms-jira/               # Jira integration
│   ├── qa-pms-postman/            # Postman integration
│   ├── qa-pms-testmo/             # Testmo integration
│   ├── qa-pms-splunk/             # Splunk integration
│   └── qa-pms-ai/                 # AI companion (optional)
├── frontend/                       # React SPA
│   ├── src/
│   │   ├── components/ui/         # Radix-based primitives
│   │   ├── components/workflow/   # Workflow features
│   │   ├── components/dashboard/  # Dashboard components
│   │   ├── pages/                 # Route pages
│   │   ├── stores/                # Zustand stores
│   │   ├── hooks/                 # Custom hooks
│   │   ├── lib/                   # Utilities
│   │   └── types/                 # TypeScript types
│   └── ...
└── migrations/                     # SQLx migrations
```

---

## Integration Authentication

| System | Method | Crate |
|--------|--------|-------|
| Jira | OAuth 2.0 + PKCE | `qa-pms-jira` |
| Postman | API Key | `qa-pms-postman` |
| Testmo | API Key | `qa-pms-testmo` |
| Splunk | Manual queries | `qa-pms-splunk` |
| Grafana | REST API (future) | - |

---

## Key Dependencies (Cargo.toml)

```toml
# Core
tokio = { version = "1.0", features = ["full"] }
axum = "0.7"
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "migrate"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Security
aes-gcm = "0.10"
secrecy = "0.8"
oauth2 = "4.4"

# HTTP client
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# API documentation
utoipa = "5.0"
utoipa-swagger-ui = "9.0"
```

---

## Quick Checklist for AI Agents

Before submitting code:

- [ ] Used `tracing` instead of `println!`
- [ ] Applied `#[serde(rename_all = "camelCase")]` to API types
- [ ] Used `anyhow::Result` internally, `thiserror` at API boundaries
- [ ] Followed naming conventions (snake_case Rust, camelCase JSON, PascalCase components)
- [ ] Co-located unit tests with source code
- [ ] No clippy warnings
- [ ] Async handlers use proper error handling with `?`
- [ ] Secrets wrapped in `Secret<T>` type
- [ ] Database queries use SQLx compile-time checking

---

**Document Version:** 1.1
**Last Updated:** 2026-01-04
**Related Documents:**
- `architecture.md` - Full architectural decisions
- `prd.md` - Product requirements
- `ux-design-specification.md` - UX patterns and design system
- `sprint-status.yaml` - Current implementation status (in implementation-artifacts/)
# QA Intelligent PMS - Project Documentation

> **Version:** 0.1.0  
> **Last Updated:** 2026-01-05  
> **Repository:** https://github.com/CarvaDaniel12/reactive-adaptive-test-framework

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Tech Stack](#tech-stack)
4. [Project Structure](#project-structure)
5. [Backend Crates](#backend-crates)
6. [Frontend Architecture](#frontend-architecture)
7. [API Reference](#api-reference)
8. [Database Schema](#database-schema)
9. [Security](#security)
10. [Development Guide](#development-guide)

---

## Overview

QA Intelligent PMS (Project Management System) is a comprehensive quality assurance platform that integrates with Jira, Postman, Testmo, and Splunk to provide intelligent test management, pattern detection, and AI-assisted testing workflows.

### Key Features

| Epic | Feature | Description |
|------|---------|-------------|
| 1-2 | **Foundation & Setup** | Project structure, Setup Wizard with multi-step configuration |
| 3 | **Jira Integration** | OAuth 2.0 + PKCE authentication, ticket sync, component mapping |
| 4 | **Postman Integration** | Collection sync, environment management, test execution |
| 5 | **Testmo Integration** | Test case management, run tracking, results sync |
| 6 | **Workflow Engine** | Customizable test workflows with templates |
| 7 | **Time Tracking** | Time entries, aggregations, productivity metrics |
| 8 | **Dashboard & Reports** | Real-time metrics, charts, export capabilities |
| 9 | **Pattern Detection** | Automated detection of test patterns, alerts |
| 10 | **PM/PO Observability** | Executive dashboard, bug trends, component health |
| 11 | **Splunk Integration** | Log queries, templates, error correlation |
| 12 | **Support Portal** | Error capture, diagnostics, knowledge base |
| 13 | **AI Companion** | BYOK multi-provider AI, semantic search, Gherkin analysis |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Frontend (React 19)                       │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐   │
│  │Dashboard│ │Workflows│ │  Setup  │ │ Support │ │AI Chat  │   │
│  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘   │
└───────┼──────────┼──────────┼──────────┼──────────┼─────────────┘
        │          │          │          │          │
        └──────────┴──────────┴──────────┴──────────┘
                              │
                    ┌─────────▼─────────┐
                    │   Axum REST API    │
                    │   (qa-pms-api)     │
                    └─────────┬─────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
┌───────▼───────┐    ┌───────▼───────┐    ┌───────▼───────┐
│  qa-pms-core  │    │ qa-pms-config │    │ qa-pms-workflow│
│  (Types/Auth) │    │ (Settings)    │    │ (Engine)      │
└───────────────┘    └───────────────┘    └───────────────┘
        │
┌───────┴───────────────────────────────────────────┐
│                 Integration Crates                 │
├─────────────┬─────────────┬─────────────┬─────────┤
│ qa-pms-jira │qa-pms-postman│qa-pms-testmo│qa-pms-  │
│             │             │             │ splunk  │
└─────────────┴─────────────┴─────────────┴─────────┘
        │
┌───────┴───────────────────────────────────────────┐
│               Feature Crates                       │
├─────────────┬─────────────┬─────────────┬─────────┤
│qa-pms-time  │qa-pms-      │qa-pms-      │qa-pms-  │
│             │patterns     │support      │ai       │
└─────────────┴─────────────┴─────────────┴─────────┘
        │
┌───────▼───────┐
│  PostgreSQL   │
│   Database    │
└───────────────┘
```

---

## Tech Stack

### Backend (Rust)

| Category | Technology | Version |
|----------|------------|---------|
| Language | Rust | 1.80+ |
| Web Framework | Axum | 0.7 |
| Database | PostgreSQL + SQLx | 0.7 |
| Async Runtime | Tokio | 1.42 |
| HTTP Client | Reqwest | 0.12 |
| OAuth | oauth2 | 4.4 |
| Encryption | AES-GCM | 0.10 |
| API Docs | Utoipa + Swagger UI | 5.3 |
| Serialization | Serde | 1.0 |

### Frontend (React)

| Category | Technology | Version |
|----------|------------|---------|
| Framework | React | 19.2 |
| Build Tool | Vite | 7.2 |
| Styling | Tailwind CSS | 4.1 |
| State | Zustand | 5.0 |
| Data Fetching | TanStack Query | 5.90 |
| UI Components | Radix UI | Latest |
| Charts | Recharts | 3.6 |
| Routing | React Router | 7.11 |

---

## Project Structure

```
qa-intelligent-pms/
├── crates/                    # Rust workspace crates
│   ├── qa-pms-api/           # Main API server (Axum)
│   ├── qa-pms-core/          # Core types, auth, health
│   ├── qa-pms-config/        # Settings, encryption
│   ├── qa-pms-workflow/      # Workflow engine
│   ├── qa-pms-time/          # Time tracking
│   ├── qa-pms-jira/          # Jira integration
│   ├── qa-pms-postman/       # Postman integration
│   ├── qa-pms-testmo/        # Testmo integration
│   ├── qa-pms-splunk/        # Splunk integration
│   ├── qa-pms-patterns/      # Pattern detection
│   ├── qa-pms-support/       # Support portal
│   ├── qa-pms-ai/            # AI companion
│   ├── qa-pms-dashboard/     # Dashboard metrics
│   └── qa-pms-tracking/      # Activity tracking
├── frontend/                  # React SPA
│   ├── src/
│   │   ├── components/       # UI components
│   │   ├── pages/            # Page components
│   │   ├── stores/           # Zustand stores
│   │   ├── hooks/            # Custom hooks
│   │   └── api/              # API client
│   └── public/
├── migrations/               # SQLx migrations
├── configs/                  # Configuration files
├── scripts/                  # Utility scripts
└── docs/                     # Documentation
```

---

## Backend Crates

### qa-pms-api (Main Entry Point)

The main API server built with Axum. Handles HTTP routing, middleware, and request processing.

**Key Files:**
- `main.rs` - Application entry point
- `app.rs` - Router configuration
- `startup.rs` - Startup validation
- `routes/` - API endpoint handlers

**Route Modules:**
| Module | Endpoints | Description |
|--------|-----------|-------------|
| `setup.rs` | `/api/setup/*` | Setup wizard, connection tests |
| `workflows.rs` | `/api/workflows/*` | Workflow CRUD, execution |
| `time_tracking.rs` | `/api/time/*` | Time entries, aggregations |
| `reports.rs` | `/api/reports/*` | Dashboard metrics, charts |
| `patterns.rs` | `/api/patterns/*` | Pattern detection, alerts |
| `observability.rs` | `/api/observability/*` | PM/PO dashboard |
| `splunk.rs` | `/api/splunk/*` | Log queries |
| `support.rs` | `/api/support/*` | Error logs, diagnostics |
| `ai.rs` | `/api/ai/*` | AI companion features |

### qa-pms-core

Core types, authentication, and health check infrastructure.

**Key Types:**
```rust
// Types
pub struct Ticket { id, key, summary, status, ... }
pub struct TestCase { id, name, steps, expected_result, ... }
pub struct WorkflowStep { id, name, status, time_spent, ... }

// Health
pub trait HealthCheck: Send + Sync
pub struct HealthStatus { healthy, message, latency }
```

### qa-pms-config

Configuration management and encryption utilities.

**Key Components:**
```rust
pub struct Settings {
    pub database_url: SecretString,
    pub encryption_key: SecretString,
    pub jira: JiraSettings,
    pub postman: PostmanSettings,
    pub testmo: TestmoSettings,
    pub splunk: SplunkSettings,
}

pub struct Encryptor {
    // AES-256-GCM encryption for secrets
}
```

### qa-pms-workflow

Workflow engine with template support.

**Features:**
- Pre-defined workflow templates (Bug Fix, New Feature, Regression)
- Step-by-step execution tracking
- Time tracking per step
- Completion validation

### qa-pms-jira

Jira integration with OAuth 2.0 + PKCE.

**Features:**
- OAuth 2.0 authentication flow
- PKCE for enhanced security
- Token refresh management
- Ticket sync and search
- Component mapping

### qa-pms-patterns

Pattern detection and alerting system.

**Pattern Types:**
- `HighFailureRate` - Tests failing above threshold
- `FlakeyTest` - Inconsistent test results
- `SlowTest` - Tests exceeding time limits
- `ComponentCluster` - Issues concentrated in component

### qa-pms-ai

AI companion with BYOK (Bring Your Own Key) support.

**Supported Providers:**
- OpenAI (GPT-4, GPT-4o)
- Anthropic (Claude)
- Deepseek
- z.ai
- Custom OpenAI-compatible endpoints

**Features:**
- Semantic search for related tests
- Gherkin acceptance criteria analysis
- Test suggestion generation
- Contextual chatbot

---

## Frontend Architecture

### Component Structure

```
src/
├── components/
│   ├── ai/
│   │   └── AIChatbot.tsx         # Floating AI chat
│   ├── layout/
│   │   ├── Header.tsx            # Navigation header
│   │   ├── Sidebar.tsx           # Side navigation
│   │   └── MainLayout.tsx        # Layout wrapper
│   ├── setup/
│   │   ├── SetupWizard.tsx       # Multi-step setup
│   │   ├── JiraStep.tsx          # Jira configuration
│   │   ├── PostmanStep.tsx       # Postman configuration
│   │   └── TestmoStep.tsx        # Testmo configuration
│   ├── workflow/
│   │   ├── WorkflowList.tsx      # Workflow listing
│   │   ├── WorkflowDetail.tsx    # Workflow details
│   │   └── WorkflowExecution.tsx # Step execution
│   └── ui/
│       ├── Button.tsx
│       ├── Card.tsx
│       └── ...
├── pages/
│   ├── Dashboard.tsx
│   ├── Workflows.tsx
│   ├── Reports.tsx
│   ├── Support.tsx
│   └── Settings.tsx
└── stores/
    ├── useSetupStore.ts          # Setup wizard state
    ├── useWorkflowStore.ts       # Workflow state
    └── useAIStore.ts             # AI configuration
```

### State Management (Zustand)

```typescript
// Example: AI Store
interface AIStore {
  isConfigured: boolean;
  provider: string | null;
  model: string | null;
  configure: (config: AIConfig) => void;
  reset: () => void;
}
```

### API Integration (TanStack Query)

```typescript
// Example: Fetch workflows
const { data, isLoading } = useQuery({
  queryKey: ['workflows'],
  queryFn: () => fetch('/api/workflows').then(r => r.json())
});
```

---

## API Reference

### Authentication

All API endpoints require authentication via session or API key.

### Setup Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/setup/status` | Get setup wizard status |
| POST | `/api/setup/jira/test` | Test Jira connection |
| POST | `/api/setup/postman/test` | Test Postman connection |
| POST | `/api/setup/testmo/test` | Test Testmo connection |
| POST | `/api/setup/complete` | Complete setup wizard |

### Workflow Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/workflows` | List all workflows |
| POST | `/api/workflows` | Create new workflow |
| GET | `/api/workflows/{id}` | Get workflow details |
| POST | `/api/workflows/{id}/start` | Start workflow execution |
| POST | `/api/workflows/{id}/steps/{step}/complete` | Complete step |

### AI Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/ai/status` | Get AI configuration status |
| POST | `/api/ai/configure` | Configure AI provider |
| POST | `/api/ai/test-connection` | Test AI connection |
| POST | `/api/ai/chat` | Send chat message |
| POST | `/api/ai/semantic-search` | Semantic test search |
| POST | `/api/ai/analyze-gherkin` | Analyze Gherkin criteria |

---

## Database Schema

### Core Tables

```sql
-- Workflows
CREATE TABLE workflows (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    template_type VARCHAR(50),
    status VARCHAR(50) DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Workflow Steps
CREATE TABLE workflow_steps (
    id UUID PRIMARY KEY,
    workflow_id UUID REFERENCES workflows(id),
    step_index INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    status VARCHAR(50) DEFAULT 'pending',
    time_spent_minutes INT DEFAULT 0
);

-- Time Entries
CREATE TABLE time_entries (
    id UUID PRIMARY KEY,
    workflow_id UUID REFERENCES workflows(id),
    step_id UUID REFERENCES workflow_steps(id),
    duration_minutes INT NOT NULL,
    description TEXT,
    recorded_at TIMESTAMPTZ DEFAULT NOW()
);

-- AI Configuration
CREATE TABLE ai_configs (
    id UUID PRIMARY KEY,
    provider VARCHAR(50) NOT NULL,
    model VARCHAR(100),
    api_key_encrypted TEXT NOT NULL,
    enabled BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

---

## Security

### Encryption

- **API Keys**: Encrypted with AES-256-GCM before storage
- **Secrets**: Managed via `secrecy` crate with zeroization
- **OAuth Tokens**: Stored encrypted, refreshed automatically

### Authentication

- **Jira**: OAuth 2.0 + PKCE (no client secret exposure)
- **Postman**: API Key authentication
- **Testmo**: API Key authentication
- **AI Providers**: BYOK with encrypted storage

### Best Practices

```rust
// API key encryption example
let encryptor = Encryptor::new(&settings.encryption_key)?;
let encrypted = encryptor.encrypt(api_key.as_bytes())?;
// Store encrypted value in database
```

---

## Development Guide

### Prerequisites

- Rust 1.80+
- Node.js 20+
- PostgreSQL 15+
- Docker (optional)

### Setup

```bash
# Clone repository
git clone https://github.com/CarvaDaniel12/reactive-adaptive-test-framework.git
cd reactive-adaptive-test-framework/qa-intelligent-pms

# Backend setup
cp .env.example .env
# Edit .env with your database URL

# Run migrations
sqlx database create
sqlx migrate run

# Start backend
cargo run

# Frontend setup (new terminal)
cd frontend
npm install
npm run dev
```

### Environment Variables

```bash
DATABASE_URL=postgres://user:pass@localhost:5432/qa_pms
ENCRYPTION_KEY=your-32-byte-key-here
RUST_LOG=info,qa_pms=debug
```

### Running Tests

```bash
# Backend tests
cargo test

# Frontend tests
cd frontend && npm test
```

### API Documentation

When running in development, Swagger UI is available at:
```
http://localhost:3000/swagger-ui/
```

---

## Contributing

1. Follow Rust best practices (clippy, rustfmt)
2. Write tests for new features
3. Update documentation
4. Use conventional commits

---

*Generated by BMAD Document Project Workflow - 2026-01-05*

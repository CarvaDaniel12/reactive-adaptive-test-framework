# QA Intelligent PMS - Companion Framework

A comprehensive companion framework for QA engineers, providing guided workflows, time tracking, and integrations with Jira, Postman, Testmo, and Splunk.

## Architecture

This project uses a **Cargo workspace** with modular crates:

```
qa-intelligent-pms/
├── crates/
│   ├── qa-pms-core/       # Shared types, traits, utilities
│   ├── qa-pms-config/     # Configuration management, encryption
│   ├── qa-pms-api/        # Axum web server (main binary)
│   ├── qa-pms-workflow/   # Workflow engine
│   ├── qa-pms-tracking/   # Time tracking
│   ├── qa-pms-dashboard/  # Dashboard logic
│   ├── qa-pms-jira/       # Jira integration (OAuth 2.0 + PKCE)
│   ├── qa-pms-postman/    # Postman integration
│   ├── qa-pms-testmo/     # Testmo integration
│   ├── qa-pms-splunk/     # Splunk integration
│   └── qa-pms-ai/         # AI companion (BYOK)
├── frontend/              # React SPA (Vite + Tailwind CSS v4)
└── migrations/            # SQLx database migrations
```

## Tech Stack

### Backend
- **Language**: Rust 1.80+
- **Runtime**: Tokio (async)
- **Web Framework**: Axum 0.7
- **Database**: Neon PostgreSQL (cloud) with SQLx 0.7
- **API Docs**: utoipa (OpenAPI/Swagger)

### Frontend
- **Framework**: React 18+
- **Build Tool**: Vite 5+
- **Styling**: Tailwind CSS v4 (OKLCH colors)
- **Components**: Radix UI (headless)
- **State**: Zustand

### Security
- **Encryption**: AES-256-GCM (aes-gcm + secrecy)
- **Auth**: OAuth 2.0 + PKCE (Jira)
- **TLS**: rustls (no OpenSSL)

## Getting Started

### Prerequisites

- Rust 1.80+ (`rustup update stable`)
- Node.js 20+ and npm
- PostgreSQL (or Neon account)

### Setup

1. **Clone and configure:**
   ```bash
   cd qa-intelligent-pms
   cp .env.example .env
   # Edit .env with your database URL and secrets
   ```

2. **Generate encryption key:**
   ```bash
   openssl rand -hex 32
   # Add to .env as ENCRYPTION_KEY
   ```

3. **Build the workspace:**
   ```bash
   cargo build
   ```

4. **Run the API server:**
   ```bash
   cargo run -p qa-pms-api
   ```

5. **Access the API:**
   - Health check: http://localhost:3000/api/v1/health
   - Swagger UI: http://localhost:3000/api/v1/docs

## Development

### Build Commands

```bash
# Build all crates
cargo build

# Build release
cargo build --release

# Run tests
cargo test

# Run clippy (linting)
cargo clippy

# Format code
cargo fmt
```

### Database Migrations

Migrations run automatically on startup. To run manually:

```bash
# Install sqlx-cli
cargo install sqlx-cli --features postgres

# Run migrations
sqlx migrate run

# Create new migration
sqlx migrate add description_here
```

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/health` | GET | Health check with DB status |
| `/api/v1/docs` | GET | Swagger UI documentation |
| `/api/v1/openapi.json` | GET | OpenAPI spec |

## Project Structure Details

### Crate Responsibilities

| Crate | Responsibility |
|-------|----------------|
| `qa-pms-core` | Shared types, error handling, traits |
| `qa-pms-config` | Settings loading, YAML parsing, encryption |
| `qa-pms-api` | HTTP server, routes, middleware |
| `qa-pms-workflow` | Workflow templates, execution, state |
| `qa-pms-tracking` | Timer management, time history |
| `qa-pms-dashboard` | Metrics aggregation, persona dashboards |
| `qa-pms-jira` | Jira OAuth flow, ticket operations |
| `qa-pms-postman` | Postman API client, collection search |
| `qa-pms-testmo` | Testmo API client, test run creation |
| `qa-pms-splunk` | Query templates, log formatting |
| `qa-pms-ai` | AI provider abstraction, semantic search |

### Code Conventions

- **Rust**: `snake_case` functions, `PascalCase` types
- **JSON**: `camelCase` via `#[serde(rename_all = "camelCase")]`
- **Errors**: `anyhow` internal, `thiserror` at API boundaries
- **Logging**: `tracing` (never `println!`)

## License

MIT

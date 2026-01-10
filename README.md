# 🧪 Reactive Adaptive Test Framework

> **QA Intelligent PMS** - A comprehensive quality assurance platform with AI-powered test management

[![Rust](https://img.shields.io/badge/Rust-1.80+-orange?logo=rust)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/React-19-blue?logo=react)](https://react.dev/)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-15+-blue?logo=postgresql)](https://www.postgresql.org/)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)

## ✨ Features

| Feature | Description |
|---------|-------------|
| 🔗 **Multi-Integration** | Jira, Postman, Testmo, Splunk |
| 🤖 **AI Companion** | BYOK support for OpenAI, Anthropic, Deepseek |
| 📊 **Smart Dashboard** | Real-time metrics, pattern detection |
| ⚡ **Workflow Engine** | Customizable test workflows |
| 🔒 **Security First** | AES-256-GCM encryption, OAuth 2.0 + PKCE |

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│              Frontend (React 19 + Vite 7)               │
├─────────────────────────────────────────────────────────┤
│                 REST API (Axum 0.7)                     │
├─────────────────────────────────────────────────────────┤
│  Rust Workspace (14 crates)                             │
│  ┌─────────┬─────────┬─────────┬─────────┬─────────┐   │
│  │  Core   │ Config  │Workflow │Patterns │   AI    │   │
│  ├─────────┼─────────┼─────────┼─────────┼─────────┤   │
│  │  Jira   │ Postman │ Testmo  │ Splunk  │ Support │   │
│  └─────────┴─────────┴─────────┴─────────┴─────────┘   │
├─────────────────────────────────────────────────────────┤
│                   PostgreSQL                            │
└─────────────────────────────────────────────────────────┘
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.80+
- Node.js 20+
- PostgreSQL 15+

### Setup

```bash
# Clone
git clone https://github.com/CarvaDaniel12/reactive-adaptive-test-framework.git
cd reactive-adaptive-test-framework/qa-intelligent-pms

# Configure
cp .env.example .env
# Edit .env with your DATABASE_URL

# Database
sqlx database create
sqlx migrate run

# Backend
cargo run

# Frontend (new terminal)
cd frontend
npm install
npm run dev
```

### Access

- **Frontend**: http://localhost:5173
- **API**: http://localhost:3000
- **Swagger**: http://localhost:3000/swagger-ui/

## 📚 Documentation

- [Project Documentation](qa-intelligent-pms/docs/PROJECT-DOCUMENTATION.md)
- [API Reference](qa-intelligent-pms/docs/PROJECT-DOCUMENTATION.md#api-reference)
- [Architecture](qa-intelligent-pms/docs/PROJECT-DOCUMENTATION.md#architecture)

## 🔧 Tech Stack

### Backend

| Technology | Purpose |
|------------|---------|
| Rust | Systems programming |
| Axum | Web framework |
| SQLx | Database ORM |
| Tokio | Async runtime |
| AES-GCM | Encryption |

### Frontend

| Technology | Purpose |
|------------|---------|
| React 19 | UI framework |
| Vite 7 | Build tool |
| Tailwind CSS 4 | Styling |
| Zustand | State management |
| TanStack Query | Data fetching |
| Radix UI | Components |

## 🛡️ Security

- **Encryption**: AES-256-GCM for all secrets
- **OAuth**: PKCE flow for Jira (no client secret)
- **BYOK**: Bring Your Own Key for AI providers
- **Validation**: Server-side input validation

## 📊 Implemented Epics

| # | Epic | Stories | Status |
|---|------|---------|--------|
| 1 | Project Foundation | 8 | ✅ |
| 2 | Setup Wizard | 8 | ✅ |
| 3 | Jira Integration | 8 | ✅ |
| 4 | Postman Integration | 6 | ✅ |
| 5 | Testmo Integration | 6 | ✅ |
| 6 | Workflow Engine | 8 | ✅ |
| 7 | Time Tracking | 5 | ✅ |
| 8 | Dashboard & Reports | 8 | ✅ |
| 9 | Pattern Detection | 6 | ✅ |
| 10 | PM/PO Observability | 6 | ✅ |
| 11 | Splunk Integration | 4 | ✅ |
| 12 | Support Portal | 6 | ✅ |
| 13 | AI Companion | 6 | ✅ |
| 14 | Rust Improvements | 8 | 📝 Planned |
| 15 | Authentication & Authorization | 12 | 🔴 Critical |
| 16 | Reports Enhancement | 3 | 📝 Planned |
| 17 | Audit Logging | 7 | 📝 Planned |
| 18 | User Experience Improvements | 12 | 📝 Planned |
| 19 | Advanced Features | 12 | 📝 Planned |
| 20 | Documentation & Process | 12 | 📝 Planned |

**Completed: 83/83 stories (Epics 1-13)**  
**In Progress: 0 stories**  
**Planned: 66 stories (Epics 14-20)**  
**Total: 149 stories**

## 🤝 Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing`)
3. Commit changes (`git commit -m 'feat: add amazing feature'`)
4. Push to branch (`git push origin feature/amazing`)
5. Open Pull Request

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

Built with ❤️ using [BMAD Method](https://github.com/bmadcode/BMAD-METHOD)

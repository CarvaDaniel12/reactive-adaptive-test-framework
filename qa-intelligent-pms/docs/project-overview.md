# Project Overview - QA Intelligent PMS

## Executive Summary

**Project Name:** QA Intelligent PMS (Preventiva e Reativa)

**Primary Purpose:** Sistema de QA inteligente que combina análise preventiva (antes da Sprint), análise reativa (em produção) e agente assistente para QAs, baseado em implementações reais de empresas como Shopify, Spotify e Nubank.

**Current Status:** MVP funcional em desenvolvimento com documentação extensa em Português.

## Project Type & Architecture

**Repository Type:** Monolith (single cohesive backend system)

**Architecture Pattern:** Hexagonal Architecture (Ports & Adapters)

**Primary Language:** Python 3.9+ (refatorando para Rust)

**Development Focus:** Backend QA system with API automation

## Technology Stack Summary

| Layer | Technology | Description | Rust Migration Notes |
|--------|------------|-------------|----------------------|
| **Core** | Python 3.9+ with Value Objects, Domain Models | Rust enums/structs with derive traits |
| **Application** | Python services with business logic | Pure functions with trait-based abstractions |
| **Infrastructure** | Adapters for Jira, Splunk, Postman, Testmo, Playwright | Traits for adapters, compile-time config |
| **Presentation** | CLI scripts and Flask web interface | Native CLI or web frameworks |

## Key Integrations

| System | Integration Type | Status | Rust Strategy |
|---------|----------------|--------|---------------|
| **Jira** | REST API (Basic Auth) | ✅ Functional | API Token authentication |
| **Splunk** | File-based CSV/JSON | ✅ Functional | Process exported files, not live SDK |
| **Postman** | REST API | ✅ Functional | Collections and environment management |
| **Testmo** | REST API/CLI | ✅ Functional | Test case synchronization with inheritance |
| **Playwright** | Native library | ✅ Functional | Browser automation for API testing |

## Repository Structure

```
qa-intelligent-pms/
├── configs/                    # Configuration management (Jira, Postman, Splunk, Testmo)
├── docs/                       # Complete documentation (25+ files, Portuguese)
│   ├── 01-architecture.md        # Hexagonal architecture (277 lines)
│   ├── 02-technical-decisions.md
│   ├── 03-data-models.md
│   ├── 04-workflows.md
│   ├── 05-integrations.md
│   ├── 06-setup-guide.md
│   ├── 07-roadmap.md
│   ├── 08-interface-web.md
│   ├── GUIA-USUARIO-FINAL.md   # User guide (349 lines)
│   ├── GUIA-EXPORTACAO-SPLUNK.md
│   ├── technology-stack.md        # Tech stack analysis
│   ├── source-tree-analysis.md    # Directory structure
│   └── [Testmo integration guides...]
├── scripts/                    # Python utility scripts (25+ files)
│   ├── run_preventive.py          # Preventive service (Jira + Postman)
│   ├── analyze_reactive_metrics.py  # Reactive analysis (Splunk)
│   ├── process_splunk_export.py     # Splunk CSV processing
│   ├── [20+ integration test scripts...]
├── tests/                      # Test suite (Hypothesis framework)
│   └── .hypothesis/
├── README.md                    # Project documentation
└── [Configuration files...]
```

## Development Status

**Current Phase:** MVP em desenvolvimento

**Key Features:**
- ✅ Análise preventiva de tickets do Jira
- ✅ Análise reativa de logs do Splunk (CSV/JSON)
- ✅ Geração automática de Acceptance Criteria
- ✅ Cálculo de risco baseado em histórico
- ✅ Identificação de padrões em logs
- ✅ Geração de alertas inteligentes
- ✅ Integração Postman (busca automática)
- ✅ Integração Testmo (sincronização com herança)
- ✅ Gravação de ações do QA
- ✅ Geração automática de scripts Playwright
- ✅ Interface Web completa com design Art Deco
- ✅ Progresso real do processamento

## Documentation Quality

**Language:** Português (com documentação técnica extensa)

**Architecture Documentation:** ✅ Complete (277 lines - Hexagonal architecture)

**User Guides:** ✅ Comprehensive (349 lines - GUIA-USUARIO-FINAL.md)

**Integration Guides:** ✅ Complete (Testmo strategies in Portuguese)

**Technical Stack:** ✅ Documented (Python libraries, frameworks, tools)

## Migration Strategy: Python → Rust

**Preserve:**
1. Hexagonal architecture pattern (Domain → Application → Infrastructure)
2. Business logic and domain rules
3. Integration patterns (how Jira, Splunk, Postman, Testmo interact)
4. Value Objects and domain models
5. Configuration management approach

**Rust Opportunities:**
1. Type-safe Value Objects (enums/structs with derive traits)
2. Trait-based abstractions for adapters
3. Zero-cost abstractions vs Python object overhead
4. Compile-time configuration (serde_yaml vs runtime YAML)
5. Better async patterns (tokio vs asyncio)
6. Property-based testing (proptest vs Hypothesis)

## Next Steps

1. **Document existing architecture fully** (already done - 01-architecture.md)
2. **Complete Rust-specific architecture** document
3. **Break down migration into epics and stories**
4. **Define integration patterns for Rust ecosystem**
5. **Plan test strategy for Rust**

## Risk Assessment

**Low Risk:**
- Documentação existente é extensa e bem estruturada
- Arquitetura hexagonal já documentada em detalhes
- Código Python funcional e testado

**Medium Risk:**
- Diretório `src/` não visível na raiz (mas mencionado em docs)
- 25+ scripts dispersos (potencial consolidação)
- Configurações espalhadas por múltiplos arquivos YAML

**Mitigation:**
- Preservar toda documentação existente
- Mapear padrões técnicos específicos para Rust
- Documentar estratégias de migração em `technology-stack.md`

---

**Generated:** 2026-01-01
**For AI-Assisted Development:** Use this as primary entry point + `index.md` for complete documentation navigation

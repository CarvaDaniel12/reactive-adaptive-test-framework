# QA Intelligent PMS - Project Documentation Index

## Project Overview

### Type
**Repository:** Monolith (Single cohesive backend system)
**Primary Language:** Python 3.9+ (refatorando para Rust)
**Architecture:** Hexagonal (Ports & Adapters)
**Development Focus:** Backend QA System with API automation

### Quick Reference

**Tech Stack:** Python 3.9+ → Rust (planned migration)
**Entry Points:** `scripts/run_preventive.py` (CLI), `src/presentation/web_app.py` (web interface)
**Architecture Pattern:** Hexagonal (Domain → Application → Infrastructure)

---

## Generated Documentation

### Core Documentation
- [Project Overview](./project-overview.md)
- [Architecture](./01-architecture.md) - Complete hexagonal architecture (277 lines)
- [Technology Stack](./technology-stack.md) - Python libraries, frameworks, and Rust migration notes
- [Source Tree Analysis](./source-tree-analysis.md) - Directory structure and critical paths

### Existing Documentation (Preserved)
- [User Guide - Final](./GUIA-USUARIO-FINAL.md) - End-user guide (349 lines)
- [Splunk Export Guide](./GUIA-EXPORTACAO-SPLUNK.md) - Splunk export instructions (294 lines)
- [Testmo Integration Strategy](./TESTMO-ESTRATEGIA-COMPLETA.md) - Complete Testmo strategy
- [Testmo Nomenclature](./TESTMO-NOMENCLATURA-ESTRUTURA.md) - Testmo structure
- [Testmo Implementation Plan](./TESTMO-PLANO-IMPLEMENTACAO.md) - Implementation roadmap
- [Testmo Complete Resources](./TESTMO-RECURSOS-COMPLETOS.md) - Complete resources
- [Integration Tests](./TESTES-INTEGRACOES.md) - Integration testing
- [Testmo CLI Integration](./TESTMO-CLI-INTEGRACAO.md) - CLI integration
- [How Processing Works](./COMO-FUNCIONA-PROCESSAMENTO.md) - Processing flow
- [Human in the Loop](./HUMANO-NO-LOOP.md) - Human workflow
- [Current Status](./STATUS-ATUAL.md) - Project status
- [Roadmap 2026](./ROADMAP-2026.md) - 2026 roadmap
- [Data Models](./03-data-models.md) - Data model documentation
- [Workflows](./04-workflows.md) - Workflow documentation
- [Integrations](./05-INTEGRACOES.md) - Integration documentation
- [Setup Guide](./06-setup-guide.md) - Setup instructions

### Technical Documentation
- [Technical Decisions](./02-technical-decisions.md) - Technical decisions and justifications

### Project Management
- [Reunion Preparation - DevOps](./PREPARACAO-REUNIAO-DEVOPS.md) - Meeting prep
- [Reunion Summary - DevOps](./RESUMO-REUNIAO-DEVOPS.md) - Meeting summary
- [Project Concept](./PROJECT-CONCEPT.md) - Project concept
- [Concept and Overview](./CONCEPT-AND-OVERVIEW.md) - System concept and overview

### Documentation Files
- [BMad Workflow Status](./bmm-workflow-status.yaml) - Workflow tracking (just created!)
- [Project Scan Report](./project-scan-report.json) - Scan state (just updated!)

---

## Getting Started

### Para Desenvolvedores Configurando
1. Review [Project Overview](./project-overview.md) for complete system understanding
2. Study [Architecture](./01-architecture.md) for hexagonal architecture pattern
3. Check [Technology Stack](./technology-stack.md) for Python libraries and Rust migration notes
4. Review [Source Tree Analysis](./source-tree-analysis.md) for codebase structure

### Para Refatoração Python → Rust
1. **Architecture Alignment:** Preserve hexagonal architecture pattern in Rust
   - Domain Layer → Structs with derive traits
   - Application Layer → Functions with trait-based abstractions
   - Infrastructure Layer → Traits for adapters
2. **Integration Patterns:** Document how Jira, Splunk, Postman, Testmo interact
3. **Testing Strategy:** Convert Hypothesis tests to `proptest` or maintain Python wrapper
4. **Configuration:** Replace runtime YAML parsing with compile-time `serde_yaml`
5. **API Testing:** Migrate Playwright automation scripts or use Rust equivalents

### Para QAs (Usuários Finais)
1. **Quick Start:** Read [GUIA-USUARIO-FINAL.md](./GUIA-USUARIO-FINAL.md) for complete user guide
2. **Splunk Export:** Follow [GUIA-EXPORTACAO-SPLUNK.md](./GUIA-EXPORTACAO-SPLUNK.md) for exporting logs
3. **Web Interface:** Use web interface at `http://localhost:5000` for reactive analysis
4. **Configuration:** Setup credentials in configs/ following examples

---

## Notas de Migração para Rust

### O que está bem documentado
✅ Arquitetura hexagonal completa
✅ Stack tecnológico Python completo
✅ Integrações externas documentadas (Jira, Splunk, Postman, Testmo, Playwright)
✅ Padrões de desenvolvimento documentados
✅ Estrutura de diretórios mapeada
✅ Guias de usuário completos
✅ Estratégias de migração identificadas

### O que precisa ser adicionado (para próximos workflows)
📝 **PRD Rust-Specific:** Refatorar PRD existente para contexto de migração Rust
📝 **Architecture Rust-Specific:** Atualizar arquitetura 01-architecture.md com padrões Rust
📝 **Epics and Stories for Refactoring:** Quebrar migração em tarefas gerenciáveis
📝 **Test Strategy Rust:** Definir estratégia de testes para Rust (cargo test, proptest)

### Princípios a Preservar
1. **Arquitetura Hexagonal** - Essencial para sucesso da migração
2. **Separação de Responsabilidades** - Domain, Application, Infrastructure layers
3. **Integrações Estáveis** - APIs diretas, sem MCPs experimentais
4. **Padrões Técnicos** - Value Objects, Component Mapping, Normalization
5. **Performance** - Benefício de zero-cost abstractions vs overhead Python
6. **Type Safety** - Sistema de tipos Rust vs duck typing Python

---

## Status da Migração

**Fase Atual:** Documentação completa em Português
**Próxima Etapa:** Research (Pesquisa sobre Rust) → PRD Rust-Refatorado → Arquitetura Rust-Específica → Épicos & Stories de Migração

---

**Última Atualização:** 2026-01-01

**Para BMad Workflows:** Use este `index.md` como entrada primária para workflows subsequentes. Point BMad agents to this directory for complete context.

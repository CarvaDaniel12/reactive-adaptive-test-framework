# Technology Stack - QA Intelligent PMS

## Primary Technology

**Language:** Python 3.9+

## Core Libraries & Frameworks

| Category | Technology | Version/Details | Purpose |
|-----------|------------|------------------|---------|
| **HTTP Client** | `requests` | Jira API integration |
| **Splunk SDK** | `splunk-sdk` (assumed) | Log analysis |
| **Playwright** | `playwright` | Browser automation & API testing |
| **YAML Configuration** | `PyYAML` | Config file parsing |
| **File Operations** | `pathlib` | File system operations |
| **Date/Time** | `datetime`, `timedelta` | Date handling |
| **Type Hints** | `typing` | Type annotations |

## Architecture Pattern

**Pattern:** Hexagonal Architecture (Ports & Adapters)

**Style:**
- Domain-Driven Design (DDD)
- Clean Architecture principles
- Dependency inversion
- Port & Adapter pattern for external integrations

## Key Integrations

| System | Integration Method | Status | Notes for Rust Migration |
|---------|------------------|--------|-------------------------|
| **Jira** | REST API (Basic Auth) | API Token authentication |
| **Splunk** | File-based (CSV/JSON) | Process exported files, not direct SDK |
| **Postman** | REST API | Collections and environment management |
| **Testmo** | REST API/CLI | Test case synchronization with inheritance |
| **Playwright** | Native library | Browser automation for API testing |

## Development Tools

| Tool | Purpose | Rust Equivalent |
|-------|---------|-----------------|
| **Python Scripts** | 25+ utility scripts | Native Rust binaries or CLI |
| **YAML Configs** | Environment management | Config libraries (serde_yaml) |
| **File Metrics** | JSON-based tracking | Native Rust file I/O |
| **HTML Reports** | Auto-generated reports | Templating engines (askama, tera) |

## Domain-Specific Components

| Component | Technology | Description |
|-----------|------------|-------------|
| **Value Objects** | Custom classes (RiskLevel, TestPriority) | Rust enums/structs with derive traits |
| **Ticket Analysis** | Reactive metrics calculation | Pure functions with business logic |
| **Component Mapping** | YAML-based normalization | Compile-time string processing? |
| **Name Parser** | Custom naming conventions | Rust macros or build scripts |

## Code Organization Patterns

**Current Structure:**
- `scripts/` - 25+ Python utility scripts
- `configs/` - YAML configuration files
- `docs/` - Extensive documentation (25+ files)
- `tests/` - Test files (in .hypothesis/)
- No visible `src/` directory in root

**Implied Structure** (from README):
- `src/domain/` - Domain models (DDD)
- `src/application/` - Application services
- `src/infrastructure/` - Adapters and config
- `src/presentation/` - CLI interface

## Configuration Management

**Jira:**
- Base URL, API version, authentication
- Default ticket fields and timeouts
- API token for programmatic access

**Postman:**
- Collection templates
- Environment configurations
- Endpoint normalization rules

**Splunk:**
- File-based configuration (not live SDK)
- CSV/JSON export parsing
- Reactive metrics calculation patterns

**Testmo:**
- Project synchronization
- Test case naming conventions
- Inheritance patterns (Base → Reactive)

## Testing Strategy

**Current:** Python scripts + Hypothesis (property-based testing)
**Rust Opportunity:**
- `cargo test` for unit tests
- Integration tests with test containers
- Property-based testing with `proptest`

## Technical Debt Considerations

**Identified for Refactoring:**
- No clear `src/` visibility in root listing
- Mix of scripts and documented architecture
- Configuration scattered across multiple YAML files
- 25+ utility scripts (potential consolidation needed)
- File-based Splunk integration (could be improved)
- Component mappings as runtime YAML (compile-time in Rust?)

## Migration Notes for Rust

**Opportunities:**
1. **Stronger typing** - Rust's type system vs Python's dynamic types
2. **Better performance** - Zero-cost abstractions vs Python's object overhead
3. **Memory safety** - No GC pauses, predictable performance
4. **Pattern translation** - Python's dynamic typing requires Rust's traits/generics
5. **Configuration** - Compile-time config vs runtime YAML parsing
6. **Integration patterns** - Traits for adapters vs Python classes

**Challenges:**
1. **Dynamic behavior** - Python's duck typing requires Rust trait objects
2. **Ecosystem maturity** - Python's mature libraries vs Rust's growing ecosystem
3. **Runtime reflection** - Not available in Rust (design around it)
4. **Async patterns** - `asyncio` vs Rust's `tokio`/`async-std`

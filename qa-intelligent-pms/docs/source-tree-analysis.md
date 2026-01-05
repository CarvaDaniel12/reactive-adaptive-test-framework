# Source Tree Analysis - QA Intelligent PMS

## Repository Type

**Type:** Monolith (Single cohesive Python backend)

## Directory Structure

```
qa-intelligent-pms/
├── configs/                    # Configuration management
│   ├── component_mappings.yaml    # Component name normalization
│   ├── jira_config.yaml        # Jira API settings
│   ├── postman_config.yaml      # Postman API settings
│   ├── splunk_config.yaml        # Splunk file processing
│   └── testmo_config.yaml         # Testmo API settings
│
├── docs/                       # Complete documentation (25+ files)
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
│   └── [Testmo integration guides...]
│
├── scripts/                    # Python utility scripts (25+ files)
│   ├── analyze_reactive_metrics.py  # Reactive analysis
│   ├── check_code.py
│   ├── example_minimal.py         # Demo
│   ├── format_code.py
│   ├── generate_html_report.py
│   ├── process_splunk_export.py
│   ├── run_preventive.py         # Main entry points
│   └── [20+ more test scripts...]
│
├── tests/                      # Test suite
│   └── .hypothesis/              # Hypothesis test data
│
├── .gitignore
├── README.md                    # Main project documentation
├── ruff.toml                    # Python linting
└── setup1                        # Setup scripts
```

## Critical Folders Purpose

### `configs/`
**Purpose:** Centralized configuration for all external integrations
**Critical Files:** Jira, Postman, Splunk, Testmo API credentials and settings
**Rust Migration:** Convert YAML runtime parsing to compile-time configuration using `serde_yaml`

### `docs/`
**Purpose:** Comprehensive project documentation (Portuguese)
**Critical Files:**
- `01-architecture.md` - Complete hexagonal architecture definition
- `GUIA-USUARIO-FINAL.md` - End-user guide (349 lines)
- Integration guides for Jira, Postman, Splunk, Testmo
**Rust Migration:** Documentação já está em Português e bem estruturada. Preservar e apenas adicionar contexto Rust.

### `scripts/`
**Purpose:** Python utility scripts for QA operations
**Critical Scripts:**
- `run_preventive.py` - Preventive service (Jira + Postman)
- `analyze_reactive_metrics.py` - Reactive analysis (Splunk)
- `process_splunk_export.py` - Splunk CSV processing
- `get_reactive_metrics.py` - Metrics calculation
- `example_minimal.py` - Value Objects demo
- Integration test scripts for all external systems
**Rust Migration:** 25+ scripts precisam ser reescritos ou substituídos por equivalentes Rust/cargo scripts.

### `tests/`
**Purpose:** Test suite
**Framework:** Hypothesis (property-based testing)
**Rust Migration:** Migrar para `cargo test` e `proptest` ou manter wrapper Python.

## Integration Points

### How External Systems Work Together

**Jira → Preventive Service → Postman:**
1. Jira API fetches tickets from sprint
2. `PreventiveService` analyzes tickets for risk
3. Generates Acceptance Criteria (ACs)
4. Creates Postman collections via `PostmanAdapter`
5. Normalizes component names via `ComponentNormalizer`
6. Uploads to Testmo with inheritance

**Splunk → Reactive Service:**
1. Export Splunk logs as CSV/JSON files
2. `ReactiveService` processes exported files
3. Calculates reactive metrics (error rates, trends)
4. Identifies critical and degrading endpoints
5. Generates HTML reports
6. Searches Postman for matching endpoint tests
7. Merges reactive test cases with Testmo

**Playwright → QA Agent:**
1. `QA-Agent` records QA actions in browser
2. Generates Playwright scripts automatically
3. Saves to Testmo with proper naming

## Architecture Alignment

**Current Structure Matches Hexagonal Architecture (from 01-architecture.md):**

✅ **Domain Layer:** Implied in scripts (entities, value objects)
✅ **Application Layer:** Services in scripts (preventive, reactive, agent)
✅ **Infrastructure Layer:** Adapters in scripts (Jira, Splunk, Postman integrations)
✅ **Presentation Layer:** CLI in scripts and web interface

**Note:** `src/domain/`, `src/application/`, `src/infrastructure/` structure from README not visible in root, but implementation follows hexagonal pattern.

## Entry Points

**CLI Entry:** `scripts/run_preventive.py` (with subcommands)
**Web Interface:** `src/presentation/web_app.py` (assumed, from docs/08-interface-web.md)
**Test Scripts:** All `scripts/test_*.py` files

## Data Flow Patterns

**Preventive Flow:** Jira → Ticket Analysis → Risk Calculation → AC Generation → Postman Collection → Testmo
**Reactive Flow:** Splunk Export → Log Processing → Metrics Calculation → Pattern Detection → Report Generation → Postman Sync → Testmo Merge
**QA Agent Flow:** Browser Recording → Playwright Script Generation → Testmo Upload

## Technical Debt for Refactoring

**Identified:**
1. **No visible src/ directory** in root listing (implementation likely exists)
2. **25+ scattered utility scripts** (potential consolidation opportunity)
3. **Configuration scattered** across multiple YAML files (Jira, Postman, Splunk, Testmo)
4. **Component mappings as runtime YAML** (should be compile-time in Rust)
5. **File-based Splunk integration** (could use direct SDK or improve export format)
6. **Property-based tests** with Hypothesis (consider `proptest` for Rust)

**Rust Migration Strategy:**
- **Consolidate utility scripts** into a clean CLI tool or library
- **Compile-time configurations** using `serde_yaml` instead of runtime parsing
- **Type-safe Value Objects** using Rust enums/structs with `derive` traits
- **Convert Hypothesis tests** to `proptest` for property-based testing
- **Direct SDK integrations** where possible (vs file-based workarounds)

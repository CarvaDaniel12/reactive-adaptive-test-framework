# Story 21.3: Mock Data Generators for Testing

Status: ready-for-dev

Epic: 21 - Developer Experience
Priority: P1 (High Value)
Estimated Effort: 2 days
Sprint: 1

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a **QA Engineer**,
I want to **generate realistic mock data for testing**,
So that **I can test with comprehensive datasets without manually creating test data**.

## Acceptance Criteria

1. **Given** I need test data for Jira tickets
   **When** I run `qapms data mock --type tickets --count 10`
   **Then** it generates 10 realistic Jira tickets
   **And** includes varied statuses (Open, In Progress, Done, Closed)
   **And** includes varied priorities (Highest, High, Medium, Low, Lowest)
   **And** includes varied issue types (Bug, Story, Task, Epic, Improvement)
   **And** saves to database or exports to file (JSON/CSV/YAML)

2. **Given** I need test users
   **When** I generate mock users with `qapms data mock --type users --count 20`
   **Then** it creates users with realistic names and emails
   **And** assigns appropriate roles (admin, qa_lead, qa_engineer, pm_po, viewer)
   **And** includes user metadata (department, created_at, last_login)
   **And** users have varied active statuses

3. **Given** I need test workflows
   **When** I generate mock workflows with `qapms data mock --type workflows --count 15`
   **Then** it creates workflows with realistic names and descriptions
   **And** includes varied statuses (not_started, in_progress, paused, completed, cancelled)
   **And** links workflows to tickets (optional)
   **And** includes workflow steps with varied statuses

4. **Given** I want to generate related data
   **When** I run `qapms data mock --type dataset --config config.yaml`
   **Then** it generates related entities (users, tickets, workflows, time entries)
   **And** maintains relationships between entities (workflows linked to tickets, time entries linked to workflows)
   **And** generates realistic dataset sizes

5. **Given** I want to seed database with mock data
   **When** I run `qapms data mock --type tickets --count 50 --seed`
   **Then** mock data is inserted into database
   **And** validation rules are respected (foreign keys, constraints)
   **And** data is properly formatted for database insertion
   **And** progress is shown during seeding

6. **Given** I want deterministic mock data
   **When** I run `qapms data mock --seed 12345 --type tickets --count 10`
   **Then** generated data is reproducible (same seed = same data)
   **And** seed is logged for reproducibility
   **And** seed can be saved to config file

7. **Given** I want to export mock data
   **When** I run `qapms data mock --type tickets --count 10 --output tickets.json --format json`
   **Then** data is exported to specified file
   **And** format is JSON (pretty-printed)
   **When** I specify `--format csv`
   **Then** data is exported as CSV
   **When** I specify `--format yaml`
   **Then** data is exported as YAML

8. **Given** mock data profiles exist
   **When** I use `qapms data mock --profile small` or `--profile large`
   **Then** it uses predefined configurations
   **And** small profile generates minimal dataset (10 users, 50 tickets, 20 workflows)
   **And** large profile generates comprehensive dataset (100 users, 500 tickets, 200 workflows)

## Tasks / Subtasks

- [ ] Task 1: Create mock data generator crate and types (AC: #1, #2, #3)
  - [ ] 1.1: Create `crates/qa-pms-dev-tools/mock_data/` directory (new crate or module)
  - [ ] 1.2: Create `crates/qa-pms-dev-tools/mock_data/tickets.rs` module
  - [ ] 1.3: Create `MockTicket` struct matching Jira ticket structure
  - [ ] 1.4: Implement `MockTicket::generate(rng)` method using `fake` crate
  - [ ] 1.5: Generate realistic summaries based on issue type (bug vs feature vs task)
  - [ ] 1.6: Create `crates/qa-pms-dev-tools/mock_data/users.rs` module
  - [ ] 1.7: Create `MockUser` struct matching user structure
  - [ ] 1.8: Implement `MockUser::generate(rng)` method
  - [ ] 1.9: Create `crates/qa-pms-dev-tools/mock_data/workflows.rs` module
  - [ ] 1.10: Create `MockWorkflow` struct matching workflow structure
  - [ ] 1.11: Implement `MockWorkflow::generate(rng, ticket)` method
  - [ ] 1.12: Add unit tests for mock data generation

- [ ] Task 2: Implement realistic data generation with fake crate (AC: #1, #2, #3)
  - [ ] 2.1: Add `fake` crate dependency to `qa-pms-dev-tools/Cargo.toml`
  - [ ] 2.2: Add `rand` crate dependency (for RNG)
  - [ ] 2.3: Implement `generate_bug_summary()` for bug tickets
  - [ ] 2.4: Implement `generate_feature_summary()` for feature tickets
  - [ ] 2.5: Implement `generate_task_summary()` for task tickets
  - [ ] 2.6: Generate realistic descriptions using `fake::Lorem` sentences
  - [ ] 2.7: Generate realistic names using `fake::Name` for users
  - [ ] 2.8: Generate realistic emails from names for users
  - [ ] 2.9: Generate realistic timestamps (created_at, updated_at, last_login) with chrono
  - [ ] 2.10: Add unit tests for data realism (verify fields are populated, values are valid)

- [ ] Task 3: Implement mock data CLI command (AC: #1, #2, #3, #7, #8)
  - [ ] 3.1: Add `mock` subcommand to CLI (from Story 14.7 CLI admin tool)
  - [ ] 3.2: Add flags: `--type` (tickets/users/workflows/dataset), `--count` (number), `--output` (file path), `--format` (json/csv/yaml), `--seed` (u64), `--profile` (small/large/custom), `--seed-db` (boolean)
  - [ ] 3.3: Implement `handle_mock_data()` function in CLI
  - [ ] 3.4: Parse CLI arguments and validate (type required, count > 0, etc.)
  - [ ] 3.5: Generate mock data based on type and count
  - [ ] 3.6: Export to file if `--output` specified, otherwise print to stdout
  - [ ] 3.7: Support multiple formats (JSON, CSV, YAML)
  - [ ] 3.8: Add progress bar using `indicatif` for large datasets
  - [ ] 3.9: Add unit tests for CLI command parsing

- [ ] Task 4: Implement database seeding functionality (AC: #5)
  - [ ] 4.1: Create `seed_database()` function in `mock_data` module
  - [ ] 4.2: Generate mock tickets and insert into database (use Jira ticket structure from Story 3.1)
  - [ ] 4.3: Generate mock users and insert into database (if user table exists from Story 15.1)
  - [ ] 4.4: Generate mock workflows and insert into database (use WorkflowTemplate/Instance from Story 5.1)
  - [ ] 4.5: Handle foreign key constraints (link workflows to tickets if tickets exist)
  - [ ] 4.6: Handle validation errors gracefully (skip invalid records, log errors)
  - [ ] 4.7: Batch insert for performance (insert in batches of 100)
  - [ ] 4.8: Show progress during seeding (indicatif progress bar)
  - [ ] 4.9: Add transaction rollback on failure (all or nothing)
  - [ ] 4.10: Add `--dry-run` flag to validate without inserting
  - [ ] 4.11: Add integration tests for database seeding

- [ ] Task 5: Implement deterministic generation with seed support (AC: #6)
  - [ ] 5.1: Create `SeededRng` wrapper around `rand::rngs::StdRng` with seed
  - [ ] 5.2: Implement `generate_with_seed(seed, count, type)` method
  - [ ] 5.3: Use `rand::SeedableRng` trait for reproducible RNG
  - [ ] 5.4: Generate same data for same seed (verify with tests)
  - [ ] 5.5: Log seed value when generating data (trace level)
  - [ ] 5.6: Save seed to config file if `--save-seed` flag is set
  - [ ] 5.7: Load seed from config file if available (for reproducibility)
  - [ ] 5.8: Add unit tests for deterministic generation (same seed = same output)

- [ ] Task 6: Implement export formats (JSON, CSV, YAML) (AC: #7)
  - [ ] 6.1: Create `export_to_file(data, path, format)` function
  - [ ] 6.2: Implement JSON export using `serde_json::to_string_pretty()`
  - [ ] 6.3: Implement CSV export using `csv::Writer` crate
  - [ ] 6.4: Implement YAML export using `serde_yaml::to_string()`
  - [ ] 6.5: Handle file write errors gracefully
  - [ ] 6.6: Create output directory if it doesn't exist
  - [ ] 6.7: Overwrite existing files with confirmation (or `--force` flag)
  - [ ] 6.8: Add unit tests for export formats

- [ ] Task 7: Implement mock data profiles (AC: #8)
  - [ ] 7.1: Create `MockDataProfile` enum (Small, Large, Custom)
  - [ ] 7.2: Define `Small` profile: 10 users, 50 tickets, 20 workflows
  - [ ] 7.3: Define `Large` profile: 100 users, 500 tickets, 200 workflows
  - [ ] 7.4: Load custom profile from YAML config file
  - [ ] 7.5: Implement `generate_from_profile(profile)` method
  - [ ] 7.6: Generate all entity types when using dataset profile
  - [ ] 7.7: Maintain relationships between entities in dataset profile
  - [ ] 7.8: Add default profiles to `data/profiles/` directory
  - [ ] 7.9: Add unit tests for profile generation

- [ ] Task 8: Implement related data generation (AC: #4)
  - [ ] 8.1: Create `generate_complete_dataset(config)` function
  - [ ] 8.2: Generate users first (base entities)
  - [ ] 8.3: Generate tickets with optional link to users (assignee, reporter)
  - [ ] 8.4: Generate workflows linked to tickets (ticket_key field)
  - [ ] 8.5: Generate time entries linked to workflows and users
  - [ ] 8.6: Maintain referential integrity (all foreign keys valid)
  - [ ] 8.7: Generate realistic proportions (more tickets than workflows, more workflows than time entries)
  - [ ] 8.8: Add unit tests for relationship integrity

- [ ] Task 9: Add mock data API endpoints (AC: #1, #2, #3)
  - [ ] 9.1: Create `crates/qa-pms-api/src/routes/dev_tools.rs` module
  - [ ] 9.2: Add `POST /api/v1/dev/mock-data/generate` endpoint
  - [ ] 9.3: Request body: `{ type, count, format?, seed?, profile? }`
  - [ ] 9.4: Generate mock data server-side
  - [ ] 9.5: Return generated data as JSON or stream file download
  - [ ] 9.6: Add `POST /api/v1/dev/mock-data/seed` endpoint (database seeding)
  - [ ] 9.7: Request body: `{ type, count, seed?, profile? }`
  - [ ] 9.8: Seed database with mock data
  - [ ] 9.9: Return seeding result (inserted count, errors)
  - [ ] 9.10: Protect endpoints with `Permission::AdminSystem` (if auth exists) or development mode only
  - [ ] 9.11: Add OpenAPI documentation
  - [ ] 9.12: Add integration tests

- [ ] Task 10: Add comprehensive tests (AC: All)
  - [ ] 10.1: Add unit tests for `MockTicket::generate()`
  - [ ] 10.2: Add unit tests for `MockUser::generate()`
  - [ ] 10.3: Add unit tests for `MockWorkflow::generate()`
  - [ ] 10.4: Add unit tests for deterministic generation (seed-based)
  - [ ] 10.5: Add unit tests for export formats (JSON, CSV, YAML)
  - [ ] 10.6: Add unit tests for profile generation
  - [ ] 10.7: Add integration tests for database seeding
  - [ ] 10.8: Add integration tests for mock data API endpoints
  - [ ] 10.9: Test relationship integrity in generated datasets

## Dev Notes

### Architecture Compliance

**Tech Stack:**
- Rust 1.80+ with Tokio async runtime
- `fake` crate for realistic data generation (Fake, Faker, locales::EN)
- `rand` crate for randomization (thread_rng, SeedableRng, StdRng)
- `csv` crate for CSV export
- `serde_yaml` crate for YAML export (already have serde_json)
- `indicatif` crate for progress bars
- Error handling: `anyhow` (internal) + `thiserror` (API boundaries)
- Logging: `tracing` + `tracing-subscriber` (never `println!`)

**Code Structure:**
- **Mock Data Generators:** `crates/qa-pms-dev-tools/mock_data/` (new crate or module in existing dev-tools crate)
- **CLI Integration:** Extend CLI from Story 14.7 (`qa-pms-cli` or similar)
- **API Routes:** `crates/qa-pms-api/src/routes/dev_tools.rs` (new module)
- **Data Profiles:** `data/profiles/` directory (YAML config files)

**Mock Data Generation Pattern:**
Following `fake` crate patterns for realistic data:
```rust
use fake::{Fake, Faker};
use fake::locales::EN;
use rand::Rng;

impl MockTicket {
    pub fn generate<R: Rng>(rng: &mut R) -> Self {
        let summary: String = (Fake, Faker).fake_with_rng::<String, _>(rng);
        // ... generate realistic ticket data
    }
}
```

**Deterministic Generation Pattern:**
Using seeded RNG for reproducibility:
```rust
use rand::{rngs::StdRng, SeedableRng};

let mut rng = StdRng::seed_from_u64(seed);
let ticket = MockTicket::generate(&mut rng);
```

**Database Seeding Pattern:**
Following SQLx batch insert patterns:
```rust
use sqlx::PgPool;

async fn seed_tickets(pool: &PgPool, tickets: Vec<MockTicket>) -> Result<usize> {
    let mut tx = pool.begin().await?;
    
    for batch in tickets.chunks(100) {
        // Batch insert
    }
    
    tx.commit().await?;
    Ok(tickets.len())
}
```

### Context7 Requirements (MANDATORY)

**CRITICAL:** Before implementing any code, use Context7 MCP to:

1. **Resolve library ID**: `/crate/fake` or search for "rust fake crate"
2. **Query Context7 for**: "How to generate realistic mock data in Rust using fake crate"
3. **Query Context7 for**: "How to implement deterministic random number generation with seeds in Rust"
4. **Verify patterns for**:
   - `fake::Fake` trait usage
   - `fake::locales::EN` for English locale data
   - `rand::SeedableRng` for seeded RNG
   - `csv::Writer` for CSV export
   - `serde_yaml` for YAML serialization
5. **Check best practices for**:
   - Database seeding with SQLx (batch inserts, transactions)
   - Progress bars for long operations (indicatif)
   - CLI argument parsing (clap from Story 14.7)

**Why this is mandatory:**
- `fake` crate has specific patterns for data generation
- Ensures we use current best practices for mock data
- Prevents incorrect RNG usage (non-deterministic when seed needed)
- Guarantees proper CSV/YAML export formats

### Previous Story Intelligence

**From Story 14.7 (CLI Admin Tool):**
- CLI structure already exists with `clap` crate
- CLI commands are organized in subcommands
- This story adds `mock` subcommand to existing CLI
- Reuse CLI patterns for argument parsing and output formatting

**From Story 5.1 (Workflow Templates):**
- Workflow structure is already defined (`WorkflowTemplate`, `WorkflowInstance`, `WorkflowStep`)
- Mock workflows should match existing structure
- Workflows can be linked to tickets via `ticket_id` field

**From Story 3.1 (Jira Ticket Listing):**
- Jira ticket structure is defined (key, summary, description, status, priority, issue_type, etc.)
- Mock tickets should match Jira ticket structure
- Tickets can be stored in database or exported to file

**Key Integration Points:**
- Extend CLI from Story 14.7 (add mock subcommand)
- Use existing database schemas for seeding (tickets, workflows, users if exists)
- Generate data matching existing type structures
- Use existing error handling patterns (`anyhow::Result`)

**Code Patterns to Follow:**
```rust
// From Story 14.7 - CLI command pattern
#[derive(Parser)]
struct MockCommand {
    #[arg(long)]
    type: String,
    #[arg(long)]
    count: usize,
    // ... other flags
}
```

### Project Structure Notes

**Alignment with unified structure:**
- ✅ Mock data generators in `qa-pms-dev-tools` crate (dev tools domain)
- ✅ CLI integration in CLI crate (from Story 14.7)
- ✅ API routes in `qa-pms-api/src/routes/dev_tools.rs` (API layer)
- ✅ Data profiles in `data/profiles/` (configuration layer)

**Files to Create:**
- `crates/qa-pms-dev-tools/mock_data/tickets.rs` - Mock ticket generator
- `crates/qa-pms-dev-tools/mock_data/users.rs` - Mock user generator
- `crates/qa-pms-dev-tools/mock_data/workflows.rs` - Mock workflow generator
- `crates/qa-pms-dev-tools/mock_data/mod.rs` - Module exports
- `crates/qa-pms-api/src/routes/dev_tools.rs` - Dev tools API endpoints
- `data/profiles/small.yaml` - Small profile configuration
- `data/profiles/large.yaml` - Large profile configuration

**Files to Modify:**
- CLI crate (from Story 14.7) - Add `mock` subcommand
- `crates/qa-pms-api/src/routes/mod.rs` - Add dev_tools routes module
- `crates/qa-pms-api/src/app.rs` - Add dev_tools router (development mode only)

**Naming Conventions:**
- Functions: `generate_mock_tickets()`, `seed_database()`, `export_to_file()`
- CLI command: `qapms data mock`
- API endpoints: `/api/v1/dev/mock-data/generate`, `/api/v1/dev/mock-data/seed`
- Profiles: `small`, `large`, `custom`

### Testing Standards

**Unit Tests:**
- Test mock data generation for each type (tickets, users, workflows)
- Test data realism (verify fields are populated, values are valid, relationships exist)
- Test deterministic generation (same seed = same output)
- Test export formats (JSON, CSV, YAML)
- Test profile generation

**Integration Tests:**
- Test database seeding (insert, verify data integrity)
- Test CLI command execution
- Test mock data API endpoints
- Test relationship integrity (foreign keys valid)

**Test Coverage Target:**
- Minimum 80% coverage for mock data generation
- 100% coverage for deterministic generation (seed-based)
- Integration tests for database seeding

### References

- **Source: `_bmad-output/planning-artifacts/epics-detailed/epic-21-developer-experience.md#story-21.3`** - Story requirements and acceptance criteria
- **Source: `qa-intelligent-pms/crates/qa-pms-workflow/src/types.rs`** - Workflow structure for mock workflows
- **Source: `qa-intelligent-pms/crates/qa-pms-jira/src/tickets.rs`** - Jira ticket structure for mock tickets
- **Source: fake crate documentation via Context7** - Mock data generation patterns
- **Source: `_bmad-output/planning-artifacts/project-context.md`** - Rust patterns, error handling, logging

## Dev Agent Record

### Agent Model Used

Claude Sonnet 4.5 (via Cursor)

### Debug Log References

(None yet - story not implemented)

### Completion Notes List

(None yet - story not implemented)

### File List

**Created:**
- `crates/qa-pms-dev-tools/mock_data/tickets.rs` - Mock ticket generator
- `crates/qa-pms-dev-tools/mock_data/users.rs` - Mock user generator
- `crates/qa-pms-dev-tools/mock_data/workflows.rs` - Mock workflow generator
- `crates/qa-pms-dev-tools/mock_data/mod.rs` - Module exports
- `crates/qa-pms-api/src/routes/dev_tools.rs` - Dev tools API endpoints
- `data/profiles/small.yaml` - Small profile configuration
- `data/profiles/large.yaml` - Large profile configuration

**Modified:**
- CLI crate (from Story 14.7) - Add `mock` subcommand
- `crates/qa-pms-api/src/routes/mod.rs` - Add dev_tools routes module
- `crates/qa-pms-api/src/app.rs` - Add dev_tools router (development mode only)

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete structure
- Added all required sections: Story, Metadata, Acceptance Criteria (8 ACs), Tasks (10 tasks with subtasks), Dev Notes, Dev Agent Record, File List
- Converted acceptance criteria from epic format to Given/When/Then format
- Added comprehensive dev notes with architecture patterns, Context7 requirements, integration with Story 14.7, 5.1, 3.1
- Added file list with all files to create and modify

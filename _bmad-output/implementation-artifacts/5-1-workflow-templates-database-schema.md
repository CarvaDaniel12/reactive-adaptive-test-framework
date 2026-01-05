# Story 5.1: Workflow Templates Database Schema

Status: done

## Story

As a developer,
I want database tables for workflow templates,
So that workflows can be stored and customized.

## Acceptance Criteria

1. **Given** SQLx migrations infrastructure
   **When** workflow schema migration runs
   **Then** `workflow_templates` table is created (id, name, ticket_type, steps_json, created_at, updated_at)

2. **Given** SQLx migrations infrastructure
   **When** workflow schema migration runs
   **Then** `workflow_instances` table is created (id, template_id, ticket_id, user_id, status, current_step, started_at, completed_at)

3. **Given** SQLx migrations infrastructure
   **When** workflow schema migration runs
   **Then** `workflow_step_results` table is created (id, instance_id, step_index, status, notes, started_at, completed_at)

4. **Given** tables are created
   **When** schema is analyzed
   **Then** indexes exist for common queries (template lookups, instance by ticket, step results by instance)

5. **Given** tables are created
   **When** schema is analyzed
   **Then** foreign keys enforce referential integrity

6. **Given** workflow crate
   **When** types are implemented
   **Then** Rust types match database schema with SQLx derives

## Tasks / Subtasks

- [ ] Task 1: Create workflow migration file (AC: #1, #2, #3, #4, #5)
  - [ ] 1.1: Create `workflow_templates` table
  - [ ] 1.2: Create `workflow_instances` table
  - [ ] 1.3: Create `workflow_step_results` table
  - [ ] 1.4: Add indexes for performance
  - [ ] 1.5: Add foreign key constraints

- [ ] Task 2: Create Rust types in qa-pms-workflow (AC: #6)
  - [ ] 2.1: Create WorkflowTemplate struct
  - [ ] 2.2: Create WorkflowInstance struct
  - [ ] 2.3: Create WorkflowStepResult struct
  - [ ] 2.4: Create WorkflowStatus enum
  - [ ] 2.5: Create StepStatus enum

- [ ] Task 3: Create WorkflowStep type for steps_json (AC: #6)
  - [ ] 3.1: Define WorkflowStep struct (name, description, estimated_minutes)
  - [ ] 3.2: Implement serialization/deserialization

- [ ] Task 4: Add repository functions (AC: #6)
  - [ ] 4.1: Create template CRUD functions
  - [ ] 4.2: Create instance CRUD functions
  - [ ] 4.3: Create step result functions

## Dev Notes

### Architecture Alignment

This story implements **Workflow Database Schema** per Epic 5 requirements:

- **Location**: `migrations/` for SQL, `crates/qa-pms-workflow/` for Rust types
- **Integration**: Uses SQLx for type-safe queries
- **Pattern**: Repository pattern for database access

### Technical Implementation Details

#### Migration SQL

```sql
-- migrations/20260104000001_workflow_schema.sql

-- Workflow Templates
-- Stores reusable workflow definitions
CREATE TABLE workflow_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    ticket_type VARCHAR(50) NOT NULL, -- 'bug', 'feature', 'regression', 'custom'
    steps_json JSONB NOT NULL, -- Array of WorkflowStep
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for quick template lookups
CREATE INDEX idx_workflow_templates_ticket_type ON workflow_templates(ticket_type);
CREATE INDEX idx_workflow_templates_is_default ON workflow_templates(is_default);

-- Workflow Instances
-- Tracks active/completed workflows for tickets
CREATE TABLE workflow_instances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    template_id UUID NOT NULL REFERENCES workflow_templates(id),
    ticket_id VARCHAR(50) NOT NULL, -- Jira ticket key (e.g., "PROJ-123")
    user_id VARCHAR(100) NOT NULL, -- User email or ID
    status VARCHAR(20) NOT NULL DEFAULT 'active', -- 'active', 'paused', 'completed', 'cancelled'
    current_step INTEGER NOT NULL DEFAULT 0,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    paused_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for finding workflows by ticket
CREATE INDEX idx_workflow_instances_ticket_id ON workflow_instances(ticket_id);
CREATE INDEX idx_workflow_instances_user_id ON workflow_instances(user_id);
CREATE INDEX idx_workflow_instances_status ON workflow_instances(status);
CREATE INDEX idx_workflow_instances_template_id ON workflow_instances(template_id);

-- Workflow Step Results
-- Stores completion data for each step
CREATE TABLE workflow_step_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    instance_id UUID NOT NULL REFERENCES workflow_instances(id) ON DELETE CASCADE,
    step_index INTEGER NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- 'pending', 'in_progress', 'completed', 'skipped'
    notes TEXT,
    links JSONB, -- Array of {title, url}
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(instance_id, step_index)
);

-- Index for step lookups
CREATE INDEX idx_workflow_step_results_instance_id ON workflow_step_results(instance_id);

-- Trigger to update updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_workflow_templates_updated_at
    BEFORE UPDATE ON workflow_templates
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_workflow_instances_updated_at
    BEFORE UPDATE ON workflow_instances
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_workflow_step_results_updated_at
    BEFORE UPDATE ON workflow_step_results
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
```

#### Rust Types

```rust
// crates/qa-pms-workflow/src/types.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Workflow template status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum WorkflowStatus {
    Active,
    Paused,
    Completed,
    Cancelled,
}

/// Step completion status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum StepStatus {
    Pending,
    InProgress,
    Completed,
    Skipped,
}

/// Individual step definition within a workflow template.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStep {
    /// Step name (e.g., "Reproduce Bug")
    pub name: String,
    /// Detailed description of what to do
    pub description: String,
    /// Estimated time in minutes
    pub estimated_minutes: i32,
}

/// Link attached to a step result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepLink {
    pub title: String,
    pub url: String,
}

/// Workflow template stored in database.
#[derive(Debug, Clone, FromRow)]
pub struct WorkflowTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub ticket_type: String,
    pub steps_json: sqlx::types::Json<Vec<WorkflowStep>>,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Workflow instance for a specific ticket.
#[derive(Debug, Clone, FromRow)]
pub struct WorkflowInstance {
    pub id: Uuid,
    pub template_id: Uuid,
    pub ticket_id: String,
    pub user_id: String,
    pub status: String, // Will be parsed to WorkflowStatus
    pub current_step: i32,
    pub started_at: DateTime<Utc>,
    pub paused_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Result of a completed workflow step.
#[derive(Debug, Clone, FromRow)]
pub struct WorkflowStepResult {
    pub id: Uuid,
    pub instance_id: Uuid,
    pub step_index: i32,
    pub status: String, // Will be parsed to StepStatus
    pub notes: Option<String>,
    pub links: Option<sqlx::types::Json<Vec<StepLink>>>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

#### Repository Functions

```rust
// crates/qa-pms-workflow/src/repository.rs
use sqlx::PgPool;
use uuid::Uuid;

use crate::types::{WorkflowTemplate, WorkflowInstance, WorkflowStepResult, WorkflowStep};

/// Get all default templates.
pub async fn get_default_templates(pool: &PgPool) -> Result<Vec<WorkflowTemplate>, sqlx::Error> {
    sqlx::query_as!(
        WorkflowTemplate,
        r#"
        SELECT id, name, description, ticket_type, 
               steps_json as "steps_json: _", is_default, 
               created_at, updated_at
        FROM workflow_templates
        WHERE is_default = true
        ORDER BY name
        "#
    )
    .fetch_all(pool)
    .await
}

/// Get template by ID.
pub async fn get_template(pool: &PgPool, id: Uuid) -> Result<Option<WorkflowTemplate>, sqlx::Error> {
    sqlx::query_as!(
        WorkflowTemplate,
        r#"
        SELECT id, name, description, ticket_type,
               steps_json as "steps_json: _", is_default,
               created_at, updated_at
        FROM workflow_templates
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

/// Get templates by ticket type.
pub async fn get_templates_by_type(
    pool: &PgPool,
    ticket_type: &str,
) -> Result<Vec<WorkflowTemplate>, sqlx::Error> {
    sqlx::query_as!(
        WorkflowTemplate,
        r#"
        SELECT id, name, description, ticket_type,
               steps_json as "steps_json: _", is_default,
               created_at, updated_at
        FROM workflow_templates
        WHERE ticket_type = $1
        ORDER BY is_default DESC, name
        "#,
        ticket_type
    )
    .fetch_all(pool)
    .await
}

/// Create a new workflow template.
pub async fn create_template(
    pool: &PgPool,
    name: &str,
    description: Option<&str>,
    ticket_type: &str,
    steps: &[WorkflowStep],
    is_default: bool,
) -> Result<WorkflowTemplate, sqlx::Error> {
    sqlx::query_as!(
        WorkflowTemplate,
        r#"
        INSERT INTO workflow_templates (name, description, ticket_type, steps_json, is_default)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, description, ticket_type,
                  steps_json as "steps_json: _", is_default,
                  created_at, updated_at
        "#,
        name,
        description,
        ticket_type,
        sqlx::types::Json(steps) as _,
        is_default
    )
    .fetch_one(pool)
    .await
}

/// Get active workflow for a ticket.
pub async fn get_active_workflow(
    pool: &PgPool,
    ticket_id: &str,
) -> Result<Option<WorkflowInstance>, sqlx::Error> {
    sqlx::query_as!(
        WorkflowInstance,
        r#"
        SELECT id, template_id, ticket_id, user_id, status,
               current_step, started_at, paused_at, completed_at,
               created_at, updated_at
        FROM workflow_instances
        WHERE ticket_id = $1 AND status IN ('active', 'paused')
        ORDER BY created_at DESC
        LIMIT 1
        "#,
        ticket_id
    )
    .fetch_optional(pool)
    .await
}

/// Create a new workflow instance.
pub async fn create_instance(
    pool: &PgPool,
    template_id: Uuid,
    ticket_id: &str,
    user_id: &str,
) -> Result<WorkflowInstance, sqlx::Error> {
    sqlx::query_as!(
        WorkflowInstance,
        r#"
        INSERT INTO workflow_instances (template_id, ticket_id, user_id)
        VALUES ($1, $2, $3)
        RETURNING id, template_id, ticket_id, user_id, status,
                  current_step, started_at, paused_at, completed_at,
                  created_at, updated_at
        "#,
        template_id,
        ticket_id,
        user_id
    )
    .fetch_one(pool)
    .await
}

/// Get step results for a workflow instance.
pub async fn get_step_results(
    pool: &PgPool,
    instance_id: Uuid,
) -> Result<Vec<WorkflowStepResult>, sqlx::Error> {
    sqlx::query_as!(
        WorkflowStepResult,
        r#"
        SELECT id, instance_id, step_index, status, notes,
               links as "links: _", started_at, completed_at,
               created_at, updated_at
        FROM workflow_step_results
        WHERE instance_id = $1
        ORDER BY step_index
        "#,
        instance_id
    )
    .fetch_all(pool)
    .await
}
```

### Project Structure Notes

Files to create:
```
migrations/
└── 20260104000001_workflow_schema.sql

crates/qa-pms-workflow/src/
├── lib.rs
├── types.rs
└── repository.rs
```

### Testing Notes

- Test migration runs without errors
- Test foreign key constraints reject invalid data
- Test indexes improve query performance
- Test Rust types serialize/deserialize correctly
- Test repository functions with test database

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 5.1]
- [Source: _bmad-output/planning-artifacts/architecture.md#Database Schema]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### Debug Log References

N/A

### Completion Notes List

- Migration SQL creates 3 tables: workflow_templates, workflow_instances, workflow_step_results
- Indexes on ticket_type, ticket_id, user_id, status, template_id, instance_id
- Foreign keys with ON DELETE CASCADE for step results
- Triggers for auto-updating updated_at columns
- Rust types with FromRow derives for SQLx
- WorkflowStatus and StepStatus enums with string conversion
- Repository functions using dynamic queries (sqlx::query_as)
- 4 unit tests passing

### File List

- migrations/20260104000001_workflow_schema.sql (new)
- crates/qa-pms-workflow/src/types.rs (new)
- crates/qa-pms-workflow/src/repository.rs (new)
- crates/qa-pms-workflow/src/lib.rs (updated)

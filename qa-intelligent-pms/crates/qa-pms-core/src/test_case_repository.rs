//! Test case repository for database operations.
//!
//! This module provides CRUD operations for test cases in the database.

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use tracing::{debug, instrument};

// Import from crate root where types are re-exported
use crate::{
    TestCase, TestCaseId, TestCaseStatus, TestCaseType, TestPriority, TestRepository, TicketId,
};

/// Repository for test case data.
pub struct TestCaseRepository {
    pool: PgPool,
}

impl TestCaseRepository {
    /// Create a new repository.
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new test case.
    #[instrument(skip(self), fields(test_case_id = %test_case.id))]
    pub async fn create(&self, test_case: &TestCase) -> crate::Result<()> {
        debug!("Creating test case");

        // Validate before saving
        test_case
            .validate()
            .map_err(|e| anyhow::anyhow!("Validation failed: {}", e))?;

        let repository_str = match &test_case.repository {
            TestRepository::Base => "Base".to_string(),
            TestRepository::Reativo => "Reativo".to_string(),
            TestRepository::Sprint(id) => format!("Sprint-{}", id),
        };

        sqlx::query(
            r"
            INSERT INTO test_cases (
                id, title, description, preconditions, priority, test_type,
                steps, expected_result, automatizable, component, endpoint, method,
                ticket_key, repository, folder_path, base_case_id, tags, status,
                created_date, updated_at, last_executed, execution_count, success_rate
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12,
                $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23
            )
            ",
        )
        .bind(test_case.id.to_string())
        .bind(&test_case.title)
        .bind(&test_case.description)
        .bind(&test_case.preconditions)
        .bind(test_case.priority.to_string())
        .bind(test_case.test_type.to_string())
        .bind(&test_case.steps)
        .bind(&test_case.expected_result)
        .bind(test_case.automatizable)
        .bind(&test_case.component)
        .bind(&test_case.endpoint)
        .bind(&test_case.method)
        .bind(test_case.ticket_key.as_ref().map(|k| k.to_string()))
        .bind(&repository_str)
        .bind(&test_case.folder_path)
        .bind(test_case.base_case_id)
        .bind(&test_case.tags)
        .bind(test_case.status.to_string())
        .bind(test_case.created_date)
        .bind(test_case.updated_at)
        .bind(test_case.last_executed)
        .bind(test_case.execution_count as i32)
        .bind(test_case.success_rate)
        .execute(&self.pool)
        .await?;

        debug!("Test case created successfully");
        Ok(())
    }

    /// Get a test case by ID.
    #[instrument(skip(self), fields(test_case_id = %id))]
    pub async fn get(&self, id: &TestCaseId) -> crate::Result<Option<TestCase>> {
        debug!("Fetching test case");

        let row: Option<TestCaseRow> = sqlx::query_as(
            r"
            SELECT 
                id, title, description, preconditions, priority, test_type,
                steps, expected_result, automatizable, component, endpoint, method,
                ticket_key, repository, folder_path, base_case_id, tags, status,
                created_date, updated_at, last_executed, execution_count, success_rate
            FROM test_cases
            WHERE id = $1
            ",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Into::into))
    }

    /// Update an existing test case.
    #[instrument(skip(self), fields(test_case_id = %test_case.id))]
    pub async fn update(&self, test_case: &TestCase) -> crate::Result<()> {
        debug!("Updating test case");

        // Validate before updating
        test_case
            .validate()
            .map_err(|e| anyhow::anyhow!("Validation failed: {}", e))?;

        let repository_str = match &test_case.repository {
            TestRepository::Base => "Base".to_string(),
            TestRepository::Reativo => "Reativo".to_string(),
            TestRepository::Sprint(id) => format!("Sprint-{}", id),
        };

        sqlx::query(
            r"
            UPDATE test_cases SET
                title = $2, description = $3, preconditions = $4, priority = $5,
                test_type = $6, steps = $7, expected_result = $8, automatizable = $9,
                component = $10, endpoint = $11, method = $12, ticket_key = $13,
                repository = $14, folder_path = $15, base_case_id = $16, tags = $17,
                status = $18, updated_at = $19, last_executed = $20,
                execution_count = $21, success_rate = $22
            WHERE id = $1
            ",
        )
        .bind(test_case.id.to_string())
        .bind(&test_case.title)
        .bind(&test_case.description)
        .bind(&test_case.preconditions)
        .bind(test_case.priority.to_string())
        .bind(test_case.test_type.to_string())
        .bind(&test_case.steps)
        .bind(&test_case.expected_result)
        .bind(test_case.automatizable)
        .bind(&test_case.component)
        .bind(&test_case.endpoint)
        .bind(&test_case.method)
        .bind(test_case.ticket_key.as_ref().map(|k| k.to_string()))
        .bind(&repository_str)
        .bind(&test_case.folder_path)
        .bind(test_case.base_case_id)
        .bind(&test_case.tags)
        .bind(test_case.status.to_string())
        .bind(Utc::now()) // Always update updated_at on update
        .bind(test_case.last_executed)
        .bind(test_case.execution_count as i32)
        .bind(test_case.success_rate)
        .execute(&self.pool)
        .await?;

        debug!("Test case updated successfully");
        Ok(())
    }

    /// Delete a test case by ID.
    #[instrument(skip(self), fields(test_case_id = %id))]
    pub async fn delete(&self, id: &TestCaseId) -> crate::Result<bool> {
        debug!("Deleting test case");

        let result = sqlx::query("DELETE FROM test_cases WHERE id = $1")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        let deleted = result.rows_affected() > 0;
        if deleted {
            debug!("Test case deleted successfully");
        } else {
            debug!("Test case not found");
        }

        Ok(deleted)
    }

    /// Get all test cases for a specific ticket.
    #[instrument(skip(self), fields(ticket_key = %ticket_key))]
    pub async fn get_by_ticket(&self, ticket_key: &TicketId) -> crate::Result<Vec<TestCase>> {
        debug!("Fetching test cases for ticket");

        let rows: Vec<TestCaseRow> = sqlx::query_as(
            r"
            SELECT 
                id, title, description, preconditions, priority, test_type,
                steps, expected_result, automatizable, component, endpoint, method,
                ticket_key, repository, folder_path, base_case_id, tags, status,
                created_date, updated_at, last_executed, execution_count, success_rate
            FROM test_cases
            WHERE ticket_key = $1
            ORDER BY created_date DESC
            ",
        )
        .bind(ticket_key.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    /// Get test cases by component.
    #[instrument(skip(self), fields(component = %component))]
    pub async fn get_by_component(&self, component: &str) -> crate::Result<Vec<TestCase>> {
        debug!("Fetching test cases for component");

        let rows: Vec<TestCaseRow> = sqlx::query_as(
            r"
            SELECT 
                id, title, description, preconditions, priority, test_type,
                steps, expected_result, automatizable, component, endpoint, method,
                ticket_key, repository, folder_path, base_case_id, tags, status,
                created_date, updated_at, last_executed, execution_count, success_rate
            FROM test_cases
            WHERE component = $1
            ORDER BY priority, created_date DESC
            ",
        )
        .bind(component)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    /// Get test cases by status.
    #[instrument(skip(self), fields(status = %status))]
    pub async fn get_by_status(&self, status: TestCaseStatus) -> crate::Result<Vec<TestCase>> {
        debug!("Fetching test cases by status");

        let rows: Vec<TestCaseRow> = sqlx::query_as(
            r"
            SELECT 
                id, title, description, preconditions, priority, test_type,
                steps, expected_result, automatizable, component, endpoint, method,
                ticket_key, repository, folder_path, base_case_id, tags, status,
                created_date, updated_at, last_executed, execution_count, success_rate
            FROM test_cases
            WHERE status = $1
            ORDER BY priority, created_date DESC
            ",
        )
        .bind(status.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    /// Get recent test cases.
    #[instrument(skip(self))]
    pub async fn get_recent(&self, limit: i32) -> crate::Result<Vec<TestCase>> {
        debug!("Fetching recent test cases");

        let rows: Vec<TestCaseRow> = sqlx::query_as(
            r"
            SELECT 
                id, title, description, preconditions, priority, test_type,
                steps, expected_result, automatizable, component, endpoint, method,
                ticket_key, repository, folder_path, base_case_id, tags, status,
                created_date, updated_at, last_executed, execution_count, success_rate
            FROM test_cases
            ORDER BY created_date DESC
            LIMIT $1
            ",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }
}

// Internal row type for sqlx
#[derive(sqlx::FromRow)]
struct TestCaseRow {
    id: String,
    title: String,
    description: String,
    preconditions: Vec<String>,
    priority: String,
    test_type: String,
    steps: Vec<String>,
    expected_result: String,
    automatizable: bool,
    component: String,
    endpoint: Option<String>,
    method: Option<String>,
    ticket_key: Option<String>,
    repository: String,
    folder_path: Vec<String>,
    base_case_id: Option<i64>,
    tags: Vec<String>,
    status: String,
    created_date: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    last_executed: Option<DateTime<Utc>>,
    execution_count: i32,
    success_rate: f64,
}

impl From<TestCaseRow> for TestCase {
    fn from(row: TestCaseRow) -> Self {
        let priority = match row.priority.as_str() {
            "p0" => TestPriority::P0,
            "p1" => TestPriority::P1,
            "p2" => TestPriority::P2,
            "p3" => TestPriority::P3,
            _ => TestPriority::P2, // Default fallback
        };

        let test_type = match row.test_type.as_str() {
            "API" => TestCaseType::Api,
            "Integration" => TestCaseType::Integration,
            "UI" => TestCaseType::Ui,
            "Stress" => TestCaseType::Stress,
            _ => TestCaseType::Api, // Default fallback
        };

        let repository = if row.repository == "Base" {
            TestRepository::Base
        } else if row.repository == "Reativo" {
            TestRepository::Reativo
        } else if row.repository.starts_with("Sprint-") {
            let sprint_id = row.repository.strip_prefix("Sprint-").unwrap_or("").to_string();
            TestRepository::Sprint(sprint_id)
        } else {
            TestRepository::Base // Default fallback
        };

        let status = match row.status.as_str() {
            "draft" => TestCaseStatus::Draft,
            "active" => TestCaseStatus::Active,
            "archived" => TestCaseStatus::Archived,
            "deprecated" => TestCaseStatus::Deprecated,
            _ => TestCaseStatus::Draft, // Default fallback
        };

        Self {
            id: TestCaseId::new(row.id),
            title: row.title,
            description: row.description,
            preconditions: row.preconditions,
            priority,
            test_type,
            steps: row.steps,
            expected_result: row.expected_result,
            automatizable: row.automatizable,
            component: row.component,
            endpoint: row.endpoint,
            method: row.method,
            ticket_key: row.ticket_key.map(TicketId::from),
            repository,
            folder_path: row.folder_path,
            base_case_id: row.base_case_id,
            tags: row.tags,
            status,
            created_date: row.created_date,
            updated_at: row.updated_at,
            last_executed: row.last_executed,
            execution_count: row.execution_count as u32,
            success_rate: row.success_rate,
        }
    }
}

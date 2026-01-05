//! Test case data models for QA Intelligent PMS.
//!
//! This module provides the core data structures for representing test cases,
//! including priority levels and test case metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::ids::{TestCaseId, TicketId};

/// Test case execution priority.
///
/// P0: Critical - executar sempre
/// P1: Alto - executar frequentemente
/// P2: Médio - executar regularmente
/// P3: Baixo - executar ocasionalmente
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TestPriority {
    /// P0: Crítico - executar sempre
    P0,
    /// P1: Alto - executar frequentemente
    P1,
    /// P2: Médio - executar regularmente
    P2,
    /// P3: Baixo - executar ocasionalmente
    P3,
}

impl std::fmt::Display for TestPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestPriority::P0 => write!(f, "p0"),
            TestPriority::P1 => write!(f, "p1"),
            TestPriority::P2 => write!(f, "p2"),
            TestPriority::P3 => write!(f, "p3"),
        }
    }
}

/// Test case type classification.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TestCaseType {
    /// API test
    Api,
    /// Integration test
    Integration,
    /// UI test
    Ui,
    /// Stress test
    Stress,
}

impl std::fmt::Display for TestCaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestCaseType::Api => write!(f, "API"),
            TestCaseType::Integration => write!(f, "Integration"),
            TestCaseType::Ui => write!(f, "UI"),
            TestCaseType::Stress => write!(f, "Stress"),
        }
    }
}

/// Test case status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TestCaseStatus {
    /// Draft - test case is being created or edited
    Draft,
    /// Active - test case is ready for execution
    Active,
    /// Archived - test case is no longer active
    Archived,
    /// Deprecated - test case is deprecated and should not be used
    Deprecated,
}

impl std::fmt::Display for TestCaseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestCaseStatus::Draft => write!(f, "draft"),
            TestCaseStatus::Active => write!(f, "active"),
            TestCaseStatus::Archived => write!(f, "archived"),
            TestCaseStatus::Deprecated => write!(f, "deprecated"),
        }
    }
}

/// Default test case status (Draft).
fn default_status() -> TestCaseStatus {
    TestCaseStatus::Draft
}

/// Repository location for test cases.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TestRepository {
    /// Base repository
    Base,
    /// Reativo (reactive) repository
    Reativo,
    /// Sprint-specific repository
    Sprint(String),
}

impl std::fmt::Display for TestRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestRepository::Base => write!(f, "Base"),
            TestRepository::Reativo => write!(f, "Reativo"),
            TestRepository::Sprint(id) => write!(f, "Sprint-{}", id),
        }
    }
}

/// Represents a test case, either automatically generated or manually created.
///
/// Test cases follow specific naming conventions:
/// - Base: `{METHOD}_{TestType}_{Description}`
/// - Preventivo: `{TICKET_KEY}_{METHOD}_{TestType}_{Description}`
/// - Reativo: `{METHOD}_{Endpoint}_{Priority}_{Date}`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestCase {
    /// Unique identifier (Testmo ID or internal ID)
    pub id: TestCaseId,
    /// Test case title (following naming convention)
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Preconditions required before executing this test
    #[serde(default)]
    pub preconditions: Vec<String>,
    /// Priority level for execution
    pub priority: TestPriority,
    /// Test type classification
    #[serde(rename = "type")]
    pub test_type: TestCaseType,
    /// Test execution steps
    pub steps: Vec<String>,
    /// Expected result
    pub expected_result: String,
    /// Whether this test can be automated
    pub automatizable: bool,
    /// Related component (normalized)
    pub component: String,
    /// Related endpoint (normalized, optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// HTTP method (GET, POST, etc., optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Related ticket key (for preventive flow, optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_key: Option<TicketId>,
    /// Repository location ("Base", "Reativo", or "Sprint-{ID}")
    pub repository: TestRepository,
    /// Folder path in Testmo (e.g., ["Base", "Booking", "POST_api-v3-quotes"])
    pub folder_path: Vec<String>,
    /// ID of the base case if this was inherited (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_case_id: Option<i64>,
    /// Tags including inheritance links
    #[serde(default)]
    pub tags: Vec<String>,
    /// Test case status
    #[serde(default = "default_status")]
    pub status: TestCaseStatus,
    /// Creation timestamp
    pub created_date: DateTime<Utc>,
    /// Last update timestamp
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: DateTime<Utc>,
    /// Last execution timestamp (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_executed: Option<DateTime<Utc>>,
    /// Number of times this test was executed
    #[serde(default)]
    pub execution_count: u32,
    /// Success rate (0.0-1.0)
    #[serde(default)]
    pub success_rate: f64,
}

impl TestCase {
    /// Create a new test case with default values.
    #[must_use]
    pub fn new(
        id: TestCaseId,
        title: String,
        description: String,
        priority: TestPriority,
        test_type: TestCaseType,
        steps: Vec<String>,
        expected_result: String,
        component: String,
        repository: TestRepository,
    ) -> Self {
        Self {
            id,
            title,
            description,
            priority,
            test_type,
            steps,
            expected_result,
            automatizable: true,
            component,
            endpoint: None,
            method: None,
            ticket_key: None,
            repository,
            folder_path: vec![],
            base_case_id: None,
            tags: vec![],
            preconditions: vec![],
            status: TestCaseStatus::Draft,
            created_date: Utc::now(),
            updated_at: Utc::now(),
            last_executed: None,
            execution_count: 0,
            success_rate: 0.0,
        }
    }

    /// Update the test case's updated_at timestamp.
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    /// Validate test case structure.
    ///
    /// Returns `Ok(())` if valid, or an error message if invalid.
    pub fn validate(&self) -> Result<(), String> {
        if self.title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if self.description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }
        if self.steps.is_empty() {
            return Err("Test steps cannot be empty".to_string());
        }
        if self.expected_result.is_empty() {
            return Err("Expected result cannot be empty".to_string());
        }
        if self.component.is_empty() {
            return Err("Component cannot be empty".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_priority_serialization() {
        let priority = TestPriority::P0;
        let json = serde_json::to_string(&priority).expect("Failed to serialize");
        assert_eq!(json, r#""p0""#);
        let deserialized: TestPriority = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(priority, deserialized);
    }

    #[test]
    fn test_test_case_serialization() {
        let test_case = TestCase::new(
            TestCaseId::new("123"),
            "Test login".to_string(),
            "Test description".to_string(),
            TestPriority::P1,
            TestCaseType::Api,
            vec!["Step 1".to_string(), "Step 2".to_string()],
            "Expected result".to_string(),
            "auth".to_string(),
            TestRepository::Base,
        );
        
        let json = serde_json::to_string(&test_case).expect("Failed to serialize");
        assert!(json.contains(r#""id":"123""#));
        assert!(json.contains(r#""priority":"p1""#));
        assert!(json.contains(r#""type":"API""#));
        
        let deserialized: TestCase = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(test_case.id, deserialized.id);
        assert_eq!(test_case.title, deserialized.title);
    }

    #[test]
    fn test_test_repository_display() {
        assert_eq!(TestRepository::Base.to_string(), "Base");
        assert_eq!(TestRepository::Reativo.to_string(), "Reativo");
        assert_eq!(TestRepository::Sprint("42".to_string()).to_string(), "Sprint-42");
    }

    #[test]
    fn test_test_priority_display() {
        assert_eq!(TestPriority::P0.to_string(), "p0");
        assert_eq!(TestPriority::P1.to_string(), "p1");
        assert_eq!(TestPriority::P2.to_string(), "p2");
        assert_eq!(TestPriority::P3.to_string(), "p3");
    }

    #[test]
    fn test_test_case_status_serialization() {
        let status = TestCaseStatus::Active;
        let json = serde_json::to_string(&status).expect("Failed to serialize");
        assert_eq!(json, r#""active""#);
        let deserialized: TestCaseStatus = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(status, deserialized);
    }

    #[test]
    fn test_test_case_status_display() {
        assert_eq!(TestCaseStatus::Draft.to_string(), "draft");
        assert_eq!(TestCaseStatus::Active.to_string(), "active");
        assert_eq!(TestCaseStatus::Archived.to_string(), "archived");
        assert_eq!(TestCaseStatus::Deprecated.to_string(), "deprecated");
    }

    #[test]
    fn test_test_case_validation_success() {
        let test_case = TestCase::new(
            TestCaseId::new("123"),
            "Test login".to_string(),
            "Test description".to_string(),
            TestPriority::P1,
            TestCaseType::Api,
            vec!["Step 1".to_string()],
            "Expected result".to_string(),
            "auth".to_string(),
            TestRepository::Base,
        );
        assert!(test_case.validate().is_ok());
    }

    #[test]
    fn test_test_case_validation_empty_title() {
        let test_case = TestCase::new(
            TestCaseId::new("123"),
            "".to_string(),
            "Test description".to_string(),
            TestPriority::P1,
            TestCaseType::Api,
            vec!["Step 1".to_string()],
            "Expected result".to_string(),
            "auth".to_string(),
            TestRepository::Base,
        );
        assert!(test_case.validate().is_err());
        assert!(test_case.validate().unwrap_err().contains("Title"));
    }

    #[test]
    fn test_test_case_validation_empty_steps() {
        let test_case = TestCase::new(
            TestCaseId::new("123"),
            "Test login".to_string(),
            "Test description".to_string(),
            TestPriority::P1,
            TestCaseType::Api,
            vec![],
            "Expected result".to_string(),
            "auth".to_string(),
            TestRepository::Base,
        );
        assert!(test_case.validate().is_err());
        assert!(test_case.validate().unwrap_err().contains("steps"));
    }

    #[test]
    fn test_test_case_touch() {
        let mut test_case = TestCase::new(
            TestCaseId::new("123"),
            "Test login".to_string(),
            "Test description".to_string(),
            TestPriority::P1,
            TestCaseType::Api,
            vec!["Step 1".to_string()],
            "Expected result".to_string(),
            "auth".to_string(),
            TestRepository::Base,
        );
        let old_updated = test_case.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(10));
        test_case.touch();
        assert!(test_case.updated_at > old_updated);
    }
}

//! Testmo API response types.
//!
//! Typed structs for Testmo API responses.

use serde::{Deserialize, Serialize};

// ============================================================================
// Response Wrappers
// ============================================================================

/// Response wrapper for projects list.
#[derive(Debug, Deserialize)]
pub struct ProjectsResponse {
    /// List of projects.
    pub data: Vec<Project>,
}

/// Response wrapper for test suites list.
#[derive(Debug, Deserialize)]
pub struct TestSuitesResponse {
    /// List of test suites.
    pub data: Vec<TestSuite>,
}

/// Response wrapper for test cases list.
#[derive(Debug, Deserialize)]
pub struct TestCasesResponse {
    /// List of test cases.
    pub data: Vec<TestCase>,
}

/// Response wrapper for single test case.
#[derive(Debug, Deserialize)]
pub struct TestCaseResponse {
    /// Test case data.
    pub data: TestCase,
}

/// Response wrapper for test run.
#[derive(Debug, Deserialize)]
pub struct TestRunResponse {
    /// Test run data.
    pub data: TestRun,
}

// ============================================================================
// Core Types
// ============================================================================

/// Testmo project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Project unique ID.
    pub id: i64,
    /// Project name.
    pub name: String,
    /// Project description.
    pub description: Option<String>,
    /// Creation timestamp.
    pub created_at: String,
    /// Last update timestamp.
    pub updated_at: String,
}

/// Test suite (folder for organizing test cases).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    /// Suite unique ID.
    pub id: i64,
    /// Parent project ID.
    pub project_id: i64,
    /// Suite name.
    pub name: String,
    /// Suite description.
    pub description: Option<String>,
    /// Parent suite ID (for nested suites).
    pub parent_id: Option<i64>,
    /// Nesting depth.
    pub depth: i32,
    /// Creation timestamp.
    pub created_at: String,
    /// Last update timestamp.
    pub updated_at: String,
}

/// Test case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    /// Test case unique ID.
    pub id: i64,
    /// Parent project ID.
    pub project_id: i64,
    /// Parent suite ID.
    pub suite_id: Option<i64>,
    /// Test case title.
    pub title: String,
    /// Preconditions for the test.
    pub preconditions: Option<String>,
    /// Priority level ID.
    pub priority_id: Option<i32>,
    /// Test type ID.
    pub type_id: Option<i32>,
    /// Template ID.
    pub template_id: Option<i32>,
    /// Test steps.
    pub steps: Option<Vec<TestStep>>,
    /// Creation timestamp.
    pub created_at: String,
    /// Last update timestamp.
    pub updated_at: String,
}

/// Test step within a test case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStep {
    /// Step content/action.
    pub content: String,
    /// Expected result.
    pub expected: Option<String>,
}

/// Test run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRun {
    /// Test run unique ID.
    pub id: i64,
    /// Parent project ID.
    pub project_id: i64,
    /// Test run name.
    pub name: String,
    /// Test run description.
    pub description: Option<String>,
    /// Status ID.
    pub status_id: i32,
    /// Creation timestamp.
    pub created_at: String,
    /// Last update timestamp.
    pub updated_at: String,
}

// ============================================================================
// Request Types
// ============================================================================

/// Request body for creating a test run.
#[derive(Debug, Serialize)]
pub struct CreateTestRunRequest {
    /// Test run name.
    pub name: String,
    /// Test case IDs to include in the run.
    pub case_ids: Vec<i64>,
}

// ============================================================================
// Search Types
// ============================================================================

/// Search result for test cases.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    /// Source integration (always "testmo").
    pub source: String,
    /// Test case ID.
    pub id: String,
    /// Test case title.
    pub name: String,
    /// Test case preconditions.
    pub description: Option<String>,
    /// URL to view test case in Testmo.
    pub url: String,
    /// Match score (higher is better).
    pub score: f32,
    /// Matching text snippets.
    pub matches: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_project() {
        let json = r#"{
            "id": 1,
            "name": "My Project",
            "description": "Test project",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-02T00:00:00Z"
        }"#;
        let project: Project = serde_json::from_str(json).unwrap();
        assert_eq!(project.id, 1);
        assert_eq!(project.name, "My Project");
    }

    #[test]
    fn test_deserialize_test_suite() {
        let json = r#"{
            "id": 10,
            "project_id": 1,
            "name": "Login Tests",
            "description": "All login related tests",
            "parent_id": null,
            "depth": 0,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-02T00:00:00Z"
        }"#;
        let suite: TestSuite = serde_json::from_str(json).unwrap();
        assert_eq!(suite.id, 10);
        assert_eq!(suite.name, "Login Tests");
        assert_eq!(suite.depth, 0);
    }

    #[test]
    fn test_deserialize_test_case() {
        let json = r#"{
            "id": 100,
            "project_id": 1,
            "suite_id": 10,
            "title": "Verify login with valid credentials",
            "preconditions": "User must have a valid account",
            "priority_id": 1,
            "type_id": 2,
            "template_id": 1,
            "steps": [
                {"content": "Enter username", "expected": "Username field accepts input"},
                {"content": "Enter password", "expected": "Password field masks input"}
            ],
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-02T00:00:00Z"
        }"#;
        let case: TestCase = serde_json::from_str(json).unwrap();
        assert_eq!(case.id, 100);
        assert_eq!(case.title, "Verify login with valid credentials");
        assert!(case.steps.is_some());
        assert_eq!(case.steps.unwrap().len(), 2);
    }

    #[test]
    fn test_deserialize_test_case_minimal() {
        let json = r#"{
            "id": 101,
            "project_id": 1,
            "suite_id": null,
            "title": "Simple test",
            "preconditions": null,
            "priority_id": null,
            "type_id": null,
            "template_id": null,
            "steps": null,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-02T00:00:00Z"
        }"#;
        let case: TestCase = serde_json::from_str(json).unwrap();
        assert_eq!(case.id, 101);
        assert!(case.suite_id.is_none());
        assert!(case.steps.is_none());
    }

    #[test]
    fn test_serialize_create_test_run_request() {
        let request = CreateTestRunRequest {
            name: "Sprint 1 Regression".to_string(),
            case_ids: vec![100, 101, 102],
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("Sprint 1 Regression"));
        assert!(json.contains("[100,101,102]"));
    }
}

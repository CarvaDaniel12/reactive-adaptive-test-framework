//! Testmo API client.
//!
//! HTTP client for interacting with the Testmo API.

use crate::error::TestmoError;
use crate::types::{
    CreateTestRunRequest, Project, ProjectsResponse, SearchResult, TestCase, TestCaseResponse,
    TestCasesResponse, TestRun, TestRunResponse, TestSuite, TestSuitesResponse,
};
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, warn};

/// Default request timeout in seconds.
const DEFAULT_TIMEOUT_SECS: u64 = 10;

/// Maximum retry attempts.
const MAX_RETRIES: u32 = 3;

/// Base delay for exponential backoff (1 second).
const BASE_DELAY_SECS: u64 = 1;

/// Testmo API client.
///
/// Provides methods for interacting with the Testmo API including
/// listing projects, test suites, test cases, and creating test runs.
#[derive(Clone)]
pub struct TestmoClient {
    http_client: Client,
    api_key: String,
    base_url: String,
}

impl TestmoClient {
    /// Create a new Testmo client.
    ///
    /// # Arguments
    /// * `base_url` - Testmo instance URL (e.g., "<https://company.testmo.net>")
    /// * `api_key` - Testmo API key for authentication
    ///
    /// # Panics
    /// Panics if the HTTP client cannot be created.
    #[must_use]
    pub fn new(base_url: String, api_key: String) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()
            .expect("Failed to create HTTP client");

        // Ensure base_url doesn't have trailing slash
        let base_url = base_url.trim_end_matches('/').to_string();

        Self {
            http_client,
            api_key,
            base_url,
        }
    }

    /// Get the base URL.
    #[must_use]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Make an authenticated GET request with retry logic.
    async fn request<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T, TestmoError> {
        let url = format!("{}/api/v1{}", self.base_url, endpoint);

        self.with_retry(|| async {
            debug!(endpoint = %endpoint, "Making Testmo API request");

            let response = self
                .http_client
                .get(&url)
                .bearer_auth(&self.api_key)
                .send()
                .await?;

            let status = response.status();

            if status.is_success() {
                let body = response.text().await?;
                serde_json::from_str(&body).map_err(|e| {
                    TestmoError::Parse(format!("{}: {}", e, &body[..200.min(body.len())]))
                })
            } else if status == reqwest::StatusCode::UNAUTHORIZED {
                Err(TestmoError::Unauthorized)
            } else if status == reqwest::StatusCode::NOT_FOUND {
                Err(TestmoError::NotFound(endpoint.to_string()))
            } else if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                Err(TestmoError::RateLimited)
            } else {
                let body = response.text().await.unwrap_or_default();
                Err(TestmoError::ApiError { status, body })
            }
        })
        .await
    }

    /// Make an authenticated POST request with retry logic.
    async fn post<T, B>(&self, endpoint: &str, body: &B) -> Result<T, TestmoError>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        let url = format!("{}/api/v1{}", self.base_url, endpoint);

        self.with_retry(|| async {
            debug!(endpoint = %endpoint, "Making Testmo API POST request");

            let response = self
                .http_client
                .post(&url)
                .bearer_auth(&self.api_key)
                .json(body)
                .send()
                .await?;

            let status = response.status();

            if status.is_success() {
                let body = response.text().await?;
                serde_json::from_str(&body).map_err(|e| {
                    TestmoError::Parse(format!("{}: {}", e, &body[..200.min(body.len())]))
                })
            } else if status == reqwest::StatusCode::UNAUTHORIZED {
                Err(TestmoError::Unauthorized)
            } else {
                let body = response.text().await.unwrap_or_default();
                Err(TestmoError::ApiError { status, body })
            }
        })
        .await
    }

    /// Execute a function with exponential backoff retry.
    async fn with_retry<T, F, Fut>(&self, f: F) -> Result<T, TestmoError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, TestmoError>>,
    {
        let mut attempt = 0;

        loop {
            attempt += 1;
            match f().await {
                Ok(result) => return Ok(result),
                Err(e) if e.is_retryable() && attempt < MAX_RETRIES => {
                    let delay = Duration::from_secs(BASE_DELAY_SECS * 2u64.pow(attempt - 1));
                    warn!(
                        attempt = attempt,
                        max_attempts = MAX_RETRIES,
                        delay_ms = delay.as_millis(),
                        error = %e,
                        "Testmo API error, retrying"
                    );
                    sleep(delay).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    // ========================================================================
    // Project Operations
    // ========================================================================

    /// List all projects.
    ///
    /// Returns all projects accessible with the configured API key.
    ///
    /// # Errors
    /// Returns error if the API call fails or response cannot be parsed.
    pub async fn list_projects(&self) -> Result<Vec<Project>, TestmoError> {
        debug!("Listing Testmo projects");
        let response: ProjectsResponse = self.request("/projects").await?;
        debug!(count = response.data.len(), "Retrieved projects");
        Ok(response.data)
    }

    // ========================================================================
    // Test Suite Operations
    // ========================================================================

    /// List test suites in a project.
    ///
    /// # Arguments
    /// * `project_id` - Project ID to list suites from
    ///
    /// # Errors
    /// Returns error if the API call fails or response cannot be parsed.
    pub async fn list_test_suites(&self, project_id: i64) -> Result<Vec<TestSuite>, TestmoError> {
        let endpoint = format!("/projects/{project_id}/suites");
        debug!(project_id = project_id, "Listing Testmo test suites");
        let response: TestSuitesResponse = self.request(&endpoint).await?;
        debug!(count = response.data.len(), "Retrieved test suites");
        Ok(response.data)
    }

    // ========================================================================
    // Test Case Operations
    // ========================================================================

    /// List test cases in a project.
    ///
    /// Optionally filter by suite ID.
    ///
    /// # Arguments
    /// * `project_id` - Project ID to list cases from
    /// * `suite_id` - Optional suite ID to filter cases
    ///
    /// # Errors
    /// Returns error if the API call fails or response cannot be parsed.
    pub async fn list_test_cases(
        &self,
        project_id: i64,
        suite_id: Option<i64>,
    ) -> Result<Vec<TestCase>, TestmoError> {
        let endpoint = match suite_id {
            Some(id) => format!("/projects/{project_id}/cases?suite_id={id}"),
            None => format!("/projects/{project_id}/cases"),
        };

        debug!(
            project_id = project_id,
            suite_id = suite_id,
            "Listing Testmo test cases"
        );

        let response: TestCasesResponse = self.request(&endpoint).await?;
        debug!(count = response.data.len(), "Retrieved test cases");
        Ok(response.data)
    }

    /// Get test case details.
    ///
    /// Returns the complete test case including steps.
    ///
    /// # Arguments
    /// * `project_id` - Project ID
    /// * `case_id` - Test case ID
    ///
    /// # Errors
    /// Returns error if the test case is not found or API call fails.
    pub async fn get_test_case(
        &self,
        project_id: i64,
        case_id: i64,
    ) -> Result<TestCase, TestmoError> {
        let endpoint = format!("/projects/{project_id}/cases/{case_id}");
        debug!(
            project_id = project_id,
            case_id = case_id,
            "Getting test case details"
        );
        let response: TestCaseResponse = self.request(&endpoint).await?;
        Ok(response.data)
    }

    // ========================================================================
    // Search Operations
    // ========================================================================

    /// Search test cases by keywords.
    ///
    /// Searches test case titles, preconditions, and steps.
    /// Returns results ranked by match score.
    ///
    /// # Arguments
    /// * `project_id` - Project ID to search within
    /// * `keywords` - Keywords to search for
    ///
    /// # Errors
    /// Returns error if API calls fail.
    pub async fn search_test_cases(
        &self,
        project_id: i64,
        keywords: &[String],
    ) -> Result<Vec<SearchResult>, TestmoError> {
        debug!(
            project_id = project_id,
            keywords = ?keywords,
            "Searching Testmo test cases"
        );

        // Get all test cases in the project
        let test_cases = self.list_test_cases(project_id, None).await?;

        let mut results = Vec::new();

        for case in test_cases {
            let mut score = 0.0;

            // Score by title (weighted higher)
            score += calculate_match_score(&case.title, keywords) * 2.0;

            // Score by preconditions
            if let Some(ref preconditions) = case.preconditions {
                score += calculate_match_score(preconditions, keywords);
            }

            // Score by steps
            if let Some(ref steps) = case.steps {
                for step in steps {
                    score += calculate_match_score(&step.content, keywords) * 0.5;
                    if let Some(ref expected) = step.expected {
                        score += calculate_match_score(expected, keywords) * 0.3;
                    }
                }
            }

            if score > 0.0 {
                results.push(SearchResult {
                    source: "testmo".to_string(),
                    id: case.id.to_string(),
                    name: case.title.clone(),
                    description: case.preconditions.clone(),
                    url: format!("{}/projects/{}/cases/{}", self.base_url, project_id, case.id),
                    score,
                    matches: vec![case.title],
                });
            }
        }

        // Sort by score descending
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        debug!(results_count = results.len(), "Search completed");
        Ok(results)
    }

    // ========================================================================
    // Test Run Operations
    // ========================================================================

    /// Create a test run.
    ///
    /// Creates a new test run with the specified test cases.
    ///
    /// # Arguments
    /// * `project_id` - Project ID to create the run in
    /// * `name` - Name for the test run
    /// * `case_ids` - Test case IDs to include in the run
    ///
    /// # Errors
    /// Returns error if the API call fails.
    pub async fn create_test_run(
        &self,
        project_id: i64,
        name: &str,
        case_ids: &[i64],
    ) -> Result<TestRun, TestmoError> {
        let endpoint = format!("/projects/{project_id}/runs");

        debug!(
            project_id = project_id,
            name = name,
            case_count = case_ids.len(),
            "Creating Testmo test run"
        );

        let body = CreateTestRunRequest {
            name: name.to_string(),
            case_ids: case_ids.to_vec(),
        };

        let response: TestRunResponse = self.post(&endpoint, &body).await?;
        debug!(run_id = response.data.id, "Test run created");
        Ok(response.data)
    }
}

/// Calculate match score for text against keywords.
fn calculate_match_score(text: &str, keywords: &[String]) -> f32 {
    if keywords.is_empty() {
        return 0.0;
    }

    let text_lower = text.to_lowercase();
    let mut score = 0.0;

    for keyword in keywords {
        let keyword_lower = keyword.to_lowercase();
        if text_lower.contains(&keyword_lower) {
            score += 1.0;
            // Bonus for exact word match
            if text_lower.split_whitespace().any(|w| w == keyword_lower) {
                score += 0.5;
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TestStep;

    #[test]
    fn test_new_strips_trailing_slash() {
        let client = TestmoClient::new(
            "https://company.testmo.net/".to_string(),
            "api-key".to_string(),
        );
        assert_eq!(client.base_url(), "https://company.testmo.net");
    }

    #[test]
    fn test_calculate_match_score_no_match() {
        let score = calculate_match_score("Login test case", &["payment".to_string()]);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_calculate_match_score_partial_match() {
        let score = calculate_match_score("User Login Test", &["login".to_string()]);
        assert!(score > 0.0);
    }

    #[test]
    fn test_calculate_match_score_exact_word_bonus() {
        let partial = calculate_match_score("LoginTest", &["login".to_string()]);
        let exact = calculate_match_score("Login Test", &["login".to_string()]);
        assert!(exact > partial);
    }

    #[test]
    fn test_calculate_match_score_multiple_keywords() {
        let score =
            calculate_match_score("User Login Test", &["user".to_string(), "login".to_string()]);
        assert!(score >= 2.0);
    }

    #[test]
    fn test_calculate_match_score_empty_keywords() {
        let score = calculate_match_score("Anything", &[]);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_search_scoring_with_steps() {
        let case = TestCase {
            id: 1,
            project_id: 1,
            suite_id: None,
            title: "Verify user login".to_string(),
            preconditions: Some("User has valid credentials".to_string()),
            priority_id: None,
            type_id: None,
            template_id: None,
            steps: Some(vec![
                TestStep {
                    content: "Enter username".to_string(),
                    expected: Some("Username accepted".to_string()),
                },
                TestStep {
                    content: "Enter password".to_string(),
                    expected: Some("Password masked".to_string()),
                },
            ]),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        };

        // Title match should have highest weight
        let title_score = calculate_match_score(&case.title, &["login".to_string()]) * 2.0;
        assert!(title_score > 0.0);

        // Steps should also contribute
        let step_score =
            calculate_match_score(&case.steps.as_ref().unwrap()[0].content, &["username".to_string()]) * 0.5;
        assert!(step_score > 0.0);
    }
}

//! Test generation API endpoints.
//!
//! Story 31.1: Auto-Test Generation from Tickets

use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};
use utoipa::ToSchema;

use crate::app::AppState;
use crate::user_config_health::jira_tickets_client_from_user_config;
use qa_pms_ai::{GeneratedTestCase, TestGenerator, TicketDetails};
use qa_pms_core::{
    error::ApiError, TestCase, TestCaseId, TestCaseRepository, TestCaseType, TestPriority,
    TestRepository, TicketId,
};
use secrecy::ExposeSecret;

type ApiResult<T> = Result<T, ApiError>;

/// Create the test generation router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/ai/generate-tests", post(generate_tests))
        .route("/api/v1/ai/regenerate-tests", post(regenerate_tests))
}

// ==================== Request/Response Types ====================

/// Request to generate test cases from a ticket.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GenerateTestsRequest {
    /// Jira ticket key (e.g., "PROJ-123")
    pub ticket_key: String,
    /// Include regression tests
    #[serde(default = "default_true")]
    pub include_regression: bool,
    /// Include security tests
    #[serde(default = "default_false")]
    pub include_security: bool,
    /// Include performance tests
    #[serde(default = "default_false")]
    pub include_performance: bool,
    /// Force regeneration (bypass cache) - Task 7.4
    #[serde(default = "default_false")]
    pub force: bool,
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

/// Request to regenerate test cases with different parameters.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegenerateTestsRequest {
    /// Jira ticket key
    pub ticket_key: String,
    /// Include regression tests
    #[serde(default = "default_true")]
    pub include_regression: bool,
    /// Include security tests
    #[serde(default = "default_false")]
    pub include_security: bool,
    /// Include performance tests
    #[serde(default = "default_false")]
    pub include_performance: bool,
    /// Force regeneration (bypass cache)
    #[serde(default = "default_true")]
    pub force: bool,
}

/// Response for test generation.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GenerateTestsResponse {
    /// Generated test cases
    pub test_cases: Vec<TestCaseResponse>,
    /// Count of generated test cases
    pub count: usize,
    /// Ticket key
    pub ticket_key: String,
}

/// Test case response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TestCaseResponse {
    /// Test case ID
    pub id: String,
    /// Test case title
    pub title: String,
    /// Test case description
    pub description: String,
    /// Preconditions
    pub preconditions: Vec<String>,
    /// Priority
    pub priority: String,
    /// Test type
    #[serde(rename = "type")]
    pub test_type: String,
    /// Test steps
    pub steps: Vec<String>,
    /// Expected result
    pub expected_result: String,
    /// Whether test is automatizable
    pub automatizable: bool,
    /// Component
    pub component: String,
    /// Endpoint (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// HTTP method (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Tags
    pub tags: Vec<String>,
    /// Status
    pub status: String,
}

impl From<&TestCase> for TestCaseResponse {
    fn from(test_case: &TestCase) -> Self {
        Self {
            id: test_case.id.to_string(),
            title: test_case.title.clone(),
            description: test_case.description.clone(),
            preconditions: test_case.preconditions.clone(),
            priority: test_case.priority.to_string(),
            test_type: test_case.test_type.to_string(),
            steps: test_case.steps.clone(),
            expected_result: test_case.expected_result.clone(),
            automatizable: test_case.automatizable,
            component: test_case.component.clone(),
            endpoint: test_case.endpoint.clone(),
            method: test_case.method.clone(),
            tags: test_case.tags.clone(),
            status: test_case.status.to_string(),
        }
    }
}

// ==================== Handlers ====================

/// Generate test cases from a Jira ticket.
#[utoipa::path(
    post,
    path = "/api/v1/ai/generate-tests",
    request_body = GenerateTestsRequest,
    responses(
        (status = 200, description = "Test cases generated successfully", body = GenerateTestsResponse),
        (status = 404, description = "Ticket not found"),
        (status = 503, description = "AI not configured")
    ),
    tag = "AI"
)]
pub async fn generate_tests(
    State(state): State<AppState>,
    Json(req): Json<GenerateTestsRequest>,
) -> ApiResult<Json<GenerateTestsResponse>> {
    info!("Generating test cases for ticket: {} (force: {})", req.ticket_key, req.force);

    let repository = TestCaseRepository::new(state.db.clone());
    let ticket_id = TicketId::from(req.ticket_key.clone());

    // Check cache (Task 7.1, 7.2): Return cached test cases if they exist and force=false
    if !req.force {
        if let Ok(cached_test_cases) = repository.get_by_ticket(&ticket_id).await {
            if !cached_test_cases.is_empty() {
                info!("Returning {} cached test cases for ticket {}", cached_test_cases.len(), req.ticket_key);
                return Ok(Json(GenerateTestsResponse {
                    test_cases: cached_test_cases.iter().map(TestCaseResponse::from).collect(),
                    count: cached_test_cases.len(),
                    ticket_key: req.ticket_key,
                }));
            }
        }
    } else {
        // Force regeneration: Delete existing test cases (Task 7.4)
        if let Ok(existing) = repository.get_by_ticket(&ticket_id).await {
            let count = existing.len();
            for test_case in existing {
                let _ = repository.delete(&test_case.id).await;
            }
            debug!("Deleted {} existing test cases for forced regeneration", count);
        }
    }

    // Fetch Jira ticket (Task 4.4)
    let jira_client = jira_tickets_client_from_user_config(&state.settings)
        .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to create Jira client: {e}")))?
        .ok_or_else(|| ApiError::ServiceUnavailable("Jira not configured".into()))?;

    let ticket_detail = jira_client
        .get_ticket(&req.ticket_key)
        .await
        .map_err(|e| {
            warn!("Failed to fetch ticket {}: {}", req.ticket_key, e);
            ApiError::NotFound(format!("Ticket {} not found: {}", req.ticket_key, e))
        })?;

    // Convert Jira ticket to TicketDetails (Task 4.4)
    let ticket_details = TicketDetails {
        key: ticket_detail.key.clone(),
        title: ticket_detail.fields.summary.clone(),
        ticket_type: "Task".to_string(), // TODO: Extract from ticket.fields if available
        description: extract_description(&ticket_detail.fields.description),
        acceptance_criteria: extract_acceptance_criteria(&ticket_detail.fields.description),
    };

    // Generate test cases using TestGenerator (Task 4.5)
    let ai_client = create_ai_client(&state)
        .await
        .map_err(|e| ApiError::ServiceUnavailable(format!("AI not configured: {e}")))?;

    let generator = TestGenerator::new(ai_client);
    let generated_test_cases = generator
        .generate_from_ticket(&ticket_details)
        .await
        .map_err(|e| {
            warn!("AI test generation failed: {}", e);
            ApiError::Internal(anyhow::anyhow!("Test generation failed: {e}"))
        })?;

    debug!("Generated {} test cases for ticket {}", generated_test_cases.len(), req.ticket_key);

    // Post-process and validate test cases (Task 6)
    let ticket_type_lower = ticket_details.ticket_type.to_lowercase();
    let processed_test_cases = generator.post_process_test_cases(generated_test_cases, &ticket_type_lower);
    debug!("After post-processing: {} test cases remaining", processed_test_cases.len());

    // Convert GeneratedTestCase to TestCase and save to database (Task 4.6, 7.2)
    
    let mut saved_test_cases = Vec::new();
    for (index, generated) in processed_test_cases.iter().enumerate() {
        let test_case = convert_to_test_case(generated, &ticket_id, index)?;
        
        repository
            .create(&test_case)
            .await
            .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to save test case: {e}")))?;
        
        saved_test_cases.push(test_case);
    }

    info!("Saved {} test cases to database", saved_test_cases.len());

    // Return generated test cases with count (Task 4.7)
    Ok(Json(GenerateTestsResponse {
        test_cases: saved_test_cases.iter().map(TestCaseResponse::from).collect(),
        count: saved_test_cases.len(),
        ticket_key: req.ticket_key,
    }))
}

/// Regenerate test cases with different parameters.
#[utoipa::path(
    post,
    path = "/api/v1/ai/regenerate-tests",
    request_body = RegenerateTestsRequest,
    responses(
        (status = 200, description = "Test cases regenerated successfully", body = GenerateTestsResponse),
        (status = 404, description = "Ticket not found"),
        (status = 503, description = "AI not configured")
    ),
    tag = "AI"
)]
pub async fn regenerate_tests(
    State(state): State<AppState>,
    Json(req): Json<RegenerateTestsRequest>,
) -> ApiResult<Json<GenerateTestsResponse>> {
    info!("Regenerating test cases for ticket: {} (force: {})", req.ticket_key, req.force);

    // If force=true, delete existing test cases for this ticket
    if req.force {
        let repository = TestCaseRepository::new(state.db.clone());
        let ticket_id = TicketId::from(req.ticket_key.clone());
        
        let existing = repository
            .get_by_ticket(&ticket_id)
            .await
            .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to query test cases: {e}")))?;

        let count = existing.len();
        for test_case in existing {
            repository
                .delete(&test_case.id)
                .await
                .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to delete test case: {e}")))?;
        }

        debug!("Deleted {} existing test cases for ticket {}", count, req.ticket_key);
    }

    // Use same logic as generate_tests with force flag
    let generate_req = GenerateTestsRequest {
        ticket_key: req.ticket_key.clone(),
        include_regression: req.include_regression,
        include_security: req.include_security,
        include_performance: req.include_performance,
        force: req.force,
    };

    generate_tests(State(state), Json(generate_req)).await
}

// ==================== Helper Functions ====================

/// Get encryption key from settings.
fn get_encryption_key(state: &AppState) -> Result<qa_pms_config::Encryptor, ApiError> {
    let key = state.settings.encryption_key.expose_secret();
    qa_pms_config::Encryptor::from_hex_key(key).map_err(ApiError::Internal)
}

/// Create AI client from app state.
async fn create_ai_client(state: &AppState) -> Result<qa_pms_ai::AIClient, ApiError> {
    use qa_pms_ai::provider::AIClient;
    use secrecy::SecretString;

    // Get AI configuration from database
    let config: Option<(String, String, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT provider, model_id, api_key_encrypted, custom_base_url FROM ai_configs WHERE user_id = $1 AND enabled = TRUE LIMIT 1",
    )
    .bind(uuid::Uuid::from_u128(0)) // GLOBAL_AI_USER_ID
    .fetch_optional(&state.db)
    .await
    .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to query AI config: {e}")))?;

    let (provider_str, model_id, encrypted_key, custom_url) = config.ok_or_else(|| {
        ApiError::ServiceUnavailable("AI not configured. Please configure AI in Settings.".into())
    })?;

    // Decrypt API key - must be configured via Settings (wizard)
    let encrypted = encrypted_key.ok_or_else(|| {
        ApiError::ServiceUnavailable(
            "AI API key not configured. Please configure AI in Settings.".into()
        )
    })?;

    let encryptor = get_encryption_key(state)?;
    let api_key = encryptor
        .decrypt(&encrypted)
        .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to decrypt API key: {e}")))?
        .expose_secret()
        .clone();

    if api_key.is_empty() {
        return Err(ApiError::ServiceUnavailable(
            "AI API key is empty. Please reconfigure AI in Settings.".into()
        ));
    }

    // Parse provider
    let provider = match provider_str.to_lowercase().as_str() {
        "openai" | "open_ai" => qa_pms_ai::types::ProviderType::OpenAi,
        "anthropic" => qa_pms_ai::types::ProviderType::Anthropic,
        "deepseek" => qa_pms_ai::types::ProviderType::Deepseek,
        "zai" | "z.ai" => qa_pms_ai::types::ProviderType::Zai,
        "custom" => qa_pms_ai::types::ProviderType::Custom,
        _ => {
            return Err(ApiError::Internal(anyhow::anyhow!("Unknown provider: {}", provider_str)));
        }
    };

    let custom_base_url = custom_url.filter(|s| !s.is_empty());

    AIClient::from_config(
        provider,
        SecretString::new(api_key),
        model_id,
        custom_base_url,
    )
    .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to create AI client: {e}")))
}

//// Convert GeneratedTestCase to TestCase.
fn convert_to_test_case(
    generated: &GeneratedTestCase,
    ticket_id: &TicketId,
    _index: usize,
) -> Result<TestCase, ApiError> {
    use uuid::Uuid;

    // Generate ID (will be Testmo ID when synced, or internal UUID for now)
    let id = TestCaseId::new(format!("generated-{}", Uuid::new_v4()));

    // Map priority string to enum
    let priority = match generated.priority.as_str() {
        "Critical" | "critical" | "P0" | "p0" => TestPriority::P0,
        "High" | "high" | "P1" | "p1" => TestPriority::P1,
        "Medium" | "medium" | "P2" | "p2" => TestPriority::P2,
        "Low" | "low" | "P3" | "p3" => TestPriority::P3,
        _ => TestPriority::P2, // Default
    };

    // Map category to test type (heuristic)
    let test_type = match generated.category.as_str() {
        "integration" => TestCaseType::Integration,
        "performance" => TestCaseType::Stress,
        _ => {
            // Infer from tags or title
            let content_lower = format!("{} {}", generated.title, generated.description).to_lowercase();
            if content_lower.contains("api") || content_lower.contains("endpoint") {
                TestCaseType::Api
            } else if content_lower.contains("ui") || content_lower.contains("page") || content_lower.contains("button") {
                TestCaseType::Ui
            } else {
                TestCaseType::Api // Default
            }
        }
    };

    // Parse preconditions (split by newline or comma)
    let _preconditions = if generated.preconditions.is_empty() {
        vec![]
    } else {
        generated
            .preconditions
            .split(['\n', ','])
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    };

    // Infer component from ticket key (e.g., "PMP-123" -> "PMP")
    let component = ticket_id
        .0
        .split('-')
        .next()
        .unwrap_or("unknown")
        .to_lowercase();

    Ok(TestCase::new(
        id,
        generated.title.clone(),
        generated.description.clone(),
        priority,
        test_type,
        generated.steps.clone(),
        generated.expected_result.clone(),
        component,
        TestRepository::Base, // Generated tests go to Base repository
    ))
}

/// Extract description from Jira ticket description (ADF or text).
fn extract_description(description: &Option<serde_json::Value>) -> String {
    if let Some(desc) = description {
        // Try to extract text from ADF format
        if let Some(content) = desc.get("content") {
            let mut text = String::new();
            extract_text_from_adf(content, &mut text);
            if !text.trim().is_empty() {
                return text.trim().to_string();
            }
        }
        // Fallback: try as plain string
        if let Some(text) = desc.as_str() {
            return text.to_string();
        }
    }
    "No description provided".to_string()
}

/// Recursively extract text from ADF nodes (simplified version).
fn extract_text_from_adf(node: &serde_json::Value, output: &mut String) {
    match node {
        serde_json::Value::Array(arr) => {
            for item in arr {
                extract_text_from_adf(item, output);
            }
        }
        serde_json::Value::Object(obj) => {
            if let Some(serde_json::Value::String(text)) = obj.get("text") {
                output.push_str(text);
            }
            if let Some(content) = obj.get("content") {
                extract_text_from_adf(content, output);
            }
        }
        _ => {}
    }
}

/// Extract acceptance criteria from ticket description.
fn extract_acceptance_criteria(_description: &Option<serde_json::Value>) -> Option<String> {
    // TODO: Implement proper ADF parsing to extract ACs
    // For now, return None
    None
}

/// Invalidate test case cache for a ticket (Task 7.3).
/// 
/// Deletes all generated test cases for a given ticket key.
/// This should be called when a ticket is updated to ensure
/// test cases are regenerated with the latest ticket information.
pub async fn invalidate_test_case_cache(
    state: &AppState,
    ticket_key: &str,
) -> Result<usize, ApiError> {
    let repository = TestCaseRepository::new(state.db.clone());
    let ticket_id = TicketId::from(ticket_key.to_string());
    
    let existing = repository
        .get_by_ticket(&ticket_id)
        .await
        .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to query test cases: {e}")))?;

    let count = existing.len();
    for test_case in &existing {
        repository
            .delete(&test_case.id)
            .await
            .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to delete test case: {e}")))?;
    }

    if count > 0 {
        debug!("Invalidated cache: deleted {} test cases for ticket {}", count, ticket_key);
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use qa_pms_core::{TestCase, TestCaseId, TestCaseType, TestPriority, TestRepository};

    #[test]
    fn test_convert_to_test_case() {
        // Task 8.4: Test conversion from GeneratedTestCase to TestCase
        let generated = GeneratedTestCase {
            title: "Test login".to_string(),
            description: "Test description".to_string(),
            preconditions: "Precondition 1, Precondition 2".to_string(),
            steps: vec!["Step 1".to_string(), "Step 2".to_string()],
            expected_result: "Expected result".to_string(),
            priority: "High".to_string(),
            tags: vec!["tag1".to_string()],
            category: "positive".to_string(),
        };

        let ticket_id = TicketId::from("PROJ-123".to_string());
        let result = convert_to_test_case(&generated, &ticket_id, 0);

        assert!(result.is_ok(), "Conversion should succeed");
        let test_case = result.unwrap();
        assert_eq!(test_case.title, "Test login");
        assert_eq!(test_case.description, "Test description");
        assert_eq!(test_case.steps.len(), 2);
        assert_eq!(test_case.priority, TestPriority::P1); // "High" maps to P1
        assert_eq!(test_case.test_type, TestCaseType::Api); // Default for positive category
        assert_eq!(test_case.component, "proj"); // Extracted from ticket key
        assert_eq!(test_case.repository, TestRepository::Base);
    }

    #[test]
    fn test_convert_to_test_case_priority_mapping() {
        // Task 8.4: Test priority mapping
        let priorities = vec![
            ("Critical", TestPriority::P0),
            ("critical", TestPriority::P0),
            ("P0", TestPriority::P0),
            ("High", TestPriority::P1),
            ("high", TestPriority::P1),
            ("P1", TestPriority::P1),
            ("Medium", TestPriority::P2),
            ("medium", TestPriority::P2),
            ("P2", TestPriority::P2),
            ("Low", TestPriority::P3),
            ("low", TestPriority::P3),
            ("P3", TestPriority::P3),
        ];

        for (priority_str, expected_priority) in priorities {
            let generated = GeneratedTestCase {
                title: "Test".to_string(),
                description: "Desc".to_string(),
                preconditions: String::new(),
                steps: vec!["Step 1".to_string()],
                expected_result: "Result".to_string(),
                priority: priority_str.to_string(),
                tags: vec![],
                category: "positive".to_string(),
            };

            let ticket_id = TicketId::from("PROJ-123".to_string());
            let result = convert_to_test_case(&generated, &ticket_id, 0);
            assert!(result.is_ok(), "Conversion should succeed for priority: {}", priority_str);
            assert_eq!(result.unwrap().priority, expected_priority, "Priority mapping incorrect for: {}", priority_str);
        }
    }

    #[test]
    fn test_convert_to_test_case_type_mapping() {
        // Task 8.4: Test test type mapping from category
        let test_cases = vec![
            ("integration", TestCaseType::Integration),
            ("performance", TestCaseType::Stress),
            ("positive", TestCaseType::Api), // Default
        ];

        for (category, expected_type) in test_cases {
            let generated = GeneratedTestCase {
                title: "Test".to_string(),
                description: "Desc".to_string(),
                preconditions: String::new(),
                steps: vec!["Step 1".to_string()],
                expected_result: "Result".to_string(),
                priority: "High".to_string(),
                tags: vec![],
                category: category.to_string(),
            };

            let ticket_id = TicketId::from("PROJ-123".to_string());
            let result = convert_to_test_case(&generated, &ticket_id, 0);
            assert!(result.is_ok(), "Conversion should succeed for category: {}", category);
            assert_eq!(result.unwrap().test_type, expected_type, "Type mapping incorrect for category: {}", category);
        }
    }

    #[test]
    fn test_extract_description_from_plain_text() {
        // Task 8.3: Test description extraction from plain text
        let desc = serde_json::json!("This is a plain text description");
        let result = extract_description(&Some(desc));
        assert_eq!(result, "This is a plain text description");
    }

    #[test]
    fn test_extract_description_from_adf() {
        // Task 8.3: Test description extraction from ADF format
        let adf = serde_json::json!({
            "content": [
                {
                    "content": [
                        {
                            "text": "This is ",
                            "type": "text"
                        },
                        {
                            "text": "ADF text",
                            "type": "text"
                        }
                    ],
                    "type": "paragraph"
                }
            ],
            "type": "doc",
            "version": 1
        });
        let result = extract_description(&Some(adf));
        assert!(result.contains("This is"));
        assert!(result.contains("ADF text"));
    }

    #[test]
    fn test_extract_description_no_description() {
        // Task 8.3: Test description extraction when missing
        let result = extract_description(&None);
        assert_eq!(result, "No description provided");
    }

    #[test]
    fn test_extract_description_empty_value() {
        // Task 8.3: Test description extraction with empty value
        let empty = serde_json::json!({});
        let result = extract_description(&Some(empty));
        assert_eq!(result, "No description provided");
    }

    #[test]
    fn test_generate_tests_request_serialization() {
        // Task 8.5: Test request serialization
        let json = r#"{
            "ticketKey": "PROJ-123",
            "includeRegression": true,
            "includeSecurity": false,
            "includePerformance": true,
            "force": false
        }"#;

        let req: Result<GenerateTestsRequest, _> = serde_json::from_str(json);
        assert!(req.is_ok(), "Should deserialize request");
        let req = req.unwrap();
        assert_eq!(req.ticket_key, "PROJ-123");
        assert_eq!(req.include_regression, true);
        assert_eq!(req.include_security, false);
        assert_eq!(req.include_performance, true);
        assert_eq!(req.force, false);
    }

    #[test]
    fn test_generate_tests_request_defaults() {
        // Task 8.5: Test request defaults
        let json = r#"{
            "ticketKey": "PROJ-123"
        }"#;

        let req: Result<GenerateTestsRequest, _> = serde_json::from_str(json);
        assert!(req.is_ok(), "Should deserialize with defaults");
        let req = req.unwrap();
        assert_eq!(req.include_regression, true); // default_true()
        assert_eq!(req.include_security, false); // default_false()
        assert_eq!(req.include_performance, false); // default_false()
        assert_eq!(req.force, false); // default_false()
    }

    #[test]
    fn test_test_case_response_from_test_case() {
        // Task 8.5: Test response conversion
        let test_case = TestCase::new(
            TestCaseId::new("test-123"),
            "Test title".to_string(),
            "Test description".to_string(),
            TestPriority::P1,
            TestCaseType::Api,
            vec!["Step 1".to_string()],
            "Expected result".to_string(),
            "component".to_string(),
            TestRepository::Base,
        );

        let response = TestCaseResponse::from(&test_case);
        assert_eq!(response.id, "test-123");
        assert_eq!(response.title, "Test title");
        assert_eq!(response.description, "Test description");
        assert_eq!(response.priority, "p1");
        assert_eq!(response.test_type, "API");
        assert_eq!(response.steps.len(), 1);
    }

    #[test]
    fn test_regenerate_tests_request_serialization() {
        // Task 8.5: Test regenerate request serialization
        let json = r#"{
            "ticketKey": "PROJ-456",
            "includeRegression": false,
            "includeSecurity": true,
            "includePerformance": false,
            "force": true
        }"#;

        let req: Result<RegenerateTestsRequest, _> = serde_json::from_str(json);
        assert!(req.is_ok(), "Should deserialize regenerate request");
        let req = req.unwrap();
        assert_eq!(req.ticket_key, "PROJ-456");
        assert_eq!(req.include_regression, false);
        assert_eq!(req.include_security, true);
        assert_eq!(req.force, true); // Default is true for regenerate
    }

    #[test]
    fn test_regenerate_tests_request_defaults() {
        // Task 8.5: Test regenerate request defaults
        let json = r#"{
            "ticketKey": "PROJ-456"
        }"#;

        let req: Result<RegenerateTestsRequest, _> = serde_json::from_str(json);
        assert!(req.is_ok(), "Should deserialize with defaults");
        let req = req.unwrap();
        assert_eq!(req.include_regression, true); // default_true()
        assert_eq!(req.include_security, false); // default_false()
        assert_eq!(req.include_performance, false); // default_false()
        assert_eq!(req.force, true); // default_true() for regenerate
    }

    // Note: Full integration tests for API endpoints (Task 8.5) would require:
    // - Database setup (testcontainers or in-memory)
    // - Mock Jira client
    // - Mock AI client
    // - Mock AppState with all dependencies
    // 
    // Integration test scenarios to implement:
    // - POST /api/v1/ai/generate-tests: successful generation
    // - POST /api/v1/ai/generate-tests: ticket not found (404)
    // - POST /api/v1/ai/generate-tests: AI not configured (503)
    // - POST /api/v1/ai/generate-tests: cache hit (returns cached tests)
    // - POST /api/v1/ai/generate-tests: force regeneration (bypasses cache)
    // - POST /api/v1/ai/regenerate-tests: successful regeneration
    // - POST /api/v1/ai/regenerate-tests: deletes existing before regeneration when force=true
    //
    // These require proper test infrastructure setup as shown in setup_integration_test.rs

    // Note: Caching functionality tests (Task 8.6) would require:
    // - Database setup with test cases
    // - Repository operations
    //
    // Caching test scenarios to implement:
    // - Cache hit: existing test cases returned when force=false
    // - Cache miss: new test cases generated when none exist
    // - Force regeneration: cache bypassed when force=true
    // - Cache invalidation: test cases deleted on ticket update
    // - Cache storage: test cases persisted with timestamps
    //
    // These require database integration tests
}
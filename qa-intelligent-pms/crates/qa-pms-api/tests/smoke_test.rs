//! Smoke tests for critical API endpoints (P0 priority).
//!
//! These tests verify that the most critical endpoints are accessible and respond correctly.
//! They serve as a quick health check for deployments and can catch breaking changes early.
//!
//! **Note**: Some tests may require a database connection. In CI/CD, these should run
//! against a test database. For local development, they can be skipped if DB is unavailable.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::Value;
use tower::ServiceExt;
use tracing::warn;
use uuid::Uuid;

use qa_pms_api::app::create_app;
use qa_pms_config::Settings;

/// Helper to create minimal test settings.
///
/// Note: This requires DATABASE_URL and ENCRYPTION_KEY environment variables.
/// For smoke tests, these should be set in the test environment.
/// Returns None if settings cannot be loaded (allows tests to skip gracefully).
fn create_test_settings() -> Option<Settings> {
    Settings::from_env().map_err(|e| {
        warn!(
            error = %e,
            "Failed to load test settings (DATABASE_URL and ENCRYPTION_KEY required)"
        );
        e
    }).ok()
}

/// Helper to create test app state (if DB is available).
///
/// Returns None if database is not available (allows tests to skip gracefully).
/// Note: After calling `.with_state()`, the Router type becomes `Router<()>`.
/// Note: HealthScheduler is intentionally ignored in tests as it's only needed for production background tasks.
async fn create_test_app_state() -> Option<axum::Router<()>> {
    let settings = create_test_settings()?;
    
    match create_app(settings).await {
        Ok((app, _scheduler)) => {
            // Scheduler is intentionally ignored - smoke tests don't need background health checks
            Some(app)
        }
        Err(e) => {
            // If DB is not available, return None (test can skip)
            warn!(
                error = %e,
                "Could not create test app (DB may be unavailable). Smoke tests will be skipped. This is OK if DB is not configured."
            );
            None
        }
    }
}

/// Test helper to make HTTP request and parse JSON response.
///
/// Returns a Result to allow proper error handling in tests.
/// Uses timeout to prevent tests from hanging indefinitely.
async fn make_request(
    app: &axum::Router<()>,
    method: &str,
    path: &str,
    body: Option<&str>,
) -> Result<(StatusCode, Value), String> {
    use http_body_util::BodyExt;
    use tokio::time::{timeout, Duration};
    
    let body = body
        .map(|s| Body::from(s.to_string()))
        .unwrap_or_else(Body::empty);

    let request = Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json")
        .body(body)
        .map_err(|e| format!("Failed to build request: {}", e))?;

    // Add timeout to prevent tests from hanging (10 seconds should be more than enough)
    let response_future = app.clone().oneshot(request);
    let response = timeout(Duration::from_secs(10), response_future)
        .await
        .map_err(|_| "Request timed out after 10 seconds".to_string())?
        .map_err(|e| format!("Request failed: {}", e))?;
    
    let status = response.status();
    let body_bytes = timeout(
        Duration::from_secs(10),
        response.into_body().collect(),
    )
    .await
    .map_err(|_| "Body collection timed out".to_string())?
    .map_err(|e| format!("Failed to collect body: {}", e))?
    .to_bytes();
    
    let json: Value = if body_bytes.is_empty() {
        serde_json::json!({})
    } else {
        serde_json::from_slice(&body_bytes).unwrap_or_else(|_| {
            // If not JSON, return as string value
            Value::String(String::from_utf8_lossy(&body_bytes).to_string())
        })
    };

    Ok((status, json))
}

// ============================================================================
// P0 Smoke Tests - Critical Endpoints
// ============================================================================

/// [P0] Health check endpoint should respond.
///
/// This is the most critical endpoint - it's used by load balancers and
/// monitoring systems to verify the service is running.
#[tokio::test]
async fn test_health_check_endpoint() {
    let Some(app) = create_test_app_state().await else {
        return;
    };

    let (status, json) = make_request(&app, "GET", "/api/v1/health", None)
        .await
        .expect("Request should succeed");

    assert_eq!(
        status,
        StatusCode::OK,
        "Health check should return 200 OK"
    );

    // Verify response structure and types
    let status_str = json.get("status")
        .and_then(|s| s.as_str())
        .expect("Health response should have 'status' field as string");
    assert!(
        status_str == "healthy" || status_str == "unhealthy",
        "status should be 'healthy' or 'unhealthy', got: {}",
        status_str
    );

    let version = json.get("version")
        .and_then(|v| v.as_str())
        .expect("Health response should have 'version' field as string");
    assert!(!version.is_empty(), "version should not be empty");

    let timestamp = json.get("timestamp")
        .and_then(|t| t.as_str())
        .expect("Health response should have 'timestamp' field as string");
    assert!(!timestamp.is_empty(), "timestamp should not be empty");

    let database = json.get("database")
        .expect("Health response should have 'database' field");
    assert!(
        database.is_object(),
        "database should be an object"
    );
    assert!(
        database.get("connected").and_then(|c| c.as_bool()).is_some(),
        "database.connected should be a boolean"
    );
}

/// [P0] Startup validation endpoint should respond.
///
/// Critical for frontend to verify integrations before allowing user to proceed.
#[tokio::test]
async fn test_startup_validation_endpoint() {
    let Some(app) = create_test_app_state().await else {
        return;
    };

    let (status, json) = make_request(&app, "GET", "/api/v1/startup/validate", None)
        .await
        .expect("Request should succeed");

    assert_eq!(
        status,
        StatusCode::OK,
        "Startup validation should return 200 OK"
    );

    // Verify response structure - API returns "valid" not "canStart"
    let valid = json.get("valid")
        .and_then(|v| v.as_bool())
        .expect("Startup validation should have 'valid' field as boolean");
    // Valid can be true or false depending on integration status
    
    let has_critical_failure = json.get("hasCriticalFailure")
        .and_then(|h| h.as_bool())
        .expect("Startup validation should have 'hasCriticalFailure' field as boolean");
    
    let _results = json.get("results")
        .and_then(|r| r.as_array())
        .expect("Startup validation should have 'results' field as array");
    
    let total_time_ms = json.get("totalTimeMs")
        .and_then(|t| t.as_u64())
        .expect("Startup validation should have 'totalTimeMs' field as number");
    
    // Basic sanity checks
    assert!(total_time_ms > 0, "totalTimeMs should be positive");
    assert!(!valid || !has_critical_failure, "If valid is true, hasCriticalFailure should be false");
}

/// [P0] Setup status endpoint should respond.
///
/// Critical for setup wizard flow - tells frontend if setup is needed.
#[tokio::test]
async fn test_setup_status_endpoint() {
    let Some(app) = create_test_app_state().await else {
        return;
    };

    let (status, json) = make_request(&app, "GET", "/api/v1/setup/status", None)
        .await
        .expect("Request should succeed");

    assert_eq!(
        status,
        StatusCode::OK,
        "Setup status should return 200 OK"
    );

    // Verify response structure and types
    let complete = json.get("complete")
        .and_then(|c| c.as_bool())
        .expect("Setup status should have 'complete' field as boolean");
    
    let _configured_integrations = json.get("configuredIntegrations")
        .and_then(|i| i.as_array())
        .expect("Setup status should have 'configuredIntegrations' field as array");
    
    let profile_configured = json.get("profileConfigured")
        .and_then(|p| p.as_bool())
        .expect("Setup status should have 'profileConfigured' field as boolean");
    
    let server_address = json.get("serverAddress")
        .and_then(|s| s.as_str())
        .expect("Setup status should have 'serverAddress' field as string");
    
    // Basic sanity checks
    assert!(!server_address.is_empty(), "serverAddress should not be empty");
    // If setup is complete, profile should be configured
    assert!(!complete || profile_configured, "If setup is complete, profile should be configured");
}

/// [P0] OpenAPI documentation endpoint should respond.
///
/// Verifies API documentation is available and properly configured.
/// This endpoint doesn't require database, so it's a good baseline test.
#[tokio::test]
async fn test_openapi_docs_endpoint() {
    let Some(app) = create_test_app_state().await else {
        return;
    };

    let (status, json) = make_request(&app, "GET", "/api/v1/openapi.json", None)
        .await
        .expect("Request should succeed");

    assert_eq!(
        status,
        StatusCode::OK,
        "OpenAPI docs should return 200 OK"
    );

    // Verify OpenAPI structure and types
    let openapi_version = json.get("openapi")
        .and_then(|o| o.as_str())
        .expect("OpenAPI response should have 'openapi' field as string");
    assert!(openapi_version.starts_with("3."), "OpenAPI version should be 3.x");
    
    let info = json.get("info")
        .expect("OpenAPI response should have 'info' field")
        .as_object()
        .expect("info should be an object");
    
    let title = info.get("title")
        .and_then(|t| t.as_str())
        .expect("OpenAPI info should have 'title' field as string");
    assert!(!title.is_empty(), "title should not be empty");
    
    let version = info.get("version")
        .and_then(|v| v.as_str())
        .expect("OpenAPI info should have 'version' field as string");
    assert!(!version.is_empty(), "version should not be empty");
    
    let paths = json.get("paths")
        .and_then(|p| p.as_object())
        .expect("OpenAPI response should have 'paths' field as object");
    assert!(!paths.is_empty(), "paths should not be empty");
}

/// [P0] Request ID middleware should be applied.
///
/// Verifies that the request ID middleware is working (critical for tracing/logging).
#[tokio::test]
async fn test_request_id_middleware() {
    let Some(app) = create_test_app_state().await else {
        return;
    };

    use tokio::time::{timeout, Duration};

    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/health")
        .body(Body::empty())
        .expect("Request should be valid");

    let response_future = app.clone().oneshot(request);
    let response = timeout(Duration::from_secs(10), response_future)
        .await
        .expect("Request should not timeout")
        .expect("Request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    
    let request_id_header = response.headers()
        .get("x-request-id")
        .expect("Response should include x-request-id header");
    
    let request_id_str = request_id_header
        .to_str()
        .expect("Request ID should be valid UTF-8");
    
    // Verify it's a valid UUID
    Uuid::parse_str(request_id_str)
        .expect("Request ID should be a valid UUID");
}

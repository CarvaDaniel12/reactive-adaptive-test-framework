//! Request ID middleware for request correlation.
//!
//! Generates or preserves a unique request ID for each HTTP request,
//! adding it to tracing spans and response headers for correlation.

use axum::{
    extract::Request,
    http::{header::HeaderValue, HeaderMap},
    middleware::Next,
    response::Response,
};
use tracing::Span;
use uuid::Uuid;

const REQUEST_ID_HEADER: &str = "x-request-id";

/// Middleware that generates or preserves a request ID for correlation.
///
/// This middleware:
/// - Extracts `x-request-id` header from incoming request if present
/// - Generates a new UUID v4 if header is missing
/// - Records the request ID in the current tracing span
/// - Adds the request ID to response headers
///
/// # Examples
///
/// The middleware should be registered first in the middleware chain:
/// ```rust
/// use axum::{Router, routing::get};
/// use qa_pms_api::middleware::request_id_middleware;
///
/// let app: Router = Router::new()
///     .route("/", get(|| async { "ok" }))
///     .layer(axum::middleware::from_fn(request_id_middleware));
/// ```
pub async fn request_id_middleware(request: Request, next: Next) -> Response {
    // Extract or generate request ID
    let request_id = extract_or_generate_request_id(request.headers());

    // Add to tracing span context
    Span::current().record("request_id", &request_id);

    // Process request and get response
    let mut response = next.run(request).await;

    // Add request ID to response headers
    add_request_id_to_response(&mut response, &request_id);

    response
}

/// Extract request ID from headers or generate a new one.
fn extract_or_generate_request_id(headers: &HeaderMap) -> String {
    headers
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(ToString::to_string)
        .unwrap_or_else(|| Uuid::new_v4().to_string())
}

/// Add request ID to response headers.
fn add_request_id_to_response(response: &mut Response, request_id: &str) {
    if let Ok(header_value) = HeaderValue::from_str(request_id) {
        response.headers_mut().insert(REQUEST_ID_HEADER, header_value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_generates_uuid_when_header_missing() {
        let app = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(axum::middleware::from_fn(request_id_middleware));

        let request = Request::builder()
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.headers().get(REQUEST_ID_HEADER).is_some());

        let request_id = response
            .headers()
            .get(REQUEST_ID_HEADER)
            .unwrap()
            .to_str()
            .unwrap();

        // Verify it's a valid UUID
        assert!(Uuid::parse_str(request_id).is_ok());
    }

    #[tokio::test]
    async fn test_preserves_existing_request_id() {
        let app = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(axum::middleware::from_fn(request_id_middleware));

        let existing_id = "550e8400-e29b-41d4-a716-446655440000";
        let request = Request::builder()
            .uri("/")
            .header(REQUEST_ID_HEADER, existing_id)
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_id = response
            .headers()
            .get(REQUEST_ID_HEADER)
            .unwrap()
            .to_str()
            .unwrap();

        assert_eq!(response_id, existing_id);
    }

    #[tokio::test]
    async fn test_concurrent_requests_have_unique_ids() {
        let app = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(axum::middleware::from_fn(request_id_middleware));

        let mut handles = Vec::new();

        for _ in 0..10 {
            let app_clone = app.clone();
            let handle = tokio::spawn(async move {
                let request = Request::builder()
                    .uri("/")
                    .body(Body::empty())
                    .unwrap();

                let response = app_clone.oneshot(request).await.unwrap();

                response
                    .headers()
                    .get(REQUEST_ID_HEADER)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            });

            handles.push(handle);
        }

        let ids: Vec<String> = futures::future::join_all(handles)
            .await
            .into_iter()
            .map(|r| r.unwrap())
            .collect();

        // All IDs should be unique
        let unique_ids: std::collections::HashSet<&String> = ids.iter().collect();
        assert_eq!(unique_ids.len(), ids.len());
    }

    #[tokio::test]
    async fn test_invalid_header_value_generates_new_uuid() {
        let app = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(axum::middleware::from_fn(request_id_middleware));

        // Create request with non-UTF-8 header value
        let mut request = Request::builder()
            .uri("/")
            .body(Body::empty())
            .unwrap();

        // Insert invalid header directly (bypassing normal header validation)
        request.headers_mut().insert(
            REQUEST_ID_HEADER,
            HeaderValue::from_bytes(&[0xFF, 0xFE]).unwrap(),
        );

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.headers().get(REQUEST_ID_HEADER).is_some());

        let request_id = response
            .headers()
            .get(REQUEST_ID_HEADER)
            .unwrap()
            .to_str()
            .unwrap();

        // Should generate new UUID when header is invalid UTF-8
        assert!(Uuid::parse_str(request_id).is_ok());
    }
}

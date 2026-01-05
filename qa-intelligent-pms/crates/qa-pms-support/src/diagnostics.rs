//! Integration diagnostics service.

use chrono::Utc;
use sqlx::PgPool;
use std::time::Instant;

use crate::error::SupportError;
use crate::repository::SupportRepository;
use crate::types::{DiagnosticResult, DiagnosticsReport};

/// Service for running integration diagnostics.
pub struct DiagnosticsService {
    pool: PgPool,
    repo: SupportRepository,
}

impl DiagnosticsService {
    /// Create a new diagnostics service.
    #[must_use] 
    pub fn new(pool: PgPool) -> Self {
        let repo = SupportRepository::new(pool.clone());
        Self { pool, repo }
    }

    /// Run diagnostics on all integrations.
    pub async fn run_all_diagnostics(&self) -> Result<DiagnosticsReport, SupportError> {
        let mut results = Vec::new();

        // Check database
        results.push(self.check_database().await);

        // Check Jira integration
        results.push(self.check_jira().await);

        // Check Postman integration
        results.push(self.check_postman().await);

        // Check Testmo integration
        results.push(self.check_testmo().await);

        let overall_healthy = results.iter().all(|r| r.passed);
        let failed_count = results.iter().filter(|r| !r.passed).count();

        let summary = if overall_healthy {
            "All integrations are healthy".to_string()
        } else {
            format!("{failed_count} integration(s) have issues")
        };

        Ok(DiagnosticsReport {
            overall_healthy,
            results,
            summary,
            generated_at: Utc::now(),
        })
    }

    /// Run diagnostics for a specific integration.
    pub async fn run_diagnostic(&self, integration: &str) -> Result<DiagnosticResult, SupportError> {
        match integration.to_lowercase().as_str() {
            "database" | "db" => Ok(self.check_database().await),
            "jira" => Ok(self.check_jira().await),
            "postman" => Ok(self.check_postman().await),
            "testmo" => Ok(self.check_testmo().await),
            _ => Err(SupportError::InvalidInput(format!(
                "Unknown integration: {integration}"
            ))),
        }
    }

    /// Check database connectivity and health.
    async fn check_database(&self) -> DiagnosticResult {
        let start = Instant::now();
        
        let check_result = sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await;

        let latency_ms = start.elapsed().as_millis() as u64;
        let recent_error_count = self
            .repo
            .get_integration_error_count("database", 24)
            .await
            .unwrap_or(0);

        match check_result {
            Ok(_) => DiagnosticResult {
                integration: "Database".to_string(),
                passed: true,
                message: "Database connection successful".to_string(),
                latency_ms: Some(latency_ms),
                recent_error_count,
                suggestions: vec![],
                checked_at: Utc::now(),
            },
            Err(e) => DiagnosticResult {
                integration: "Database".to_string(),
                passed: false,
                message: format!("Database connection failed: {e}"),
                latency_ms: Some(latency_ms),
                recent_error_count,
                suggestions: vec![
                    "Check DATABASE_URL environment variable".to_string(),
                    "Verify PostgreSQL server is running".to_string(),
                    "Check network connectivity to database host".to_string(),
                ],
                checked_at: Utc::now(),
            },
        }
    }

    /// Check Jira integration health.
    async fn check_jira(&self) -> DiagnosticResult {
        let start = Instant::now();

        // Check if Jira credentials exist
        let creds_exist: Result<(i64,), _> = sqlx::query_as(
            "SELECT COUNT(*) FROM integration_credentials WHERE integration_type = 'jira'"
        )
        .fetch_one(&self.pool)
        .await;

        let latency_ms = start.elapsed().as_millis() as u64;
        let recent_error_count = self
            .repo
            .get_integration_error_count("jira", 24)
            .await
            .unwrap_or(0);

        match creds_exist {
            Ok((count,)) if count > 0 => {
                // Credentials exist, check token validity
                let token_valid: Result<Option<(bool,)>, _> = sqlx::query_as(
                    r"
                    SELECT (expires_at > NOW()) as valid
                    FROM integration_credentials
                    WHERE integration_type = 'jira'
                    LIMIT 1
                    "
                )
                .fetch_optional(&self.pool)
                .await;

                match token_valid {
                    Ok(Some((true,))) => DiagnosticResult {
                        integration: "Jira".to_string(),
                        passed: true,
                        message: "Jira integration configured and token valid".to_string(),
                        latency_ms: Some(latency_ms),
                        recent_error_count,
                        suggestions: vec![],
                        checked_at: Utc::now(),
                    },
                    Ok(Some((false,))) => DiagnosticResult {
                        integration: "Jira".to_string(),
                        passed: false,
                        message: "Jira OAuth token has expired".to_string(),
                        latency_ms: Some(latency_ms),
                        recent_error_count,
                        suggestions: vec![
                            "Re-authenticate with Jira to refresh the token".to_string(),
                            "Check if the Jira app permissions are still valid".to_string(),
                        ],
                        checked_at: Utc::now(),
                    },
                    _ => DiagnosticResult {
                        integration: "Jira".to_string(),
                        passed: false,
                        message: "Could not verify Jira token status".to_string(),
                        latency_ms: Some(latency_ms),
                        recent_error_count,
                        suggestions: vec![
                            "Check integration_credentials table".to_string(),
                        ],
                        checked_at: Utc::now(),
                    },
                }
            }
            Ok(_) => DiagnosticResult {
                integration: "Jira".to_string(),
                passed: false,
                message: "Jira integration not configured".to_string(),
                latency_ms: Some(latency_ms),
                recent_error_count,
                suggestions: vec![
                    "Go to Settings > Integrations > Jira to configure".to_string(),
                    "You'll need Jira Cloud OAuth credentials".to_string(),
                ],
                checked_at: Utc::now(),
            },
            Err(e) => DiagnosticResult {
                integration: "Jira".to_string(),
                passed: false,
                message: format!("Failed to check Jira configuration: {e}"),
                latency_ms: Some(latency_ms),
                recent_error_count,
                suggestions: vec![
                    "Check database connectivity".to_string(),
                ],
                checked_at: Utc::now(),
            },
        }
    }

    /// Check Postman integration health.
    async fn check_postman(&self) -> DiagnosticResult {
        let start = Instant::now();

        // Check if Postman API key exists
        let config_exists: Result<(i64,), _> = sqlx::query_as(
            "SELECT COUNT(*) FROM user_configs WHERE postman_api_key IS NOT NULL"
        )
        .fetch_one(&self.pool)
        .await;

        let latency_ms = start.elapsed().as_millis() as u64;
        let recent_error_count = self
            .repo
            .get_integration_error_count("postman", 24)
            .await
            .unwrap_or(0);

        match config_exists {
            Ok((count,)) if count > 0 => DiagnosticResult {
                integration: "Postman".to_string(),
                passed: true,
                message: "Postman API key configured".to_string(),
                latency_ms: Some(latency_ms),
                recent_error_count,
                suggestions: if recent_error_count > 5 {
                    vec!["High error count detected - verify API key is still valid".to_string()]
                } else {
                    vec![]
                },
                checked_at: Utc::now(),
            },
            Ok(_) => DiagnosticResult {
                integration: "Postman".to_string(),
                passed: false,
                message: "Postman API key not configured".to_string(),
                latency_ms: Some(latency_ms),
                recent_error_count,
                suggestions: vec![
                    "Go to Settings > Integrations > Postman to configure".to_string(),
                    "Generate an API key from your Postman account settings".to_string(),
                ],
                checked_at: Utc::now(),
            },
            Err(e) => DiagnosticResult {
                integration: "Postman".to_string(),
                passed: false,
                message: format!("Failed to check Postman configuration: {e}"),
                latency_ms: Some(latency_ms),
                recent_error_count,
                suggestions: vec![
                    "Check database connectivity".to_string(),
                ],
                checked_at: Utc::now(),
            },
        }
    }

    /// Check Testmo integration health.
    async fn check_testmo(&self) -> DiagnosticResult {
        let start = Instant::now();

        // Check if Testmo credentials exist
        let config_exists: Result<(i64,), _> = sqlx::query_as(
            "SELECT COUNT(*) FROM user_configs WHERE testmo_url IS NOT NULL AND testmo_api_key IS NOT NULL"
        )
        .fetch_one(&self.pool)
        .await;

        let latency_ms = start.elapsed().as_millis() as u64;
        let recent_error_count = self
            .repo
            .get_integration_error_count("testmo", 24)
            .await
            .unwrap_or(0);

        match config_exists {
            Ok((count,)) if count > 0 => DiagnosticResult {
                integration: "Testmo".to_string(),
                passed: true,
                message: "Testmo integration configured".to_string(),
                latency_ms: Some(latency_ms),
                recent_error_count,
                suggestions: if recent_error_count > 5 {
                    vec!["High error count detected - verify credentials are still valid".to_string()]
                } else {
                    vec![]
                },
                checked_at: Utc::now(),
            },
            Ok(_) => DiagnosticResult {
                integration: "Testmo".to_string(),
                passed: false,
                message: "Testmo integration not configured".to_string(),
                latency_ms: Some(latency_ms),
                recent_error_count,
                suggestions: vec![
                    "Go to Settings > Integrations > Testmo to configure".to_string(),
                    "You'll need your Testmo URL and API key".to_string(),
                ],
                checked_at: Utc::now(),
            },
            Err(e) => DiagnosticResult {
                integration: "Testmo".to_string(),
                passed: false,
                message: format!("Failed to check Testmo configuration: {e}"),
                latency_ms: Some(latency_ms),
                recent_error_count,
                suggestions: vec![
                    "Check database connectivity".to_string(),
                ],
                checked_at: Utc::now(),
            },
        }
    }
}

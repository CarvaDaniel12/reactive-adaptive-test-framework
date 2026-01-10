//! Startup validation service.
//!
//! Validates all configured integration credentials on application startup.
//! Critical integrations (Jira) block the app if validation fails.
//! Optional integrations (Postman, Testmo) only show warnings.

use futures::future::join_all;
use qa_pms_core::health::{HealthCheck, HealthStatus};
use serde::Serialize;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::timeout;
use tracing::{info, warn};
use utoipa::ToSchema;

/// Validation timeout for each integration (5 seconds).
const VALIDATION_TIMEOUT_SECS: u64 = 5;

/// Criticality level of an integration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationCriticality {
    /// Critical integrations block the app if validation fails (e.g., Jira)
    Critical,
    /// Optional integrations only show warnings (e.g., Postman, Testmo)
    Optional,
}

/// Result of validating a single integration.
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    /// Integration name
    pub integration: String,
    /// Whether validation succeeded
    pub success: bool,
    /// Error message if validation failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Response time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_ms: Option<u64>,
    /// Whether this is a critical integration
    pub is_critical: bool,
}

/// Complete startup validation report.
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StartupValidationReport {
    /// Whether the application is valid to start (no critical failures)
    pub valid: bool,
    /// Whether any critical integration failed
    pub has_critical_failure: bool,
    /// Individual validation results
    pub results: Vec<ValidationResult>,
    /// Total validation time in milliseconds
    pub total_time_ms: u64,
}

/// Startup validator that checks all configured integrations.
pub struct StartupValidator {
    checks: Vec<(Arc<dyn HealthCheck>, IntegrationCriticality)>,
}

impl Default for StartupValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl StartupValidator {
    /// Create a new startup validator.
    pub fn new() -> Self {
        Self { checks: Vec::new() }
    }

    /// Add a critical integration check (blocks app on failure).
    #[must_use]
    pub fn add_critical(mut self, check: Arc<dyn HealthCheck>) -> Self {
        info!(
            integration = check.integration_name(),
            "Adding critical integration to startup validator"
        );
        self.checks.push((check, IntegrationCriticality::Critical));
        self
    }

    /// Add an optional integration check (warning only on failure).
    #[must_use]
    pub fn add_optional(mut self, check: Arc<dyn HealthCheck>) -> Self {
        info!(
            integration = check.integration_name(),
            "Adding optional integration to startup validator"
        );
        self.checks.push((check, IntegrationCriticality::Optional));
        self
    }

    /// Get the number of configured checks.
    pub fn check_count(&self) -> usize {
        self.checks.len()
    }

    /// Run all validations in parallel.
    ///
    /// Each validation has a 5-second timeout.
    /// Returns a complete report with individual results.
    pub async fn validate(&self) -> StartupValidationReport {
        let start = Instant::now();

        if self.checks.is_empty() {
            info!("No integrations configured for startup validation");
            return StartupValidationReport {
                valid: true,
                has_critical_failure: false,
                results: Vec::new(),
                total_time_ms: 0,
            };
        }

        info!(
            count = self.checks.len(),
            "Running startup validation for integrations"
        );

        // Run all validations in parallel with timeout
        let futures: Vec<_> = self
            .checks
            .iter()
            .map(|(check, criticality)| {
                let check = Arc::clone(check);
                let criticality = *criticality;
                async move {
                    let name = check.integration_name().to_string();
                    let result =
                        timeout(Duration::from_secs(VALIDATION_TIMEOUT_SECS), check.check()).await;
                    (result, criticality, name)
                }
            })
            .collect();

        let results_raw = join_all(futures).await;

        let results: Vec<ValidationResult> = results_raw
            .into_iter()
            .map(|(result, criticality, name)| {
                let is_critical = criticality == IntegrationCriticality::Critical;

                if let Ok(health_result) = result {
                    let success = matches!(
                        health_result.status,
                        HealthStatus::Online | HealthStatus::Degraded
                    );

                    if success {
                        info!(
                            integration = %name,
                            response_time_ms = ?health_result.response_time_ms,
                            "Startup validation passed"
                        );
                    } else {
                        warn!(
                            integration = %name,
                            error = ?health_result.error_message,
                            critical = is_critical,
                            "Startup validation failed"
                        );
                    }

                    ValidationResult {
                        integration: name,
                        success,
                        error_message: health_result.error_message,
                        response_time_ms: health_result.response_time_ms,
                        is_critical,
                    }
                } else {
                    warn!(
                        integration = %name,
                        timeout_secs = VALIDATION_TIMEOUT_SECS,
                        critical = is_critical,
                        "Startup validation timed out"
                    );

                    ValidationResult {
                        integration: name,
                        success: false,
                        error_message: Some(format!(
                            "Validation timed out (>{VALIDATION_TIMEOUT_SECS}s)"
                        )),
                        response_time_ms: None,
                        is_critical,
                    }
                }
            })
            .collect();

        let has_critical_failure = results.iter().any(|r| r.is_critical && !r.success);
        let valid = !has_critical_failure;

        let total_time_ms = start.elapsed().as_millis() as u64;

        if has_critical_failure {
            warn!(
                total_time_ms = total_time_ms,
                "Startup validation completed with critical failures"
            );
        } else {
            info!(
                total_time_ms = total_time_ms,
                "Startup validation completed successfully"
            );
        }

        StartupValidationReport {
            valid,
            has_critical_failure,
            results,
            total_time_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use qa_pms_core::health::HealthCheckResult;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::time::Duration as StdDuration;

    struct MockHealthCheck {
        name: String,
        status: HealthStatus,
        delay_ms: u64,
        call_count: AtomicU32,
    }

    impl MockHealthCheck {
        fn new(name: &str, status: HealthStatus) -> Self {
            Self {
                name: name.to_string(),
                status,
                delay_ms: 0,
                call_count: AtomicU32::new(0),
            }
        }

        fn with_delay(name: &str, status: HealthStatus, delay_ms: u64) -> Self {
            Self {
                name: name.to_string(),
                status,
                delay_ms,
                call_count: AtomicU32::new(0),
            }
        }
    }

    #[async_trait]
    impl HealthCheck for MockHealthCheck {
        fn integration_name(&self) -> &str {
            &self.name
        }

        async fn check(&self) -> HealthCheckResult {
            self.call_count.fetch_add(1, Ordering::SeqCst);

            if self.delay_ms > 0 {
                tokio::time::sleep(StdDuration::from_millis(self.delay_ms)).await;
            }

            match self.status {
                HealthStatus::Online => {
                    HealthCheckResult::online(&self.name, StdDuration::from_millis(50))
                }
                HealthStatus::Degraded => {
                    HealthCheckResult::degraded(&self.name, StdDuration::from_secs(3), "Slow")
                }
                HealthStatus::Offline => {
                    HealthCheckResult::offline(&self.name, "Connection failed")
                }
            }
        }
    }

    #[tokio::test]
    async fn test_empty_validator() {
        let validator = StartupValidator::new();
        let report = validator.validate().await;

        assert!(report.valid);
        assert!(!report.has_critical_failure);
        assert!(report.results.is_empty());
    }

    #[tokio::test]
    async fn test_all_online() {
        let validator = StartupValidator::new()
            .add_critical(Arc::new(MockHealthCheck::new("jira", HealthStatus::Online)))
            .add_optional(Arc::new(MockHealthCheck::new(
                "postman",
                HealthStatus::Online,
            )));

        let report = validator.validate().await;

        assert!(report.valid);
        assert!(!report.has_critical_failure);
        assert_eq!(report.results.len(), 2);
        assert!(report.results.iter().all(|r| r.success));
    }

    #[tokio::test]
    async fn test_critical_failure_blocks() {
        let validator = StartupValidator::new()
            .add_critical(Arc::new(MockHealthCheck::new(
                "jira",
                HealthStatus::Offline,
            )))
            .add_optional(Arc::new(MockHealthCheck::new(
                "postman",
                HealthStatus::Online,
            )));

        let report = validator.validate().await;

        assert!(!report.valid);
        assert!(report.has_critical_failure);

        let jira = report
            .results
            .iter()
            .find(|r| r.integration == "jira")
            .unwrap();
        assert!(!jira.success);
        assert!(jira.is_critical);
    }

    #[tokio::test]
    async fn test_optional_failure_allows_continue() {
        let validator = StartupValidator::new()
            .add_critical(Arc::new(MockHealthCheck::new("jira", HealthStatus::Online)))
            .add_optional(Arc::new(MockHealthCheck::new(
                "postman",
                HealthStatus::Offline,
            )));

        let report = validator.validate().await;

        assert!(report.valid);
        assert!(!report.has_critical_failure);

        let postman = report
            .results
            .iter()
            .find(|r| r.integration == "postman")
            .unwrap();
        assert!(!postman.success);
        assert!(!postman.is_critical);
    }

    #[tokio::test]
    async fn test_degraded_counts_as_success() {
        let validator = StartupValidator::new().add_critical(Arc::new(MockHealthCheck::new(
            "jira",
            HealthStatus::Degraded,
        )));

        let report = validator.validate().await;

        assert!(report.valid);
        let jira = report
            .results
            .iter()
            .find(|r| r.integration == "jira")
            .unwrap();
        assert!(jira.success);
    }

    #[tokio::test]
    async fn test_parallel_execution() {
        // Two checks that each take 100ms
        let validator = StartupValidator::new()
            .add_critical(Arc::new(MockHealthCheck::with_delay(
                "jira",
                HealthStatus::Online,
                100,
            )))
            .add_optional(Arc::new(MockHealthCheck::with_delay(
                "postman",
                HealthStatus::Online,
                100,
            )));

        let report = validator.validate().await;

        // If parallel, should complete in ~100ms, not ~200ms
        // Allow some margin for test execution
        assert!(report.total_time_ms < 150);
        assert!(report.valid);
    }
}

//! Health check scheduler.
//!
//! Background task that periodically checks integration health.

use futures::future::join_all;
use qa_pms_core::health::HealthCheck;
use qa_pms_core::HealthStore;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tracing::{debug, info};

/// Default health check interval (60 seconds).
pub const DEFAULT_INTERVAL_SECS: u64 = 60;

/// Health check scheduler configuration.
pub struct HealthSchedulerConfig {
    /// Interval between health checks in seconds
    pub interval_secs: u64,
    /// Whether to run an initial check immediately
    pub run_initial_check: bool,
}

impl Default for HealthSchedulerConfig {
    fn default() -> Self {
        Self {
            interval_secs: DEFAULT_INTERVAL_SECS,
            run_initial_check: true,
        }
    }
}

/// Health check scheduler.
///
/// Runs periodic health checks for all configured integrations.
pub struct HealthScheduler {
    checks: Vec<Arc<dyn HealthCheck>>,
    store: Arc<HealthStore>,
    config: HealthSchedulerConfig,
}

impl HealthScheduler {
    /// Create a new health scheduler.
    ///
    /// # Arguments
    /// * `store` - Health store for persisting results
    /// * `config` - Scheduler configuration
    pub fn new(store: Arc<HealthStore>, config: HealthSchedulerConfig) -> Self {
        Self {
            checks: Vec::new(),
            store,
            config,
        }
    }

    /// Create with default configuration.
    pub fn with_defaults(store: Arc<HealthStore>) -> Self {
        Self::new(store, HealthSchedulerConfig::default())
    }

    /// Add a health check.
    ///
    /// Returns self for method chaining.
    #[must_use]
    pub fn add_check(mut self, check: Arc<dyn HealthCheck>) -> Self {
        self.checks.push(check);
        self
    }

    /// Add multiple health checks.
    #[must_use]
    #[allow(dead_code)]
    pub fn add_checks(mut self, checks: Vec<Arc<dyn HealthCheck>>) -> Self {
        self.checks.extend(checks);
        self
    }

    /// Get the number of configured health checks.
    pub fn check_count(&self) -> usize {
        self.checks.len()
    }

    /// Run all health checks once.
    ///
    /// Runs checks in parallel and updates the store.
    pub async fn run_checks(&self) {
        if self.checks.is_empty() {
            debug!("No health checks configured");
            return;
        }

        debug!(
            "Running {} health checks",
            self.checks.len()
        );

        // Run all checks in parallel
        let futures: Vec<_> = self.checks.iter().map(|c| c.check()).collect();
        let results = join_all(futures).await;

        // Update store with results
        for result in results {
            debug!(
                integration = %result.integration,
                status = ?result.status,
                response_time_ms = ?result.response_time_ms,
                "Health check completed"
            );
            self.store.update(result).await;
        }
    }

    /// Start the scheduler as a background task.
    ///
    /// This spawns a tokio task that runs health checks at the configured interval.
    /// The task runs indefinitely until the application shuts down.
    pub fn start(self) {
        let interval_secs = self.config.interval_secs;
        let run_initial = self.config.run_initial_check;
        let check_count = self.checks.len();

        tokio::spawn(async move {
            info!(
                interval_secs = interval_secs,
                check_count = check_count,
                "Health scheduler started"
            );

            // Run initial check if configured
            if run_initial {
                self.run_checks().await;
            }

            let mut ticker = interval(Duration::from_secs(interval_secs));

            loop {
                ticker.tick().await;
                self.run_checks().await;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use qa_pms_core::health::{HealthCheckResult, HealthStatus};
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::time::Duration as StdDuration;

    /// Mock health check for testing.
    struct MockHealthCheck {
        name: String,
        status: HealthStatus,
        call_count: AtomicU32,
    }

    impl MockHealthCheck {
        fn new(name: &str, status: HealthStatus) -> Self {
            Self {
                name: name.to_string(),
                status,
                call_count: AtomicU32::new(0),
            }
        }

        fn calls(&self) -> u32 {
            self.call_count.load(Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl HealthCheck for MockHealthCheck {
        fn integration_name(&self) -> &str {
            &self.name
        }

        async fn check(&self) -> HealthCheckResult {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            match self.status {
                HealthStatus::Online => {
                    HealthCheckResult::online(&self.name, StdDuration::from_millis(50))
                }
                HealthStatus::Degraded => {
                    HealthCheckResult::degraded(&self.name, StdDuration::from_secs(3), "Slow")
                }
                HealthStatus::Offline => HealthCheckResult::offline(&self.name, "Error"),
            }
        }
    }

    #[tokio::test]
    async fn test_scheduler_add_check() {
        let store = Arc::new(HealthStore::new());
        let check = Arc::new(MockHealthCheck::new("test", HealthStatus::Online));

        let scheduler = HealthScheduler::with_defaults(store).add_check(check);

        assert_eq!(scheduler.check_count(), 1);
    }

    #[tokio::test]
    async fn test_scheduler_run_checks() {
        let store = Arc::new(HealthStore::new());
        let check1 = Arc::new(MockHealthCheck::new("jira", HealthStatus::Online));
        let check2 = Arc::new(MockHealthCheck::new("postman", HealthStatus::Offline));

        let scheduler = HealthScheduler::with_defaults(Arc::clone(&store))
            .add_check(Arc::clone(&check1) as Arc<dyn HealthCheck>)
            .add_check(Arc::clone(&check2) as Arc<dyn HealthCheck>);

        scheduler.run_checks().await;

        // Verify checks were called
        assert_eq!(check1.calls(), 1);
        assert_eq!(check2.calls(), 1);

        // Verify store was updated
        let jira = store.get("jira").await.unwrap();
        assert_eq!(jira.status, HealthStatus::Online);

        let postman = store.get("postman").await.unwrap();
        assert_eq!(postman.status, HealthStatus::Offline);
    }

    #[tokio::test]
    async fn test_scheduler_empty_checks() {
        let store = Arc::new(HealthStore::new());
        let scheduler = HealthScheduler::with_defaults(store);

        // Should not panic with empty checks
        scheduler.run_checks().await;
    }

    #[tokio::test]
    async fn test_scheduler_multiple_runs() {
        let store = Arc::new(HealthStore::new());
        let check = Arc::new(MockHealthCheck::new("test", HealthStatus::Online));

        let scheduler = HealthScheduler::with_defaults(Arc::clone(&store))
            .add_check(Arc::clone(&check) as Arc<dyn HealthCheck>);

        // Run multiple times
        scheduler.run_checks().await;
        scheduler.run_checks().await;
        scheduler.run_checks().await;

        assert_eq!(check.calls(), 3);
    }
}

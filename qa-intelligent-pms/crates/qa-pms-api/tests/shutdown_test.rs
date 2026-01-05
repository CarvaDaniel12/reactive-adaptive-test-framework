//! Tests for graceful shutdown and signal handling.
//!
//! Tests cover:
//! - Signal handler functionality
//! - Health scheduler shutdown
//! - Shutdown timeout configuration
//! - Integration tests for graceful shutdown flow

use qa_pms_api::health_scheduler::HealthScheduler;
use qa_pms_core::health::{HealthCheck, HealthCheckResult};
use qa_pms_core::HealthStore;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;
use tokio::time::sleep;

/// Mock health check for testing shutdown behavior.
struct MockHealthCheck {
    name: String,
    call_count: Arc<AtomicU32>,
}

impl MockHealthCheck {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            call_count: Arc::new(AtomicU32::new(0)),
        }
    }

    fn calls(&self) -> u32 {
        self.call_count.load(Ordering::SeqCst)
    }
}

#[async_trait::async_trait]
impl HealthCheck for MockHealthCheck {
    fn integration_name(&self) -> &str {
        &self.name
    }

    async fn check(&self) -> HealthCheckResult {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        // Simulate some work
        sleep(Duration::from_millis(10)).await;
        HealthCheckResult::online(&self.name, Duration::from_millis(50))
    }
}

#[tokio::test]
async fn test_scheduler_shutdown_handle() {
    let store = Arc::new(HealthStore::new());
    let check = Arc::new(MockHealthCheck::new("test"));
    let call_count_before = check.calls();

    let scheduler = HealthScheduler::with_defaults(store)
        .add_check(Arc::clone(&check) as Arc<dyn HealthCheck>);

    let handle = scheduler.start();

    // Wait a bit for scheduler to run initial check
    sleep(Duration::from_millis(100)).await;

    // Verify scheduler is running (check was called)
    assert!(check.calls() > call_count_before, "Scheduler should have run initial check");

    // Signal shutdown
    handle.shutdown().expect("Should signal shutdown successfully");

    // Wait for scheduler to stop
    sleep(Duration::from_millis(200)).await;

    // Record call count after shutdown signal
    let calls_after_shutdown = check.calls();

    // Wait a bit more to ensure scheduler stopped (no new calls)
    sleep(Duration::from_millis(300)).await;

    // Verify scheduler stopped (no new calls after shutdown)
    assert_eq!(
        check.calls(),
        calls_after_shutdown,
        "Scheduler should have stopped after shutdown signal"
    );
}

#[tokio::test]
async fn test_scheduler_shutdown_handle_clone() {
    let store = Arc::new(HealthStore::new());
    let scheduler = HealthScheduler::with_defaults(store);
    let handle1 = scheduler.start();

    // Clone handle should work
    let handle2 = handle1.clone();

    // Both handles should be able to signal shutdown
    handle1.shutdown().expect("First handle should signal shutdown");
    handle2.shutdown().expect("Second handle should signal shutdown");
}

#[tokio::test]
async fn test_scheduler_shutdown_handle_drop() {
    let store = Arc::new(HealthStore::new());
    let check = Arc::new(MockHealthCheck::new("test"));

    let scheduler = HealthScheduler::with_defaults(store)
        .add_check(Arc::clone(&check) as Arc<dyn HealthCheck>);

    // Start scheduler and immediately drop handle
    // Drop should signal shutdown
    {
        let _handle = scheduler.start();
        sleep(Duration::from_millis(50)).await;
    } // Handle dropped here

    // Wait for scheduler to process shutdown
    sleep(Duration::from_millis(300)).await;

    // Scheduler should have stopped (Drop implementation sends shutdown signal)
    let calls_before_wait = check.calls();
    sleep(Duration::from_millis(300)).await;
    assert_eq!(
        check.calls(),
        calls_before_wait,
        "Scheduler should have stopped when handle was dropped"
    );
}

#[tokio::test]
async fn test_scheduler_no_checks_shutdown() {
    let store = Arc::new(HealthStore::new());
    let scheduler = HealthScheduler::with_defaults(store);

    // Should not panic with no checks
    let handle = scheduler.start();
    handle.shutdown().expect("Should handle shutdown with no checks");
}

#[tokio::test]
async fn test_shutdown_handle_multiple_calls() {
    let store = Arc::new(HealthStore::new());
    let scheduler = HealthScheduler::with_defaults(store);
    let handle = scheduler.start();

    // Multiple shutdown calls should succeed (idempotent)
    handle.shutdown().expect("First shutdown call should succeed");
    handle.shutdown().expect("Second shutdown call should succeed");
    handle.shutdown().expect("Third shutdown call should succeed");
}

#[tokio::test]
async fn test_scheduler_stops_cleanly_with_checks() {
    let store = Arc::new(HealthStore::new());
    let check1 = Arc::new(MockHealthCheck::new("check1"));
    let check2 = Arc::new(MockHealthCheck::new("check2"));

    let scheduler = HealthScheduler::with_defaults(store)
        .add_check(Arc::clone(&check1) as Arc<dyn HealthCheck>)
        .add_check(Arc::clone(&check2) as Arc<dyn HealthCheck>);

    let handle = scheduler.start();

    // Let scheduler run a few cycles
    sleep(Duration::from_millis(200)).await;

    let check1_calls_before = check1.calls();
    let check2_calls_before = check2.calls();

    // Signal shutdown
    handle.shutdown().expect("Should signal shutdown");

    // Wait for scheduler to stop
    sleep(Duration::from_millis(300)).await;

    // Verify both checks were called before shutdown
    assert!(check1_calls_before > 0, "Check1 should have been called");
    assert!(check2_calls_before > 0, "Check2 should have been called");

    // Verify scheduler stopped (no new calls after shutdown)
    let check1_calls_after = check1.calls();
    let check2_calls_after = check2.calls();

    sleep(Duration::from_millis(300)).await;

    assert_eq!(
        check1.calls(),
        check1_calls_after,
        "Check1 should have stopped after shutdown"
    );
    assert_eq!(
        check2.calls(),
        check2_calls_after,
        "Check2 should have stopped after shutdown"
    );
}

// Note: Full integration tests for signal handling (SIGINT/SIGTERM) require
// actual signal sending, which is complex in test environment. These would require:
// - Separate process to send signals
// - Test harness with proper signal handling
// - Or using test utilities that mock signal behavior
//
// For now, we test the components in isolation:
// - Health scheduler shutdown (above)
// - Settings timeout configuration (below)
// - Signal handler function compilation (implicit in main.rs compilation)

mod settings_timeout_tests {
    use qa_pms_config::settings::ServerSettings;

    #[test]
    fn test_shutdown_timeout_default() {
        // This test verifies default timeout behavior
        // Actual Settings creation requires environment variables
        // But we can test the timeout calculation logic
        let server = ServerSettings::default();
        assert_eq!(server.shutdown_timeout(), 30, "Default timeout should be 30 seconds");
    }

    #[test]
    fn test_shutdown_timeout_configured() {
        let server = ServerSettings {
            host: "127.0.0.1".to_string(),
            port: 3000,
            shutdown_timeout_secs: Some(60),
        };
        assert_eq!(server.shutdown_timeout(), 60, "Configured timeout should be used");
    }

    #[test]
    fn test_shutdown_timeout_clamped_min() {
        let server = ServerSettings {
            host: "127.0.0.1".to_string(),
            port: 3000,
            shutdown_timeout_secs: Some(0),
        };
        assert_eq!(server.shutdown_timeout(), 1, "Timeout should be clamped to minimum 1s");
    }

    #[test]
    fn test_shutdown_timeout_clamped_max() {
        let server = ServerSettings {
            host: "127.0.0.1".to_string(),
            port: 3000,
            shutdown_timeout_secs: Some(500),
        };
        assert_eq!(server.shutdown_timeout(), 300, "Timeout should be clamped to maximum 300s");
    }

    #[test]
    fn test_shutdown_timeout_in_range() {
        let server = ServerSettings {
            host: "127.0.0.1".to_string(),
            port: 3000,
            shutdown_timeout_secs: Some(45),
        };
        assert_eq!(server.shutdown_timeout(), 45, "Timeout in valid range should be used as-is");
    }
}

// Integration test for full shutdown flow would require:
// - Starting actual server with axum::serve
// - Sending HTTP requests
// - Sending shutdown signal
// - Verifying in-flight requests complete
// - Verifying new requests are rejected with 503
//
// This requires:
// 1. Full app setup (database, settings)
// 2. HTTP client (reqwest or similar)
// 3. Signal sending capability (complex in tests)
// 4. Proper test isolation
//
// For now, component tests above verify the critical shutdown logic.
// Full integration test can be added later with proper test infrastructure.

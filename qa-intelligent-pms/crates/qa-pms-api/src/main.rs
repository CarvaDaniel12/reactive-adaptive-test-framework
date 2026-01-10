//! QA Intelligent PMS API Server
//!
//! Main entry point for the Axum web server.

use anyhow::Result;
use std::time::Instant;
use tokio::time::{timeout, Duration};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod health_scheduler;
mod middleware;
mod routes;
mod startup;
mod user_config_health;

/// Wait for shutdown signal (SIGINT or SIGTERM).
///
/// Cross-platform signal handling:
/// - SIGINT (Ctrl+C): Available on all platforms via `tokio::signal::ctrl_c()`
/// - SIGTERM: Unix-only via `tokio::signal::unix::signal()`
///
/// # Returns
///
/// Returns `Ok(())` when a shutdown signal is received.
///
/// # Errors
///
/// Returns an error if signal handlers cannot be set up (Unix only, for SIGTERM).
async fn shutdown_signal() -> Result<()> {
    let ctrl_c = tokio::signal::ctrl_c();

    #[cfg(unix)]
    {
        let mut terminate = tokio::signal::unix::signal(
            tokio::signal::unix::SignalKind::terminate(),
        )?;

        tokio::select! {
            _ = ctrl_c => {
                info!("Received Ctrl+C (SIGINT), initiating graceful shutdown...");
                Ok(())
            }
            _ = terminate.recv() => {
                info!("Received SIGTERM, initiating graceful shutdown...");
                Ok(())
            }
        }
    }

    #[cfg(not(unix))]
    {
        ctrl_c.await?;
        info!("Received Ctrl+C (SIGINT), initiating graceful shutdown...");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,qa_pms_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting QA Intelligent PMS API Server");

    // Load configuration
    let settings = qa_pms_config::Settings::from_env()?;
    let addr = settings.server_addr();

    info!("Database: {}", settings.database.url_masked());
    info!("Listening on: http://{}", addr);

    // Build the application (returns router and health scheduler)
    let (app, health_scheduler) = app::create_app(settings.clone()).await?;

    // Start the health scheduler as a background task and store shutdown handle
    let scheduler_shutdown_handle = health_scheduler.map(|scheduler| scheduler.start());

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!(
        shutdown_timeout_secs = settings.server.shutdown_timeout(),
        "Starting server with graceful shutdown"
    );

    // Clone handle and settings for async block
    let scheduler_handle_clone = scheduler_shutdown_handle.clone();
    let settings_clone = settings.clone();

    // Start server with graceful shutdown
    let shutdown_result = axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            let shutdown_start = Instant::now();
            let timeout_duration = Duration::from_secs(settings_clone.server.shutdown_timeout());

            match timeout(timeout_duration, shutdown_signal()).await {
                Ok(Ok(())) => {
                    info!("Shutdown signal received, initiating graceful shutdown...");
                }
                Ok(Err(e)) => {
                    tracing::error!(error = %e, "Error waiting for shutdown signal");
                    info!("Forcing shutdown due to signal handler error...");
                }
                Err(_) => {
                    tracing::warn!(
                        timeout_secs = settings_clone.server.shutdown_timeout(),
                        "Shutdown timeout reached while waiting for signal, forcing shutdown"
                    );
                }
            }

            info!("Initiating graceful shutdown...");

            // Signal health scheduler to shutdown
            if let Some(ref handle) = scheduler_handle_clone {
                let _ = handle.shutdown();
                info!("Signaled health scheduler to shutdown");
            }

            // Note: Database pool will be closed automatically when AppState is dropped
            // SQLx PgPool implements Drop trait and closes connections cleanly

            // Log shutdown duration when graceful shutdown completes
            let shutdown_duration = shutdown_start.elapsed();
            info!(
                shutdown_duration_ms = shutdown_duration.as_millis(),
                shutdown_duration_secs = shutdown_duration.as_secs_f64(),
                "Graceful shutdown completed"
            );
        })
        .await;

    match shutdown_result {
        Ok(()) => {
            info!("Server shut down successfully");
        }
        Err(e) => {
            tracing::error!(error = %e, "Error during server shutdown");
            return Err(anyhow::anyhow!("Server shutdown error: {}", e));
        }
    }

    Ok(())
}

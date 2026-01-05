//! QA Intelligent PMS API Server
//!
//! Main entry point for the Axum web server.

use anyhow::Result;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod health_scheduler;
mod routes;
mod startup;

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
    let (app, health_scheduler) = app::create_app(settings).await?;

    // Start the health scheduler as a background task
    if let Some(scheduler) = health_scheduler {
        scheduler.start();
    }

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

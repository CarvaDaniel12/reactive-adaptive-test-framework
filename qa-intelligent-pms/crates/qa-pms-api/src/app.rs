//! Application setup and configuration.
//!
//! Creates the Axum router with all routes and middleware.

use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use axum::Router;
use qa_pms_core::health::HealthCheck;
use qa_pms_core::{AppCache, HealthStore};
use qa_pms_testmo::TestmoClient;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use axum::routing::get;
use axum_prometheus::PrometheusMetricLayer;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;

use qa_pms_config::Settings;

use crate::health_scheduler::HealthScheduler;
use crate::middleware::request_id_middleware;
use crate::routes;
use crate::routes::setup::{create_setup_store, SetupStore};
use crate::user_config_health::UserConfigHealthCheck;

/// Application state shared across all handlers.
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool
    pub db: PgPool,
    /// Application settings
    pub settings: Arc<Settings>,
    /// Temporary setup wizard state
    pub setup_store: SetupStore,
    /// Integration health store
    pub health_store: Arc<HealthStore>,
    /// Testmo client (optional, if configured)
    pub testmo_client: Option<Arc<TestmoClient>>,
    /// Testmo project ID for test runs
    pub testmo_project_id: Option<i64>,
    /// In-memory cache for frequently accessed data
    pub cache: Arc<AppCache>,
}

/// Create the Axum application with all routes and middleware.
///
/// Returns the router and an optional health scheduler to start as a background task.
pub async fn create_app(settings: Settings) -> Result<(Router, Option<HealthScheduler>)> {
    // Create database connection pool
    let db = create_db_pool(&settings).await?;

    // Run migrations
    info!("Running database migrations...");
    sqlx::migrate!("../../migrations")
        .run(&db)
        .await
        .context("Failed to run database migrations")?;
    info!("Migrations complete");

    // Seed default workflow templates
    info!("Seeding default workflow templates...");
    match qa_pms_workflow::seed_default_templates(&db).await {
        Ok(result) => {
            info!(
                created = result.created,
                skipped = result.skipped,
                "Workflow template seeding complete"
            );
        }
        Err(e) => {
            tracing::warn!(error = %e, "Failed to seed workflow templates (non-fatal)");
        }
    }

    // Seed default Splunk query templates
    info!("Seeding default Splunk query templates...");
    let splunk_service = qa_pms_splunk::QueryTemplateService::new(db.clone());
    match splunk_service.seed_default_templates().await {
        Ok(()) => {
            info!("Splunk query template seeding complete");
        }
        Err(e) => {
            tracing::warn!(error = %e, "Failed to seed Splunk query templates (non-fatal)");
        }
    }

    // Create health store for integration monitoring
    let health_store = Arc::new(HealthStore::new());

    // Create health scheduler with the same checks for periodic monitoring
    let health_scheduler = create_health_scheduler(&settings, Arc::clone(&health_store));

    // Create Testmo client if configured
    let (testmo_client, testmo_project_id) = create_testmo_client(&settings);

    // Create cache instance
    let cache = Arc::new(AppCache::new());

    // Create shared state
    let state = AppState {
        db,
        settings: Arc::new(settings),
        setup_store: create_setup_store(),
        health_store,
        testmo_client,
        testmo_project_id,
        cache,
    };

    // Create Prometheus metrics layer
    // Note: Using PrometheusMetricLayer::pair() for now (default config)
    // TODO: Add prefix and ignore patterns when builder API is available
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    // Build the router
    let app = Router::new()
        // Metrics endpoint (must be before other routes)
        .route("/metrics", get(move || {
            let handle = metric_handle.clone();
            async move { handle.render() }
        }))
        .merge(routes::alerts::router())
        .merge(routes::dashboard::router())
        .merge(routes::pm_dashboard::router())
        .merge(routes::health::router())
        .merge(routes::setup::router())
        .merge(routes::tickets::router())
        .merge(routes::startup::router())
        .merge(routes::search::router())
        .nest("/api/v1/testmo", routes::testmo::router())
        .merge(routes::workflows::router())
        .merge(routes::time::router())
        .merge(routes::reports::router())
        .merge(routes::splunk::router())
        .nest("/api/v1/support", routes::support::router())
        .nest("/api/v1/ai", routes::ai::router())
        .merge(routes::integrations::router())
        .merge(routes::api_docs())
        .with_state(state)
        .layer(
            tower::ServiceBuilder::new()
                // Prometheus metrics (outermost to capture all requests)
                .layer(prometheus_layer)
                // Request ID middleware MUST be first for correlation
                .layer(axum::middleware::from_fn(request_id_middleware))
                // Tracing for all requests (can use request_id from span)
                .layer(TraceLayer::new_for_http())
                // Response compression
                .layer(CompressionLayer::new())
                // CORS configuration
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any),
                ),
        );

    Ok((app, health_scheduler))
}

/// Create Testmo client from settings.
fn create_testmo_client(settings: &Settings) -> (Option<Arc<TestmoClient>>, Option<i64>) {
    let Some(testmo_settings) = settings.testmo.as_ref() else {
        return (None, None);
    };

    let api_key = testmo_settings.api_key.expose_secret();
    let base_url = &testmo_settings.base_url;

    if api_key.is_empty() || base_url.is_empty() {
        return (None, None);
    }

    info!("Testmo client configured for {}", base_url);
    let client = TestmoClient::new(base_url.clone(), api_key.clone());
    (Some(Arc::new(client)), testmo_settings.project_id)
}

/// Create health scheduler for periodic integration monitoring.
///
/// Returns `None` if no integrations are configured for monitoring.
fn create_health_scheduler(
    settings: &Settings,
    health_store: Arc<HealthStore>,
) -> Option<HealthScheduler> {
    let mut scheduler = HealthScheduler::with_defaults(health_store);
    let mut has_checks = false;

    // Use per-user config file (`UserConfig`) as the source of truth for credentials.
    // These checks reload config on each run, so they stay in sync after setup.
    if let Some(check) = UserConfigHealthCheck::jira(settings) {
        info!("Adding Jira to health scheduler (UserConfig-backed)");
        scheduler = scheduler.add_check(Arc::new(check) as Arc<dyn HealthCheck>);
        has_checks = true;
    }
    if let Some(check) = UserConfigHealthCheck::postman(settings) {
        info!("Adding Postman to health scheduler (UserConfig-backed)");
        scheduler = scheduler.add_check(Arc::new(check) as Arc<dyn HealthCheck>);
        has_checks = true;
    }
    if let Some(check) = UserConfigHealthCheck::testmo(settings) {
        info!("Adding Testmo to health scheduler (UserConfig-backed)");
        scheduler = scheduler.add_check(Arc::new(check) as Arc<dyn HealthCheck>);
        has_checks = true;
    }

    if has_checks {
        info!(
            check_count = scheduler.check_count(),
            "Health scheduler configured"
        );
        Some(scheduler)
    } else {
        info!("No integrations configured for health monitoring");
        None
    }
}

/// Create the database connection pool.
async fn create_db_pool(settings: &Settings) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .min_connections(settings.database.min_connections)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .connect(settings.database.url.expose_secret())
        .await
        .context("Failed to connect to database")?;

    info!(
        "Database pool created (max: {}, min: {})",
        settings.database.max_connections, settings.database.min_connections
    );

    Ok(pool)
}

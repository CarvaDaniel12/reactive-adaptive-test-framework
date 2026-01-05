//! Dashboard API endpoints.
//!
//! Provides QA performance metrics, trends, and recent activity.
//! Story 6.7: Updated to use real efficiency from time aggregates.

use axum::{extract::Query, extract::State, routing::get, Json, Router};
use chrono::{Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::app::AppState;
use qa_pms_core::error::ApiError;

type ApiResult<T> = Result<T, ApiError>;

trait SqlxResultExt<T> {
    fn map_internal(self, context: &str) -> Result<T, ApiError>;
}

impl<T> SqlxResultExt<T> for Result<T, sqlx::Error> {
    fn map_internal(self, context: &str) -> Result<T, ApiError> {
        self.map_err(|e| ApiError::Internal(anyhow::anyhow!("{context}: {e}")))
    }
}

/// Create the dashboard router.
pub fn router() -> Router<AppState> {
    Router::new().route("/api/v1/dashboard", get(get_dashboard))
}

/// Query parameters for dashboard data.
#[derive(Debug, Deserialize)]
pub struct DashboardQuery {
    /// Period: 7d, 30d, 90d, 1y
    #[serde(default = "default_period")]
    pub period: String,
}

fn default_period() -> String {
    "30d".to_string()
}

/// Dashboard response with KPIs, trend, and activity.
#[derive(Debug, Serialize, ToSchema)]
pub struct DashboardResponse {
    pub kpis: DashboardKPIs,
    pub trend: Vec<TrendDataPoint>,
    pub recent_activity: Vec<ActivityItem>,
}

/// KPI metrics for the dashboard.
#[derive(Debug, Serialize, ToSchema)]
pub struct DashboardKPIs {
    pub tickets_completed: KPIMetric,
    pub avg_time_per_ticket: KPIMetric,
    pub efficiency: KPIMetric,
    pub total_hours: KPIMetric,
}

/// Individual KPI metric with value, change, and trend.
#[derive(Debug, Serialize, ToSchema)]
pub struct KPIMetric {
    pub value: f64,
    pub change: f64,
    pub trend: String, // "up", "down", "neutral"
}

/// Trend data point for charts.
#[derive(Debug, Serialize, ToSchema)]
pub struct TrendDataPoint {
    pub date: String,
    pub tickets: i32,
    pub hours: f64,
}

/// Recent activity item.
#[derive(Debug, Serialize, ToSchema)]
pub struct ActivityItem {
    pub id: String,
    #[serde(rename = "type")]
    pub activity_type: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_key: Option<String>,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}

/// Get dashboard data.
#[utoipa::path(
    get,
    path = "/api/v1/dashboard",
    params(
        ("period" = String, Query, description = "Period: 7d, 30d, 90d, 1y")
    ),
    responses(
        (status = 200, description = "Dashboard data", body = DashboardResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Dashboard"
)]
pub async fn get_dashboard(
    State(state): State<AppState>,
    Query(query): Query<DashboardQuery>,
) -> ApiResult<Json<DashboardResponse>> {
    let days = parse_period(&query.period);
    let pool = &state.db;

    let kpis = calculate_kpis(pool, days).await?;
    let trend = get_trend_data(pool, days).await?;
    let recent_activity = get_recent_activity(pool, 10).await?;

    Ok(Json(DashboardResponse {
        kpis,
        trend,
        recent_activity,
    }))
}

fn parse_period(period: &str) -> i64 {
    match period {
        "7d" => 7,
        "30d" => 30,
        "90d" => 90,
        "1y" => 365,
        _ => 30,
    }
}

async fn calculate_kpis(pool: &PgPool, days: i64) -> Result<DashboardKPIs, ApiError> {
    let now = Utc::now();
    let period_start = now - Duration::days(days);
    let prev_period_start = period_start - Duration::days(days);

    // Current period metrics
    let current = get_period_metrics(pool, period_start, now).await?;
    // Previous period metrics for comparison
    let previous = get_period_metrics(pool, prev_period_start, period_start).await?;

    Ok(DashboardKPIs {
        tickets_completed: KPIMetric {
            value: current.tickets_completed as f64,
            change: calculate_change(current.tickets_completed as f64, previous.tickets_completed as f64),
            trend: calculate_trend(current.tickets_completed as f64, previous.tickets_completed as f64),
        },
        avg_time_per_ticket: KPIMetric {
            value: current.avg_time_seconds,
            change: calculate_change(current.avg_time_seconds, previous.avg_time_seconds),
            trend: calculate_trend(previous.avg_time_seconds, current.avg_time_seconds), // Inverted: lower is better
        },
        efficiency: KPIMetric {
            value: current.efficiency,
            change: calculate_change(current.efficiency, previous.efficiency),
            trend: calculate_trend(current.efficiency, previous.efficiency),
        },
        total_hours: KPIMetric {
            value: current.total_hours,
            change: calculate_change(current.total_hours, previous.total_hours),
            trend: calculate_trend(current.total_hours, previous.total_hours),
        },
    })
}

struct PeriodMetrics {
    tickets_completed: i64,
    avg_time_seconds: f64,
    efficiency: f64,
    total_hours: f64,
}

async fn get_period_metrics(
    pool: &PgPool,
    start: chrono::DateTime<Utc>,
    end: chrono::DateTime<Utc>,
) -> Result<PeriodMetrics, ApiError> {
    let start_date: NaiveDate = start.date_naive();
    let end_date: NaiveDate = end.date_naive();

    // Story 6.7: Try to get metrics from time_daily_aggregates first (more accurate)
    let aggregate_stats: Option<(i64, Option<i64>, Option<i64>, Option<f64>)> = sqlx::query_as(
        r#"
        SELECT 
            COALESCE(SUM(tickets_completed), 0) as tickets,
            SUM(total_time_seconds) as total_time,
            SUM(total_estimated_seconds) as total_estimated,
            AVG(efficiency_ratio)::FLOAT8 as avg_efficiency
        FROM time_daily_aggregates
        WHERE aggregate_date >= $1 AND aggregate_date < $2
        "#,
    )
    .bind(start_date)
    .bind(end_date)
    .fetch_optional(pool)
    .await
    .map_internal("Failed to fetch aggregate stats")?;

    // If we have aggregate data, use it
    if let Some((tickets, total_time, total_estimated, avg_eff)) = aggregate_stats {
        if tickets > 0 {
            let total_seconds = total_time.unwrap_or(0);
            let estimated_seconds = total_estimated.unwrap_or(0);
            
            // Calculate real efficiency: estimated/actual (higher is better, capped at 2.0)
            let efficiency = if total_seconds > 0 && estimated_seconds > 0 {
                (estimated_seconds as f64 / total_seconds as f64).min(2.0)
            } else {
                avg_eff.unwrap_or(1.0)
            };

            return Ok(PeriodMetrics {
                tickets_completed: tickets,
                avg_time_seconds: if tickets > 0 { total_seconds as f64 / tickets as f64 } else { 0.0 },
                efficiency,
                total_hours: total_seconds as f64 / 3600.0,
            });
        }
    }

    // Fallback: Query completed workflows in period (for backward compatibility)
    let workflow_stats: Option<(i64, Option<f64>)> = sqlx::query_as(
        r#"
        SELECT 
            COUNT(*) as count,
            AVG(EXTRACT(EPOCH FROM (completed_at - started_at))) as avg_seconds
        FROM workflow_instances
        WHERE status = 'completed'
          AND completed_at >= $1
          AND completed_at < $2
        "#,
    )
    .bind(start)
    .bind(end)
    .fetch_optional(pool)
    .await
    .map_internal("Failed to fetch workflow stats")?;

    let (tickets_completed, avg_time_seconds) = workflow_stats.unwrap_or((0, None));

    // Query total time tracked in period
    let total_seconds: Option<(Option<f64>,)> = sqlx::query_as(
        r#"
        SELECT SUM(total_seconds) as total
        FROM time_sessions
        WHERE ended_at >= $1
          AND ended_at < $2
        "#,
    )
    .bind(start)
    .bind(end)
    .fetch_optional(pool)
    .await
    .map_internal("Failed to fetch time stats")?;

    let total_hours = total_seconds
        .and_then(|(t,)| t)
        .map(|s| s / 3600.0)
        .unwrap_or(0.0);

    // Fallback efficiency calculation from time_sessions
    let efficiency_result: Option<(Option<i64>, Option<i64>)> = sqlx::query_as(
        r#"
        SELECT 
            SUM(ts.total_seconds) as actual,
            SUM(te.estimated_seconds) as estimated
        FROM time_sessions ts
        LEFT JOIN workflow_instances wi ON ts.workflow_instance_id = wi.id
        LEFT JOIN time_estimates te ON wi.template_id = te.template_id AND ts.step_index = te.step_index
        WHERE ts.ended_at >= $1 AND ts.ended_at < $2
        "#,
    )
    .bind(start)
    .bind(end)
    .fetch_optional(pool)
    .await
    .map_internal("Failed to fetch efficiency data")?;

    let efficiency = match efficiency_result {
        Some((Some(actual), Some(estimated))) if actual > 0 && estimated > 0 => {
            (estimated as f64 / actual as f64).min(2.0)
        }
        _ => 1.0, // Default to 100% efficiency
    };

    Ok(PeriodMetrics {
        tickets_completed,
        avg_time_seconds: avg_time_seconds.unwrap_or(0.0),
        efficiency,
        total_hours,
    })
}

fn calculate_change(current: f64, previous: f64) -> f64 {
    if previous == 0.0 {
        if current > 0.0 { 100.0 } else { 0.0 }
    } else {
        ((current - previous) / previous * 100.0).round()
    }
}

fn calculate_trend(current: f64, previous: f64) -> String {
    if current > previous {
        "up".to_string()
    } else if current < previous {
        "down".to_string()
    } else {
        "neutral".to_string()
    }
}

async fn get_trend_data(pool: &PgPool, days: i64) -> Result<Vec<TrendDataPoint>, ApiError> {
    let now = Utc::now();
    let start_date = now.date_naive() - chrono::Duration::days(days);

    // Story 6.7: Try to get trend from time_daily_aggregates first
    let aggregate_rows: Vec<(NaiveDate, i32, i32)> = sqlx::query_as(
        r#"
        SELECT 
            aggregate_date,
            tickets_completed,
            total_time_seconds
        FROM time_daily_aggregates
        WHERE aggregate_date >= $1
        ORDER BY aggregate_date
        "#,
    )
    .bind(start_date)
    .fetch_all(pool)
    .await
    .map_internal("Failed to fetch aggregate trend data")?;

    if !aggregate_rows.is_empty() {
        return Ok(aggregate_rows
            .into_iter()
            .map(|(date, tickets, seconds)| TrendDataPoint {
                date: date.format("%b %d").to_string(),
                tickets,
                hours: seconds as f64 / 3600.0,
            })
            .collect());
    }

    // Fallback: Query workflow_instances directly
    let start = now - Duration::days(days);
    let rows: Vec<(NaiveDate, i64, Option<f64>)> = sqlx::query_as(
        r#"
        SELECT 
            DATE(completed_at) as date,
            COUNT(*) as tickets,
            SUM(EXTRACT(EPOCH FROM (completed_at - started_at)) / 3600.0) as hours
        FROM workflow_instances
        WHERE status = 'completed'
          AND completed_at >= $1
        GROUP BY DATE(completed_at)
        ORDER BY date
        "#,
    )
    .bind(start)
    .fetch_all(pool)
    .await
    .map_internal("Failed to fetch trend data")?;

    Ok(rows
        .into_iter()
        .map(|(date, tickets, hours)| TrendDataPoint {
            date: date.format("%b %d").to_string(),
            tickets: tickets as i32,
            hours: hours.unwrap_or(0.0),
        })
        .collect())
}

async fn get_recent_activity(pool: &PgPool, limit: i32) -> Result<Vec<ActivityItem>, ApiError> {
    // Get recent completed workflows
    let workflows: Vec<(String, String, Option<String>, chrono::DateTime<Utc>, Option<i64>)> = sqlx::query_as(
        r#"
        SELECT 
            wi.id::text,
            wt.name,
            wi.ticket_key,
            wi.completed_at,
            EXTRACT(EPOCH FROM (wi.completed_at - wi.started_at))::bigint as duration
        FROM workflow_instances wi
        JOIN workflow_templates wt ON wi.template_id = wt.id
        WHERE wi.status = 'completed'
        ORDER BY wi.completed_at DESC
        LIMIT $1
        "#,
    )
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_internal("Failed to fetch recent activity")?;

    Ok(workflows
        .into_iter()
        .map(|(id, name, ticket_key, timestamp, duration)| ActivityItem {
            id,
            activity_type: "workflow_completed".to_string(),
            title: name,
            ticket_key,
            timestamp: timestamp.to_rfc3339(),
            duration,
        })
        .collect())
}

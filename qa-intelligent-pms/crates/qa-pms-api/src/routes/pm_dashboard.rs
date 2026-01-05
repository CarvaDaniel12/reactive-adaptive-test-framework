//! PM Dashboard API endpoints.
//!
//! Epic 10: Provides PM/PO observability metrics including:
//! - Bugs discovered vs prevented
//! - Economy metrics (hours saved, cost savings)
//! - Component health visualization
//! - Problematic endpoints tracking
//! - Dashboard export

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use chrono::{Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
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

/// Create the PM dashboard router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/pm-dashboard", get(get_pm_dashboard))
        .route("/api/v1/pm-dashboard/export", get(export_pm_dashboard))
}

/// Query parameters for PM dashboard.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PMDashboardQuery {
    /// Period: 7d, 30d, 90d, 1y
    #[serde(default = "default_period")]
    pub period: String,
}

fn default_period() -> String {
    "30d".to_string()
}

/// PM Dashboard response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PMDashboardResponse {
    pub summary: PMSummary,
    pub bugs_metrics: BugsMetrics,
    pub economy_metrics: EconomyMetrics,
    pub component_health: Vec<ComponentHealth>,
    pub problematic_endpoints: Vec<ProblematicEndpoint>,
    pub period: String,
    pub generated_at: String,
}

/// High-level summary for PM.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PMSummary {
    pub total_tickets_tested: i64,
    pub total_workflows_completed: i64,
    pub active_qa_users: i64,
    pub avg_time_per_ticket_minutes: f64,
}

/// Bugs discovered vs prevented metrics.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BugsMetrics {
    /// Bugs found during testing (from workflow notes with bug-related keywords)
    pub bugs_discovered: i64,
    /// Bugs prevented (from proactive pattern detection alerts)
    pub bugs_prevented: i64,
    /// Prevention rate: prevented / (discovered + prevented)
    pub prevention_rate: f64,
    /// Change vs previous period
    pub discovered_change: f64,
    pub prevented_change: f64,
}

/// Economy metrics showing ROI.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EconomyMetrics {
    /// Hours saved (when actual < estimated)
    pub hours_saved: f64,
    /// Cost saved (hours * hourly_rate)
    pub cost_saved: f64,
    /// Bug prevention value (bugs_prevented * avg_fix_cost)
    pub bug_prevention_value: f64,
    /// Total economy estimate
    pub total_economy: f64,
    /// Configurable rates used
    pub hourly_rate: f64,
    pub avg_bug_fix_cost: f64,
}

/// Component health status.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ComponentHealth {
    pub component: String,
    pub bug_count: i64,
    pub ticket_count: i64,
    /// "healthy", "degraded", "critical"
    pub status: String,
    /// "improving", "degrading", "stable"
    pub trend: String,
    pub last_issue_date: Option<String>,
}

/// Problematic endpoint tracking.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProblematicEndpoint {
    pub endpoint: String,
    pub issue_count: i64,
    pub common_issues: Vec<String>,
    /// Trend over time
    pub trend: String,
    pub affected_tickets: Vec<String>,
}

/// Get PM dashboard data.
#[utoipa::path(
    get,
    path = "/api/v1/pm-dashboard",
    params(
        ("period" = String, Query, description = "Period: 7d, 30d, 90d, 1y")
    ),
    responses(
        (status = 200, description = "PM Dashboard data", body = PMDashboardResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "PM Dashboard"
)]
pub async fn get_pm_dashboard(
    State(state): State<AppState>,
    Query(query): Query<PMDashboardQuery>,
) -> ApiResult<Json<PMDashboardResponse>> {
    let days = parse_period(&query.period);
    let pool = &state.db;

    let summary = get_pm_summary(pool, days).await?;
    let bugs_metrics = get_bugs_metrics(pool, days).await?;
    let economy_metrics = get_economy_metrics(pool, days).await?;
    let component_health = get_component_health(pool, days).await?;
    let problematic_endpoints = get_problematic_endpoints(pool, days).await?;

    Ok(Json(PMDashboardResponse {
        summary,
        bugs_metrics,
        economy_metrics,
        component_health,
        problematic_endpoints,
        period: query.period,
        generated_at: Utc::now().to_rfc3339(),
    }))
}

/// Export PM dashboard as CSV.
#[utoipa::path(
    get,
    path = "/api/v1/pm-dashboard/export",
    params(
        ("period" = String, Query, description = "Period: 7d, 30d, 90d, 1y")
    ),
    responses(
        (status = 200, description = "CSV export", content_type = "text/csv"),
        (status = 500, description = "Internal server error")
    ),
    tag = "PM Dashboard"
)]
pub async fn export_pm_dashboard(
    State(state): State<AppState>,
    Query(query): Query<PMDashboardQuery>,
) -> ApiResult<String> {
    let days = parse_period(&query.period);
    let pool = &state.db;

    let summary = get_pm_summary(pool, days).await?;
    let bugs_metrics = get_bugs_metrics(pool, days).await?;
    let economy_metrics = get_economy_metrics(pool, days).await?;

    // Generate CSV
    let mut csv = String::new();
    csv.push_str("QA Metrics Report\n");
    csv.push_str(&format!("Period,{}\n", query.period));
    csv.push_str(&format!("Generated,{}\n\n", Utc::now().format("%Y-%m-%d %H:%M UTC")));
    
    csv.push_str("Summary\n");
    csv.push_str("Metric,Value\n");
    csv.push_str(&format!("Total Tickets Tested,{}\n", summary.total_tickets_tested));
    csv.push_str(&format!("Total Workflows Completed,{}\n", summary.total_workflows_completed));
    csv.push_str(&format!("Active QA Users,{}\n", summary.active_qa_users));
    csv.push_str(&format!("Avg Time Per Ticket (min),{:.1}\n\n", summary.avg_time_per_ticket_minutes));
    
    csv.push_str("Bugs Metrics\n");
    csv.push_str("Metric,Value\n");
    csv.push_str(&format!("Bugs Discovered,{}\n", bugs_metrics.bugs_discovered));
    csv.push_str(&format!("Bugs Prevented,{}\n", bugs_metrics.bugs_prevented));
    csv.push_str(&format!("Prevention Rate,{:.1}%\n\n", bugs_metrics.prevention_rate * 100.0));
    
    csv.push_str("Economy Metrics\n");
    csv.push_str("Metric,Value\n");
    csv.push_str(&format!("Hours Saved,{:.1}\n", economy_metrics.hours_saved));
    csv.push_str(&format!("Cost Saved,${:.2}\n", economy_metrics.cost_saved));
    csv.push_str(&format!("Bug Prevention Value,${:.2}\n", economy_metrics.bug_prevention_value));
    csv.push_str(&format!("Total Economy,${:.2}\n", economy_metrics.total_economy));

    Ok(csv)
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

async fn get_pm_summary(pool: &PgPool, days: i64) -> Result<PMSummary, ApiError> {
    let start = Utc::now() - Duration::days(days);

    // Get workflow stats
    let stats: Option<(i64, i64, Option<f64>)> = sqlx::query_as(
        r#"
        SELECT 
            COUNT(DISTINCT ticket_key) as tickets,
            COUNT(*) as workflows,
            AVG(EXTRACT(EPOCH FROM (completed_at - started_at)) / 60.0) as avg_minutes
        FROM workflow_instances
        WHERE status = 'completed'
          AND completed_at >= $1
        "#,
    )
    .bind(start)
    .fetch_optional(pool)
    .await
    .map_internal("Failed to fetch PM summary")?;

    let (tickets, workflows, avg_minutes) = stats.unwrap_or((0, 0, None));

    // Count active users (users with completed workflows in period)
    let (active_users,): (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(DISTINCT user_id)
        FROM workflow_instances
        WHERE status = 'completed'
          AND completed_at >= $1
          AND user_id IS NOT NULL
        "#,
    )
    .bind(start)
    .fetch_one(pool)
    .await
    .map_internal("Failed to count active users")?;

    Ok(PMSummary {
        total_tickets_tested: tickets,
        total_workflows_completed: workflows,
        active_qa_users: active_users.max(1), // At least 1 if there are workflows
        avg_time_per_ticket_minutes: avg_minutes.unwrap_or(0.0),
    })
}

async fn get_bugs_metrics(pool: &PgPool, days: i64) -> Result<BugsMetrics, ApiError> {
    let now = Utc::now();
    let period_start = now - Duration::days(days);
    let prev_period_start = period_start - Duration::days(days);

    // Bugs discovered: Count workflow step notes containing bug-related keywords
    let bug_keywords = vec!["bug", "defect", "issue", "error", "fail", "broken", "crash"];
    let keyword_pattern = bug_keywords.join("|");

    let (current_discovered,): (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM workflow_step_results wsr
        JOIN workflow_instances wi ON wsr.instance_id = wi.id
        WHERE wi.completed_at >= $1
          AND wi.completed_at < $2
          AND wsr.notes ~* $3
        "#,
    )
    .bind(period_start)
    .bind(now)
    .bind(&keyword_pattern)
    .fetch_one(pool)
    .await
    .map_internal("Failed to count discovered bugs")?;

    let (prev_discovered,): (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM workflow_step_results wsr
        JOIN workflow_instances wi ON wsr.instance_id = wi.id
        WHERE wi.completed_at >= $1
          AND wi.completed_at < $2
          AND wsr.notes ~* $3
        "#,
    )
    .bind(prev_period_start)
    .bind(period_start)
    .bind(&keyword_pattern)
    .fetch_one(pool)
    .await
    .map_internal("Failed to count previous discovered bugs")?;

    // Bugs prevented: Count pattern detection alerts (proactive detection)
    let (current_prevented,): (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM alerts
        WHERE created_at >= $1
          AND created_at < $2
          AND severity IN ('warning', 'critical')
        "#,
    )
    .bind(period_start)
    .bind(now)
    .fetch_one(pool)
    .await
    .map_internal("Failed to count prevented bugs")?;

    let (prev_prevented,): (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM alerts
        WHERE created_at >= $1
          AND created_at < $2
          AND severity IN ('warning', 'critical')
        "#,
    )
    .bind(prev_period_start)
    .bind(period_start)
    .fetch_one(pool)
    .await
    .map_internal("Failed to count previous prevented bugs")?;

    let total = current_discovered + current_prevented;
    let prevention_rate = if total > 0 {
        current_prevented as f64 / total as f64
    } else {
        0.0
    };

    let discovered_change = if prev_discovered > 0 {
        ((current_discovered - prev_discovered) as f64 / prev_discovered as f64 * 100.0).round()
    } else if current_discovered > 0 {
        100.0
    } else {
        0.0
    };

    let prevented_change = if prev_prevented > 0 {
        ((current_prevented - prev_prevented) as f64 / prev_prevented as f64 * 100.0).round()
    } else if current_prevented > 0 {
        100.0
    } else {
        0.0
    };

    Ok(BugsMetrics {
        bugs_discovered: current_discovered,
        bugs_prevented: current_prevented,
        prevention_rate,
        discovered_change,
        prevented_change,
    })
}

async fn get_economy_metrics(pool: &PgPool, days: i64) -> Result<EconomyMetrics, ApiError> {
    let start = Utc::now() - Duration::days(days);

    // Configurable rates (could be stored in config)
    let hourly_rate = 50.0; // $50/hour
    let avg_bug_fix_cost = 500.0; // $500 per bug fix

    // Calculate hours saved (when actual < estimated)
    let time_stats: Option<(Option<i64>, Option<i64>)> = sqlx::query_as(
        r#"
        SELECT 
            SUM(ts.total_seconds) as actual,
            SUM(te.estimated_seconds) as estimated
        FROM time_sessions ts
        JOIN workflow_instances wi ON ts.workflow_instance_id = wi.id
        LEFT JOIN time_estimates te ON wi.template_id = te.template_id AND ts.step_index = te.step_index
        WHERE ts.ended_at >= $1
        "#,
    )
    .bind(start)
    .fetch_optional(pool)
    .await
    .map_internal("Failed to fetch time stats")?;

    let hours_saved = match time_stats {
        Some((Some(actual), Some(estimated))) if estimated > actual => {
            (estimated - actual) as f64 / 3600.0
        }
        _ => 0.0,
    };

    // Get bugs prevented count
    let (bugs_prevented,): (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM alerts
        WHERE created_at >= $1
          AND severity IN ('warning', 'critical')
        "#,
    )
    .bind(start)
    .fetch_one(pool)
    .await
    .map_internal("Failed to count prevented bugs for economy")?;

    let cost_saved = hours_saved * hourly_rate;
    let bug_prevention_value = bugs_prevented as f64 * avg_bug_fix_cost;
    let total_economy = cost_saved + bug_prevention_value;

    Ok(EconomyMetrics {
        hours_saved,
        cost_saved,
        bug_prevention_value,
        total_economy,
        hourly_rate,
        avg_bug_fix_cost,
    })
}

async fn get_component_health(pool: &PgPool, days: i64) -> Result<Vec<ComponentHealth>, ApiError> {
    let now = Utc::now();
    let period_start = now - Duration::days(days);
    let prev_period_start = period_start - Duration::days(days);

    // Extract components from ticket keys (e.g., "PMP-1234" -> "PMP")
    // and from workflow notes mentioning components
    let current_stats: Vec<(String, i64, i64, Option<NaiveDate>)> = sqlx::query_as(
        r#"
        SELECT 
            COALESCE(SPLIT_PART(wi.ticket_key, '-', 1), 'Unknown') as component,
            COUNT(*) FILTER (WHERE wsr.notes ~* 'bug|error|fail|issue') as bug_count,
            COUNT(DISTINCT wi.ticket_key) as ticket_count,
            MAX(DATE(wi.completed_at)) as last_issue_date
        FROM workflow_instances wi
        LEFT JOIN workflow_step_results wsr ON wsr.instance_id = wi.id
        WHERE wi.completed_at >= $1
          AND wi.status = 'completed'
        GROUP BY SPLIT_PART(wi.ticket_key, '-', 1)
        ORDER BY bug_count DESC
        LIMIT 10
        "#,
    )
    .bind(period_start)
    .fetch_all(pool)
    .await
    .map_internal("Failed to fetch component health")?;

    // Get previous period stats for trend calculation
    let prev_stats: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT 
            COALESCE(SPLIT_PART(wi.ticket_key, '-', 1), 'Unknown') as component,
            COUNT(*) FILTER (WHERE wsr.notes ~* 'bug|error|fail|issue') as bug_count
        FROM workflow_instances wi
        LEFT JOIN workflow_step_results wsr ON wsr.instance_id = wi.id
        WHERE wi.completed_at >= $1
          AND wi.completed_at < $2
          AND wi.status = 'completed'
        GROUP BY SPLIT_PART(wi.ticket_key, '-', 1)
        "#,
    )
    .bind(prev_period_start)
    .bind(period_start)
    .fetch_all(pool)
    .await
    .map_internal("Failed to fetch previous component stats")?;

    let prev_map: HashMap<String, i64> = prev_stats.into_iter().collect();

    Ok(current_stats
        .into_iter()
        .map(|(component, bug_count, ticket_count, last_issue)| {
            let prev_bugs = prev_map.get(&component).copied().unwrap_or(0);
            let trend = if bug_count > prev_bugs {
                "degrading"
            } else if bug_count < prev_bugs {
                "improving"
            } else {
                "stable"
            };

            let status = if bug_count == 0 {
                "healthy"
            } else if bug_count <= 3 {
                "degraded"
            } else {
                "critical"
            };

            ComponentHealth {
                component,
                bug_count,
                ticket_count,
                status: status.to_string(),
                trend: trend.to_string(),
                last_issue_date: last_issue.map(|d| d.format("%Y-%m-%d").to_string()),
            }
        })
        .collect())
}

async fn get_problematic_endpoints(pool: &PgPool, days: i64) -> Result<Vec<ProblematicEndpoint>, ApiError> {
    let start = Utc::now() - Duration::days(days);

    // Extract endpoints from workflow notes (looking for API paths like /api/v1/...)
    let endpoint_stats: Vec<(String, i64, Vec<String>, Vec<String>)> = sqlx::query_as(
        r#"
        WITH endpoint_mentions AS (
            SELECT 
                regexp_matches(wsr.notes, '/api/[a-zA-Z0-9/_-]+', 'g') as endpoint_match,
                wsr.notes,
                wi.ticket_key
            FROM workflow_step_results wsr
            JOIN workflow_instances wi ON wsr.instance_id = wi.id
            WHERE wi.completed_at >= $1
              AND wsr.notes ~* '/api/'
        )
        SELECT 
            endpoint_match[1] as endpoint,
            COUNT(*) as issue_count,
            ARRAY_AGG(DISTINCT LEFT(notes, 100)) as sample_issues,
            ARRAY_AGG(DISTINCT ticket_key) as tickets
        FROM endpoint_mentions
        GROUP BY endpoint_match[1]
        ORDER BY issue_count DESC
        LIMIT 10
        "#,
    )
    .bind(start)
    .fetch_all(pool)
    .await
    .unwrap_or_default(); // Return empty if no matches

    Ok(endpoint_stats
        .into_iter()
        .map(|(endpoint, issue_count, issues, tickets)| {
            ProblematicEndpoint {
                endpoint,
                issue_count,
                common_issues: issues.into_iter().take(3).collect(),
                trend: if issue_count > 5 { "increasing" } else { "stable" }.to_string(),
                affected_tickets: tickets.into_iter().take(5).collect(),
            }
        })
        .collect())
}

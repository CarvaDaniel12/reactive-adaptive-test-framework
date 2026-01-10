//! Time aggregation repository functions.
//!
//! Story 6.7: Historical Time Data Storage
//! Provides functions for storing and querying aggregated time data.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

// ============================================================================
// Types
// ============================================================================

/// Daily aggregate for a user.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DailyAggregate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub aggregate_date: NaiveDate,
    pub tickets_completed: i32,
    pub total_time_seconds: i32,
    pub total_estimated_seconds: i32,
    pub avg_time_per_ticket_seconds: i32,
    pub efficiency_ratio: rust_decimal::Decimal,
    pub bug_tickets: i32,
    pub bug_time_seconds: i32,
    pub feature_tickets: i32,
    pub feature_time_seconds: i32,
    pub regression_tickets: i32,
    pub regression_time_seconds: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Step-level average for a template.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StepAverage {
    pub id: Uuid,
    pub template_id: Uuid,
    pub step_index: i32,
    pub sample_count: i32,
    pub total_seconds: i32,
    pub avg_seconds: i32,
    pub min_seconds: i32,
    pub max_seconds: i32,
    pub std_dev_seconds: rust_decimal::Decimal,
    pub last_sample_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User average by ticket type.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserAverage {
    pub id: Uuid,
    pub user_id: Uuid,
    pub ticket_type: String,
    pub sample_count: i32,
    pub total_seconds: i32,
    pub avg_seconds: i32,
    pub min_seconds: i32,
    pub max_seconds: i32,
    pub rolling_avg_seconds: i32,
    pub rolling_sample_count: i32,
    pub last_sample_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Time gap alert.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TimeGapAlert {
    pub id: Uuid,
    pub workflow_instance_id: Uuid,
    pub step_index: Option<i32>,
    pub user_id: Uuid,
    pub actual_seconds: i32,
    pub estimated_seconds: i32,
    pub gap_percentage: rust_decimal::Decimal,
    pub alert_type: String,
    pub dismissed: bool,
    pub dismissed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Summary of historical time data for a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalSummary {
    pub user_id: Uuid,
    pub period_days: i32,
    pub total_tickets: i64,
    pub total_time_seconds: i64,
    pub avg_time_per_ticket_seconds: f64,
    pub efficiency_ratio: f64,
    pub by_ticket_type: Vec<TicketTypeSummary>,
}

/// Summary by ticket type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketTypeSummary {
    pub ticket_type: String,
    pub count: i64,
    pub total_seconds: i64,
    pub avg_seconds: f64,
}

/// Trend data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendPoint {
    pub date: NaiveDate,
    pub tickets: i32,
    pub hours: f64,
    pub efficiency: f64,
}

// ============================================================================
// Update Functions (called when workflows complete)
// ============================================================================

/// Record workflow completion and update all aggregates.
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `workflow_instance_id` - The workflow instance that completed
/// * `user_id` - The user who completed the workflow
/// * `template_id` - The workflow template used
/// * `ticket_type` - Type of ticket: "bug", "feature", or "regression"
/// * `actual_seconds` - Total actual time spent
/// * `estimated_seconds` - Total estimated time
/// * `step_times` - Vector of (`step_index`, `actual_seconds`) for each step
pub async fn record_workflow_completion(
    pool: &PgPool,
    workflow_instance_id: Uuid,
    user_id: Uuid,
    template_id: Uuid,
    ticket_type: &str,
    actual_seconds: i32,
    estimated_seconds: i32,
    step_times: &[(i32, i32)], // (step_index, actual_seconds)
) -> Result<(), sqlx::Error> {
    let today = Utc::now().date_naive();

    // Update daily aggregate using PostgreSQL function
    sqlx::query("SELECT update_daily_aggregate($1, $2, $3, $4, $5)")
        .bind(user_id)
        .bind(today)
        .bind(ticket_type)
        .bind(actual_seconds)
        .bind(estimated_seconds)
        .execute(pool)
        .await?;

    // Update user average using PostgreSQL function
    sqlx::query("SELECT update_user_average($1, $2, $3)")
        .bind(user_id)
        .bind(ticket_type)
        .bind(actual_seconds)
        .execute(pool)
        .await?;

    // Update step averages using PostgreSQL function
    for (step_index, step_seconds) in step_times {
        sqlx::query("SELECT update_step_average($1, $2, $3)")
            .bind(template_id)
            .bind(*step_index)
            .bind(*step_seconds)
            .execute(pool)
            .await?;
    }

    // Check for gap alert (>20% over estimate per FR-TRK-06)
    if estimated_seconds > 0 {
        let gap_pct = (f64::from(actual_seconds) / f64::from(estimated_seconds)) * 100.0;
        if gap_pct > 120.0 {
            create_gap_alert(
                pool,
                workflow_instance_id,
                None,
                user_id,
                actual_seconds,
                estimated_seconds,
                "workflow_excess",
            )
            .await?;
        }
    }

    Ok(())
}

/// Create a time gap alert.
pub async fn create_gap_alert(
    pool: &PgPool,
    workflow_instance_id: Uuid,
    step_index: Option<i32>,
    user_id: Uuid,
    actual_seconds: i32,
    estimated_seconds: i32,
    alert_type: &str,
) -> Result<TimeGapAlert, sqlx::Error> {
    let gap_pct = if estimated_seconds > 0 {
        (f64::from(actual_seconds) / f64::from(estimated_seconds)) * 100.0
    } else {
        100.0
    };

    sqlx::query_as::<_, TimeGapAlert>(
        r"
        INSERT INTO time_gap_alerts (
            workflow_instance_id, step_index, user_id,
            actual_seconds, estimated_seconds, gap_percentage, alert_type
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        ",
    )
    .bind(workflow_instance_id)
    .bind(step_index)
    .bind(user_id)
    .bind(actual_seconds)
    .bind(estimated_seconds)
    .bind(rust_decimal::Decimal::from_f64_retain(gap_pct).unwrap_or_default())
    .bind(alert_type)
    .fetch_one(pool)
    .await
}

// ============================================================================
// Query Functions
// ============================================================================

/// Get historical summary for a user over a period.
pub async fn get_historical_summary(
    pool: &PgPool,
    user_id: Uuid,
    days: i32,
) -> Result<HistoricalSummary, sqlx::Error> {
    let start_date = Utc::now().date_naive() - chrono::Duration::days(i64::from(days));

    // Get totals
    let totals: (i64, i64, Option<f64>, Option<f64>) = sqlx::query_as(
        r"
        SELECT 
            COALESCE(SUM(tickets_completed), 0) as total_tickets,
            COALESCE(SUM(total_time_seconds), 0) as total_time,
            AVG(avg_time_per_ticket_seconds)::FLOAT8 as avg_time,
            AVG(efficiency_ratio)::FLOAT8 as avg_efficiency
        FROM time_daily_aggregates
        WHERE user_id = $1 AND aggregate_date >= $2
        ",
    )
    .bind(user_id)
    .bind(start_date)
    .fetch_one(pool)
    .await?;

    // Get by ticket type
    let by_type: Vec<(String, i64, i64)> = sqlx::query_as(
        r"
        SELECT 
            'bug' as ticket_type,
            COALESCE(SUM(bug_tickets), 0) as count,
            COALESCE(SUM(bug_time_seconds), 0) as total_seconds
        FROM time_daily_aggregates
        WHERE user_id = $1 AND aggregate_date >= $2
        UNION ALL
        SELECT 
            'feature',
            COALESCE(SUM(feature_tickets), 0),
            COALESCE(SUM(feature_time_seconds), 0)
        FROM time_daily_aggregates
        WHERE user_id = $1 AND aggregate_date >= $2
        UNION ALL
        SELECT 
            'regression',
            COALESCE(SUM(regression_tickets), 0),
            COALESCE(SUM(regression_time_seconds), 0)
        FROM time_daily_aggregates
        WHERE user_id = $1 AND aggregate_date >= $2
        ",
    )
    .bind(user_id)
    .bind(start_date)
    .fetch_all(pool)
    .await?;

    let by_ticket_type: Vec<TicketTypeSummary> = by_type
        .into_iter()
        .filter(|(_, count, _)| *count > 0)
        .map(|(ticket_type, count, total_seconds)| TicketTypeSummary {
            ticket_type,
            count,
            total_seconds,
            avg_seconds: if count > 0 {
                total_seconds as f64 / count as f64
            } else {
                0.0
            },
        })
        .collect();

    Ok(HistoricalSummary {
        user_id,
        period_days: days,
        total_tickets: totals.0,
        total_time_seconds: totals.1,
        avg_time_per_ticket_seconds: totals.2.unwrap_or(0.0),
        efficiency_ratio: totals.3.unwrap_or(1.0),
        by_ticket_type,
    })
}

/// Get trend data for charts.
pub async fn get_trend_data(
    pool: &PgPool,
    user_id: Uuid,
    days: i32,
) -> Result<Vec<TrendPoint>, sqlx::Error> {
    let start_date = Utc::now().date_naive() - chrono::Duration::days(i64::from(days));

    let rows: Vec<(NaiveDate, i32, i32, rust_decimal::Decimal)> = sqlx::query_as(
        r"
        SELECT 
            aggregate_date,
            tickets_completed,
            total_time_seconds,
            efficiency_ratio
        FROM time_daily_aggregates
        WHERE user_id = $1 AND aggregate_date >= $2
        ORDER BY aggregate_date
        ",
    )
    .bind(user_id)
    .bind(start_date)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(date, tickets, seconds, efficiency)| TrendPoint {
            date,
            tickets,
            hours: f64::from(seconds) / 3600.0,
            // Use ToPrimitive trait for proper Decimal to f64 conversion
            efficiency: efficiency.to_f64().unwrap_or(1.0),
        })
        .collect())
}

/// Get step averages for a template.
pub async fn get_step_averages(
    pool: &PgPool,
    template_id: Uuid,
) -> Result<Vec<StepAverage>, sqlx::Error> {
    sqlx::query_as::<_, StepAverage>(
        r"
        SELECT * FROM time_step_averages
        WHERE template_id = $1
        ORDER BY step_index
        ",
    )
    .bind(template_id)
    .fetch_all(pool)
    .await
}

/// Get user averages by ticket type.
pub async fn get_user_averages(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<UserAverage>, sqlx::Error> {
    sqlx::query_as::<_, UserAverage>(
        r"
        SELECT * FROM time_user_averages
        WHERE user_id = $1
        ",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

/// Get undismissed gap alerts for a user.
pub async fn get_undismissed_alerts(
    pool: &PgPool,
    user_id: Uuid,
    limit: i32,
) -> Result<Vec<TimeGapAlert>, sqlx::Error> {
    sqlx::query_as::<_, TimeGapAlert>(
        r"
        SELECT * FROM time_gap_alerts
        WHERE user_id = $1 AND dismissed = false
        ORDER BY created_at DESC
        LIMIT $2
        ",
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// Dismiss a gap alert.
pub async fn dismiss_alert(pool: &PgPool, alert_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r"
        UPDATE time_gap_alerts
        SET dismissed = true, dismissed_at = NOW()
        WHERE id = $1
        ",
    )
    .bind(alert_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Get daily aggregates for a date range.
pub async fn get_daily_aggregates(
    pool: &PgPool,
    user_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<DailyAggregate>, sqlx::Error> {
    sqlx::query_as::<_, DailyAggregate>(
        r"
        SELECT * FROM time_daily_aggregates
        WHERE user_id = $1 AND aggregate_date >= $2 AND aggregate_date <= $3
        ORDER BY aggregate_date
        ",
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
}

/// Run cleanup of old data.
pub async fn cleanup_old_data(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT cleanup_old_aggregates()")
        .execute(pool)
        .await?;
    Ok(())
}

// ============================================================================
// Dashboard Helper Functions
// ============================================================================

/// Calculate real efficiency from aggregates.
pub async fn calculate_efficiency(
    pool: &PgPool,
    user_id: Uuid,
    days: i32,
) -> Result<f64, sqlx::Error> {
    let start_date = Utc::now().date_naive() - chrono::Duration::days(i64::from(days));

    let result: (Option<i64>, Option<i64>) = sqlx::query_as(
        r"
        SELECT 
            SUM(total_time_seconds) as actual,
            SUM(total_estimated_seconds) as estimated
        FROM time_daily_aggregates
        WHERE user_id = $1 AND aggregate_date >= $2
        ",
    )
    .bind(user_id)
    .bind(start_date)
    .fetch_one(pool)
    .await?;

    let (actual, estimated) = result;
    match (actual, estimated) {
        (Some(a), Some(e)) if e > 0 => Ok(e as f64 / a as f64), // Inverted: estimated/actual (higher is better)
        _ => Ok(1.0),                                           // Default to 100% efficiency
    }
}

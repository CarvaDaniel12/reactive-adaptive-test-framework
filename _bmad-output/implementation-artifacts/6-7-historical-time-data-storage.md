# Story 6.7: Historical Time Data Storage

Status: ready-for-dev

## Story

As a developer,
I want historical time data aggregated,
So that dashboards can show trends.

## Acceptance Criteria

1. **Given** time sessions are recorded
   **When** a workflow completes
   **Then** aggregated data is stored

2. **Given** aggregation runs
   **When** data is computed
   **Then** total time by ticket type is calculated

3. **Given** aggregation runs
   **When** data is computed
   **Then** average time per step is calculated

4. **Given** aggregation runs
   **When** data is computed
   **Then** user's historical averages are calculated

5. **Given** aggregation runs
   **When** data is computed
   **Then** trends over time are computed

6. **Given** aggregated data exists
   **When** queried
   **Then** data is queryable by date range

7. **Given** aggregated data exists
   **When** dashboards render
   **Then** data is used for dashboard metrics (Epic 8)

8. **Given** data retention policy
   **When** old data is checked
   **Then** old sessions are retained per NFR-REL-04 (30 days minimum)

## Tasks

- [ ] Task 1: Create time_aggregations table
- [ ] Task 2: Create aggregation service
- [ ] Task 3: Trigger aggregation on workflow completion
- [ ] Task 4: Create date range query methods
- [ ] Task 5: Implement data retention cleanup job
- [ ] Task 6: Create aggregation summary API endpoint

## Dev Notes

### Database Schema

```sql
-- migrations/20260103_create_time_aggregations.sql

-- Daily aggregations per user
CREATE TABLE time_aggregations_daily (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id VARCHAR(255),
    date DATE NOT NULL,
    ticket_type VARCHAR(100),
    workflows_completed INTEGER DEFAULT 0,
    total_active_seconds INTEGER DEFAULT 0,
    total_estimated_seconds INTEGER DEFAULT 0,
    total_paused_seconds INTEGER DEFAULT 0,
    avg_efficiency_ratio DECIMAL(5, 2),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(user_id, date, ticket_type)
);

-- Step-level aggregations for trend analysis
CREATE TABLE time_aggregations_step (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id VARCHAR(255),
    template_id UUID REFERENCES workflow_templates(id),
    step_index INTEGER NOT NULL,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    sample_count INTEGER DEFAULT 0,
    avg_seconds INTEGER,
    min_seconds INTEGER,
    max_seconds INTEGER,
    std_dev_seconds DECIMAL(10, 2),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(user_id, template_id, step_index, period_start)
);

-- Indexes
CREATE INDEX idx_time_agg_daily_user_date 
    ON time_aggregations_daily(user_id, date DESC);
CREATE INDEX idx_time_agg_daily_date 
    ON time_aggregations_daily(date DESC);
CREATE INDEX idx_time_agg_step_user_template 
    ON time_aggregations_step(user_id, template_id);
```

### Aggregation Service

```rust
// crates/qa-pms-workflow/src/time/aggregation.rs
use chrono::{NaiveDate, Utc};

pub struct TimeAggregationService {
    pool: PgPool,
}

impl TimeAggregationService {
    /// Called when workflow completes
    pub async fn aggregate_workflow_completion(
        &self,
        instance: &WorkflowInstance,
        sessions: &[TimeSession],
        template: &WorkflowTemplate,
    ) -> Result<()> {
        let date = instance.completed_at
            .map(|t| t.date_naive())
            .unwrap_or_else(|| Utc::now().date_naive());
        
        let total_active: i32 = sessions.iter()
            .filter_map(|s| s.total_seconds)
            .sum();
        
        let total_paused: i32 = sessions.iter()
            .map(|s| s.total_paused_seconds)
            .sum();
        
        let total_estimated: i32 = template.steps.iter()
            .map(|s| (s.estimated_minutes * 60) as i32)
            .sum();
        
        let efficiency = if total_estimated > 0 {
            total_active as f64 / total_estimated as f64
        } else {
            1.0
        };

        // Upsert daily aggregation
        sqlx::query(
            r#"
            INSERT INTO time_aggregations_daily 
                (user_id, date, ticket_type, workflows_completed, 
                 total_active_seconds, total_estimated_seconds, 
                 total_paused_seconds, avg_efficiency_ratio)
            VALUES ($1, $2, $3, 1, $4, $5, $6, $7)
            ON CONFLICT (user_id, date, ticket_type) DO UPDATE
            SET workflows_completed = time_aggregations_daily.workflows_completed + 1,
                total_active_seconds = time_aggregations_daily.total_active_seconds + $4,
                total_estimated_seconds = time_aggregations_daily.total_estimated_seconds + $5,
                total_paused_seconds = time_aggregations_daily.total_paused_seconds + $6,
                avg_efficiency_ratio = (
                    (time_aggregations_daily.total_active_seconds + $4)::DECIMAL /
                    NULLIF(time_aggregations_daily.total_estimated_seconds + $5, 0)
                ),
                updated_at = NOW()
            "#,
        )
        .bind(&instance.user_id)
        .bind(date)
        .bind(&template.ticket_type)
        .bind(total_active)
        .bind(total_estimated)
        .bind(total_paused)
        .bind(efficiency)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get aggregations for date range
    pub async fn get_daily_aggregations(
        &self,
        user_id: Option<&str>,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<DailyAggregation>> {
        let query = match user_id {
            Some(uid) => {
                sqlx::query_as::<_, DailyAggregation>(
                    r#"
                    SELECT * FROM time_aggregations_daily
                    WHERE user_id = $1 AND date BETWEEN $2 AND $3
                    ORDER BY date DESC
                    "#,
                )
                .bind(uid)
                .bind(start_date)
                .bind(end_date)
            }
            None => {
                sqlx::query_as::<_, DailyAggregation>(
                    r#"
                    SELECT * FROM time_aggregations_daily
                    WHERE date BETWEEN $1 AND $2
                    ORDER BY date DESC
                    "#,
                )
                .bind(start_date)
                .bind(end_date)
            }
        };
        
        query.fetch_all(&self.pool).await.map_err(Into::into)
    }

    /// Get trend data for dashboard
    pub async fn get_trend_data(
        &self,
        user_id: &str,
        days: i32,
    ) -> Result<Vec<TrendDataPoint>> {
        sqlx::query_as::<_, TrendDataPoint>(
            r#"
            SELECT 
                date,
                SUM(workflows_completed) as workflows,
                SUM(total_active_seconds) / 3600.0 as hours,
                AVG(avg_efficiency_ratio) as efficiency
            FROM time_aggregations_daily
            WHERE user_id = $1 
              AND date >= CURRENT_DATE - $2::INTEGER
            GROUP BY date
            ORDER BY date
            "#,
        )
        .bind(user_id)
        .bind(days)
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TrendDataPoint {
    pub date: NaiveDate,
    pub workflows: i64,
    pub hours: f64,
    pub efficiency: Option<f64>,
}
```

### Data Retention Job

```rust
// crates/qa-pms-api/src/jobs/cleanup.rs
pub async fn cleanup_old_time_data(pool: &PgPool) -> Result<u64> {
    // Keep raw sessions for 30 days per NFR-REL-04
    let deleted = sqlx::query(
        r#"
        DELETE FROM time_sessions
        WHERE ended_at < NOW() - INTERVAL '30 days'
          AND is_active = false
        "#,
    )
    .execute(pool)
    .await?
    .rows_affected();

    tracing::info!(deleted = deleted, "Cleaned up old time sessions");
    
    Ok(deleted)
}
```

### API Endpoint

```rust
// GET /api/v1/time/aggregations
#[utoipa::path(
    get,
    path = "/api/v1/time/aggregations",
    params(
        ("start_date" = NaiveDate, Query, description = "Start date"),
        ("end_date" = NaiveDate, Query, description = "End date"),
        ("user_id" = Option<String>, Query, description = "Filter by user"),
    ),
    responses(
        (status = 200, description = "Time aggregations", body = Vec<DailyAggregation>),
    ),
    tag = "time"
)]
pub async fn get_aggregations(/* ... */) -> Result<Json<Vec<DailyAggregation>>, ApiError> {
    // Implementation
}
```

### References

- [Source: epics.md#Story 6.7]
- [NFR: NFR-REL-04 - 30 days data retention]
- [Dependency: Epic 8 - QA Individual Dashboard]

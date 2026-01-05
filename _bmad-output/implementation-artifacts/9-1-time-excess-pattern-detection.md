# Story 9.1: Time Excess Pattern Detection

Status: done

## Story

As a system,
I want to detect when users consistently exceed time estimates,
So that I can alert about potential issues.

## Acceptance Criteria

1. **Given** workflow time data is being collected
   **When** analysis runs (after each workflow completion)
   **Then** system detects steps taking >50% longer than estimated

2. **Given** analysis runs
   **When** patterns detected
   **Then** system detects tickets taking >50% longer than similar tickets

3. **Given** analysis runs
   **When** trends analyzed
   **Then** system detects trend of increasing time over last 5 tickets

4. **Given** detection logic
   **When** workflow completes
   **Then** detection runs in background

5. **Given** patterns detected
   **When** stored
   **Then** detected patterns are stored in database

6. **Given** pattern is stored
   **When** details recorded
   **Then** pattern includes: affected tickets, average excess %, suggested cause

## Tasks

- [ ] Task 1: Create patterns database table
- [ ] Task 2: Create PatternDetectionService
- [ ] Task 3: Implement step time excess detection
- [ ] Task 4: Implement ticket time excess detection
- [ ] Task 5: Implement trend detection algorithm
- [ ] Task 6: Trigger detection on workflow completion
- [ ] Task 7: Store patterns with metadata

## Dev Notes

### Database Schema

```sql
-- migrations/20260103_create_patterns.sql
CREATE TYPE pattern_type AS ENUM (
    'step_time_excess',
    'ticket_time_excess',
    'increasing_trend',
    'consecutive_issues'
);

CREATE TYPE pattern_severity AS ENUM ('info', 'warning', 'critical');

CREATE TABLE detected_patterns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pattern_type pattern_type NOT NULL,
    severity pattern_severity NOT NULL DEFAULT 'info',
    user_id VARCHAR(255),
    affected_ticket_ids TEXT[], -- Array of ticket IDs
    details JSONB NOT NULL,
    confidence_score DECIMAL(5, 2),
    resolved BOOLEAN DEFAULT false,
    resolved_at TIMESTAMPTZ,
    resolution_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_patterns_user ON detected_patterns(user_id);
CREATE INDEX idx_patterns_type ON detected_patterns(pattern_type);
CREATE INDEX idx_patterns_unresolved ON detected_patterns(resolved) WHERE resolved = false;
```

### Pattern Detection Service

```rust
// crates/qa-pms-analytics/src/pattern_detection.rs
use chrono::{DateTime, Utc};

pub struct PatternDetectionService {
    pool: PgPool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedPattern {
    pub id: Uuid,
    pub pattern_type: PatternType,
    pub severity: Severity,
    pub affected_ticket_ids: Vec<String>,
    pub details: PatternDetails,
    pub confidence_score: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PatternDetails {
    pub description: String,
    pub average_excess_percent: f64,
    pub suggested_cause: Option<String>,
    pub affected_steps: Option<Vec<String>>,
    pub recommendations: Vec<String>,
}

impl PatternDetectionService {
    /// Called after each workflow completion
    pub async fn analyze_workflow_completion(
        &self,
        instance: &WorkflowInstance,
        time_sessions: &[TimeSession],
        template: &WorkflowTemplate,
    ) -> Result<Vec<DetectedPattern>> {
        let mut patterns = Vec::new();

        // Check step time excess
        if let Some(pattern) = self.detect_step_time_excess(instance, time_sessions, template).await? {
            patterns.push(self.save_pattern(pattern).await?);
        }

        // Check ticket time excess vs similar tickets
        if let Some(pattern) = self.detect_ticket_time_excess(instance, time_sessions).await? {
            patterns.push(self.save_pattern(pattern).await?);
        }

        // Check for increasing trend
        if let Some(pattern) = self.detect_increasing_trend(instance.user_id.as_deref()).await? {
            patterns.push(self.save_pattern(pattern).await?);
        }

        Ok(patterns)
    }

    async fn detect_step_time_excess(
        &self,
        instance: &WorkflowInstance,
        sessions: &[TimeSession],
        template: &WorkflowTemplate,
    ) -> Result<Option<NewPattern>> {
        let mut excess_steps = Vec::new();

        for (i, step) in template.steps.iter().enumerate() {
            let session = sessions.iter().find(|s| s.step_index == i as i32);
            if let Some(session) = session {
                let actual = session.total_seconds.unwrap_or(0);
                let estimated = (step.estimated_minutes * 60) as i32;
                
                if estimated > 0 {
                    let ratio = actual as f64 / estimated as f64;
                    if ratio > 1.5 { // >50% over
                        excess_steps.push((step.name.clone(), ratio));
                    }
                }
            }
        }

        if excess_steps.is_empty() {
            return Ok(None);
        }

        let avg_excess = excess_steps.iter().map(|(_, r)| r).sum::<f64>() / excess_steps.len() as f64;
        let severity = if avg_excess > 2.0 { Severity::Critical }
            else if avg_excess > 1.5 { Severity::Warning }
            else { Severity::Info };

        Ok(Some(NewPattern {
            pattern_type: PatternType::StepTimeExcess,
            severity,
            user_id: instance.user_id.clone(),
            affected_ticket_ids: vec![instance.ticket_key.clone()],
            details: PatternDetails {
                description: format!(
                    "{} step(s) took significantly longer than estimated",
                    excess_steps.len()
                ),
                average_excess_percent: (avg_excess - 1.0) * 100.0,
                suggested_cause: self.suggest_cause_for_excess(&excess_steps),
                affected_steps: Some(excess_steps.iter().map(|(n, _)| n.clone()).collect()),
                recommendations: vec![
                    "Review step estimates for accuracy".into(),
                    "Consider breaking down complex steps".into(),
                ],
            },
            confidence_score: 0.8,
        }))
    }

    async fn detect_ticket_time_excess(
        &self,
        instance: &WorkflowInstance,
        sessions: &[TimeSession],
    ) -> Result<Option<NewPattern>> {
        let total_actual: i32 = sessions.iter()
            .filter_map(|s| s.total_seconds)
            .sum();

        // Get average time for similar tickets (same template)
        let avg_similar: Option<i64> = sqlx::query_scalar(
            r#"
            SELECT AVG(total_seconds)::BIGINT
            FROM (
                SELECT SUM(ts.total_seconds) as total_seconds
                FROM workflow_instances wi
                JOIN time_sessions ts ON ts.workflow_instance_id = wi.id
                WHERE wi.template_id = $1
                  AND wi.status = 'completed'
                  AND wi.id != $2
                  AND wi.completed_at > NOW() - INTERVAL '30 days'
                GROUP BY wi.id
            ) t
            "#,
        )
        .bind(instance.template_id)
        .bind(instance.id)
        .fetch_one(&self.pool)
        .await?;

        let Some(avg) = avg_similar else { return Ok(None) };
        if avg == 0 { return Ok(None); }

        let ratio = total_actual as f64 / avg as f64;
        if ratio <= 1.5 { return Ok(None); } // Not significant

        let severity = if ratio > 2.0 { Severity::Warning } else { Severity::Info };

        Ok(Some(NewPattern {
            pattern_type: PatternType::TicketTimeExcess,
            severity,
            user_id: instance.user_id.clone(),
            affected_ticket_ids: vec![instance.ticket_key.clone()],
            details: PatternDetails {
                description: format!(
                    "Ticket took {:.0}% longer than similar tickets",
                    (ratio - 1.0) * 100.0
                ),
                average_excess_percent: (ratio - 1.0) * 100.0,
                suggested_cause: Some("May indicate increased complexity or blockers".into()),
                affected_steps: None,
                recommendations: vec![
                    "Review if ticket scope was accurately estimated".into(),
                    "Check for recurring blockers".into(),
                ],
            },
            confidence_score: 0.7,
        }))
    }

    async fn detect_increasing_trend(
        &self,
        user_id: Option<&str>,
    ) -> Result<Option<NewPattern>> {
        let Some(user_id) = user_id else { return Ok(None) };

        // Get last 5 completed workflows
        let recent: Vec<(String, i64)> = sqlx::query_as(
            r#"
            SELECT wi.ticket_key, SUM(ts.total_seconds)::BIGINT as total
            FROM workflow_instances wi
            JOIN time_sessions ts ON ts.workflow_instance_id = wi.id
            WHERE wi.user_id = $1
              AND wi.status = 'completed'
            GROUP BY wi.id, wi.ticket_key, wi.completed_at
            ORDER BY wi.completed_at DESC
            LIMIT 5
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        if recent.len() < 5 { return Ok(None); }

        // Check for increasing trend (each newer ticket takes longer)
        let times: Vec<i64> = recent.iter().map(|(_, t)| *t).collect();
        let is_increasing = times.windows(2).all(|w| w[0] >= w[1]);

        if !is_increasing { return Ok(None); }

        let increase_percent = if times.last().unwrap() > &0 {
            ((times.first().unwrap() - times.last().unwrap()) as f64 
             / *times.last().unwrap() as f64) * 100.0
        } else { 0.0 };

        if increase_percent < 20.0 { return Ok(None); }

        Ok(Some(NewPattern {
            pattern_type: PatternType::IncreasingTrend,
            severity: Severity::Warning,
            user_id: Some(user_id.to_string()),
            affected_ticket_ids: recent.iter().map(|(k, _)| k.clone()).collect(),
            details: PatternDetails {
                description: format!(
                    "Time spent has increased {:.0}% over last 5 tickets",
                    increase_percent
                ),
                average_excess_percent: increase_percent,
                suggested_cause: Some("Possible burnout, increased complexity, or blockers".into()),
                affected_steps: None,
                recommendations: vec![
                    "Review recent ticket complexity".into(),
                    "Check for patterns in types of delays".into(),
                    "Consider workload adjustment".into(),
                ],
            },
            confidence_score: 0.75,
        }))
    }

    fn suggest_cause_for_excess(&self, excess_steps: &[(String, f64)]) -> Option<String> {
        // Simple heuristic based on step names
        for (name, _) in excess_steps {
            let lower = name.to_lowercase();
            if lower.contains("investigate") || lower.contains("debug") {
                return Some("Investigation/debugging often reveals unexpected complexity".into());
            }
            if lower.contains("regression") {
                return Some("Regression testing scope may need adjustment".into());
            }
        }
        None
    }
}
```

### Trigger on Workflow Completion

```rust
// In complete_workflow or complete_step (final step)
let pattern_service = PatternDetectionService::new(state.db_pool.clone());
tokio::spawn(async move {
    if let Err(e) = pattern_service
        .analyze_workflow_completion(&instance, &sessions, &template)
        .await
    {
        tracing::error!("Pattern detection failed: {}", e);
    }
});
```

### References

- [Source: epics.md#Story 9.1]

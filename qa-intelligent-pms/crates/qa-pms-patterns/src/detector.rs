//! Pattern detection logic.
//!
//! Analyzes workflow data to detect patterns:
//! - Time excess (>50% over estimate)
//! - Consecutive problems (3+ tickets with same issue)
//! - Spikes (sudden increase in tickets)

use sqlx::PgPool;
use tracing::info;

use crate::types::*;
use crate::repository::PatternRepository;

/// Time excess threshold (50% over estimate).
const TIME_EXCESS_THRESHOLD: f64 = 0.5;

/// Minimum consecutive tickets for problem detection.
const CONSECUTIVE_THRESHOLD: usize = 3;

/// Pattern detector service.
pub struct PatternDetector {
    pool: PgPool,
    repo: PatternRepository,
}

impl PatternDetector {
    /// Create a new pattern detector.
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: PatternRepository::new(pool.clone()),
            pool,
        }
    }

    /// Run pattern detection after a workflow completion.
    ///
    /// This should be called in the background after each workflow completes.
    pub async fn analyze_workflow(&self, workflow_id: uuid::Uuid) -> anyhow::Result<Vec<DetectedPattern>> {
        let mut detected = Vec::new();

        // Get workflow data
        let workflow_data = self.get_workflow_data(workflow_id).await?;
        
        // 1. Check for time excess
        if let Some(pattern) = self.detect_time_excess(&workflow_data).await? {
            detected.push(pattern);
        }

        // 2. Check for consecutive problems (last 5 tickets)
        if let Some(pattern) = self.detect_consecutive_problems(&workflow_data).await? {
            detected.push(pattern);
        }

        // 3. Check for spikes (compared to baseline)
        if let Some(pattern) = self.detect_spike(&workflow_data).await? {
            detected.push(pattern);
        }

        info!(
            workflow_id = %workflow_id,
            patterns_detected = detected.len(),
            "Pattern analysis complete"
        );

        Ok(detected)
    }

    async fn get_workflow_data(&self, workflow_id: uuid::Uuid) -> anyhow::Result<WorkflowAnalysisData> {
        let row: (uuid::Uuid, String, String, i64, Option<i64>, Option<String>, chrono::DateTime<chrono::Utc>) = 
            sqlx::query_as(
                r#"
                SELECT 
                    wi.id,
                    wi.ticket_key,
                    wt.name as template_name,
                    EXTRACT(EPOCH FROM (wi.completed_at - wi.started_at))::BIGINT as actual_duration,
                    (SELECT SUM((step->>'estimatedMinutes')::INT * 60) 
                     FROM jsonb_array_elements(wt.steps_json) as step) as estimated_duration,
                    wi.ticket_key as component,
                    wi.completed_at
                FROM workflow_instances wi
                JOIN workflow_templates wt ON wi.template_id = wt.id
                WHERE wi.id = $1
                "#,
            )
            .bind(workflow_id)
            .fetch_one(&self.pool)
            .await?;

        // Get step notes
        let notes: Vec<(Option<String>,)> = sqlx::query_as(
            r#"
            SELECT notes FROM workflow_step_results
            WHERE instance_id = $1 AND notes IS NOT NULL
            "#,
        )
        .bind(workflow_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(WorkflowAnalysisData {
            workflow_id: row.0,
            ticket_key: row.1,
            template_name: row.2,
            actual_duration_seconds: row.3,
            estimated_duration_seconds: row.4,
            step_notes: notes.into_iter().filter_map(|(n,)| n).collect(),
            component: row.5,
            completed_at: row.6,
        })
    }

    /// Detect time excess pattern (>50% over estimate).
    async fn detect_time_excess(&self, data: &WorkflowAnalysisData) -> anyhow::Result<Option<DetectedPattern>> {
        let Some(estimated) = data.estimated_duration_seconds else {
            return Ok(None);
        };

        if estimated <= 0 {
            return Ok(None);
        }

        let excess_percent = (data.actual_duration_seconds as f64 - estimated as f64) / estimated as f64;

        if excess_percent <= TIME_EXCESS_THRESHOLD {
            return Ok(None);
        }

        let severity = if excess_percent > 1.0 {
            Severity::Critical
        } else if excess_percent > 0.75 {
            Severity::Warning
        } else {
            Severity::Info
        };

        let pattern = NewPattern {
            pattern_type: PatternType::TimeExcess,
            severity,
            title: format!("Time excess on {}", data.ticket_key),
            description: Some(format!(
                "Workflow took {:.0}% longer than estimated ({} actual vs {} estimated)",
                excess_percent * 100.0,
                format_duration(data.actual_duration_seconds),
                format_duration(estimated)
            )),
            affected_tickets: vec![data.ticket_key.clone()],
            common_factor: Some(data.template_name.clone()),
            average_excess_percent: Some(excess_percent * 100.0),
            confidence_score: 1.0, // Direct measurement
            suggested_actions: vec![
                "Review step estimates for this workflow type".to_string(),
                "Check if ticket complexity was underestimated".to_string(),
            ],
            metadata: serde_json::json!({
                "actual_seconds": data.actual_duration_seconds,
                "estimated_seconds": estimated,
                "template": data.template_name
            }),
        };

        let saved = self.repo.create_pattern(pattern).await?;
        Ok(Some(saved))
    }

    /// Detect consecutive problems (3+ tickets with same issue).
    async fn detect_consecutive_problems(&self, _data: &WorkflowAnalysisData) -> anyhow::Result<Option<DetectedPattern>> {
        // Get last 5 completed workflows
        let recent: Vec<(String, Option<String>)> = sqlx::query_as(
            r#"
            SELECT 
                wi.ticket_key,
                (SELECT string_agg(notes, ' ') FROM workflow_step_results WHERE instance_id = wi.id) as all_notes
            FROM workflow_instances wi
            WHERE wi.status = 'completed'
            ORDER BY wi.completed_at DESC
            LIMIT 5
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        if recent.len() < CONSECUTIVE_THRESHOLD {
            return Ok(None);
        }

        // Extract keywords from notes and find common factors
        let keywords = self.extract_common_keywords(&recent);
        
        if keywords.is_empty() {
            return Ok(None);
        }

        // Find most common keyword
        let (common_keyword, count) = keywords
            .iter()
            .max_by_key(|(_, c)| *c)
            .map(|(k, c)| (k.clone(), *c))
            .unwrap_or_default();

        if count < CONSECUTIVE_THRESHOLD {
            return Ok(None);
        }

        let affected: Vec<String> = recent.iter().map(|(k, _)| k.clone()).collect();
        let confidence = count as f64 / recent.len() as f64;

        let severity = if count >= 5 {
            Severity::Critical
        } else if count >= 4 {
            Severity::Warning
        } else {
            Severity::Info
        };

        let pattern = NewPattern {
            pattern_type: PatternType::ConsecutiveProblem,
            severity,
            title: format!("Recurring issue: {}", common_keyword),
            description: Some(format!(
                "{} of the last {} tickets mention '{}'",
                count, recent.len(), common_keyword
            )),
            affected_tickets: affected,
            common_factor: Some(common_keyword),
            average_excess_percent: None,
            confidence_score: confidence,
            suggested_actions: vec![
                "Investigate root cause of recurring issue".to_string(),
                "Consider creating a dedicated workflow for this issue type".to_string(),
                "Review affected component for systemic problems".to_string(),
            ],
            metadata: serde_json::json!({
                "keyword_count": count,
                "total_analyzed": recent.len()
            }),
        };

        let saved = self.repo.create_pattern(pattern).await?;
        Ok(Some(saved))
    }

    /// Detect spike in tickets for an area.
    async fn detect_spike(&self, data: &WorkflowAnalysisData) -> anyhow::Result<Option<DetectedPattern>> {
        // Compare today's count to 7-day average
        let stats: Option<(i64, f64)> = sqlx::query_as(
            r#"
            WITH today_count AS (
                SELECT COUNT(*) as cnt
                FROM workflow_instances
                WHERE DATE(completed_at) = CURRENT_DATE
                  AND status = 'completed'
            ),
            avg_count AS (
                SELECT AVG(daily_count) as avg
                FROM (
                    SELECT DATE(completed_at) as day, COUNT(*) as daily_count
                    FROM workflow_instances
                    WHERE completed_at >= CURRENT_DATE - INTERVAL '7 days'
                      AND completed_at < CURRENT_DATE
                      AND status = 'completed'
                    GROUP BY DATE(completed_at)
                ) daily
            )
            SELECT today_count.cnt, COALESCE(avg_count.avg, 0)
            FROM today_count, avg_count
            "#,
        )
        .fetch_optional(&self.pool)
        .await?;

        let Some((today_count, avg_count)) = stats else {
            return Ok(None);
        };

        // Spike if today > 2x average
        if avg_count <= 0.0 || (today_count as f64) <= avg_count * 2.0 {
            return Ok(None);
        }

        let spike_ratio = today_count as f64 / avg_count;

        let severity = if spike_ratio > 3.0 {
            Severity::Critical
        } else if spike_ratio > 2.5 {
            Severity::Warning
        } else {
            Severity::Info
        };

        let pattern = NewPattern {
            pattern_type: PatternType::Spike,
            severity,
            title: "Ticket volume spike detected".to_string(),
            description: Some(format!(
                "Today's ticket count ({}) is {:.1}x the 7-day average ({:.1})",
                today_count, spike_ratio, avg_count
            )),
            affected_tickets: vec![data.ticket_key.clone()],
            common_factor: None,
            average_excess_percent: Some((spike_ratio - 1.0) * 100.0),
            confidence_score: 0.9,
            suggested_actions: vec![
                "Check for new deployments or changes".to_string(),
                "Review recent tickets for common issues".to_string(),
                "Consider escalating if trend continues".to_string(),
            ],
            metadata: serde_json::json!({
                "today_count": today_count,
                "avg_count": avg_count,
                "spike_ratio": spike_ratio
            }),
        };

        let saved = self.repo.create_pattern(pattern).await?;
        Ok(Some(saved))
    }

    /// Extract common keywords from notes.
    fn extract_common_keywords(&self, data: &[(String, Option<String>)]) -> Vec<(String, usize)> {
        use std::collections::HashMap;

        let stop_words = ["the", "a", "an", "is", "was", "were", "been", "be", "have", "has", 
                         "had", "do", "does", "did", "will", "would", "could", "should",
                         "and", "or", "but", "if", "then", "else", "when", "at", "by",
                         "for", "with", "about", "to", "from", "in", "on", "of", "it", "this"];

        let mut keyword_counts: HashMap<String, usize> = HashMap::new();

        for (_, notes) in data {
            if let Some(text) = notes {
                let lowercase = text.to_lowercase();
                let words: Vec<&str> = lowercase
                    .split_whitespace()
                    .filter(|w| w.len() > 3 && !stop_words.contains(w))
                    .collect();

                for word in words {
                    *keyword_counts.entry(word.to_string()).or_insert(0) += 1;
                }
            }
        }

        let mut sorted: Vec<_> = keyword_counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        sorted.truncate(10);
        sorted
    }
}

fn format_duration(seconds: i64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m", seconds / 60)
    } else {
        let hours = seconds / 3600;
        let mins = (seconds % 3600) / 60;
        if mins > 0 {
            format!("{}h {}m", hours, mins)
        } else {
            format!("{}h", hours)
        }
    }
}

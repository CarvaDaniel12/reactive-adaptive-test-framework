# Story 9.2: Consecutive Problem Detection

Status: done

## Story

As a system,
I want to detect consecutive tickets with same issue,
So that systemic problems are identified.

## Acceptance Criteria

1. **Given** workflow and ticket data
   **When** analysis runs
   **Then** system detects 3+ consecutive tickets with same component affected

2. **Given** analysis runs
   **When** notes analyzed
   **Then** system detects 3+ consecutive tickets with similar notes/issues

3. **Given** analysis runs
   **When** spike detected
   **Then** system detects spike in tickets for same area

4. **Given** detection runs
   **When** notes analyzed
   **Then** detection uses keyword matching on notes

5. **Given** pattern detected
   **When** stored
   **Then** pattern includes: affected tickets, common factor

6. **Given** pattern detected
   **When** confidence calculated
   **Then** confidence score based on match quality

## Tasks

- [ ] Task 1: Create component extraction from tickets
- [ ] Task 2: Implement consecutive component detection
- [ ] Task 3: Create keyword extraction from notes
- [ ] Task 4: Implement similar notes detection
- [ ] Task 5: Create spike detection algorithm
- [ ] Task 6: Calculate confidence scores

## Dev Notes

### Consecutive Problem Detection

```rust
// crates/qa-pms-analytics/src/pattern_detection.rs (additions)

impl PatternDetectionService {
    /// Detect consecutive problems after workflow completion
    pub async fn detect_consecutive_problems(
        &self,
        user_id: Option<&str>,
    ) -> Result<Vec<DetectedPattern>> {
        let mut patterns = Vec::new();

        // Check component patterns
        if let Some(pattern) = self.detect_component_pattern(user_id).await? {
            patterns.push(self.save_pattern(pattern).await?);
        }

        // Check notes similarity
        if let Some(pattern) = self.detect_similar_notes_pattern(user_id).await? {
            patterns.push(self.save_pattern(pattern).await?);
        }

        // Check for spikes
        if let Some(pattern) = self.detect_area_spike().await? {
            patterns.push(self.save_pattern(pattern).await?);
        }

        Ok(patterns)
    }

    async fn detect_component_pattern(
        &self,
        user_id: Option<&str>,
    ) -> Result<Option<NewPattern>> {
        // Get recent completed workflows with component info
        let recent = sqlx::query_as::<_, (String, String, Option<String>)>(
            r#"
            SELECT 
                wi.ticket_key,
                wi.ticket_title,
                r.content_json->>'component' as component
            FROM workflow_instances wi
            LEFT JOIN reports r ON r.workflow_instance_id = wi.id
            WHERE wi.status = 'completed'
              AND ($1::TEXT IS NULL OR wi.user_id = $1)
            ORDER BY wi.completed_at DESC
            LIMIT 10
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        // Extract components from ticket titles/descriptions
        let components = self.extract_components(&recent);
        
        // Find consecutive sequences
        let consecutive = self.find_consecutive_component(&components, 3);

        if let Some((component, tickets)) = consecutive {
            return Ok(Some(NewPattern {
                pattern_type: PatternType::ConsecutiveIssues,
                severity: Severity::Warning,
                user_id: user_id.map(String::from),
                affected_ticket_ids: tickets,
                details: PatternDetails {
                    description: format!(
                        "3+ consecutive tickets affecting '{}'",
                        component
                    ),
                    average_excess_percent: 0.0,
                    suggested_cause: Some(format!(
                        "The '{}' component may have underlying issues",
                        component
                    )),
                    affected_steps: None,
                    recommendations: vec![
                        "Review recent changes to this component".into(),
                        "Consider dedicated testing session".into(),
                        "Escalate to development team if pattern continues".into(),
                    ],
                },
                confidence_score: 0.85,
            }));
        }

        Ok(None)
    }

    fn extract_components(&self, tickets: &[(String, String, Option<String>)]) -> Vec<(String, Option<String>)> {
        let component_keywords = [
            "login", "auth", "payment", "checkout", "cart", "search",
            "profile", "dashboard", "api", "database", "notification",
            "email", "report", "export", "import", "settings"
        ];

        tickets.iter().map(|(key, title, explicit_component)| {
            let component = explicit_component.clone().or_else(|| {
                let title_lower = title.to_lowercase();
                component_keywords.iter()
                    .find(|&kw| title_lower.contains(kw))
                    .map(|s| s.to_string())
            });
            (key.clone(), component)
        }).collect()
    }

    fn find_consecutive_component(
        &self,
        components: &[(String, Option<String>)],
        min_count: usize,
    ) -> Option<(String, Vec<String>)> {
        let mut current_component: Option<&str> = None;
        let mut current_tickets: Vec<String> = Vec::new();

        for (ticket, component) in components {
            match (current_component, component.as_deref()) {
                (Some(curr), Some(comp)) if curr == comp => {
                    current_tickets.push(ticket.clone());
                }
                (_, Some(comp)) => {
                    if current_tickets.len() >= min_count {
                        return Some((
                            current_component.unwrap().to_string(),
                            current_tickets,
                        ));
                    }
                    current_component = Some(comp);
                    current_tickets = vec![ticket.clone()];
                }
                _ => {
                    if current_tickets.len() >= min_count {
                        return Some((
                            current_component.unwrap().to_string(),
                            current_tickets,
                        ));
                    }
                    current_component = None;
                    current_tickets.clear();
                }
            }
        }

        if current_tickets.len() >= min_count {
            return Some((
                current_component.unwrap().to_string(),
                current_tickets,
            ));
        }

        None
    }

    async fn detect_similar_notes_pattern(
        &self,
        user_id: Option<&str>,
    ) -> Result<Option<NewPattern>> {
        // Get recent step notes
        let notes = sqlx::query_as::<_, (String, String)>(
            r#"
            SELECT wi.ticket_key, wsr.notes
            FROM workflow_instances wi
            JOIN workflow_step_results wsr ON wsr.instance_id = wi.id
            WHERE wi.status = 'completed'
              AND wsr.notes IS NOT NULL
              AND ($1::TEXT IS NULL OR wi.user_id = $1)
            ORDER BY wi.completed_at DESC
            LIMIT 30
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        // Extract keywords from notes
        let keyword_sets: Vec<(String, HashSet<String>)> = notes.iter()
            .map(|(key, note)| (key.clone(), self.extract_keywords(note)))
            .collect();

        // Find similar patterns
        let similar = self.find_similar_notes(&keyword_sets, 3, 0.5);

        if let Some((common_keywords, tickets)) = similar {
            return Ok(Some(NewPattern {
                pattern_type: PatternType::ConsecutiveIssues,
                severity: Severity::Info,
                user_id: user_id.map(String::from),
                affected_ticket_ids: tickets,
                details: PatternDetails {
                    description: format!(
                        "Multiple tickets mention similar issues: {}",
                        common_keywords.join(", ")
                    ),
                    average_excess_percent: 0.0,
                    suggested_cause: Some("Recurring issue may indicate systemic problem".into()),
                    affected_steps: None,
                    recommendations: vec![
                        "Review common factors across these tickets".into(),
                        "Consider root cause analysis".into(),
                    ],
                },
                confidence_score: 0.6,
            }));
        }

        Ok(None)
    }

    fn extract_keywords(&self, text: &str) -> HashSet<String> {
        let stop_words: HashSet<&str> = ["the", "a", "an", "is", "was", "were", "be", "been",
            "being", "have", "has", "had", "do", "does", "did", "will", "would", "could",
            "should", "may", "might", "must", "shall", "can", "need", "to", "of", "in",
            "for", "on", "with", "at", "by", "from", "as", "into", "through", "during",
            "before", "after", "above", "below", "between", "under", "again", "further",
            "then", "once", "here", "there", "when", "where", "why", "how", "all", "each",
            "few", "more", "most", "other", "some", "such", "no", "nor", "not", "only",
            "same", "so", "than", "too", "very", "just", "and", "but", "if", "or",
            "because", "until", "while", "this", "that", "these", "those", "test", "tested",
            "testing", "step", "steps"].into_iter().collect();

        text.to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| w.len() > 3 && !stop_words.contains(w))
            .map(String::from)
            .collect()
    }

    fn find_similar_notes(
        &self,
        keyword_sets: &[(String, HashSet<String>)],
        min_tickets: usize,
        min_similarity: f64,
    ) -> Option<(Vec<String>, Vec<String>)> {
        use std::collections::HashMap;

        // Count keyword occurrences across tickets
        let mut keyword_counts: HashMap<String, Vec<String>> = HashMap::new();
        
        for (ticket, keywords) in keyword_sets {
            for kw in keywords {
                keyword_counts
                    .entry(kw.clone())
                    .or_default()
                    .push(ticket.clone());
            }
        }

        // Find keywords appearing in multiple tickets
        let common: Vec<(String, Vec<String>)> = keyword_counts
            .into_iter()
            .filter(|(_, tickets)| tickets.len() >= min_tickets)
            .collect();

        if common.is_empty() {
            return None;
        }

        // Return most common pattern
        let (keyword, tickets) = common.into_iter()
            .max_by_key(|(_, t)| t.len())?;

        Some((vec![keyword], tickets))
    }

    async fn detect_area_spike(&self) -> Result<Option<NewPattern>> {
        // Detect unusual spike in tickets for a specific area in last 24h vs 7d avg
        let spikes = sqlx::query_as::<_, (String, i64, f64)>(
            r#"
            WITH daily AS (
                SELECT 
                    COALESCE(content_json->>'component', 'unknown') as component,
                    COUNT(*) as today_count
                FROM workflow_instances wi
                LEFT JOIN reports r ON r.workflow_instance_id = wi.id
                WHERE wi.completed_at > NOW() - INTERVAL '24 hours'
                GROUP BY component
            ),
            weekly AS (
                SELECT 
                    COALESCE(content_json->>'component', 'unknown') as component,
                    COUNT(*)::FLOAT / 7 as daily_avg
                FROM workflow_instances wi
                LEFT JOIN reports r ON r.workflow_instance_id = wi.id
                WHERE wi.completed_at > NOW() - INTERVAL '7 days'
                GROUP BY component
            )
            SELECT d.component, d.today_count, w.daily_avg
            FROM daily d
            JOIN weekly w ON d.component = w.component
            WHERE d.today_count > w.daily_avg * 2
              AND d.today_count >= 3
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        if spikes.is_empty() {
            return Ok(None);
        }

        let (component, count, avg) = &spikes[0];
        
        Ok(Some(NewPattern {
            pattern_type: PatternType::ConsecutiveIssues,
            severity: Severity::Critical,
            user_id: None, // System-wide
            affected_ticket_ids: vec![], // Would need to fetch
            details: PatternDetails {
                description: format!(
                    "Spike detected: {} tickets for '{}' today (avg: {:.1}/day)",
                    count, component, avg
                ),
                average_excess_percent: ((*count as f64 / avg) - 1.0) * 100.0,
                suggested_cause: Some("Possible deployment issue or system degradation".into()),
                affected_steps: None,
                recommendations: vec![
                    "Check recent deployments to this area".into(),
                    "Review system health metrics".into(),
                    "Consider incident escalation".into(),
                ],
            },
            confidence_score: 0.9,
        }))
    }
}
```

### References

- [Source: epics.md#Story 9.2]

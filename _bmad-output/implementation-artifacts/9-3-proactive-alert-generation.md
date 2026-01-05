# Story 9.3: Proactive Alert Generation

Status: done

## Story

As a QA/PM,
I want to receive alerts about detected patterns,
So that I can take action before problems escalate.

## Acceptance Criteria

1. **Given** a pattern has been detected
   **When** pattern meets alert threshold
   **Then** alert is generated with alert type (time excess / consecutive problem / spike)

2. **Given** alert is generated
   **When** severity assigned
   **Then** severity is set (info / warning / critical)

3. **Given** alert is generated
   **When** content populated
   **Then** affected tickets list is included

4. **Given** alert is generated
   **When** content populated
   **Then** suggested actions are included

5. **Given** alert is generated
   **When** content populated
   **Then** timestamp is included

6. **Given** alert is generated
   **When** persisted
   **Then** alert is stored in database

7. **Given** alert is generated
   **When** UI updates
   **Then** alert notification appears in UI (toast)

8. **Given** alert exists
   **When** dashboard viewed
   **Then** alert badge shows on dashboard

## Tasks

- [ ] Task 1: Create alerts database table
- [ ] Task 2: Create AlertService for generation
- [ ] Task 3: Define alert thresholds configuration
- [ ] Task 4: Generate alerts from patterns
- [ ] Task 5: Create toast notification system
- [ ] Task 6: Add alert badge to header

## Dev Notes

### Database Schema

```sql
-- migrations/20260103_create_alerts.sql
CREATE TABLE alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pattern_id UUID REFERENCES detected_patterns(id),
    alert_type VARCHAR(50) NOT NULL,
    severity pattern_severity NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    affected_tickets TEXT[],
    suggested_actions JSONB,
    user_id VARCHAR(255),
    is_read BOOLEAN DEFAULT false,
    is_dismissed BOOLEAN DEFAULT false,
    dismissed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_alerts_user ON alerts(user_id);
CREATE INDEX idx_alerts_unread ON alerts(user_id, is_read) WHERE is_read = false;
```

### Alert Service

```rust
// crates/qa-pms-analytics/src/alerts.rs
pub struct AlertService {
    pool: PgPool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub id: Uuid,
    pub pattern_id: Option<Uuid>,
    pub alert_type: String,
    pub severity: String,
    pub title: String,
    pub description: String,
    pub affected_tickets: Vec<String>,
    pub suggested_actions: Vec<String>,
    pub user_id: Option<String>,
    pub is_read: bool,
    pub is_dismissed: bool,
    pub created_at: DateTime<Utc>,
}

impl AlertService {
    /// Generate alert from detected pattern
    pub async fn create_alert_from_pattern(
        &self,
        pattern: &DetectedPattern,
    ) -> Result<Alert> {
        let (title, actions) = self.build_alert_content(pattern);

        let alert = sqlx::query_as::<_, Alert>(
            r#"
            INSERT INTO alerts 
                (pattern_id, alert_type, severity, title, description, 
                 affected_tickets, suggested_actions, user_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(pattern.id)
        .bind(pattern.pattern_type.as_str())
        .bind(pattern.severity.as_str())
        .bind(&title)
        .bind(&pattern.details.description)
        .bind(&pattern.affected_ticket_ids)
        .bind(sqlx::types::Json(&actions))
        .bind(&pattern.user_id)
        .fetch_one(&self.pool)
        .await?;

        tracing::info!(
            alert_id = %alert.id,
            alert_type = %alert.alert_type,
            severity = %alert.severity,
            "Alert generated"
        );

        Ok(alert)
    }

    fn build_alert_content(&self, pattern: &DetectedPattern) -> (String, Vec<String>) {
        let title = match pattern.pattern_type {
            PatternType::StepTimeExcess => "Time Estimate Exceeded".to_string(),
            PatternType::TicketTimeExcess => "Ticket Taking Longer Than Usual".to_string(),
            PatternType::IncreasingTrend => "Increasing Time Trend Detected".to_string(),
            PatternType::ConsecutiveIssues => "Recurring Issue Pattern".to_string(),
        };

        let actions = pattern.details.recommendations.clone();

        (title, actions)
    }

    /// Get unread alerts for user
    pub async fn get_unread_alerts(&self, user_id: &str) -> Result<Vec<Alert>> {
        sqlx::query_as::<_, Alert>(
            r#"
            SELECT * FROM alerts
            WHERE (user_id = $1 OR user_id IS NULL)
              AND is_dismissed = false
            ORDER BY 
                CASE severity 
                    WHEN 'critical' THEN 1 
                    WHEN 'warning' THEN 2 
                    ELSE 3 
                END,
                created_at DESC
            LIMIT 50
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }

    /// Get unread count for badge
    pub async fn get_unread_count(&self, user_id: &str) -> Result<i64> {
        sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM alerts
            WHERE (user_id = $1 OR user_id IS NULL)
              AND is_read = false
              AND is_dismissed = false
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }

    /// Mark alert as read
    pub async fn mark_read(&self, alert_id: Uuid) -> Result<()> {
        sqlx::query("UPDATE alerts SET is_read = true WHERE id = $1")
            .bind(alert_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Dismiss alert
    pub async fn dismiss(&self, alert_id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE alerts SET is_dismissed = true, dismissed_at = NOW() WHERE id = $1"
        )
        .bind(alert_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
```

### API Endpoints

```rust
// GET /api/v1/alerts
pub async fn list_alerts(
    State(state): State<Arc<AppState>>,
    // user_id from auth context
) -> Result<Json<Vec<Alert>>, ApiError> {
    let service = AlertService::new(state.db_pool.clone());
    let alerts = service.get_unread_alerts("current_user").await?;
    Ok(Json(alerts))
}

// GET /api/v1/alerts/count
pub async fn get_alert_count(/* ... */) -> Result<Json<i64>, ApiError> {
    let service = AlertService::new(state.db_pool.clone());
    let count = service.get_unread_count("current_user").await?;
    Ok(Json(count))
}

// PUT /api/v1/alerts/:id/read
pub async fn mark_alert_read(/* ... */) -> Result<StatusCode, ApiError> {
    let service = AlertService::new(state.db_pool.clone());
    service.mark_read(alert_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// PUT /api/v1/alerts/:id/dismiss
pub async fn dismiss_alert(/* ... */) -> Result<StatusCode, ApiError> {
    let service = AlertService::new(state.db_pool.clone());
    service.dismiss(alert_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
```

### Frontend - Alert Badge

```tsx
// frontend/src/components/AlertBadge.tsx
import { useAlertCount } from "@/hooks/useAlerts";

export function AlertBadge() {
  const { data: count } = useAlertCount();

  if (!count || count === 0) return null;

  return (
    <span className="absolute -top-1 -right-1 flex items-center justify-center 
                     w-5 h-5 text-xs font-bold text-white bg-error-500 rounded-full">
      {count > 9 ? "9+" : count}
    </span>
  );
}

// In Header
<button className="relative p-2">
  <BellIcon className="w-6 h-6" />
  <AlertBadge />
</button>
```

### Frontend - Toast Notifications

```tsx
// frontend/src/hooks/useAlertNotifications.ts
import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { toast } from "sonner";

export function useAlertNotifications() {
  const { data: alerts } = useQuery({
    queryKey: ["alerts", "new"],
    queryFn: fetchNewAlerts,
    refetchInterval: 30000, // Check every 30s
  });

  useEffect(() => {
    if (!alerts?.length) return;

    // Show toast for new critical/warning alerts
    const newAlerts = alerts.filter(a => !a.isRead);
    
    for (const alert of newAlerts.slice(0, 3)) {
      const toastFn = alert.severity === "critical" ? toast.error 
        : alert.severity === "warning" ? toast.warning 
        : toast.info;

      toastFn(alert.title, {
        description: alert.description,
        duration: 10000,
        action: {
          label: "View",
          onClick: () => openAlertPanel(),
        },
      });
    }
  }, [alerts]);
}
```

### References

- [Source: epics.md#Story 9.3]

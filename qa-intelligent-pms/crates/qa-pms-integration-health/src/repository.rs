//! Repository for integration health data operations.

use chrono::{DateTime, Utc};
use rust_decimal::prelude::ToPrimitive;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::error::IntegrationHealthError;
use crate::types::{HealthStatus, IntegrationEvent, IntegrationHealth, IntegrationId};
use qa_pms_dashboard::SqlxResultExt;

/// Repository for integration health data.
pub struct IntegrationHealthRepository {
    pool: PgPool,
}

impl IntegrationHealthRepository {
    /// Create a new repository.
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get latest health status for an integration.
    ///
    /// # Errors
    /// Returns error if database query fails.
    pub async fn get_latest_health(
        &self,
        integration_id: IntegrationId,
    ) -> Result<Option<IntegrationHealth>, IntegrationHealthError> {
        let integration_id_str = integration_id.to_string();

        let row: Option<HealthRow> = sqlx::query_as(
            r"
            SELECT 
                id, integration_id, status, pricing_sync_status, fees_sync_status,
                booking_loss_rate, error_rate, last_checked, created_at, updated_at
            FROM integration_health
            WHERE integration_id = $1
            ORDER BY last_checked DESC
            LIMIT 1
            ",
        )
        .bind(&integration_id_str)
        .fetch_optional(&self.pool)
        .await
        .map_internal("Failed to fetch integration health")
        .map_err(IntegrationHealthError::from)?;

        match row {
            Some(r) => Ok(Some(try_from_health_row(r)?)),
            None => Ok(None),
        }
    }

    /// Get health history for an integration within a period.
    ///
    /// # Errors
    /// Returns error if database query fails.
    pub async fn get_health_history(
        &self,
        integration_id: IntegrationId,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<IntegrationHealth>, IntegrationHealthError> {
        let integration_id_str = integration_id.to_string();

        let rows: Vec<HealthRow> = sqlx::query_as(
            r"
            SELECT 
                id, integration_id, status, pricing_sync_status, fees_sync_status,
                booking_loss_rate, error_rate, last_checked, created_at, updated_at
            FROM integration_health
            WHERE integration_id = $1
              AND last_checked >= $2
              AND last_checked <= $3
            ORDER BY last_checked DESC
            ",
        )
        .bind(&integration_id_str)
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await
        .map_internal("Failed to fetch integration health history")
        .map_err(IntegrationHealthError::from)?;

        let mut results = Vec::new();
        for row in rows {
            results.push(try_from_health_row(row)?);
        }
        Ok(results)
    }

    /// Store health status.
    ///
    /// Uses ON CONFLICT UPDATE to handle unique constraint on (integration_id, last_checked).
    ///
    /// # Errors
    /// Returns error if database insert/update fails.
    pub async fn store_health_status(
        &self,
        health: &IntegrationHealth,
    ) -> Result<(), IntegrationHealthError> {
        let integration_id_str = health.integration_id.to_string();
        let status_str = health.status.to_string();
        let pricing_sync_status_str = health.pricing_sync_status.map(|s| s.to_string());
        let fees_sync_status_str = health.fees_sync_status.map(|s| s.to_string());

        // Convert f64 to rust_decimal::Decimal for database storage
        let booking_loss_rate_decimal = health
            .booking_loss_rate
            .and_then(|r| rust_decimal::Decimal::from_f64_retain(r));
        let error_rate_decimal = health
            .error_rate
            .and_then(|r| rust_decimal::Decimal::from_f64_retain(r));

        sqlx::query(
            r"
            INSERT INTO integration_health (
                integration_id, status, pricing_sync_status, fees_sync_status,
                booking_loss_rate, error_rate, last_checked, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
            ON CONFLICT (integration_id, last_checked) DO UPDATE SET
                status = EXCLUDED.status,
                pricing_sync_status = EXCLUDED.pricing_sync_status,
                fees_sync_status = EXCLUDED.fees_sync_status,
                booking_loss_rate = EXCLUDED.booking_loss_rate,
                error_rate = EXCLUDED.error_rate,
                updated_at = NOW()
            ",
        )
        .bind(&integration_id_str)
        .bind(&status_str)
        .bind(&pricing_sync_status_str)
        .bind(&fees_sync_status_str)
        .bind(&booking_loss_rate_decimal)
        .bind(&error_rate_decimal)
        .bind(health.last_checked)
        .execute(&self.pool)
        .await
        .map_internal("Failed to store integration health status")
        .map_err(IntegrationHealthError::from)?;

        Ok(())
    }

    /// Store integration event.
    ///
    /// # Errors
    /// Returns error if database insert fails.
    pub async fn store_event(&self, event: &IntegrationEvent) -> Result<(), IntegrationHealthError> {
        let integration_id_str = event.integration_id.to_string();
        let metadata_json = event.metadata.as_ref().map(serde_json::to_value).transpose()
            .map_err(|e| IntegrationHealthError::Internal(anyhow::anyhow!("Failed to serialize metadata: {}", e)))?;

        sqlx::query(
            r"
            INSERT INTO integration_events (
                id, integration_id, event_type, severity, message, metadata, occurred_at, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
            ",
        )
        .bind(event.id)
        .bind(&integration_id_str)
        .bind(&event.event_type)
        .bind(&event.severity)
        .bind(&event.message)
        .bind(&metadata_json)
        .bind(event.occurred_at)
        .execute(&self.pool)
        .await
        .map_internal("Failed to store integration event")
        .map_err(IntegrationHealthError::from)?;

        Ok(())
    }
}

// Internal row type for sqlx
#[derive(FromRow)]
#[allow(dead_code)] // id, created_at, updated_at are needed for FromRow but not used in conversion
struct HealthRow {
    id: Uuid,
    integration_id: String,
    status: String,
    pricing_sync_status: Option<String>,
    fees_sync_status: Option<String>,
    booking_loss_rate: Option<rust_decimal::Decimal>,
    error_rate: Option<rust_decimal::Decimal>,
    last_checked: DateTime<Utc>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Convert HealthRow to IntegrationHealth with validation.
///
/// # Errors
/// Returns error if database contains invalid enum values (data corruption).
fn try_from_health_row(row: HealthRow) -> Result<IntegrationHealth, IntegrationHealthError> {
    // Parse integration_id string to enum
    let integration_id = match row.integration_id.as_str() {
        "booking-com" => IntegrationId::BookingCom,
        "airbnb" => IntegrationId::Airbnb,
        "vrbo" => IntegrationId::Vrbo,
        "hmbn" => IntegrationId::Hmbn,
        unknown => {
            return Err(IntegrationHealthError::Internal(anyhow::anyhow!(
                "Invalid integration_id in database: {}",
                unknown
            )));
        }
    };

    // Parse status string to enum
    let status = match row.status.as_str() {
        "healthy" => HealthStatus::Healthy,
        "warning" => HealthStatus::Warning,
        "critical" => HealthStatus::Critical,
        unknown => {
            return Err(IntegrationHealthError::Internal(anyhow::anyhow!(
                "Invalid status in database: {}",
                unknown
            )));
        }
    };

    // Parse optional pricing_sync_status
    let pricing_sync_status = row.pricing_sync_status.as_deref().and_then(|s| {
        match s {
            "healthy" => Some(HealthStatus::Healthy),
            "warning" => Some(HealthStatus::Warning),
            "critical" => Some(HealthStatus::Critical),
            _ => None, // Invalid values are ignored (optional field)
        }
    });

    // Parse optional fees_sync_status
    let fees_sync_status = row.fees_sync_status.as_deref().and_then(|s| {
        match s {
            "healthy" => Some(HealthStatus::Healthy),
            "warning" => Some(HealthStatus::Warning),
            "critical" => Some(HealthStatus::Critical),
            _ => None, // Invalid values are ignored (optional field)
        }
    });

    // Convert Decimal to f64
    let booking_loss_rate = row
        .booking_loss_rate
        .and_then(|d| d.to_f64());
    let error_rate = row
        .error_rate
        .and_then(|d| d.to_f64());

    // Calculate trend (simplified - would need previous value for real trend calculation)
    // For now, default to "neutral"
    let trend = "neutral".to_string();

    Ok(IntegrationHealth {
        integration_id,
        status,
        pricing_sync_status,
        fees_sync_status,
        booking_loss_rate,
        error_rate,
        last_checked: row.last_checked,
        trend,
    })
}

// Keep From implementation for backward compatibility (but prefer try_from_health_row)
impl From<HealthRow> for IntegrationHealth {
    fn from(row: HealthRow) -> Self {
        try_from_health_row(row).expect("Invalid data in database - this should not happen")
    }
}

//! Types for integration health monitoring.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

/// Integration ID for PMS integrations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "kebab-case")]
#[serde(rename_all = "camelCase")]
pub enum IntegrationId {
    /// Booking.com integration
    BookingCom,
    /// Airbnb integration
    Airbnb,
    /// Vrbo integration
    Vrbo,
    /// HMBN integration
    Hmbn,
}

impl std::fmt::Display for IntegrationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BookingCom => write!(f, "booking-com"),
            Self::Airbnb => write!(f, "airbnb"),
            Self::Vrbo => write!(f, "vrbo"),
            Self::Hmbn => write!(f, "hmbn"),
        }
    }
}

/// Health status of an integration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum HealthStatus {
    /// Integration is healthy
    Healthy,
    /// Integration has warnings
    Warning,
    /// Integration is critical
    Critical,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Healthy => write!(f, "healthy"),
            Self::Warning => write!(f, "warning"),
            Self::Critical => write!(f, "critical"),
        }
    }
}

/// Integration health information.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationHealth {
    /// Integration ID
    pub integration_id: IntegrationId,
    /// Overall health status
    pub status: HealthStatus,
    /// Pricing sync status (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_sync_status: Option<HealthStatus>,
    /// Fees sync status (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees_sync_status: Option<HealthStatus>,
    /// Booking loss rate (0.0 to 1.0, optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booking_loss_rate: Option<f64>,
    /// Error rate (0.0 to 1.0, optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_rate: Option<f64>,
    /// When the health was last checked
    pub last_checked: DateTime<Utc>,
    /// Trend direction: "up", "down", or "neutral"
    pub trend: String,
}

/// Integration event record.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationEvent {
    /// Event ID
    pub id: Uuid,
    /// Integration ID
    pub integration_id: IntegrationId,
    /// Event type (e.g., "pricing_sync_error", "fee_sync_error", "booking_loss")
    pub event_type: String,
    /// Event severity ("low", "medium", "high", "critical")
    pub severity: String,
    /// Event message (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Additional metadata (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    /// When the event occurred
    pub occurred_at: DateTime<Utc>,
    /// When the event was created
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_integration_id_serialization() {
        let id = IntegrationId::BookingCom;
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"bookingCom\"");
    }

    #[test]
    fn test_integration_id_deserialization() {
        let json = "\"bookingCom\"";
        let id: IntegrationId = serde_json::from_str(json).unwrap();
        assert_eq!(id, IntegrationId::BookingCom);
    }

    #[test]
    fn test_health_status_serialization() {
        let status = HealthStatus::Healthy;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"healthy\"");
    }

    #[test]
    fn test_health_status_deserialization() {
        let json = "\"warning\"";
        let status: HealthStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, HealthStatus::Warning);
    }

    #[test]
    fn test_integration_health_serialization() {
        let health = IntegrationHealth {
            integration_id: IntegrationId::BookingCom,
            status: HealthStatus::Healthy,
            pricing_sync_status: Some(HealthStatus::Healthy),
            fees_sync_status: None,
            booking_loss_rate: Some(0.05),
            error_rate: Some(0.01),
            last_checked: Utc::now(),
            trend: "up".to_string(),
        };
        let json = serde_json::to_string(&health).unwrap();
        assert!(json.contains("\"integrationId\":\"bookingCom\""));
        assert!(json.contains("\"status\":\"healthy\""));
        assert!(json.contains("\"bookingLossRate\":0.05"));
        assert!(!json.contains("feesSyncStatus")); // Should be skipped when None
    }

    #[test]
    fn test_integration_health_deserialization() {
        let json = r#"{
            "integrationId": "airbnb",
            "status": "warning",
            "pricingSyncStatus": "healthy",
            "bookingLossRate": 0.1,
            "errorRate": 0.02,
            "lastChecked": "2026-01-11T12:00:00Z",
            "trend": "down"
        }"#;
        let health: IntegrationHealth = serde_json::from_str(json).unwrap();
        assert_eq!(health.integration_id, IntegrationId::Airbnb);
        assert_eq!(health.status, HealthStatus::Warning);
        assert_eq!(health.booking_loss_rate, Some(0.1));
        assert_eq!(health.trend, "down");
    }

    #[test]
    fn test_integration_event_serialization() {
        let event = IntegrationEvent {
            id: Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
            integration_id: IntegrationId::Vrbo,
            event_type: "pricing_sync_error".to_string(),
            severity: "high".to_string(),
            message: Some("Sync failed".to_string()),
            metadata: Some(serde_json::json!({"key": "value"})),
            occurred_at: Utc::now(),
            created_at: Utc::now(),
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"eventType\":\"pricing_sync_error\""));
        assert!(json.contains("\"severity\":\"high\""));
        assert!(json.contains("\"message\":\"Sync failed\""));
    }

    #[test]
    fn test_integration_event_deserialization() {
        let json = r#"{
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "integrationId": "hmbn",
            "eventType": "booking_loss",
            "severity": "critical",
            "message": "Lost booking",
            "occurredAt": "2026-01-11T12:00:00Z",
            "createdAt": "2026-01-11T12:00:00Z"
        }"#;
        let event: IntegrationEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.integration_id, IntegrationId::Hmbn);
        assert_eq!(event.event_type, "booking_loss");
        assert_eq!(event.severity, "critical");
        assert_eq!(event.message, Some("Lost booking".to_string()));
    }

    #[test]
    fn test_display_integration_id() {
        assert_eq!(IntegrationId::BookingCom.to_string(), "booking-com");
        assert_eq!(IntegrationId::Airbnb.to_string(), "airbnb");
        assert_eq!(IntegrationId::Vrbo.to_string(), "vrbo");
        assert_eq!(IntegrationId::Hmbn.to_string(), "hmbn");
    }

    #[test]
    fn test_display_health_status() {
        assert_eq!(HealthStatus::Healthy.to_string(), "healthy");
        assert_eq!(HealthStatus::Warning.to_string(), "warning");
        assert_eq!(HealthStatus::Critical.to_string(), "critical");
    }
}
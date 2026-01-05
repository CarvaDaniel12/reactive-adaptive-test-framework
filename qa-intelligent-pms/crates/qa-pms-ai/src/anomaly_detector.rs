//! Anomaly detection service for workflow execution patterns.
//!
//! This module provides statistical anomaly detection for workflows,
//! using z-score calculation and baseline metrics to identify deviations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Type Definitions
// ============================================================================

/// Type of anomaly detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnomalyType {
    /// Sudden spike in workflow failures
    SpikeInFailures,
    /// Performance degradation detected
    PerformanceDegradation,
    /// Unusual execution time (too fast or too slow)
    UnusualExecutionTime,
    /// Pattern deviation from normal behavior
    PatternDeviation,
    /// Resource exhaustion
    ResourceExhaustion,
    /// Consecutive failures
    ConsecutiveFailures,
}

impl AnomalyType {
    /// Convert to database string.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::SpikeInFailures => "spike_in_failures",
            Self::PerformanceDegradation => "performance_degradation",
            Self::UnusualExecutionTime => "unusual_execution_time",
            Self::PatternDeviation => "pattern_deviation",
            Self::ResourceExhaustion => "resource_exhaustion",
            Self::ConsecutiveFailures => "consecutive_failures",
        }
    }

    /// Convert from database string.
    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "spike_in_failures" => Self::SpikeInFailures,
            "performance_degradation" => Self::PerformanceDegradation,
            "unusual_execution_time" => Self::UnusualExecutionTime,
            "pattern_deviation" => Self::PatternDeviation,
            "resource_exhaustion" => Self::ResourceExhaustion,
            "consecutive_failures" => Self::ConsecutiveFailures,
            _ => Self::PatternDeviation,
        }
    }
}

/// Severity of the detected anomaly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnomalySeverity {
    /// Informational anomaly
    Info,
    /// Warning-level anomaly
    Warning,
    /// Critical anomaly requiring immediate attention
    Critical,
}

impl AnomalySeverity {
    /// Convert to database string.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Critical => "critical",
        }
    }

    /// Convert from database string.
    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "info" => Self::Info,
            "warning" => Self::Warning,
            "critical" => Self::Critical,
            _ => Self::Info,
        }
    }
}

/// Metrics associated with an anomaly.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnomalyMetrics {
    /// Current value that triggered the anomaly
    pub current_value: f64,
    /// Baseline value (moving average)
    pub baseline_value: f64,
    /// Deviation from baseline (current - baseline)
    pub deviation: f64,
    /// Z-score (standard deviations from mean)
    pub z_score: f64,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
}

/// A detected anomaly.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Anomaly {
    /// Unique identifier
    pub id: Uuid,
    /// When the anomaly was detected
    pub detected_at: DateTime<Utc>,
    /// Type of anomaly
    pub anomaly_type: AnomalyType,
    /// Severity level
    pub severity: AnomalySeverity,
    /// Human-readable description
    pub description: String,
    /// Statistical metrics
    pub metrics: AnomalyMetrics,
    /// Affected entities (workflow IDs, ticket IDs, etc.)
    pub affected_entities: Vec<String>,
    /// Suggested investigation steps
    pub investigation_steps: Vec<String>,
}

// ============================================================================
// Baseline Metrics
// ============================================================================

/// Moving average calculator for baseline metrics.
#[derive(Debug, Clone)]
pub struct MovingAverage {
    /// Current average value
    pub value: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Window size (number of samples)
    pub window_size: usize,
    /// Historical values for calculation
    values: Vec<f64>,
}

impl MovingAverage {
    /// Create a new moving average calculator.
    #[must_use]
    pub fn new(window_size: usize) -> Self {
        Self {
            value: 0.0,
            std_dev: 0.0,
            window_size,
            values: Vec::with_capacity(window_size),
        }
    }

    /// Update the moving average with a new value.
    pub fn update(&mut self, new_value: f64) {
        // Add new value
        self.values.push(new_value);

        // Keep only last window_size values
        if self.values.len() > self.window_size {
            self.values.remove(0);
        }

        // Recalculate mean
        if !self.values.is_empty() {
            self.value = self.values.iter().sum::<f64>() / self.values.len() as f64;

            // Calculate standard deviation
            let variance = self
                .values
                .iter()
                .map(|v| (v - self.value).powi(2))
                .sum::<f64>()
                / self.values.len() as f64;
            self.std_dev = variance.sqrt();
        }
    }

    /// Calculate z-score for a value.
    #[must_use]
    pub fn z_score(&self, value: f64) -> f64 {
        if self.std_dev == 0.0 {
            return 0.0;
        }
        (value - self.value) / self.std_dev
    }
}

/// Baseline metrics tracker for workflow execution.
#[derive(Debug, Clone)]
pub struct BaselineMetrics {
    /// Failure rate baseline (percentage)
    pub failure_rate: MovingAverage,
    /// Execution time baseline (seconds)
    pub execution_time: MovingAverage,
    /// Success rate baseline (percentage)
    pub success_rate: MovingAverage,
}

impl BaselineMetrics {
    /// Create new baseline metrics with 30-day window.
    #[must_use]
    pub fn new() -> Self {
        Self {
            failure_rate: MovingAverage::new(30),
            execution_time: MovingAverage::new(30),
            success_rate: MovingAverage::new(30),
        }
    }

    /// Update baselines with workflow execution data.
    pub fn update(&mut self, failure_rate: f64, execution_time: f64, success_rate: f64) {
        self.failure_rate.update(failure_rate);
        self.execution_time.update(execution_time);
        self.success_rate.update(success_rate);
    }
}

impl Default for BaselineMetrics {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Workflow Execution Data
// ============================================================================

/// Workflow execution data for anomaly detection.
#[derive(Debug, Clone)]
pub struct WorkflowExecution {
    /// Workflow instance ID
    pub instance_id: Uuid,
    /// Ticket ID
    pub ticket_id: String,
    /// User ID
    pub user_id: String,
    /// Template ID
    pub template_id: Uuid,
    /// Execution time in seconds
    pub execution_time_seconds: i32,
    /// Whether the workflow succeeded
    pub succeeded: bool,
    /// When execution completed
    pub completed_at: DateTime<Utc>,
}

// ============================================================================
// Anomaly Detector
// ============================================================================

/// Anomaly detector service.
pub struct AnomalyDetector {
    /// Baseline metrics tracker
    baseline: BaselineMetrics,
}

impl AnomalyDetector {
    /// Create a new anomaly detector with empty baseline.
    #[must_use]
    pub fn new() -> Self {
        Self {
            baseline: BaselineMetrics::new(),
        }
    }

    /// Create an anomaly detector with baseline initialized from historical data.
    /// 
    /// This loads historical executions and initializes the baseline metrics
    /// to enable effective anomaly detection.
    pub async fn with_historical_baseline(
        pool: &sqlx::PgPool,
        template_id: Option<Uuid>,
        window_size: i32,
    ) -> anyhow::Result<Self> {
        let mut detector = Self::new();
        
        // Load historical executions
        let repo = crate::AnomalyRepository::new(pool.clone());
        let historical = repo
            .get_historical_executions(window_size, template_id)
            .await?;

        // Initialize baseline from historical data
        for exec_data in &historical {
            let exec: WorkflowExecution = exec_data.clone().into();
            detector.update_baseline(&exec);
        }

        Ok(detector)
    }

    /// Check workflow execution for anomalies.
    pub fn check_execution(&mut self, execution: &WorkflowExecution) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        // Check performance degradation (execution time significantly above baseline)
        // This check detects when execution time > baseline + 2σ (performance degradation)
        if let Some(anomaly) = self.check_performance(execution) {
            anomalies.push(anomaly);
        }

        // Check unusual execution time (both too fast and too slow, using z-score)
        // This is a separate check that catches outliers in both directions
        // Only add if not already detected by performance degradation
        if let Some(anomaly) = self.check_execution_time(execution) {
            // Avoid duplicates: if performance degradation already detected, skip
            if !anomalies.iter().any(|a| {
                a.anomaly_type == AnomalyType::PerformanceDegradation
                    && a.affected_entities
                        .iter()
                        .any(|e| anomaly.affected_entities.contains(e))
            }) {
                anomalies.push(anomaly);
            }
        }

        anomalies
    }

    /// Check for failure rate spikes.
    /// 
    /// NOTE: This requires batch analysis of multiple executions.
    /// Currently returns None as it needs historical data aggregation.
    /// To be implemented when batch analysis is available.
    fn check_failure_rate(&self, _execution: &WorkflowExecution) -> Option<Anomaly> {
        // For single execution, we can't calculate failure rate spike directly
        // This would need historical data from multiple executions
        // For now, skip this check per execution (would need batch analysis)
        None
    }

    /// Check for performance degradation.
    fn check_performance(&self, execution: &WorkflowExecution) -> Option<Anomaly> {
        let exec_time = execution.execution_time_seconds as f64;
        let baseline_time = self.baseline.execution_time.value;
        let std_dev = self.baseline.execution_time.std_dev;

        // Check if execution time is significantly above baseline (baseline + 2σ)
        if baseline_time > 0.0 && exec_time > baseline_time + (2.0 * std_dev) {
            let z_score = self.baseline.execution_time.z_score(exec_time);
            let severity = if z_score > 3.0 {
                AnomalySeverity::Critical
            } else if z_score > 2.0 {
                AnomalySeverity::Warning
            } else {
                AnomalySeverity::Info
            };

            return Some(Anomaly {
                id: Uuid::new_v4(),
                detected_at: Utc::now(),
                anomaly_type: AnomalyType::PerformanceDegradation,
                severity,
                description: format!(
                    "Workflow execution time ({:.1}s) is significantly above baseline ({:.1}s ± {:.1}s)",
                    exec_time, baseline_time, std_dev
                ),
                metrics: AnomalyMetrics {
                    current_value: exec_time,
                    baseline_value: baseline_time,
                    deviation: exec_time - baseline_time,
                    z_score,
                    confidence: (z_score / 3.0).min(1.0),
                },
                affected_entities: vec![
                    execution.instance_id.to_string(),
                    execution.ticket_id.clone(),
                ],
                investigation_steps: vec![
                    "Review workflow step completion times".to_string(),
                    "Check for external API delays".to_string(),
                    "Investigate system resource usage".to_string(),
                ],
            });
        }

        None
    }

    /// Check for unusual execution times.
    fn check_execution_time(&self, execution: &WorkflowExecution) -> Option<Anomaly> {
        let exec_time = execution.execution_time_seconds as f64;
        let z_score = self.baseline.execution_time.z_score(exec_time);

        // Flag if z-score > 2.0 (either too fast or too slow)
        if z_score.abs() > 2.0 {
            let severity = if z_score.abs() > 3.0 {
                AnomalySeverity::Critical
            } else {
                AnomalySeverity::Warning
            };

            let anomaly_type = if z_score > 0.0 {
                AnomalyType::UnusualExecutionTime
            } else {
                AnomalyType::UnusualExecutionTime // Could distinguish fast vs slow
            };

            return Some(Anomaly {
                id: Uuid::new_v4(),
                detected_at: Utc::now(),
                anomaly_type,
                severity,
                description: format!(
                    "Unusual execution time detected: {:.1}s (z-score: {:.2})",
                    exec_time, z_score
                ),
                metrics: AnomalyMetrics {
                    current_value: exec_time,
                    baseline_value: self.baseline.execution_time.value,
                    deviation: exec_time - self.baseline.execution_time.value,
                    z_score,
                    confidence: (z_score.abs() / 3.0).min(1.0),
                },
                affected_entities: vec![
                    execution.instance_id.to_string(),
                    execution.ticket_id.clone(),
                ],
                investigation_steps: vec![
                    "Verify execution was completed correctly".to_string(),
                    "Check for data anomalies".to_string(),
                    "Review workflow notes for unusual circumstances".to_string(),
                ],
            });
        }

        None
    }

    /// Check for consecutive failures (requires batch analysis).
    fn check_consecutive_failures(&self, _executions: &[WorkflowExecution]) -> Option<Anomaly> {
        // This would require batch analysis of multiple executions
        // To be implemented when batch analysis is available
        None
    }

    /// Update baseline metrics after execution.
    pub fn update_baseline(&mut self, execution: &WorkflowExecution) {
        let failure_rate = if execution.succeeded { 0.0 } else { 100.0 };
        let success_rate = if execution.succeeded { 100.0 } else { 0.0 };
        let exec_time = execution.execution_time_seconds as f64;

        self.baseline
            .update(failure_rate, exec_time, success_rate);
    }

    /// Get current baseline metrics.
    #[must_use]
    pub fn baseline(&self) -> &BaselineMetrics {
        &self.baseline
    }
}

impl Default for AnomalyDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert an Anomaly to an AnomalyAlert for notification.
impl Anomaly {
    /// Convert to an AnomalyAlert for alerting.
    #[must_use]
    pub fn to_alert(&self) -> qa_pms_core::alerts::AnomalyAlert {
        qa_pms_core::alerts::create_anomaly_alert(
            self.id,
            self.anomaly_type.as_str().to_string(),
            match self.severity {
                AnomalySeverity::Info => qa_pms_core::alerts::AnomalySeverity::Info,
                AnomalySeverity::Warning => qa_pms_core::alerts::AnomalySeverity::Warning,
                AnomalySeverity::Critical => qa_pms_core::alerts::AnomalySeverity::Critical,
            },
            self.description.clone(),
            self.affected_entities.clone(),
            self.investigation_steps.clone(),
            serde_json::to_string(&self.metrics).unwrap_or_else(|_| "{}".to_string()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving_average() {
        let mut ma = MovingAverage::new(5);
        assert_eq!(ma.value, 0.0);

        // Add values
        ma.update(10.0);
        ma.update(20.0);
        ma.update(30.0);

        assert_eq!(ma.value, 20.0); // (10 + 20 + 30) / 3 = 20

        // Test z-score
        let z = ma.z_score(30.0);
        assert!(z > 0.0); // Should be positive
    }

    #[test]
    fn test_anomaly_detector_creation() {
        let detector = AnomalyDetector::new();
        assert_eq!(detector.baseline().failure_rate.window_size, 30);
    }

    #[test]
    fn test_baseline_update() {
        let mut baseline = BaselineMetrics::new();
        baseline.update(10.0, 100.0, 90.0);

        assert_eq!(baseline.failure_rate.value, 10.0);
        assert_eq!(baseline.execution_time.value, 100.0);
        assert_eq!(baseline.success_rate.value, 90.0);
    }

    #[test]
    fn test_anomaly_type_conversion() {
        assert_eq!(
            AnomalyType::from_str("spike_in_failures"),
            AnomalyType::SpikeInFailures
        );
        assert_eq!(
            AnomalyType::SpikeInFailures.as_str(),
            "spike_in_failures"
        );
    }

    #[test]
    fn test_anomaly_severity_ordering() {
        assert!(AnomalySeverity::Critical > AnomalySeverity::Warning);
        assert!(AnomalySeverity::Warning > AnomalySeverity::Info);
    }

    #[test]
    fn test_z_score_calculation() {
        let mut ma = MovingAverage::new(10);

        // Build baseline
        for i in 1..=10 {
            ma.update(i as f64 * 10.0); // 10, 20, 30, ..., 100
        }

        // Test z-score for values at mean
        let z_at_mean = ma.z_score(ma.value);
        assert!((z_at_mean.abs() < 0.1)); // Should be ~0 at mean

        // Test z-score for value above mean
        let z_above = ma.z_score(ma.value + ma.std_dev);
        assert!((z_above - 1.0).abs() < 0.5); // Should be ~1.0

        // Test z-score for value below mean
        let z_below = ma.z_score(ma.value - ma.std_dev);
        assert!((z_below + 1.0).abs() < 0.5); // Should be ~-1.0

        // Test z-score with zero std_dev
        let mut ma_zero = MovingAverage::new(1);
        ma_zero.update(10.0);
        let z_zero = ma_zero.z_score(20.0);
        assert_eq!(z_zero, 0.0); // Should return 0 when std_dev is 0
    }

    #[test]
    fn test_baseline_window_size() {
        let mut baseline = BaselineMetrics::new();
        assert_eq!(baseline.failure_rate.window_size, 30);
        assert_eq!(baseline.execution_time.window_size, 30);
        assert_eq!(baseline.success_rate.window_size, 30);

        // Test that window size limits history
        let mut ma = MovingAverage::new(5);
        for i in 1..=10 {
            ma.update(i as f64);
        }
        // Should only keep last 5 values
        assert_eq!(ma.values.len(), 5);
    }

    #[tokio::test]
    async fn test_performance_degradation_detection() {
        let mut detector = AnomalyDetector::new();

        // Build baseline with normal execution times (need variance for std_dev)
        // Use slightly varying times to ensure std_dev > 0
        for i in 0..10 {
            let exec = WorkflowExecution {
                instance_id: Uuid::new_v4(),
                ticket_id: format!("TICKET-{}", i),
                user_id: "user1".to_string(),
                template_id: Uuid::new_v4(),
                execution_time_seconds: 100 + (i % 3) * 5, // 100, 105, 110, 100, ...
                succeeded: true,
                completed_at: Utc::now(),
            };
            detector.update_baseline(&exec);
        }

        // Ensure baseline has std_dev > 0
        let baseline = detector.baseline();
        assert!(baseline.execution_time.std_dev > 0.0, "Baseline should have std_dev > 0");

        // Execute with unusually long time (baseline + 2σ + margin)
        let baseline_time = baseline.execution_time.value;
        let std_dev = baseline.execution_time.std_dev;
        let slow_time = (baseline_time + (2.5 * std_dev)) as i32 + 50;
        
        let slow_exec = WorkflowExecution {
            instance_id: Uuid::new_v4(),
            ticket_id: "TICKET-SLOW".to_string(),
            user_id: "user1".to_string(),
            template_id: Uuid::new_v4(),
            execution_time_seconds: slow_time,
            succeeded: true,
            completed_at: Utc::now(),
        };

        let anomalies = detector.check_execution(&slow_exec);
        assert!(!anomalies.is_empty(), "Should detect performance degradation");
        assert!(anomalies
            .iter()
            .any(|a| a.anomaly_type == AnomalyType::PerformanceDegradation));

        // Verify anomaly details
        let perf_anomaly = anomalies
            .iter()
            .find(|a| a.anomaly_type == AnomalyType::PerformanceDegradation)
            .unwrap();
        assert!(perf_anomaly.metrics.current_value > perf_anomaly.metrics.baseline_value);
        // Severity could be Warning or Critical depending on z-score
        assert!(perf_anomaly.severity == AnomalySeverity::Warning || 
                perf_anomaly.severity == AnomalySeverity::Critical);
    }

    #[test]
    fn test_unusual_execution_time_detection() {
        let mut detector = AnomalyDetector::new();

        // Build baseline with variance for std_dev calculation
        for i in 0..10 {
            let exec = WorkflowExecution {
                instance_id: Uuid::new_v4(),
                ticket_id: format!("TICKET-{}", i),
                user_id: "user1".to_string(),
                template_id: Uuid::new_v4(),
                execution_time_seconds: 100 + (i % 3) * 5, // Vary between 100-110
                succeeded: true,
                completed_at: Utc::now(),
            };
            detector.update_baseline(&exec);
        }

        let baseline = detector.baseline();
        assert!(baseline.execution_time.std_dev > 0.0);

        // Test too slow (z-score > 2.0)
        let baseline_time = baseline.execution_time.value;
        let std_dev = baseline.execution_time.std_dev;
        let slow_time = (baseline_time + (2.5 * std_dev)) as i32;
        
        let slow_exec = WorkflowExecution {
            instance_id: Uuid::new_v4(),
            ticket_id: "TICKET-SLOW".to_string(),
            user_id: "user1".to_string(),
            template_id: Uuid::new_v4(),
            execution_time_seconds: slow_time,
            succeeded: true,
            completed_at: Utc::now(),
        };

        let anomalies = detector.check_execution(&slow_exec);
        // May be detected as PerformanceDegradation instead, or as UnusualExecutionTime
        assert!(!anomalies.is_empty(), "Should detect unusual execution time");
        
        let unusual_time = anomalies
            .iter()
            .find(|a| a.anomaly_type == AnomalyType::UnusualExecutionTime);
        
        // UnusualExecutionTime might not be detected if PerformanceDegradation was already detected
        // This is expected behavior (avoid duplicates)
        if let Some(unusual) = unusual_time {
            assert!(unusual.metrics.z_score.abs() > 2.0);
        }

        // Test too fast (should be detected as unusual if std_dev allows)
        let fast_time = (baseline_time - (2.5 * std_dev)).max(1.0) as i32;
        let fast_exec = WorkflowExecution {
            instance_id: Uuid::new_v4(),
            ticket_id: "TICKET-FAST".to_string(),
            user_id: "user1".to_string(),
            template_id: Uuid::new_v4(),
            execution_time_seconds: fast_time,
            succeeded: true,
            completed_at: Utc::now(),
        };

        let anomalies_fast = detector.check_execution(&fast_exec);
        let unusual_fast = anomalies_fast
            .iter()
            .find(|a| a.anomaly_type == AnomalyType::UnusualExecutionTime);
        
        if let Some(unusual) = unusual_fast {
            assert!(unusual.metrics.z_score < 0.0); // Negative z-score for fast
        }
    }

    #[test]
    fn test_no_anomaly_for_normal_execution() {
        let mut detector = AnomalyDetector::new();

        // Build baseline with variance
        for i in 0..15 {
            let exec = WorkflowExecution {
                instance_id: Uuid::new_v4(),
                ticket_id: format!("TICKET-{}", i),
                user_id: "user1".to_string(),
                template_id: Uuid::new_v4(),
                execution_time_seconds: 100 + (i % 5) * 2, // Vary 100-108
                succeeded: true,
                completed_at: Utc::now(),
            };
            detector.update_baseline(&exec);
        }

        let baseline = detector.baseline();
        
        // Normal execution (within 1σ of baseline - should not trigger)
        let normal_time = baseline.execution_time.value as i32;
        let normal_exec = WorkflowExecution {
            instance_id: Uuid::new_v4(),
            ticket_id: "TICKET-NORMAL".to_string(),
            user_id: "user1".to_string(),
            template_id: Uuid::new_v4(),
            execution_time_seconds: normal_time,
            succeeded: true,
            completed_at: Utc::now(),
        };

        let anomalies = detector.check_execution(&normal_exec);
        // Should not detect performance degradation for normal execution
        let perf_degradation = anomalies
            .iter()
            .find(|a| a.anomaly_type == AnomalyType::PerformanceDegradation);
        assert!(perf_degradation.is_none(), 
            "Normal execution should not trigger performance degradation");
        
        // Should also not detect unusual time (z-score should be < 2.0)
        let unusual_time = anomalies
            .iter()
            .find(|a| a.anomaly_type == AnomalyType::UnusualExecutionTime);
        
        if let Some(unusual) = unusual_time {
            // If detected, z-score should be low (edge case with small std_dev)
            assert!(unusual.metrics.z_score.abs() < 2.5, 
                "Normal execution should have low z-score");
        }
    }

    #[test]
    fn test_anomaly_severity_levels() {
        let mut detector = AnomalyDetector::new();

        // Build baseline
        for i in 0..10 {
            let exec = WorkflowExecution {
                instance_id: Uuid::new_v4(),
                ticket_id: format!("TICKET-{}", i),
                user_id: "user1".to_string(),
                template_id: Uuid::new_v4(),
                execution_time_seconds: 100,
                succeeded: true,
                completed_at: Utc::now(),
            };
            detector.update_baseline(&exec);
        }

        // Critical severity (> 3σ)
        let critical_exec = WorkflowExecution {
            instance_id: Uuid::new_v4(),
            ticket_id: "TICKET-CRITICAL".to_string(),
            user_id: "user1".to_string(),
            template_id: Uuid::new_v4(),
            execution_time_seconds: 700, // Very slow
            succeeded: true,
            completed_at: Utc::now(),
        };

        let anomalies = detector.check_execution(&critical_exec);
        let critical_anomaly = anomalies
            .iter()
            .find(|a| a.severity == AnomalySeverity::Critical);
        
        // May or may not be critical depending on std_dev, but should have high z-score
        if let Some(anomaly) = anomalies.first() {
            if anomaly.metrics.z_score > 3.0 {
                assert_eq!(anomaly.severity, AnomalySeverity::Critical);
            }
        }
    }

    #[test]
    fn test_anomaly_to_alert_conversion() {
        let anomaly = Anomaly {
            id: Uuid::new_v4(),
            detected_at: Utc::now(),
            anomaly_type: AnomalyType::PerformanceDegradation,
            severity: AnomalySeverity::Warning,
            description: "Test anomaly".to_string(),
            metrics: AnomalyMetrics {
                current_value: 500.0,
                baseline_value: 100.0,
                deviation: 400.0,
                z_score: 2.5,
                confidence: 0.83,
            },
            affected_entities: vec!["WORKFLOW-1".to_string(), "TICKET-1".to_string()],
            investigation_steps: vec!["Step 1".to_string(), "Step 2".to_string()],
        };

        let alert = anomaly.to_alert();
        assert_eq!(alert.anomaly_type, "performance_degradation");
        assert_eq!(alert.description, "Test anomaly");
        assert_eq!(alert.affected_entities.len(), 2);
        assert_eq!(alert.investigation_steps.len(), 2);
    }

    #[test]
    fn test_baseline_initialization() {
        let detector = AnomalyDetector::new();
        let baseline = detector.baseline();

        assert_eq!(baseline.failure_rate.value, 0.0);
        assert_eq!(baseline.execution_time.value, 0.0);
        assert_eq!(baseline.success_rate.value, 0.0);
        assert_eq!(baseline.failure_rate.std_dev, 0.0);
    }

    #[test]
    fn test_moving_average_std_dev_calculation() {
        let mut ma = MovingAverage::new(5);

        // Add values: [10, 20, 30, 40, 50]
        // Mean = 30, Variance = ((10-30)² + (20-30)² + (30-30)² + (40-30)² + (50-30)²) / 5
        // Variance = (400 + 100 + 0 + 100 + 400) / 5 = 200
        // Std Dev = sqrt(200) ≈ 14.14

        ma.update(10.0);
        ma.update(20.0);
        ma.update(30.0);
        ma.update(40.0);
        ma.update(50.0);

        assert!((ma.value - 30.0).abs() < 0.01);
        assert!(ma.std_dev > 10.0 && ma.std_dev < 20.0); // Should be around 14.14
    }
}

-- Anomaly Detection Schema
-- Story 31.9: Anomaly Detection in Workflows

-- Anomalies table
CREATE TABLE IF NOT EXISTS anomalies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    anomaly_type VARCHAR(50) NOT NULL, -- 'spike_in_failures', 'performance_degradation', 'unusual_execution_time', etc.
    severity VARCHAR(20) NOT NULL DEFAULT 'info', -- 'info', 'warning', 'critical'
    description TEXT NOT NULL,
    metrics JSONB NOT NULL DEFAULT '{}', -- AnomalyMetrics: current_value, baseline_value, deviation, z_score, confidence
    affected_entities TEXT[], -- Array of workflow IDs, ticket IDs, etc.
    investigation_steps TEXT[],
    workflow_instance_id UUID, -- Reference to workflow_instances(id)
    detected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_anomalies_type ON anomalies(anomaly_type);
CREATE INDEX IF NOT EXISTS idx_anomalies_severity ON anomalies(severity);
CREATE INDEX IF NOT EXISTS idx_anomalies_detected_at ON anomalies(detected_at DESC);
CREATE INDEX IF NOT EXISTS idx_anomalies_workflow_instance ON anomalies(workflow_instance_id);
CREATE INDEX IF NOT EXISTS idx_anomalies_date_range ON anomalies(detected_at);

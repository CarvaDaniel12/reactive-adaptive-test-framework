-- Integration Health Schema
-- Story 22.1: Integration Health Database Schema

-- Integration health table (denormalized for fast reads - current status)
CREATE TABLE IF NOT EXISTS integration_health (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    integration_id VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL, -- 'healthy', 'warning', 'critical'
    pricing_sync_status VARCHAR(20), -- 'ok', 'warning', 'error'
    fees_sync_status VARCHAR(20), -- 'ok', 'warning', 'error'
    booking_loss_rate DECIMAL(5,4), -- 0.0000 to 1.0000
    error_rate DECIMAL(5,4), -- 0.0000 to 1.0000
    last_checked TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(integration_id, last_checked)
);

-- Integration events table (normalized for query flexibility - historical events)
CREATE TABLE IF NOT EXISTS integration_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    integration_id VARCHAR(50) NOT NULL,
    event_type VARCHAR(50) NOT NULL, -- 'pricing_sync_error', 'fee_sync_error', 'booking_loss'
    severity VARCHAR(20) NOT NULL, -- 'low', 'medium', 'high', 'critical'
    message TEXT,
    metadata JSONB,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for integration_health table
CREATE INDEX IF NOT EXISTS idx_integration_health_integration_id ON integration_health(integration_id);
CREATE INDEX IF NOT EXISTS idx_integration_health_last_checked ON integration_health(last_checked DESC);
CREATE INDEX IF NOT EXISTS idx_integration_health_id_checked ON integration_health(integration_id, last_checked DESC);

-- Indexes for integration_events table
CREATE INDEX IF NOT EXISTS idx_integration_events_integration_id ON integration_events(integration_id);
CREATE INDEX IF NOT EXISTS idx_integration_events_occurred_at ON integration_events(occurred_at DESC);
CREATE INDEX IF NOT EXISTS idx_integration_events_event_type ON integration_events(event_type);
CREATE INDEX IF NOT EXISTS idx_integration_events_severity ON integration_events(severity);

-- CHECK constraints for data validation
ALTER TABLE integration_health
    ADD CONSTRAINT chk_status CHECK (status IN ('healthy', 'warning', 'critical'));

ALTER TABLE integration_health
    ADD CONSTRAINT chk_pricing_sync_status CHECK (pricing_sync_status IN ('ok', 'warning', 'error') OR pricing_sync_status IS NULL);

ALTER TABLE integration_health
    ADD CONSTRAINT chk_fees_sync_status CHECK (fees_sync_status IN ('ok', 'warning', 'error') OR fees_sync_status IS NULL);

ALTER TABLE integration_health
    ADD CONSTRAINT chk_booking_loss_rate CHECK (booking_loss_rate >= 0.0 AND booking_loss_rate <= 1.0 OR booking_loss_rate IS NULL);

ALTER TABLE integration_health
    ADD CONSTRAINT chk_error_rate CHECK (error_rate >= 0.0 AND error_rate <= 1.0 OR error_rate IS NULL);

ALTER TABLE integration_events
    ADD CONSTRAINT chk_severity CHECK (severity IN ('low', 'medium', 'high', 'critical'));

-- Trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

DROP TRIGGER IF EXISTS update_integration_health_updated_at ON integration_health;
CREATE TRIGGER update_integration_health_updated_at
    BEFORE UPDATE ON integration_health
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Comments for documentation
COMMENT ON TABLE integration_health IS 'Current integration health status (denormalized for fast reads)';
COMMENT ON TABLE integration_events IS 'Historical integration events (normalized for query flexibility)';
COMMENT ON COLUMN integration_health.booking_loss_rate IS 'Booking loss rate (0.0000 to 1.0000)';
COMMENT ON COLUMN integration_health.error_rate IS 'Error rate (0.0000 to 1.0000)';
COMMENT ON COLUMN integration_events.metadata IS 'Additional event data (flexible JSONB structure)';
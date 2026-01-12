-- Fix Integration Health Constraints and Triggers
-- Story 22.1: Code Review Fixes
-- Adds missing CHECK constraints, triggers, indexes, and comments

-- Additional indexes for performance
CREATE INDEX IF NOT EXISTS idx_integration_health_id_checked ON integration_health(integration_id, last_checked DESC);
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

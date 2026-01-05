-- Pattern Detection Schema
-- Epic 9: Pattern Detection & Proactive Alerts

-- Detected patterns table
CREATE TABLE IF NOT EXISTS detected_patterns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pattern_type VARCHAR(50) NOT NULL, -- 'time_excess', 'consecutive_problem', 'spike'
    severity VARCHAR(20) NOT NULL DEFAULT 'info', -- 'info', 'warning', 'critical'
    title VARCHAR(255) NOT NULL,
    description TEXT,
    affected_tickets TEXT[], -- Array of ticket keys
    common_factor VARCHAR(255), -- e.g., component name, step name
    average_excess_percent FLOAT, -- For time_excess patterns
    confidence_score FLOAT DEFAULT 0.0, -- 0.0 to 1.0
    suggested_actions TEXT[],
    metadata JSONB DEFAULT '{}',
    detected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Alerts table
CREATE TABLE IF NOT EXISTS alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pattern_id UUID REFERENCES detected_patterns(id) ON DELETE CASCADE,
    alert_type VARCHAR(50) NOT NULL, -- 'time_excess', 'consecutive_problem', 'spike'
    severity VARCHAR(20) NOT NULL DEFAULT 'info',
    title VARCHAR(255) NOT NULL,
    message TEXT,
    affected_tickets TEXT[],
    suggested_actions TEXT[],
    is_read BOOLEAN DEFAULT FALSE,
    is_dismissed BOOLEAN DEFAULT FALSE,
    dismissed_at TIMESTAMPTZ,
    dismissed_by VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Pattern resolution tracking
CREATE TABLE IF NOT EXISTS pattern_resolutions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pattern_id UUID REFERENCES detected_patterns(id) ON DELETE CASCADE,
    resolution_status VARCHAR(50) NOT NULL, -- 'addressed', 'ignored', 'recurring'
    resolution_notes TEXT,
    resolved_by VARCHAR(255),
    resolved_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_patterns_type ON detected_patterns(pattern_type);
CREATE INDEX IF NOT EXISTS idx_patterns_severity ON detected_patterns(severity);
CREATE INDEX IF NOT EXISTS idx_patterns_detected_at ON detected_patterns(detected_at DESC);
CREATE INDEX IF NOT EXISTS idx_alerts_read ON alerts(is_read) WHERE NOT is_read;
CREATE INDEX IF NOT EXISTS idx_alerts_dismissed ON alerts(is_dismissed) WHERE NOT is_dismissed;
CREATE INDEX IF NOT EXISTS idx_alerts_created_at ON alerts(created_at DESC);

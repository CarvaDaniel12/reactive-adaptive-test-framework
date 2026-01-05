-- Time Aggregates Schema Migration
-- Story 6.7: Historical Time Data Storage
-- Aggregated time data for dashboard analytics and trends

-- Daily time aggregates per user
-- Stores pre-computed daily metrics for fast dashboard queries
CREATE TABLE time_daily_aggregates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    aggregate_date DATE NOT NULL,
    -- Ticket metrics
    tickets_completed INT NOT NULL DEFAULT 0,
    -- Time metrics (in seconds)
    total_time_seconds INT NOT NULL DEFAULT 0,
    total_estimated_seconds INT NOT NULL DEFAULT 0,
    -- Derived metrics
    avg_time_per_ticket_seconds INT NOT NULL DEFAULT 0,
    efficiency_ratio DECIMAL(5,4) NOT NULL DEFAULT 1.0, -- actual/estimated ratio
    -- Breakdown by ticket type
    bug_tickets INT NOT NULL DEFAULT 0,
    bug_time_seconds INT NOT NULL DEFAULT 0,
    feature_tickets INT NOT NULL DEFAULT 0,
    feature_time_seconds INT NOT NULL DEFAULT 0,
    regression_tickets INT NOT NULL DEFAULT 0,
    regression_time_seconds INT NOT NULL DEFAULT 0,
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, aggregate_date)
);

CREATE INDEX idx_time_daily_user ON time_daily_aggregates (user_id);
CREATE INDEX idx_time_daily_date ON time_daily_aggregates (aggregate_date);
CREATE INDEX idx_time_daily_user_date ON time_daily_aggregates (user_id, aggregate_date DESC);

-- Step-level averages per template
-- Tracks historical averages for time estimates refinement
CREATE TABLE time_step_averages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    template_id UUID NOT NULL REFERENCES workflow_templates(id) ON DELETE CASCADE,
    step_index INT NOT NULL,
    -- Aggregated stats
    sample_count INT NOT NULL DEFAULT 0,
    total_seconds INT NOT NULL DEFAULT 0,
    avg_seconds INT NOT NULL DEFAULT 0,
    min_seconds INT NOT NULL DEFAULT 0,
    max_seconds INT NOT NULL DEFAULT 0,
    std_dev_seconds DECIMAL(10,2) NOT NULL DEFAULT 0,
    -- Last updated
    last_sample_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (template_id, step_index)
);

CREATE INDEX idx_time_step_avg_template ON time_step_averages (template_id);

-- User historical averages
-- Per-user averages by ticket type for personalized estimates
CREATE TABLE time_user_averages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    ticket_type VARCHAR(50) NOT NULL, -- 'bug', 'feature', 'regression'
    -- Aggregated stats
    sample_count INT NOT NULL DEFAULT 0,
    total_seconds INT NOT NULL DEFAULT 0,
    avg_seconds INT NOT NULL DEFAULT 0,
    min_seconds INT NOT NULL DEFAULT 0,
    max_seconds INT NOT NULL DEFAULT 0,
    -- Rolling averages (last 30 days)
    rolling_avg_seconds INT NOT NULL DEFAULT 0,
    rolling_sample_count INT NOT NULL DEFAULT 0,
    -- Timestamps
    last_sample_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, ticket_type)
);

CREATE INDEX idx_time_user_avg_user ON time_user_averages (user_id);
CREATE INDEX idx_time_user_avg_type ON time_user_averages (ticket_type);

-- Time gap alerts history
-- Records when actual time exceeds estimates significantly
CREATE TABLE time_gap_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_instance_id UUID NOT NULL REFERENCES workflow_instances(id) ON DELETE CASCADE,
    step_index INT,
    user_id UUID NOT NULL,
    -- Gap details
    actual_seconds INT NOT NULL,
    estimated_seconds INT NOT NULL,
    gap_percentage DECIMAL(5,2) NOT NULL, -- e.g., 150.00 means 50% over
    -- Alert metadata
    alert_type VARCHAR(20) NOT NULL, -- 'step_excess', 'workflow_excess'
    dismissed BOOLEAN NOT NULL DEFAULT false,
    dismissed_at TIMESTAMPTZ,
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_time_gap_alerts_user ON time_gap_alerts (user_id);
CREATE INDEX idx_time_gap_alerts_workflow ON time_gap_alerts (workflow_instance_id);
CREATE INDEX idx_time_gap_alerts_created ON time_gap_alerts (created_at DESC);
CREATE INDEX idx_time_gap_alerts_undismissed ON time_gap_alerts (user_id, dismissed) WHERE dismissed = false;

-- Function to update daily aggregates when a workflow completes
-- Called from application code after workflow completion
CREATE OR REPLACE FUNCTION update_daily_aggregate(
    p_user_id UUID,
    p_date DATE,
    p_ticket_type VARCHAR(50),
    p_actual_seconds INT,
    p_estimated_seconds INT
) RETURNS VOID AS $$
DECLARE
    v_efficiency DECIMAL(5,4);
BEGIN
    -- Calculate efficiency (capped at 2.0 to avoid outliers)
    IF p_estimated_seconds > 0 THEN
        v_efficiency := LEAST(p_actual_seconds::DECIMAL / p_estimated_seconds, 2.0);
    ELSE
        v_efficiency := 1.0;
    END IF;

    INSERT INTO time_daily_aggregates (
        user_id, aggregate_date, tickets_completed, 
        total_time_seconds, total_estimated_seconds,
        avg_time_per_ticket_seconds, efficiency_ratio,
        bug_tickets, bug_time_seconds,
        feature_tickets, feature_time_seconds,
        regression_tickets, regression_time_seconds
    ) VALUES (
        p_user_id, p_date, 1,
        p_actual_seconds, p_estimated_seconds,
        p_actual_seconds, v_efficiency,
        CASE WHEN p_ticket_type = 'bug' THEN 1 ELSE 0 END,
        CASE WHEN p_ticket_type = 'bug' THEN p_actual_seconds ELSE 0 END,
        CASE WHEN p_ticket_type = 'feature' THEN 1 ELSE 0 END,
        CASE WHEN p_ticket_type = 'feature' THEN p_actual_seconds ELSE 0 END,
        CASE WHEN p_ticket_type = 'regression' THEN 1 ELSE 0 END,
        CASE WHEN p_ticket_type = 'regression' THEN p_actual_seconds ELSE 0 END
    )
    ON CONFLICT (user_id, aggregate_date) DO UPDATE SET
        tickets_completed = time_daily_aggregates.tickets_completed + 1,
        total_time_seconds = time_daily_aggregates.total_time_seconds + p_actual_seconds,
        total_estimated_seconds = time_daily_aggregates.total_estimated_seconds + p_estimated_seconds,
        avg_time_per_ticket_seconds = (time_daily_aggregates.total_time_seconds + p_actual_seconds) / (time_daily_aggregates.tickets_completed + 1),
        efficiency_ratio = CASE 
            WHEN time_daily_aggregates.total_estimated_seconds + p_estimated_seconds > 0 
            THEN LEAST((time_daily_aggregates.total_time_seconds + p_actual_seconds)::DECIMAL / (time_daily_aggregates.total_estimated_seconds + p_estimated_seconds), 2.0)
            ELSE 1.0 
        END,
        bug_tickets = time_daily_aggregates.bug_tickets + CASE WHEN p_ticket_type = 'bug' THEN 1 ELSE 0 END,
        bug_time_seconds = time_daily_aggregates.bug_time_seconds + CASE WHEN p_ticket_type = 'bug' THEN p_actual_seconds ELSE 0 END,
        feature_tickets = time_daily_aggregates.feature_tickets + CASE WHEN p_ticket_type = 'feature' THEN 1 ELSE 0 END,
        feature_time_seconds = time_daily_aggregates.feature_time_seconds + CASE WHEN p_ticket_type = 'feature' THEN p_actual_seconds ELSE 0 END,
        regression_tickets = time_daily_aggregates.regression_tickets + CASE WHEN p_ticket_type = 'regression' THEN 1 ELSE 0 END,
        regression_time_seconds = time_daily_aggregates.regression_time_seconds + CASE WHEN p_ticket_type = 'regression' THEN p_actual_seconds ELSE 0 END,
        updated_at = NOW();
END;
$$ LANGUAGE plpgsql;

-- Function to update step averages
CREATE OR REPLACE FUNCTION update_step_average(
    p_template_id UUID,
    p_step_index INT,
    p_actual_seconds INT
) RETURNS VOID AS $$
BEGIN
    INSERT INTO time_step_averages (
        template_id, step_index, sample_count, total_seconds,
        avg_seconds, min_seconds, max_seconds, last_sample_at
    ) VALUES (
        p_template_id, p_step_index, 1, p_actual_seconds,
        p_actual_seconds, p_actual_seconds, p_actual_seconds, NOW()
    )
    ON CONFLICT (template_id, step_index) DO UPDATE SET
        sample_count = time_step_averages.sample_count + 1,
        total_seconds = time_step_averages.total_seconds + p_actual_seconds,
        avg_seconds = (time_step_averages.total_seconds + p_actual_seconds) / (time_step_averages.sample_count + 1),
        min_seconds = LEAST(time_step_averages.min_seconds, p_actual_seconds),
        max_seconds = GREATEST(time_step_averages.max_seconds, p_actual_seconds),
        last_sample_at = NOW(),
        updated_at = NOW();
END;
$$ LANGUAGE plpgsql;

-- Function to update user averages
CREATE OR REPLACE FUNCTION update_user_average(
    p_user_id UUID,
    p_ticket_type VARCHAR(50),
    p_actual_seconds INT
) RETURNS VOID AS $$
BEGIN
    INSERT INTO time_user_averages (
        user_id, ticket_type, sample_count, total_seconds,
        avg_seconds, min_seconds, max_seconds,
        rolling_avg_seconds, rolling_sample_count, last_sample_at
    ) VALUES (
        p_user_id, p_ticket_type, 1, p_actual_seconds,
        p_actual_seconds, p_actual_seconds, p_actual_seconds,
        p_actual_seconds, 1, NOW()
    )
    ON CONFLICT (user_id, ticket_type) DO UPDATE SET
        sample_count = time_user_averages.sample_count + 1,
        total_seconds = time_user_averages.total_seconds + p_actual_seconds,
        avg_seconds = (time_user_averages.total_seconds + p_actual_seconds) / (time_user_averages.sample_count + 1),
        min_seconds = LEAST(time_user_averages.min_seconds, p_actual_seconds),
        max_seconds = GREATEST(time_user_averages.max_seconds, p_actual_seconds),
        rolling_sample_count = time_user_averages.rolling_sample_count + 1,
        rolling_avg_seconds = (time_user_averages.rolling_avg_seconds * time_user_averages.rolling_sample_count + p_actual_seconds) / (time_user_averages.rolling_sample_count + 1),
        last_sample_at = NOW(),
        updated_at = NOW();
END;
$$ LANGUAGE plpgsql;

-- Cleanup job: Remove aggregates older than 365 days (keep 1 year for trends)
-- Run this periodically via application scheduler
CREATE OR REPLACE FUNCTION cleanup_old_aggregates() RETURNS VOID AS $$
BEGIN
    DELETE FROM time_daily_aggregates WHERE aggregate_date < CURRENT_DATE - INTERVAL '365 days';
    DELETE FROM time_gap_alerts WHERE created_at < CURRENT_DATE - INTERVAL '30 days';
END;
$$ LANGUAGE plpgsql;

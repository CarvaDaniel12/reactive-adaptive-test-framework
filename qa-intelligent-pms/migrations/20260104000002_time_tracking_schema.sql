-- Time Tracking Schema Migration
-- Creates tables for tracking time spent on workflow steps

-- Time sessions table
CREATE TABLE time_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_instance_id UUID NOT NULL REFERENCES workflow_instances(id) ON DELETE CASCADE,
    step_index INT NOT NULL,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    paused_at TIMESTAMPTZ,
    resumed_at TIMESTAMPTZ,
    ended_at TIMESTAMPTZ,
    total_seconds INT NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for efficient querying
CREATE INDEX idx_time_sessions_workflow ON time_sessions (workflow_instance_id);
CREATE INDEX idx_time_sessions_active ON time_sessions (is_active) WHERE is_active = true;
CREATE UNIQUE INDEX idx_time_sessions_workflow_step ON time_sessions (workflow_instance_id, step_index);

-- Time estimates table (for comparing actual vs expected)
CREATE TABLE time_estimates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    template_id UUID NOT NULL REFERENCES workflow_templates(id) ON DELETE CASCADE,
    step_index INT NOT NULL,
    estimated_seconds INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (template_id, step_index)
);

CREATE INDEX idx_time_estimates_template ON time_estimates (template_id);

-- Pause events for detailed tracking
CREATE TABLE time_pause_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id UUID NOT NULL REFERENCES time_sessions(id) ON DELETE CASCADE,
    paused_at TIMESTAMPTZ NOT NULL,
    resumed_at TIMESTAMPTZ,
    duration_seconds INT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_time_pause_events_session ON time_pause_events (session_id);

-- Reports Schema Migration
-- Creates tables for workflow reports

CREATE TABLE workflow_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_instance_id UUID NOT NULL REFERENCES workflow_instances(id) ON DELETE CASCADE,
    ticket_id VARCHAR(255) NOT NULL,
    ticket_title TEXT,
    template_name VARCHAR(255) NOT NULL,
    content JSONB NOT NULL,
    total_time_seconds INT NOT NULL DEFAULT 0,
    generated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_workflow_reports_workflow ON workflow_reports (workflow_instance_id);
CREATE INDEX idx_workflow_reports_ticket ON workflow_reports (ticket_id);
CREATE INDEX idx_workflow_reports_date ON workflow_reports (generated_at);

-- Workflow Schema Migration
-- Creates tables for workflow templates, instances, and step results

-- Workflow Templates
-- Stores reusable workflow definitions
CREATE TABLE workflow_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    ticket_type VARCHAR(50) NOT NULL, -- 'bug', 'feature', 'regression', 'custom'
    steps_json JSONB NOT NULL, -- Array of WorkflowStep
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for quick template lookups
CREATE INDEX idx_workflow_templates_ticket_type ON workflow_templates(ticket_type);
CREATE INDEX idx_workflow_templates_is_default ON workflow_templates(is_default);

-- Workflow Instances
-- Tracks active/completed workflows for tickets
CREATE TABLE workflow_instances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    template_id UUID NOT NULL REFERENCES workflow_templates(id),
    ticket_id VARCHAR(50) NOT NULL, -- Jira ticket key (e.g., "PROJ-123")
    user_id VARCHAR(100) NOT NULL, -- User email or ID
    status VARCHAR(20) NOT NULL DEFAULT 'active', -- 'active', 'paused', 'completed', 'cancelled'
    current_step INTEGER NOT NULL DEFAULT 0,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    paused_at TIMESTAMPTZ,
    resumed_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for finding workflows by ticket
CREATE INDEX idx_workflow_instances_ticket_id ON workflow_instances(ticket_id);
CREATE INDEX idx_workflow_instances_user_id ON workflow_instances(user_id);
CREATE INDEX idx_workflow_instances_status ON workflow_instances(status);
CREATE INDEX idx_workflow_instances_template_id ON workflow_instances(template_id);

-- Workflow Step Results
-- Stores completion data for each step
CREATE TABLE workflow_step_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    instance_id UUID NOT NULL REFERENCES workflow_instances(id) ON DELETE CASCADE,
    step_index INTEGER NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- 'pending', 'in_progress', 'completed', 'skipped'
    notes TEXT,
    links JSONB, -- Array of {title, url}
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(instance_id, step_index)
);

-- Index for step lookups
CREATE INDEX idx_workflow_step_results_instance_id ON workflow_step_results(instance_id);

-- Trigger to update updated_at
CREATE OR REPLACE FUNCTION update_workflow_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_workflow_templates_updated_at
    BEFORE UPDATE ON workflow_templates
    FOR EACH ROW EXECUTE FUNCTION update_workflow_updated_at_column();

CREATE TRIGGER update_workflow_instances_updated_at
    BEFORE UPDATE ON workflow_instances
    FOR EACH ROW EXECUTE FUNCTION update_workflow_updated_at_column();

CREATE TRIGGER update_workflow_step_results_updated_at
    BEFORE UPDATE ON workflow_step_results
    FOR EACH ROW EXECUTE FUNCTION update_workflow_updated_at_column();

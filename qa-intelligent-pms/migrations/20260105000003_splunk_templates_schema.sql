-- Epic 11: Splunk Query Templates Schema
-- Stores user-defined and system query templates for Splunk

-- Create splunk_query_templates table
CREATE TABLE IF NOT EXISTS splunk_query_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    query TEXT NOT NULL,
    category VARCHAR(50) NOT NULL DEFAULT 'custom',
    is_system BOOLEAN NOT NULL DEFAULT FALSE,
    created_by UUID, -- User identifier (no FK for now, users managed externally)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create unique constraint for system templates by name
CREATE UNIQUE INDEX IF NOT EXISTS idx_splunk_templates_system_name 
    ON splunk_query_templates (name, is_system) 
    WHERE is_system = true;

-- Create index for user templates
CREATE INDEX IF NOT EXISTS idx_splunk_templates_user 
    ON splunk_query_templates (created_by) 
    WHERE created_by IS NOT NULL;

-- Create index for category filtering
CREATE INDEX IF NOT EXISTS idx_splunk_templates_category 
    ON splunk_query_templates (category);

-- Create splunk_query_history table for tracking executed queries
CREATE TABLE IF NOT EXISTS splunk_query_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    template_id UUID REFERENCES splunk_query_templates(id) ON DELETE SET NULL,
    query TEXT NOT NULL,
    time_start TIMESTAMPTZ NOT NULL,
    time_end TIMESTAMPTZ NOT NULL,
    index_name VARCHAR(255),
    execution_time_ms INTEGER,
    result_count INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index for user query history
CREATE INDEX IF NOT EXISTS idx_splunk_history_user 
    ON splunk_query_history (user_id, created_at DESC);

-- Add comment for documentation
COMMENT ON TABLE splunk_query_templates IS 'Stores Splunk SPL query templates for reuse';
COMMENT ON TABLE splunk_query_history IS 'Tracks executed Splunk queries for history and analytics';
COMMENT ON COLUMN splunk_query_templates.query IS 'SPL query with placeholders like {TICKET_KEY}, {USER_ID}';
COMMENT ON COLUMN splunk_query_templates.is_system IS 'System templates are read-only and seeded on startup';

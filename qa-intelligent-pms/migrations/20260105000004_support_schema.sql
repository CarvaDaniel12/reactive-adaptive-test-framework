-- Support Portal Schema
-- Epic 12: Support Portal & Troubleshooting

-- Custom types for support module
DO $$ BEGIN
    CREATE TYPE error_status AS ENUM ('new', 'investigating', 'resolved', 'dismissed');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE error_severity AS ENUM ('low', 'medium', 'high', 'critical');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE error_source AS ENUM ('frontend', 'backend', 'integration', 'database', 'unknown');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Error logs table
CREATE TABLE IF NOT EXISTS error_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    message TEXT NOT NULL,
    stack_trace TEXT,
    severity error_severity NOT NULL DEFAULT 'medium',
    source error_source NOT NULL DEFAULT 'unknown',
    status error_status NOT NULL DEFAULT 'new',
    user_id UUID,
    session_id VARCHAR(255),
    page_url VARCHAR(500),
    action VARCHAR(255),
    browser_info VARCHAR(500),
    device_info VARCHAR(500),
    context JSONB NOT NULL DEFAULT '{}'::jsonb,
    occurrence_count INTEGER NOT NULL DEFAULT 1,
    first_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolution_notes TEXT,
    kb_entry_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Knowledge base entries table
CREATE TABLE IF NOT EXISTS knowledge_base_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    problem TEXT NOT NULL,
    cause TEXT NOT NULL,
    solution TEXT NOT NULL,
    related_errors JSONB NOT NULL DEFAULT '[]'::jsonb,
    tags JSONB NOT NULL DEFAULT '[]'::jsonb,
    view_count INTEGER NOT NULL DEFAULT 0,
    helpful_count INTEGER NOT NULL DEFAULT 0,
    not_helpful_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for error_logs
CREATE INDEX IF NOT EXISTS idx_error_logs_status ON error_logs(status);
CREATE INDEX IF NOT EXISTS idx_error_logs_severity ON error_logs(severity);
CREATE INDEX IF NOT EXISTS idx_error_logs_source ON error_logs(source);
CREATE INDEX IF NOT EXISTS idx_error_logs_user_id ON error_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_error_logs_last_seen ON error_logs(last_seen_at DESC);
CREATE INDEX IF NOT EXISTS idx_error_logs_message ON error_logs USING gin(to_tsvector('english', message));

-- Indexes for knowledge_base_entries
CREATE INDEX IF NOT EXISTS idx_kb_entries_title ON knowledge_base_entries USING gin(to_tsvector('english', title));
CREATE INDEX IF NOT EXISTS idx_kb_entries_problem ON knowledge_base_entries USING gin(to_tsvector('english', problem));
CREATE INDEX IF NOT EXISTS idx_kb_entries_view_count ON knowledge_base_entries(view_count DESC);
CREATE INDEX IF NOT EXISTS idx_kb_entries_tags ON knowledge_base_entries USING gin(tags);

-- Foreign key constraint (optional, as kb_entry_id can be null)
ALTER TABLE error_logs 
    DROP CONSTRAINT IF EXISTS fk_error_logs_kb_entry;
ALTER TABLE error_logs 
    ADD CONSTRAINT fk_error_logs_kb_entry 
    FOREIGN KEY (kb_entry_id) 
    REFERENCES knowledge_base_entries(id) 
    ON DELETE SET NULL;

-- Trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

DROP TRIGGER IF EXISTS update_error_logs_updated_at ON error_logs;
CREATE TRIGGER update_error_logs_updated_at
    BEFORE UPDATE ON error_logs
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_kb_entries_updated_at ON knowledge_base_entries;
CREATE TRIGGER update_kb_entries_updated_at
    BEFORE UPDATE ON knowledge_base_entries
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Comments
COMMENT ON TABLE error_logs IS 'Captured application errors for support troubleshooting';
COMMENT ON TABLE knowledge_base_entries IS 'Knowledge base of common issues and solutions';
COMMENT ON COLUMN error_logs.occurrence_count IS 'Number of times this error has occurred';
COMMENT ON COLUMN error_logs.kb_entry_id IS 'Link to knowledge base entry for resolution';
COMMENT ON COLUMN knowledge_base_entries.related_errors IS 'Error message patterns that match this entry';

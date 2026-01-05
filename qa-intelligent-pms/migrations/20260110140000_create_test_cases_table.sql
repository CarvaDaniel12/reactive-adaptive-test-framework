-- Test Cases Schema
-- Story 31.1: Auto-Test Generation from Tickets - Task 3.3

-- Test cases table
CREATE TABLE IF NOT EXISTS test_cases (
    id VARCHAR(255) PRIMARY KEY, -- TestCaseId (String), can be Testmo ID or internal ID
    title VARCHAR(500) NOT NULL,
    description TEXT NOT NULL,
    preconditions TEXT[] DEFAULT '{}', -- Array of precondition strings
    priority VARCHAR(10) NOT NULL, -- 'p0', 'p1', 'p2', 'p3'
    test_type VARCHAR(50) NOT NULL, -- 'API', 'Integration', 'UI', 'Stress'
    steps TEXT[] NOT NULL, -- Array of step strings
    expected_result TEXT NOT NULL,
    automatizable BOOLEAN NOT NULL DEFAULT TRUE,
    component VARCHAR(255) NOT NULL,
    endpoint VARCHAR(500), -- Optional endpoint
    method VARCHAR(20), -- Optional HTTP method (GET, POST, etc.)
    ticket_key VARCHAR(255), -- Optional ticket key (TicketId)
    repository VARCHAR(100) NOT NULL, -- 'Base', 'Reativo', or 'Sprint-{ID}'
    folder_path TEXT[] DEFAULT '{}', -- Array of folder path segments
    base_case_id BIGINT, -- Optional ID of base case if inherited
    tags TEXT[] DEFAULT '{}', -- Array of tags
    status VARCHAR(20) NOT NULL DEFAULT 'draft', -- 'draft', 'active', 'archived', 'deprecated'
    created_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_executed TIMESTAMPTZ, -- Optional last execution timestamp
    execution_count INTEGER NOT NULL DEFAULT 0,
    success_rate DOUBLE PRECISION NOT NULL DEFAULT 0.0 -- 0.0 to 1.0
);

-- Indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_test_cases_priority ON test_cases(priority);
CREATE INDEX IF NOT EXISTS idx_test_cases_test_type ON test_cases(test_type);
CREATE INDEX IF NOT EXISTS idx_test_cases_status ON test_cases(status);
CREATE INDEX IF NOT EXISTS idx_test_cases_component ON test_cases(component);
CREATE INDEX IF NOT EXISTS idx_test_cases_ticket_key ON test_cases(ticket_key) WHERE ticket_key IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_test_cases_repository ON test_cases(repository);
CREATE INDEX IF NOT EXISTS idx_test_cases_created_date ON test_cases(created_date DESC);
CREATE INDEX IF NOT EXISTS idx_test_cases_updated_at ON test_cases(updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_test_cases_tags ON test_cases USING GIN(tags); -- GIN index for array searches

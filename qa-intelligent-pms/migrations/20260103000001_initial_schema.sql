-- Initial schema migration for QA Intelligent PMS
-- Creates the foundational schema_version tracking

-- Schema version tracking (created by SQLx automatically, but we document it)
-- This migration establishes the baseline for the database

-- Note: Domain-specific tables will be created in their respective epic migrations:
-- - Users table: Epic 2 (Setup Wizard)
-- - Workflow tables: Epic 5 (Workflow Engine)
-- - Time tracking tables: Epic 6 (Time Tracking)

-- For now, create a simple health check table
CREATE TABLE IF NOT EXISTS health_check (
    id SERIAL PRIMARY KEY,
    checked_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Insert initial record
INSERT INTO health_check (checked_at) VALUES (NOW());

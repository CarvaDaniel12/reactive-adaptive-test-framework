-- AI Configuration Schema
-- Epic 13: AI Companion (BYOK)

-- AI configuration table
CREATE TABLE IF NOT EXISTS ai_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID UNIQUE, -- NULL for global/default config
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    provider VARCHAR(50) NOT NULL DEFAULT 'openai',
    model_id VARCHAR(100) NOT NULL DEFAULT 'gpt-4o-mini',
    api_key_encrypted BYTEA, -- Encrypted API key
    custom_base_url VARCHAR(500),
    validated_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Chat history table (session-based, cleared on logout)
CREATE TABLE IF NOT EXISTS ai_chat_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID,
    session_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_activity_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS ai_chat_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id UUID NOT NULL REFERENCES ai_chat_sessions(id) ON DELETE CASCADE,
    role VARCHAR(20) NOT NULL, -- 'user', 'assistant', 'system'
    content TEXT NOT NULL,
    token_count INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_ai_configs_user_id ON ai_configs(user_id);
CREATE INDEX IF NOT EXISTS idx_ai_chat_sessions_user_id ON ai_chat_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_ai_chat_sessions_session_id ON ai_chat_sessions(session_id);
CREATE INDEX IF NOT EXISTS idx_ai_chat_messages_session_id ON ai_chat_messages(session_id);

-- Trigger for updated_at
DROP TRIGGER IF EXISTS update_ai_configs_updated_at ON ai_configs;
CREATE TRIGGER update_ai_configs_updated_at
    BEFORE UPDATE ON ai_configs
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Comments
COMMENT ON TABLE ai_configs IS 'User AI provider configurations (BYOK)';
COMMENT ON TABLE ai_chat_sessions IS 'Chat sessions for the mini-chatbot';
COMMENT ON TABLE ai_chat_messages IS 'Messages within chat sessions';
COMMENT ON COLUMN ai_configs.api_key_encrypted IS 'AES-GCM encrypted API key';

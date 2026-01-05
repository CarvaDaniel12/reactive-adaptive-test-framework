-- AI Config storage fixes
-- Epic 13 / Story 13.1: Make AI config storage consistent and safe
--
-- Fixes:
-- 1) user_id NULL + UNIQUE does not conflict (multiple NULL rows possible). Enforce singleton by
--    setting NULL user_id to a constant UUID and making user_id NOT NULL + DEFAULT.
-- 2) api_key_encrypted was BYTEA but the application Encryptor stores hex-encoded strings.
--    Convert column to TEXT (hex string) using encode().

-- Keep only one legacy "NULL user_id" row (newest by updated_at/created_at).
WITH ranked AS (
    SELECT
        id,
        ROW_NUMBER() OVER (
            ORDER BY updated_at DESC NULLS LAST, created_at DESC NULLS LAST, id
        ) AS rn
    FROM ai_configs
    WHERE user_id IS NULL
)
DELETE FROM ai_configs
WHERE id IN (SELECT id FROM ranked WHERE rn > 1);

-- Assign a deterministic global UUID to legacy NULL rows.
UPDATE ai_configs
SET user_id = '00000000-0000-0000-0000-000000000000'::uuid
WHERE user_id IS NULL;

-- Enforce singleton semantics at the schema level.
ALTER TABLE ai_configs
    ALTER COLUMN user_id SET DEFAULT '00000000-0000-0000-0000-000000000000'::uuid;
ALTER TABLE ai_configs
    ALTER COLUMN user_id SET NOT NULL;

-- Store encrypted API key as TEXT (hex string) to match Encryptor.encrypt() output.
ALTER TABLE ai_configs
    ALTER COLUMN api_key_encrypted TYPE TEXT
    USING CASE
        WHEN api_key_encrypted IS NULL THEN NULL
        ELSE encode(api_key_encrypted, 'hex')
    END;

COMMENT ON COLUMN ai_configs.api_key_encrypted IS 'AES-GCM encrypted API key (hex string)';


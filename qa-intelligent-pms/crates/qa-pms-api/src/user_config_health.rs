//! Health checks backed by the per-user config file (`UserConfig`).
//!
//! The setup wizard writes integration credentials to a user-scoped config file
//! (see `qa_pms_config::UserConfig::default_path`). These credentials are
//! encrypted at rest. Startup validation and periodic health checks must read
//! from this file (not from server `Settings`) to avoid using stale/incorrect keys.

use async_trait::async_trait;
use qa_pms_config::{Encryptor, JiraAuthType, Settings, UserConfig};
use qa_pms_core::health::{HealthCheck, HealthCheckResult};
use qa_pms_jira::JiraHealthCheck;
use qa_pms_jira::JiraTicketsClient;
use qa_pms_postman::PostmanHealthCheck;
use qa_pms_testmo::TestmoHealthCheck;
use secrecy::ExposeSecret;
use std::path::PathBuf;
use tracing::{debug, warn};

use crate::startup::StartupValidator;

#[derive(Debug, Clone, Copy)]
enum IntegrationKind {
    Jira,
    Postman,
    Testmo,
}

/// A health check that reloads and decrypts credentials from `UserConfig` on each run.
pub struct UserConfigHealthCheck {
    kind: IntegrationKind,
    config_path: PathBuf,
    encryptor: Encryptor,
}

impl UserConfigHealthCheck {
    fn new(kind: IntegrationKind, settings: &Settings) -> Option<Self> {
        let config_path = UserConfig::default_path().ok()?;
        let encryptor = Encryptor::from_hex_key(settings.encryption_key.expose_secret()).ok()?;

        Some(Self {
            kind,
            config_path,
            encryptor,
        })
    }

    pub fn jira(settings: &Settings) -> Option<Self> {
        Self::new(IntegrationKind::Jira, settings)
    }

    pub fn postman(settings: &Settings) -> Option<Self> {
        Self::new(IntegrationKind::Postman, settings)
    }

    pub fn testmo(settings: &Settings) -> Option<Self> {
        Self::new(IntegrationKind::Testmo, settings)
    }

    fn integration_name_static(&self) -> &'static str {
        match self.kind {
            IntegrationKind::Jira => "jira",
            IntegrationKind::Postman => "postman",
            IntegrationKind::Testmo => "testmo",
        }
    }

    fn load_config(&self) -> Option<UserConfig> {
        if !self.config_path.exists() {
            return None;
        }
        UserConfig::from_file(&self.config_path).ok()
    }
}

/// Build a `StartupValidator` from the per-user config file (`UserConfig`).
///
/// This avoids using server `Settings` for per-user API keys (Postman/Testmo),
/// preventing stale keys after the setup wizard is completed.
pub fn build_startup_validator_from_user_config(settings: &Settings) -> StartupValidator {
    let mut validator = StartupValidator::new();

    let Ok(config_path) = UserConfig::default_path() else {
        return validator;
    };
    if !config_path.exists() {
        return validator;
    }

    let Ok(cfg) = UserConfig::from_file(&config_path) else {
        return validator;
    };

    let Ok(encryptor) = Encryptor::from_hex_key(settings.encryption_key.expose_secret()) else {
        return validator;
    };

    // Jira (critical)
    if matches!(cfg.integrations.jira.auth_type, JiraAuthType::ApiToken) {
        if let (Some(email_enc), Some(token_enc)) = (
            cfg.integrations.jira.email_encrypted.as_ref(),
            cfg.integrations.jira.api_token_encrypted.as_ref(),
        ) {
            if let (Ok(email), Ok(token)) =
                (encryptor.decrypt(email_enc), encryptor.decrypt(token_enc))
            {
                let check = JiraHealthCheck::with_api_token(
                    cfg.integrations.jira.instance_url.clone(),
                    email.expose_secret().to_string(),
                    token.expose_secret().to_string(),
                );
                validator = validator.add_critical(std::sync::Arc::new(check));
            }
        }
    }

    // Postman (optional)
    if let Some(postman) = cfg.integrations.postman.as_ref() {
        if let Ok(api_key) = encryptor.decrypt(&postman.api_key_encrypted) {
            let check = PostmanHealthCheck::new(api_key.expose_secret().to_string());
            validator = validator.add_optional(std::sync::Arc::new(check));
        }
    }

    // Testmo (optional)
    if let Some(testmo) = cfg.integrations.testmo.as_ref() {
        if let Ok(api_key) = encryptor.decrypt(&testmo.api_key_encrypted) {
            let check = TestmoHealthCheck::new(
                testmo.instance_url.clone(),
                api_key.expose_secret().to_string(),
            );
            validator = validator.add_optional(std::sync::Arc::new(check));
        }
    }

    validator
}

/// Build a Jira tickets client from the per-user config file (`UserConfig`).
///
/// Returns `Ok(None)` if not configured.
pub fn jira_tickets_client_from_user_config(
    settings: &Settings,
) -> Result<Option<JiraTicketsClient>, qa_pms_core::error::ApiError> {
    let Ok(config_path) = UserConfig::default_path() else {
        return Ok(None);
    };
    if !config_path.exists() {
        return Ok(None);
    }

    let cfg =
        UserConfig::from_file(&config_path).map_err(qa_pms_core::error::ApiError::Internal)?;
    if !matches!(cfg.integrations.jira.auth_type, JiraAuthType::ApiToken) {
        return Ok(None);
    }

    let encryptor = Encryptor::from_hex_key(settings.encryption_key.expose_secret())
        .map_err(qa_pms_core::error::ApiError::Internal)?;

    let Some(email_enc) = cfg.integrations.jira.email_encrypted else {
        return Ok(None);
    };
    let Some(token_enc) = cfg.integrations.jira.api_token_encrypted else {
        return Ok(None);
    };

    let email = encryptor
        .decrypt(&email_enc)
        .map_err(qa_pms_core::error::ApiError::Internal)?
        .expose_secret()
        .to_string();
    let api_token = encryptor
        .decrypt(&token_enc)
        .map_err(qa_pms_core::error::ApiError::Internal)?
        .expose_secret()
        .to_string();

    Ok(Some(JiraTicketsClient::with_api_token(
        cfg.integrations.jira.instance_url,
        email,
        api_token,
    )))
}

#[async_trait]
impl HealthCheck for UserConfigHealthCheck {
    fn integration_name(&self) -> &'static str {
        self.integration_name_static()
    }

    async fn check(&self) -> HealthCheckResult {
        let integration = self.integration_name_static();
        debug!(integration, path = %self.config_path.display(), "Loading user config for health check");

        let Some(cfg) = self.load_config() else {
            return HealthCheckResult::offline(integration, "Not configured");
        };

        match self.kind {
            IntegrationKind::Postman => {
                let Some(postman) = cfg.integrations.postman else {
                    return HealthCheckResult::offline("postman", "Not configured");
                };

                let api_key = match self.encryptor.decrypt(&postman.api_key_encrypted) {
                    Ok(s) => s.expose_secret().to_string(),
                    Err(e) => {
                        warn!(error = %e, "Failed to decrypt Postman API key");
                        return HealthCheckResult::offline("postman", "Failed to decrypt API key");
                    }
                };

                PostmanHealthCheck::new(api_key).check().await
            }
            IntegrationKind::Testmo => {
                let Some(testmo) = cfg.integrations.testmo else {
                    return HealthCheckResult::offline("testmo", "Not configured");
                };

                let api_key = match self.encryptor.decrypt(&testmo.api_key_encrypted) {
                    Ok(s) => s.expose_secret().to_string(),
                    Err(e) => {
                        warn!(error = %e, "Failed to decrypt Testmo API key");
                        return HealthCheckResult::offline("testmo", "Failed to decrypt API key");
                    }
                };

                TestmoHealthCheck::new(testmo.instance_url, api_key)
                    .check()
                    .await
            }
            IntegrationKind::Jira => {
                let jira = cfg.integrations.jira;

                // Only API token auth is supported for health validation today.
                if !matches!(jira.auth_type, qa_pms_config::JiraAuthType::ApiToken) {
                    return HealthCheckResult::offline(
                        "jira",
                        "OAuth Jira validation is not supported yet",
                    );
                }

                let Some(email_enc) = jira.email_encrypted else {
                    return HealthCheckResult::offline("jira", "Missing Jira email");
                };
                let Some(token_enc) = jira.api_token_encrypted else {
                    return HealthCheckResult::offline("jira", "Missing Jira API token");
                };

                let email = match self.encryptor.decrypt(&email_enc) {
                    Ok(s) => s.expose_secret().to_string(),
                    Err(e) => {
                        warn!(error = %e, "Failed to decrypt Jira email");
                        return HealthCheckResult::offline("jira", "Failed to decrypt Jira email");
                    }
                };
                let api_token = match self.encryptor.decrypt(&token_enc) {
                    Ok(s) => s.expose_secret().to_string(),
                    Err(e) => {
                        warn!(error = %e, "Failed to decrypt Jira API token");
                        return HealthCheckResult::offline("jira", "Failed to decrypt Jira token");
                    }
                };

                JiraHealthCheck::with_api_token(jira.instance_url, email, api_token)
                    .check()
                    .await
            }
        }
    }
}

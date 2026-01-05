//! # QA PMS Config
//!
//! Configuration management and encryption for the QA Intelligent PMS framework.
//!
//! This crate provides:
//! - YAML configuration file parsing
//! - AES-256-GCM encryption for sensitive data
//! - Environment variable loading via `dotenvy`
//! - Configuration validation
//! - User config generation from setup wizard

pub mod encryption;
pub mod settings;
pub mod user_config;

pub use encryption::Encryptor;
pub use settings::Settings;
pub use user_config::{
    JiraAuthInput, JiraAuthType, JiraConfig, JiraInput, PostmanConfig, PostmanInput, ProfileInput,
    SetupWizardInput, SplunkConfig, SplunkInput, TestmoConfig, TestmoInput, UserConfig, UserProfile,
    ValidationError, ValidationResult,
};

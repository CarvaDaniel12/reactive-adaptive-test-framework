//! Background token refresh task.
//!
//! Automatically refreshes OAuth tokens before they expire.

use crate::oauth::JiraOAuthClient;
use qa_pms_core::{StoredTokens, TokenStore};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{debug, error, info, warn};

/// Default interval for checking token expiration (60 seconds).
const CHECK_INTERVAL_SECS: u64 = 60;

/// Refresh tokens when they will expire within this many seconds (5 minutes).
const REFRESH_THRESHOLD_SECS: i64 = 300;

/// Start a background task that monitors and refreshes tokens.
///
/// This task runs indefinitely, checking token expiration every minute
/// and refreshing tokens that will expire within 5 minutes.
pub async fn start_token_refresh_task(
    token_store: Arc<dyn TokenStore>,
    oauth_client: Arc<JiraOAuthClient>,
) {
    let mut check_interval = interval(Duration::from_secs(CHECK_INTERVAL_SECS));

    info!("Starting token refresh background task");

    loop {
        check_interval.tick().await;

        match check_and_refresh_jira_token(&token_store, &oauth_client).await {
            Ok(refreshed) => {
                if refreshed {
                    info!("Successfully refreshed Jira token");
                }
            }
            Err(e) => {
                error!(error = %e, "Failed to refresh Jira token");
                // Don't crash - the user will need to re-authenticate
            }
        }
    }
}

/// Check if Jira token needs refresh and refresh if necessary.
///
/// Returns `true` if the token was refreshed.
async fn check_and_refresh_jira_token(
    token_store: &Arc<dyn TokenStore>,
    oauth_client: &Arc<JiraOAuthClient>,
) -> anyhow::Result<bool> {
    const INTEGRATION: &str = "jira";

    // Get current tokens
    let Some(tokens) = token_store.get_tokens(INTEGRATION).await? else {
        debug!("No Jira tokens stored, skipping refresh check");
        return Ok(false);
    };

    // Check if refresh is needed
    if !tokens.expires_within(REFRESH_THRESHOLD_SECS) {
        debug!(
            seconds_until_expiry = tokens.seconds_until_expiry(),
            "Jira token still valid, no refresh needed"
        );
        return Ok(false);
    }

    info!(
        seconds_until_expiry = tokens.seconds_until_expiry(),
        "Jira token expiring soon, refreshing"
    );

    // Attempt refresh
    let new_tokens = oauth_client
        .refresh_access_token(&tokens.refresh_token)
        .await
        .map_err(|e| {
            warn!(error = %e, "Token refresh failed");
            e
        })?;

    // Store new tokens
    let stored = StoredTokens::new(
        INTEGRATION,
        new_tokens.access_token,
        new_tokens.refresh_token.unwrap_or(tokens.refresh_token),
        new_tokens.expires_in,
    );

    token_store.store_tokens(stored).await?;

    Ok(true)
}

/// Spawn the token refresh task as a background task.
///
/// Returns a handle that can be used to abort the task.
#[must_use]
pub fn spawn_token_refresh_task(
    token_store: Arc<dyn TokenStore>,
    oauth_client: Arc<JiraOAuthClient>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(start_token_refresh_task(token_store, oauth_client))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Integration tests for token refresh would require mocking
    // the OAuth client and token store, which is better done in
    // a dedicated test module or integration test file.

    #[test]
    fn test_constants() {
        // Verify our constants are reasonable
        assert!(CHECK_INTERVAL_SECS >= 30); // Don't check too frequently
        assert!(REFRESH_THRESHOLD_SECS >= 60); // Give time to refresh
        assert!(REFRESH_THRESHOLD_SECS <= 600); // Don't refresh too early
    }
}

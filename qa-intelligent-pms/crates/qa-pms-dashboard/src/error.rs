//! Shared error handling utilities for dashboard and API operations.

use qa_pms_core::error::ApiError;

/// Extension trait for converting SQLx Results to ApiError.
///
/// This provides a consistent way to map database errors to API errors.
/// Two methods are available:
/// - `map_internal(context)`: Includes a context string in the error message
/// - `map_db_err()`: Simple conversion without additional context
///
/// # Example
///
/// ```ignore
/// use qa_pms_dashboard::SqlxResultExt;
///
/// // With context (preferred for debugging)
/// let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
///     .fetch_optional(&pool)
///     .await
///     .map_internal("Failed to fetch user")?;
///
/// // Simple conversion
/// let count = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
///     .fetch_one(&pool)
///     .await
///     .map_db_err()?;
/// ```
pub trait SqlxResultExt<T> {
    /// Map a SQLx error to an internal API error with context.
    fn map_internal(self, context: &str) -> Result<T, ApiError>;

    /// Map a SQLx error to an internal API error without context.
    fn map_db_err(self) -> Result<T, ApiError>;
}

impl<T> SqlxResultExt<T> for Result<T, sqlx::Error> {
    fn map_internal(self, context: &str) -> Result<T, ApiError> {
        self.map_err(|e| ApiError::Internal(anyhow::anyhow!("{context}: {e}")))
    }

    fn map_db_err(self) -> Result<T, ApiError> {
        self.map_err(|e| ApiError::Internal(e.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlx_result_ext_ok() {
        let result: Result<i32, sqlx::Error> = Ok(42);
        assert_eq!(result.map_internal("test").unwrap(), 42);
    }
}

//! Period parsing utilities for dashboard date range filtering.

use chrono::{DateTime, Duration, Utc};

/// Supported dashboard periods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Period {
    /// Last 7 days
    Week,
    /// Last 30 days
    Month,
    /// Last 90 days
    Quarter,
    /// Last 365 days
    Year,
}

impl Period {
    /// Get the number of days for this period.
    #[must_use]
    pub const fn days(&self) -> i64 {
        match self {
            Self::Week => 7,
            Self::Month => 30,
            Self::Quarter => 90,
            Self::Year => 365,
        }
    }
}

/// Parse a period string into days.
///
/// Supported formats: "7d", "30d", "90d", "1y"
/// Returns 30 days for unrecognized formats.
///
/// # Example
///
/// ```
/// use qa_pms_dashboard::parse_period;
///
/// assert_eq!(parse_period("7d"), 7);
/// assert_eq!(parse_period("1y"), 365);
/// assert_eq!(parse_period("invalid"), 30); // default
/// ```
#[must_use]
pub fn parse_period(period: &str) -> i64 {
    match period {
        "7d" => Period::Week.days(),
        "30d" => Period::Month.days(),
        "90d" => Period::Quarter.days(),
        "1y" => Period::Year.days(),
        _ => Period::Month.days(), // default to 30 days
    }
}

/// Get the default period string.
#[must_use]
pub fn default_period() -> String {
    "30d".to_string()
}

/// Calculate period boundaries for current and previous periods.
///
/// Returns (current_start, current_end, previous_start) timestamps.
#[must_use]
pub fn period_boundaries(days: i64) -> (DateTime<Utc>, DateTime<Utc>, DateTime<Utc>) {
    let now = Utc::now();
    let period_start = now - Duration::days(days);
    let prev_period_start = period_start - Duration::days(days);
    (period_start, now, prev_period_start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_period() {
        assert_eq!(parse_period("7d"), 7);
        assert_eq!(parse_period("30d"), 30);
        assert_eq!(parse_period("90d"), 90);
        assert_eq!(parse_period("1y"), 365);
        assert_eq!(parse_period("unknown"), 30);
        assert_eq!(parse_period(""), 30);
    }

    #[test]
    fn test_period_days() {
        assert_eq!(Period::Week.days(), 7);
        assert_eq!(Period::Month.days(), 30);
        assert_eq!(Period::Quarter.days(), 90);
        assert_eq!(Period::Year.days(), 365);
    }

    #[test]
    fn test_period_boundaries() {
        let (start, end, prev_start) = period_boundaries(30);

        // End should be roughly now
        assert!((Utc::now() - end).num_seconds().abs() < 2);

        // Start should be ~30 days before end
        let diff = (end - start).num_days();
        assert_eq!(diff, 30);

        // Previous start should be ~30 days before start
        let prev_diff = (start - prev_start).num_days();
        assert_eq!(prev_diff, 30);
    }
}

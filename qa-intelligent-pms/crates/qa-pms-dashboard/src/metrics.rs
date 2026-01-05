//! Shared metric calculation utilities for dashboards.

/// Calculate percentage change between two values.
///
/// Returns 100.0 if previous is 0 and current is positive.
/// Returns 0.0 if both are zero.
///
/// # Example
///
/// ```
/// use qa_pms_dashboard::calculate_change;
///
/// assert_eq!(calculate_change(110.0, 100.0), 10.0);  // 10% increase
/// assert_eq!(calculate_change(90.0, 100.0), -10.0);  // 10% decrease
/// assert_eq!(calculate_change(50.0, 0.0), 100.0);    // from zero
/// ```
#[must_use]
pub fn calculate_change(current: f64, previous: f64) -> f64 {
    if previous == 0.0 {
        if current > 0.0 {
            100.0
        } else {
            0.0
        }
    } else {
        ((current - previous) / previous * 100.0).round()
    }
}

/// Determine trend direction based on current vs previous values.
///
/// Returns "up", "down", or "neutral".
///
/// # Example
///
/// ```
/// use qa_pms_dashboard::calculate_trend;
///
/// assert_eq!(calculate_trend(110.0, 100.0), "up");
/// assert_eq!(calculate_trend(90.0, 100.0), "down");
/// assert_eq!(calculate_trend(100.0, 100.0), "neutral");
/// ```
#[must_use]
pub fn calculate_trend(current: f64, previous: f64) -> &'static str {
    if current > previous {
        "up"
    } else if current < previous {
        "down"
    } else {
        "neutral"
    }
}

/// Trend direction enum for type-safe trend handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trend {
    Up,
    Down,
    Neutral,
}

impl Trend {
    /// Create trend from comparing two values.
    #[must_use]
    pub fn from_comparison(current: f64, previous: f64) -> Self {
        if current > previous {
            Self::Up
        } else if current < previous {
            Self::Down
        } else {
            Self::Neutral
        }
    }

    /// Get string representation.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Down => "down",
            Self::Neutral => "neutral",
        }
    }
}

impl std::fmt::Display for Trend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_change() {
        // Normal cases
        assert_eq!(calculate_change(110.0, 100.0), 10.0);
        assert_eq!(calculate_change(90.0, 100.0), -10.0);
        assert_eq!(calculate_change(200.0, 100.0), 100.0);
        assert_eq!(calculate_change(50.0, 100.0), -50.0);

        // Edge cases
        assert_eq!(calculate_change(50.0, 0.0), 100.0);
        assert_eq!(calculate_change(0.0, 0.0), 0.0);
        assert_eq!(calculate_change(0.0, 100.0), -100.0);
    }

    #[test]
    fn test_calculate_trend() {
        assert_eq!(calculate_trend(110.0, 100.0), "up");
        assert_eq!(calculate_trend(90.0, 100.0), "down");
        assert_eq!(calculate_trend(100.0, 100.0), "neutral");
    }

    #[test]
    fn test_trend_enum() {
        assert_eq!(Trend::from_comparison(110.0, 100.0), Trend::Up);
        assert_eq!(Trend::from_comparison(90.0, 100.0), Trend::Down);
        assert_eq!(Trend::from_comparison(100.0, 100.0), Trend::Neutral);

        assert_eq!(Trend::Up.as_str(), "up");
        assert_eq!(format!("{}", Trend::Down), "down");
    }
}

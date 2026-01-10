//! Default workflow template seeding.
//!
//! Creates default workflow templates on application startup.

use sqlx::PgPool;
use tracing::{debug, info};

use crate::types::WorkflowStep;

// ============================================================================
// Default Templates
// ============================================================================

/// Bug Fix workflow template steps.
#[must_use]
pub fn bug_fix_template_steps() -> Vec<WorkflowStep> {
    vec![
        WorkflowStep {
            name: "Reproduce Bug".to_string(),
            description: "Follow the steps in the ticket to reproduce the bug. Document exact steps, environment, and any variations observed.".to_string(),
            estimated_minutes: 15,
        },
        WorkflowStep {
            name: "Investigate Root Cause".to_string(),
            description: "Analyze logs, code, and related components to identify the root cause. Note any related issues or dependencies.".to_string(),
            estimated_minutes: 20,
        },
        WorkflowStep {
            name: "Test Fix".to_string(),
            description: "Verify the fix resolves the original issue. Test with the same steps used to reproduce, plus variations.".to_string(),
            estimated_minutes: 30,
        },
        WorkflowStep {
            name: "Regression Check".to_string(),
            description: "Ensure the fix doesn't break existing functionality. Run related test cases and check impacted areas.".to_string(),
            estimated_minutes: 20,
        },
        WorkflowStep {
            name: "Document Findings".to_string(),
            description: "Update the ticket with test results, any issues found, and recommendations. Link related test cases.".to_string(),
            estimated_minutes: 10,
        },
    ]
}

/// Feature Test workflow template steps.
#[must_use]
pub fn feature_test_template_steps() -> Vec<WorkflowStep> {
    vec![
        WorkflowStep {
            name: "Review Requirements".to_string(),
            description: "Read the feature requirements, acceptance criteria, and design documents. Identify testable scenarios.".to_string(),
            estimated_minutes: 15,
        },
        WorkflowStep {
            name: "Exploratory Testing".to_string(),
            description: "Explore the feature freely to understand its behavior. Note unexpected behaviors and potential edge cases.".to_string(),
            estimated_minutes: 45,
        },
        WorkflowStep {
            name: "Happy Path Testing".to_string(),
            description: "Test the main user flows with valid inputs. Verify all acceptance criteria are met.".to_string(),
            estimated_minutes: 30,
        },
        WorkflowStep {
            name: "Edge Case Testing".to_string(),
            description: "Test boundary conditions, invalid inputs, error handling, and unusual scenarios.".to_string(),
            estimated_minutes: 30,
        },
        WorkflowStep {
            name: "Document Test Cases".to_string(),
            description: "Record test cases executed, results, and any bugs found. Update test documentation.".to_string(),
            estimated_minutes: 15,
        },
    ]
}

/// Regression Test workflow template steps.
#[must_use]
pub fn regression_template_steps() -> Vec<WorkflowStep> {
    vec![
        WorkflowStep {
            name: "Setup Test Environment".to_string(),
            description: "Prepare the test environment with correct version, data, and configurations. Verify environment health.".to_string(),
            estimated_minutes: 20,
        },
        WorkflowStep {
            name: "Run Test Suite".to_string(),
            description: "Execute the regression test suite. Monitor for failures and performance issues.".to_string(),
            estimated_minutes: 60,
        },
        WorkflowStep {
            name: "Analyze Failures".to_string(),
            description: "Investigate any test failures. Determine if failures are bugs, test issues, or environment problems.".to_string(),
            estimated_minutes: 30,
        },
        WorkflowStep {
            name: "Generate Report".to_string(),
            description: "Create a summary report with pass/fail rates, identified issues, and recommendations.".to_string(),
            estimated_minutes: 15,
        },
    ]
}

// ============================================================================
// Seeding Functions
// ============================================================================

/// Default template definitions.
struct DefaultTemplate {
    name: &'static str,
    description: &'static str,
    ticket_type: &'static str,
    steps_fn: fn() -> Vec<WorkflowStep>,
}

const DEFAULT_TEMPLATES: &[DefaultTemplate] = &[
    DefaultTemplate {
        name: "Bug Fix Workflow",
        description: "Guided workflow for testing bug fixes. Covers reproduction, investigation, fix verification, and regression testing.",
        ticket_type: "bug",
        steps_fn: bug_fix_template_steps,
    },
    DefaultTemplate {
        name: "Feature Test Workflow",
        description: "Comprehensive workflow for testing new features. Includes requirements review, exploratory testing, and edge case coverage.",
        ticket_type: "feature",
        steps_fn: feature_test_template_steps,
    },
    DefaultTemplate {
        name: "Regression Test Workflow",
        description: "Workflow for regression testing. Guides through environment setup, test execution, failure analysis, and reporting.",
        ticket_type: "regression",
        steps_fn: regression_template_steps,
    },
];

/// Seed default workflow templates.
///
/// This function is idempotent - it will only create templates that don't exist.
///
/// # Errors
/// Returns error if database operations fail.
pub async fn seed_default_templates(pool: &PgPool) -> Result<SeedingResult, sqlx::Error> {
    let mut result = SeedingResult::default();

    for template in DEFAULT_TEMPLATES {
        // Check if template already exists
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM workflow_templates WHERE name = $1 AND is_default = true)",
        )
        .bind(template.name)
        .fetch_one(pool)
        .await?;

        if exists {
            debug!(
                template = template.name,
                "Default template already exists, skipping"
            );
            result.skipped += 1;
            continue;
        }

        // Create the template
        let steps = (template.steps_fn)();
        let steps_json = sqlx::types::Json(steps);

        sqlx::query(
            r"
            INSERT INTO workflow_templates (name, description, ticket_type, steps_json, is_default)
            VALUES ($1, $2, $3, $4, true)
            ",
        )
        .bind(template.name)
        .bind(template.description)
        .bind(template.ticket_type)
        .bind(steps_json)
        .execute(pool)
        .await?;

        info!(
            template = template.name,
            "Created default workflow template"
        );
        result.created += 1;
    }

    Ok(result)
}

/// Result of seeding operation.
#[derive(Debug, Default)]
pub struct SeedingResult {
    /// Number of templates created.
    pub created: usize,
    /// Number of templates skipped (already exist).
    pub skipped: usize,
}

impl SeedingResult {
    /// Total templates processed.
    #[must_use]
    pub const fn total(&self) -> usize {
        self.created + self.skipped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bug_fix_template_has_5_steps() {
        let steps = bug_fix_template_steps();
        assert_eq!(steps.len(), 5);
        assert_eq!(steps[0].name, "Reproduce Bug");
        assert_eq!(steps[4].name, "Document Findings");
    }

    #[test]
    fn test_feature_test_template_has_5_steps() {
        let steps = feature_test_template_steps();
        assert_eq!(steps.len(), 5);
        assert_eq!(steps[0].name, "Review Requirements");
        assert_eq!(steps[4].name, "Document Test Cases");
    }

    #[test]
    fn test_regression_template_has_4_steps() {
        let steps = regression_template_steps();
        assert_eq!(steps.len(), 4);
        assert_eq!(steps[0].name, "Setup Test Environment");
        assert_eq!(steps[3].name, "Generate Report");
    }

    #[test]
    fn test_all_steps_have_descriptions() {
        for steps in [
            bug_fix_template_steps(),
            feature_test_template_steps(),
            regression_template_steps(),
        ] {
            for step in steps {
                assert!(
                    !step.description.is_empty(),
                    "Step {} has empty description",
                    step.name
                );
                assert!(
                    step.estimated_minutes > 0,
                    "Step {} has invalid time",
                    step.name
                );
            }
        }
    }

    #[test]
    fn test_total_estimated_times() {
        // Bug fix: 15 + 20 + 30 + 20 + 10 = 95 minutes
        let bug_total: i32 = bug_fix_template_steps()
            .iter()
            .map(|s| s.estimated_minutes)
            .sum();
        assert_eq!(bug_total, 95);

        // Feature: 15 + 45 + 30 + 30 + 15 = 135 minutes
        let feature_total: i32 = feature_test_template_steps()
            .iter()
            .map(|s| s.estimated_minutes)
            .sum();
        assert_eq!(feature_total, 135);

        // Regression: 20 + 60 + 30 + 15 = 125 minutes
        let regression_total: i32 = regression_template_steps()
            .iter()
            .map(|s| s.estimated_minutes)
            .sum();
        assert_eq!(regression_total, 125);
    }
}

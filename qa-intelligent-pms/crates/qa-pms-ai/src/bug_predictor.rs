//! Bug prediction ML model.
//!
//! Story 31.2: Bug Prediction ML Model
//!
//! Provides functionality to:
//! - Predict bug risk scores for tickets
//! - Identify risk factors
//! - Predict bug types
//! - Train ML model on historical data

use serde::{Deserialize, Serialize};
use tracing::debug;
use utoipa::ToSchema;

use crate::error::AIError;
use crate::test_generator::TicketDetails;

// ============================================================================
// Risk Level
// ============================================================================

/// Risk level for bug prediction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    /// Low risk (0.0-0.25)
    Low,
    /// Medium risk (0.25-0.5)
    Medium,
    /// High risk (0.5-0.75)
    High,
    /// Critical risk (0.75-1.0)
    Critical,
}

impl RiskLevel {
    /// Convert risk score to risk level.
    #[must_use]
    pub fn from_score(score: f32) -> Self {
        match score {
            s if s >= 0.75 => Self::Critical,
            s if s >= 0.5 => Self::High,
            s if s >= 0.25 => Self::Medium,
            _ => Self::Low,
        }
    }

    /// Convert to string for display.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================================
// Risk Factor
// ============================================================================

/// A risk factor identified in a ticket.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RiskFactor {
    /// Factor name (e.g., "API Integration", "High Story Points")
    pub factor: String,
    /// Impact score (0.0-1.0)
    pub impact: f32,
    /// Description of why this factor increases risk
    pub description: String,
}

// ============================================================================
// Bug Risk Score
// ============================================================================

/// Bug risk score prediction for a ticket.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BugRiskScore {
    /// Ticket key
    pub ticket_key: String,
    /// Risk score (0.0-1.0)
    pub risk_score: f32,
    /// Risk level (Low, Medium, High, Critical)
    pub risk_level: RiskLevel,
    /// Identified risk factors
    pub risk_factors: Vec<RiskFactor>,
    /// Predicted bug types
    pub predicted_bugs: Vec<String>,
    /// Confidence level (0.0-1.0)
    pub confidence: f32,
}

// ============================================================================
// Bug Predictor
// ============================================================================

/// Bug prediction service.
///
/// For MVP, uses rule-based heuristics. Can be upgraded to ML model later.
pub struct BugPredictor {
    /// Model storage (in-memory for MVP)
    /// TODO: Upgrade to file-based or ONNX Runtime for ML model
    _model_data: Option<Vec<u8>>,
}

impl BugPredictor {
    /// Create a new bug predictor with empty model.
    #[must_use]
    pub fn new() -> Self {
        Self {
            _model_data: None,
        }
    }

    /// Create a bug predictor with loaded model.
    ///
    /// # Arguments
    /// * `model_data` - Serialized model data (for future ML model support)
    #[must_use]
    pub fn with_model(model_data: Vec<u8>) -> Self {
        Self {
            _model_data: Some(model_data),
        }
    }

    /// Predict bug risk for a ticket.
    ///
    /// # Arguments
    /// * `ticket` - Ticket details to analyze
    ///
    /// # Returns
    /// Bug risk score with factors and predictions
    ///
    /// # Errors
    /// Returns error if prediction fails
    pub async fn predict(&self, ticket: &TicketDetails) -> Result<BugRiskScore, AIError> {
        debug!("Predicting bug risk for ticket: {}", ticket.key);

        // Extract features (Task 2)
        let _features = self.extract_features(ticket).await?;

        // Identify risk factors (Task 3)
        let risk_factors = self.identify_risk_factors(ticket).await?;

        // Predict bug types (Task 4)
        let predicted_bugs = self.predict_bug_types(ticket).await?;

        // Calculate risk score using rule-based heuristics (MVP)
        // TODO: Replace with ML model prediction (Task 5, 6)
        let risk_score = self.calculate_risk_score_heuristic(ticket, &risk_factors).await?;

        // Calculate confidence (based on feature quality)
        let confidence = self.calculate_confidence(ticket).await?;

        // Determine risk level from score
        let risk_level = RiskLevel::from_score(risk_score);

        Ok(BugRiskScore {
            ticket_key: ticket.key.clone(),
            risk_score,
            risk_level,
            risk_factors,
            predicted_bugs,
            confidence,
        })
    }

    /// Extract features from ticket (Task 2).
    ///
    /// Extracts normalized features (0.0-1.0 range) for ML model input.
    ///
    /// # Arguments
    /// * `ticket` - Ticket details to extract features from
    ///
    /// # Returns
    /// Feature vector as Vec<f32> with normalized values
    async fn extract_features(&self, ticket: &TicketDetails) -> Result<Vec<f32>, AIError> {
        let mut features = Vec::new();

        // 2.2: Extract ticket features (issue_type, priority, story_points, components, labels)
        // Ticket type feature (one-hot encoding: Bug=1.0, Feature=0.0, Story=0.5, Task=0.5)
        let ticket_type_score = match ticket.ticket_type.to_lowercase().as_str() {
            "bug" => 1.0,
            "feature" => 0.0,
            "story" => 0.5,
            "task" => 0.5,
            _ => 0.5,
        };
        features.push(ticket_type_score);

        // Priority feature (normalized 0.0-1.0, assuming Critical=1.0, High=0.75, Medium=0.5, Low=0.25)
        // Note: Priority not available in TicketDetails, using default 0.5
        features.push(0.5);

        // Story points feature (normalized 0.0-1.0, assuming max 20 points)
        // Note: Story points not available in TicketDetails, using default 0.0
        let story_points_normalized = 0.0; // Will be updated when story_points available
        features.push(story_points_normalized);

        // 2.3: Extract description features (length, keyword mentions: API, performance, security)
        let description_lower = ticket.description.to_lowercase();
        let description_length = ticket.description.len() as f32;
        // Normalize description length (0-5000 chars, max at 1.0)
        let description_length_normalized = (description_length / 5000.0).min(1.0);
        features.push(description_length_normalized);

        // Keyword features (binary: 1.0 if keyword found, 0.0 otherwise)
        let has_api_keywords = description_lower.contains("api") 
            || description_lower.contains("endpoint") 
            || description_lower.contains("rest")
            || description_lower.contains("graphql");
        features.push(if has_api_keywords { 1.0 } else { 0.0 });

        let has_performance_keywords = description_lower.contains("performance")
            || description_lower.contains("slow")
            || description_lower.contains("timeout")
            || description_lower.contains("latency");
        features.push(if has_performance_keywords { 1.0 } else { 0.0 });

        let has_security_keywords = description_lower.contains("security")
            || description_lower.contains("authentication")
            || description_lower.contains("authorization")
            || description_lower.contains("encrypt")
            || description_lower.contains("password")
            || description_lower.contains("token");
        features.push(if has_security_keywords { 1.0 } else { 0.0 });

        // Title features
        let title_length = ticket.title.len() as f32;
        let title_length_normalized = (title_length / 200.0).min(1.0);
        features.push(title_length_normalized);

        // 2.4: Extract historical features (author bug rate, component bug rate, recent bug count)
        // Note: Historical features require database access, using defaults for MVP
        // TODO: Add database integration for historical features
        let author_bug_rate = 0.5; // Placeholder
        features.push(author_bug_rate);

        let component_bug_rate = 0.5; // Placeholder
        features.push(component_bug_rate);

        let recent_bug_count_normalized = 0.0; // Placeholder
        features.push(recent_bug_count_normalized);

        // Acceptance criteria feature (1.0 if present, 0.0 otherwise)
        let has_acceptance_criteria = if ticket.acceptance_criteria.is_some() { 1.0 } else { 0.0 };
        features.push(has_acceptance_criteria);

        // 2.5: Features are already normalized (0.0-1.0 range)
        // 2.6: Return feature vector
        Ok(features)
    }

    /// Calculate risk score using heuristic rules (MVP).
    ///
    /// Uses rule-based heuristics to calculate risk score based on ticket features and risk factors.
    /// This is a placeholder for ML model prediction (Task 6).
    ///
    /// # Arguments
    /// * `ticket` - Ticket details
    /// * `risk_factors` - Identified risk factors
    ///
    /// # Returns
    /// Risk score (0.0-1.0)
    async fn calculate_risk_score_heuristic(
        &self,
        ticket: &TicketDetails,
        risk_factors: &[RiskFactor],
    ) -> Result<f32, AIError> {
        let mut score = 0.0;

        // Base score from ticket type
        match ticket.ticket_type.to_lowercase().as_str() {
            "bug" => score += 0.4, // Bug tickets have higher base risk
            _ => score += 0.2,      // Other tickets have lower base risk
        }

        // Add score from risk factors (weighted by impact)
        for factor in risk_factors {
            score += factor.impact * 0.15; // Each factor contributes up to 15% of score
        }

        // Cap score at 1.0
        Ok(score.min(1.0))
    }

    /// Identify risk factors (Task 3).
    ///
    /// Analyzes ticket and identifies risk factors that increase bug likelihood.
    ///
    /// # Arguments
    /// * `ticket` - Ticket details to analyze
    ///
    /// # Returns
    /// List of identified risk factors with impact scores
    async fn identify_risk_factors(&self, ticket: &TicketDetails) -> Result<Vec<RiskFactor>, AIError> {
        let mut factors = Vec::new();
        let description_lower = ticket.description.to_lowercase();
        let title_lower = ticket.title.to_lowercase();
        let combined_text = format!("{} {}", title_lower, description_lower);

        // 3.2: Check for API integration mentions (high risk)
        if combined_text.contains("api") 
            || combined_text.contains("endpoint") 
            || combined_text.contains("rest")
            || combined_text.contains("graphql")
            || combined_text.contains("integration") {
            factors.push(RiskFactor {
                factor: "API Integration".to_string(),
                impact: 0.8,
                description: "API integrations often involve external dependencies and complex error handling".to_string(),
            });
        }

        // 3.3: Check for performance keywords (medium-high risk)
        if combined_text.contains("performance")
            || combined_text.contains("slow")
            || combined_text.contains("timeout")
            || combined_text.contains("latency")
            || combined_text.contains("optimization") {
            factors.push(RiskFactor {
                factor: "Performance Concerns".to_string(),
                impact: 0.6,
                description: "Performance-related changes require careful testing under load".to_string(),
            });
        }

        // 3.4: Check for high story points (>8 = high complexity)
        // Note: Story points not available in TicketDetails, skipping for MVP
        // TODO: Add story points check when available

        // 3.5: Check for new components (higher risk)
        // Note: Components not available in TicketDetails, skipping for MVP
        // TODO: Add component check when available

        // 3.6: Check for security-related keywords (high risk)
        if combined_text.contains("security")
            || combined_text.contains("authentication")
            || combined_text.contains("authorization")
            || combined_text.contains("encrypt")
            || combined_text.contains("password")
            || combined_text.contains("token")
            || combined_text.contains("credential") {
            factors.push(RiskFactor {
                factor: "Security-Related".to_string(),
                impact: 0.9,
                description: "Security changes require thorough testing to prevent vulnerabilities".to_string(),
            });
        }

        // Additional heuristic: Bug type tickets have higher risk
        if ticket.ticket_type.to_lowercase() == "bug" {
            factors.push(RiskFactor {
                factor: "Bug Ticket".to_string(),
                impact: 0.7,
                description: "Bug tickets indicate existing issues that may recur".to_string(),
            });
        }

        // 3.7: Return list of risk factors with impact scores
        Ok(factors)
    }

    /// Predict bug types (Task 4).
    ///
    /// Analyzes ticket description for bug patterns and predicts likely bug types.
    ///
    /// # Arguments
    /// * `ticket` - Ticket details to analyze
    ///
    /// # Returns
    /// List of predicted bug types
    async fn predict_bug_types(&self, ticket: &TicketDetails) -> Result<Vec<String>, AIError> {
        let mut bug_types = Vec::new();
        let description_lower = ticket.description.to_lowercase();
        let title_lower = ticket.title.to_lowercase();
        let combined_text = format!("{} {}", title_lower, description_lower);

        // 4.2: Analyze description for bug patterns (race conditions, data integrity, etc.)
        // 4.3: Use keyword matching and heuristics (can be enhanced with ML later)

        // Race condition patterns
        if combined_text.contains("race condition")
            || combined_text.contains("concurrent")
            || combined_text.contains("thread")
            || combined_text.contains("parallel") {
            bug_types.push("Race Condition".to_string());
        }

        // Data integrity patterns
        if combined_text.contains("data integrity")
            || combined_text.contains("corrupt")
            || combined_text.contains("duplicate")
            || combined_text.contains("lost data") {
            bug_types.push("Data Integrity".to_string());
        }

        // Null pointer / null reference patterns
        if combined_text.contains("null")
            || combined_text.contains("undefined")
            || combined_text.contains("none") {
            bug_types.push("Null Reference".to_string());
        }

        // Memory leak patterns
        if combined_text.contains("memory leak")
            || combined_text.contains("out of memory")
            || combined_text.contains("oom") {
            bug_types.push("Memory Leak".to_string());
        }

        // Authentication/Authorization patterns
        if combined_text.contains("unauthorized")
            || combined_text.contains("permission denied")
            || combined_text.contains("access denied") {
            bug_types.push("Authorization".to_string());
        }

        // API/Network patterns
        if combined_text.contains("timeout")
            || combined_text.contains("connection error")
            || combined_text.contains("network") {
            bug_types.push("Network/API".to_string());
        }

        // UI/Display patterns
        if combined_text.contains("display")
            || combined_text.contains("render")
            || combined_text.contains("ui")
            || combined_text.contains("layout") {
            bug_types.push("UI/Display".to_string());
        }

        // 4.4: Return list of predicted bug types
        Ok(bug_types)
    }

    /// Calculate confidence level.
    async fn calculate_confidence(&self, ticket: &TicketDetails) -> Result<f32, AIError> {
        // Confidence based on feature quality
        // Higher confidence if ticket has rich description and metadata
        let mut confidence: f32 = 0.5; // Base confidence

        if !ticket.description.is_empty() {
            confidence += 0.2;
        }

        if ticket.acceptance_criteria.is_some() {
            confidence += 0.2;
        }

        if !ticket.title.is_empty() {
            confidence += 0.1;
        }

        Ok(confidence.min(1.0))
    }
}

impl Default for BugPredictor {
    fn default() -> Self {
        Self::new()
    }
}

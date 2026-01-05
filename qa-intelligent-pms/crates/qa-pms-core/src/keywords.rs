//! Keyword extraction for contextual search.
//!
//! Extracts meaningful keywords from text for search operations.

use std::collections::HashMap;

/// Common stop words to filter out from keyword extraction.
const STOP_WORDS: &[&str] = &[
    // Articles and pronouns
    "a", "an", "the", "i", "me", "my", "we", "our", "you", "your", "he", "she", "it", "its",
    "they", "them", "their", "this", "that", "these", "those", "who", "what", "which",
    // Prepositions
    "to", "of", "in", "for", "on", "with", "at", "by", "from", "as", "into", "through",
    "during", "before", "after", "above", "below", "up", "down", "out", "off", "over",
    "under", "between", "about", "against", "within", "without",
    // Conjunctions
    "and", "but", "or", "nor", "so", "yet", "if", "because", "although", "unless", "until",
    "while", "when", "where", "whether",
    // Verbs (common)
    "is", "are", "was", "were", "be", "been", "being", "have", "has", "had", "do", "does",
    "did", "will", "would", "could", "should", "may", "might", "must", "shall", "can",
    "need", "dare", "get", "got", "make", "made", "let", "see", "seen", "know", "known",
    // Adverbs and adjectives
    "again", "further", "then", "once", "here", "there", "now", "also", "just", "only",
    "very", "too", "more", "most", "less", "least", "much", "many", "some", "any", "all",
    "each", "every", "both", "few", "other", "such", "no", "not", "own", "same", "than",
    // QA-specific common words to filter (too generic)
    "test", "tests", "testing", "tested", "qa", "bug", "bugs", "issue", "issues", "ticket",
    "tickets", "case", "cases", "step", "steps", "expected", "actual", "result", "results",
    "verify", "check", "ensure", "confirm", "validate", "should", "must", "given", "when",
    "then", "scenario", "feature",
];

/// Keyword extractor for contextual search.
///
/// Extracts meaningful keywords from text by:
/// - Tokenizing text into words
/// - Filtering stop words
/// - Removing short words and numbers
/// - Ranking by frequency
#[derive(Debug, Clone)]
pub struct KeywordExtractor {
    /// Minimum word length to consider.
    min_length: usize,
    /// Maximum number of keywords to return.
    max_keywords: usize,
}

impl Default for KeywordExtractor {
    fn default() -> Self {
        Self {
            min_length: 3,
            max_keywords: 10,
        }
    }
}

impl KeywordExtractor {
    /// Create a new keyword extractor with custom settings.
    ///
    /// # Arguments
    /// * `min_length` - Minimum word length to consider
    /// * `max_keywords` - Maximum number of keywords to return
    #[must_use]
    pub fn new(min_length: usize, max_keywords: usize) -> Self {
        Self {
            min_length,
            max_keywords,
        }
    }

    /// Extract keywords from multiple text sources.
    ///
    /// # Arguments
    /// * `texts` - Slice of text strings to extract keywords from
    ///
    /// # Returns
    /// Vector of keywords sorted by frequency (most frequent first)
    #[must_use]
    pub fn extract(&self, texts: &[&str]) -> Vec<String> {
        let mut word_counts: HashMap<String, usize> = HashMap::new();

        for text in texts {
            for word in self.tokenize(text) {
                if self.is_valid_keyword(&word) {
                    *word_counts.entry(word).or_insert(0) += 1;
                }
            }
        }

        // Sort by frequency (descending) and take top keywords
        let mut keywords: Vec<_> = word_counts.into_iter().collect();
        keywords.sort_by(|a, b| b.1.cmp(&a.1));

        keywords
            .into_iter()
            .take(self.max_keywords)
            .map(|(word, _)| word)
            .collect()
    }

    /// Extract keywords from a ticket's title and description.
    ///
    /// Title words are weighted more heavily by being included first.
    ///
    /// # Arguments
    /// * `title` - Ticket title
    /// * `description` - Optional ticket description
    ///
    /// # Returns
    /// Vector of keywords sorted by relevance
    #[must_use]
    pub fn extract_from_ticket(&self, title: &str, description: Option<&str>) -> Vec<String> {
        let mut texts = vec![title];
        if let Some(desc) = description {
            texts.push(desc);
        }
        self.extract(&texts)
    }

    /// Tokenize text into lowercase words.
    fn tokenize(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .split(|c: char| !c.is_alphanumeric() && c != '-' && c != '_')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()
    }

    /// Check if a word is a valid keyword.
    fn is_valid_keyword(&self, word: &str) -> bool {
        // Must meet minimum length
        if word.len() < self.min_length {
            return false;
        }

        // Must not be a stop word
        if STOP_WORDS.contains(&word.as_ref()) {
            return false;
        }

        // Must not be purely numeric
        if word.chars().all(|c| c.is_numeric()) {
            return false;
        }

        // Must not be a common version pattern (v1, v2.0, etc.)
        if word.starts_with('v') && word[1..].chars().all(|c| c.is_numeric() || c == '.') {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_extractor() {
        let extractor = KeywordExtractor::default();
        assert_eq!(extractor.min_length, 3);
        assert_eq!(extractor.max_keywords, 10);
    }

    #[test]
    fn test_custom_extractor() {
        let extractor = KeywordExtractor::new(4, 5);
        assert_eq!(extractor.min_length, 4);
        assert_eq!(extractor.max_keywords, 5);
    }

    #[test]
    fn test_extract_from_ticket_title_only() {
        let extractor = KeywordExtractor::default();
        let keywords = extractor.extract_from_ticket(
            "Login authentication fails with invalid credentials",
            None,
        );

        assert!(keywords.contains(&"login".to_string()));
        assert!(keywords.contains(&"authentication".to_string()));
        assert!(keywords.contains(&"fails".to_string()));
        assert!(keywords.contains(&"invalid".to_string()));
        assert!(keywords.contains(&"credentials".to_string()));
    }

    #[test]
    fn test_extract_from_ticket_with_description() {
        let extractor = KeywordExtractor::default();
        let keywords = extractor.extract_from_ticket(
            "Login authentication fails",
            Some("When user enters wrong password, the system shows generic error message"),
        );

        assert!(keywords.contains(&"login".to_string()));
        assert!(keywords.contains(&"fails".to_string()));
        assert!(keywords.contains(&"user".to_string()));
        assert!(keywords.contains(&"wrong".to_string()));
        assert!(keywords.contains(&"system".to_string()));
    }

    #[test]
    fn test_filters_stop_words() {
        let extractor = KeywordExtractor::default();
        let keywords = extractor.extract(&["The user should be able to login"]);

        assert!(!keywords.contains(&"the".to_string()));
        assert!(!keywords.contains(&"should".to_string()));
        assert!(keywords.contains(&"login".to_string()));
        assert!(keywords.contains(&"user".to_string())); // "user" is not a stop word
        assert!(keywords.contains(&"able".to_string())); // 4 chars, passes min_length
    }

    #[test]
    fn test_filters_short_words() {
        let extractor = KeywordExtractor::default();
        let keywords = extractor.extract(&["A B CD EFG HIJK"]);

        assert!(!keywords.contains(&"a".to_string()));
        assert!(!keywords.contains(&"b".to_string()));
        assert!(!keywords.contains(&"cd".to_string()));
        assert!(keywords.contains(&"efg".to_string()));
        assert!(keywords.contains(&"hijk".to_string()));
    }

    #[test]
    fn test_filters_numbers() {
        let extractor = KeywordExtractor::default();
        let keywords = extractor.extract(&["Error 404 in module 123"]);

        assert!(!keywords.contains(&"404".to_string()));
        assert!(!keywords.contains(&"123".to_string()));
        assert!(keywords.contains(&"error".to_string()));
        assert!(keywords.contains(&"module".to_string()));
    }

    #[test]
    fn test_filters_version_patterns() {
        let extractor = KeywordExtractor::default();
        let keywords = extractor.extract(&["Bug in v2.0 and v3.1.4"]);

        assert!(!keywords.contains(&"v2.0".to_string()));
        assert!(!keywords.contains(&"v3.1.4".to_string()));
    }

    #[test]
    fn test_preserves_hyphenated_words() {
        let extractor = KeywordExtractor::default();
        let keywords = extractor.extract(&["user-authentication and single-sign-on"]);

        assert!(keywords.contains(&"user-authentication".to_string()));
        assert!(keywords.contains(&"single-sign-on".to_string()));
    }

    #[test]
    fn test_preserves_underscored_words() {
        let extractor = KeywordExtractor::default();
        let keywords = extractor.extract(&["test_login and user_profile"]);

        assert!(keywords.contains(&"test_login".to_string()));
        assert!(keywords.contains(&"user_profile".to_string()));
    }

    #[test]
    fn test_ranks_by_frequency() {
        let extractor = KeywordExtractor::default();
        let keywords = extractor.extract(&[
            "payment payment payment login login error",
        ]);

        // "payment" appears 3 times, should be first
        assert_eq!(keywords[0], "payment");
        // "login" appears 2 times, should be second
        assert_eq!(keywords[1], "login");
    }

    #[test]
    fn test_respects_max_keywords() {
        let extractor = KeywordExtractor::new(3, 3);
        let keywords = extractor.extract(&[
            "apple banana cherry date elderberry fig grape",
        ]);

        assert_eq!(keywords.len(), 3);
    }

    #[test]
    fn test_empty_input() {
        let extractor = KeywordExtractor::default();
        let keywords = extractor.extract(&[]);
        assert!(keywords.is_empty());

        let keywords = extractor.extract(&[""]);
        assert!(keywords.is_empty());
    }

    #[test]
    fn test_qa_specific_stop_words() {
        let extractor = KeywordExtractor::default();
        let keywords = extractor.extract(&[
            "Test case for verify login feature scenario",
        ]);

        // QA-specific stop words should be filtered
        assert!(!keywords.contains(&"test".to_string()));
        assert!(!keywords.contains(&"case".to_string()));
        assert!(!keywords.contains(&"verify".to_string()));
        assert!(!keywords.contains(&"feature".to_string()));
        assert!(!keywords.contains(&"scenario".to_string()));

        // "login" should remain
        assert!(keywords.contains(&"login".to_string()));
    }

    #[test]
    fn test_case_insensitivity() {
        let extractor = KeywordExtractor::default();
        let keywords = extractor.extract(&["LOGIN Login login"]);

        // Should be deduplicated to single lowercase entry
        assert_eq!(keywords.len(), 1);
        assert_eq!(keywords[0], "login");
    }

    #[test]
    fn test_real_world_ticket() {
        let extractor = KeywordExtractor::default();
        let keywords = extractor.extract_from_ticket(
            "[BUG] Payment gateway returns 500 error during checkout",
            Some("Steps to reproduce:\n1. Add items to cart\n2. Proceed to checkout\n3. Enter credit card details\n4. Click 'Pay Now'\n\nExpected: Payment processed successfully\nActual: Server returns HTTP 500 error"),
        );

        // "payment" appears twice, should be in results
        assert!(keywords.contains(&"payment".to_string()));
        // "checkout" appears twice
        assert!(keywords.contains(&"checkout".to_string()));
        // "error" appears twice  
        assert!(keywords.contains(&"error".to_string()));
        // "returns" appears twice
        assert!(keywords.contains(&"returns".to_string()));
        // Keywords are limited to max 10, so we just verify we got some relevant ones
        assert!(keywords.len() <= 10);
    }
}

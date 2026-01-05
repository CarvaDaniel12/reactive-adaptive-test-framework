//! Chat service for the mini-chatbot.

use tracing::debug;

use crate::error::AIError;
use crate::provider::AIClient;
use crate::types::{
    ChatContext, ChatInput, ChatMessage, ChatResponse, MessageRole, TicketContext,
    WorkflowStepContext,
};

/// Chat service for the mini-chatbot.
pub struct ChatService {
    client: AIClient,
}

impl ChatService {
    /// Create a new chat service.
    pub fn new(client: AIClient) -> Self {
        Self { client }
    }

    /// Process a chat message and return a response.
    pub async fn chat(&self, input: ChatInput) -> Result<ChatResponse, AIError> {
        let mut messages = Vec::new();

        // Add system message with context
        let system_message = self.build_system_message(&input.context);
        messages.push(ChatMessage {
            id: uuid::Uuid::new_v4(),
            role: MessageRole::System,
            content: system_message,
            timestamp: chrono::Utc::now(),
        });

        // Add history
        messages.extend(input.history);

        // Add user message
        messages.push(ChatMessage {
            id: uuid::Uuid::new_v4(),
            role: MessageRole::User,
            content: input.message,
            timestamp: chrono::Utc::now(),
        });

        debug!("Sending chat with {} messages", messages.len());

        let (response_message, usage) = self.client.chat(messages).await?;

        Ok(ChatResponse {
            message: response_message,
            usage,
        })
    }

    /// Build the system message with context.
    fn build_system_message(&self, context: &Option<ChatContext>) -> String {
        let mut system = String::from(
            "You are QA Assistant, a helpful AI companion for QA engineers using the QA Intelligent PMS framework. \
             You help with testing workflows, understanding tickets, and providing guidance on QA best practices.\n\n\
             Keep responses concise and actionable. Use bullet points for steps. \
             If asked about something outside your knowledge, say so clearly.\n",
        );

        if let Some(ctx) = context {
            system.push_str("\n## Current Context\n");
            system.push_str(&format!("- Current page: {}\n", ctx.current_page));

            if let Some(ticket) = &ctx.current_ticket {
                system.push_str(&format!(
                    "- Viewing ticket: {} - {}\n",
                    ticket.key, ticket.title
                ));
                system.push_str(&format!("  - Type: {}, Status: {}\n", ticket.ticket_type, ticket.status));
                if let Some(desc) = &ticket.description {
                    let truncated = if desc.len() > 200 {
                        format!("{}...", &desc[..200])
                    } else {
                        desc.clone()
                    };
                    system.push_str(&format!("  - Description: {}\n", truncated));
                }
            }

            if let Some(workflow) = &ctx.workflow_step {
                system.push_str(&format!(
                    "- In workflow: {} (Step {}/{}): {}\n",
                    workflow.workflow_name,
                    workflow.step_number,
                    workflow.total_steps,
                    workflow.step_name
                ));
            }

            if !ctx.recent_actions.is_empty() {
                system.push_str("- Recent actions:\n");
                for action in ctx.recent_actions.iter().take(5) {
                    system.push_str(&format!("  - {}\n", action));
                }
            }
        }

        system
    }

    /// Get suggested questions based on context.
    pub fn get_suggested_questions(context: &Option<ChatContext>) -> Vec<String> {
        let mut suggestions = vec![
            "What should I do next?".to_string(),
            "How do I use this feature?".to_string(),
        ];

        if let Some(ctx) = context {
            if ctx.current_ticket.is_some() {
                suggestions.push("What tests should I run for this ticket?".to_string());
                suggestions.push("Can you explain the acceptance criteria?".to_string());
            }

            if ctx.workflow_step.is_some() {
                suggestions.push("What's the purpose of this step?".to_string());
                suggestions.push("What should I check before completing this step?".to_string());
            }

            match ctx.current_page.as_str() {
                "/" | "/dashboard" => {
                    suggestions.push("How do I interpret these metrics?".to_string());
                }
                "/tickets" => {
                    suggestions.push("How do I filter tickets effectively?".to_string());
                }
                "/workflows" => {
                    suggestions.push("Which workflow should I use?".to_string());
                }
                "/patterns" => {
                    suggestions.push("What do these patterns mean?".to_string());
                }
                _ => {}
            }
        }

        suggestions.truncate(5);
        suggestions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_system_message_no_context() {
        // Can't test without a real client, but we can test the helper
        let suggestions = ChatService::get_suggested_questions(&None);
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_suggested_questions_with_ticket() {
        let context = Some(ChatContext {
            current_page: "/tickets/PROJ-123".to_string(),
            current_ticket: Some(TicketContext {
                key: "PROJ-123".to_string(),
                title: "Test ticket".to_string(),
                description: None,
                ticket_type: "Bug".to_string(),
                status: "Open".to_string(),
            }),
            workflow_step: None,
            recent_actions: vec![],
        });

        let suggestions = ChatService::get_suggested_questions(&context);
        assert!(suggestions.iter().any(|s| s.contains("tests")));
    }

    #[test]
    fn test_suggested_questions_with_workflow() {
        let context = Some(ChatContext {
            current_page: "/workflows/123".to_string(),
            current_ticket: None,
            workflow_step: Some(WorkflowStepContext {
                workflow_name: "Bug Fix".to_string(),
                step_name: "Review Code".to_string(),
                step_number: 2,
                total_steps: 5,
            }),
            recent_actions: vec![],
        });

        let suggestions = ChatService::get_suggested_questions(&context);
        assert!(suggestions.iter().any(|s| s.contains("step")));
    }
}

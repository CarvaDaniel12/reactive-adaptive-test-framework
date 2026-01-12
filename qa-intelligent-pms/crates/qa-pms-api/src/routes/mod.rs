//! API routes module.
//!
//! Organizes all API endpoints.

use axum::{routing::get, Json, Router};
use utoipa::OpenApi;

use crate::app::AppState;

pub mod ai;
pub mod alerts;
pub mod dashboard;
pub mod health;
pub mod integrations;
pub mod pm_dashboard;
pub mod reports;
pub mod search;
pub mod setup;
pub mod splunk;
pub mod startup;
pub mod support;
pub mod testmo;
pub mod tickets;
pub mod time;
pub mod workflows;

/// `OpenAPI` documentation.
#[allow(clippy::needless_for_each)]
#[derive(OpenApi)]
#[openapi(
    info(
        title = "QA Intelligent PMS API",
        version = "0.1.0",
        description = "API for QA Intelligent PMS - Companion Framework",
        contact(name = "Daniel")
    ),
    paths(
        alerts::get_alerts,
        alerts::get_unread_count,
        alerts::mark_read,
        alerts::dismiss_alert,
        alerts::get_patterns,
        alerts::get_pattern,
        dashboard::get_dashboard,
        health::health_check,
        health::get_integration_health,
        health::trigger_health_check,
        setup::save_profile,
        setup::test_jira,
        setup::test_postman,
        setup::test_testmo,
        setup::complete_setup,
        setup::get_status,
        tickets::list_tickets,
        tickets::get_ticket,
        tickets::get_transitions,
        tickets::transition_ticket,
        startup::validate_startup,
        search::contextual_search,
        search::search_postman_endpoint,
        search::search_testmo_endpoint,
        search::search_all,
        testmo::create_test_run,
        workflows::list_templates,
        workflows::get_template_by_id,
        workflows::create_workflow,
        workflows::get_workflow,
        workflows::get_active_workflow_for_ticket,
        workflows::complete_step,
        workflows::skip_step,
        workflows::pause_workflow,
        workflows::resume_workflow,
        workflows::complete_workflow,
        workflows::get_workflow_summary,
        workflows::cancel_workflow,
        workflows::get_user_active_workflows,
        time::start_time_session,
        time::end_time_session,
        time::pause_time_session,
        time::resume_time_session,
        time::get_active_time_session,
        time::get_all_time_sessions,
        // Story 6.7: Historical time data
        time::get_historical_stats,
        time::get_time_trend,
        time::get_averages,
        time::get_gap_alerts,
        time::dismiss_alert,
        reports::generate_report,
        reports::get_report,
        reports::get_report_by_workflow,
        pm_dashboard::get_pm_dashboard,
        pm_dashboard::export_pm_dashboard,
        // Epic 11: Splunk
        splunk::list_templates,
        splunk::get_template,
        splunk::create_template,
        splunk::update_template,
        splunk::delete_template,
        splunk::prepare_query,
        splunk::execute_query,
        splunk::get_query_history,
        splunk::get_placeholders,
        // Epic 12: Support
        support::list_error_logs,
        support::create_error_log,
        support::get_error_log,
        support::update_error_status,
        support::get_suggestions,
        support::get_dashboard_summary,
        support::run_all_diagnostics,
        support::run_diagnostic,
        support::list_kb_entries,
        support::create_kb_entry,
        support::get_kb_entry,
        support::update_kb_entry,
        support::delete_kb_entry,
        support::rate_kb_entry,
        // Epic 13: AI
        ai::get_ai_status,
        ai::get_providers,
        ai::configure_ai,
        ai::test_connection,
        ai::disable_ai,
        ai::chat,
        ai::get_chat_suggestions,
        ai::semantic_search,
        ai::analyze_gherkin,
        // Epic 22: Integration Health
        integrations::get_integration_health,
        integrations::get_integration_health_by_id,
        integrations::store_integration_health,
        integrations::get_integration_events,
    ),
    components(
        schemas(
            health::HealthResponse,
            health::DatabaseStatus,
            health::IntegrationHealthResponse,
            setup::ProfileRequest,
            setup::JiraTestRequest,
            setup::PostmanTestRequest,
            setup::TestmoTestRequest,
            setup::SplunkConfigRequest,
            setup::ConnectionTestResponse,
            setup::CompleteSetupRequest,
            setup::CompleteSetupResponse,
            setup::SetupStatusResponse,
            setup::SuccessResponse,
            tickets::TicketListResponse,
            tickets::TicketSummary,
            tickets::TicketDetailResponse,
            tickets::UserInfo,
            tickets::CommentInfo,
            tickets::AttachmentInfo,
            tickets::TransitionInfo,
            tickets::TransitionRequest,
            tickets::TransitionResponse,
            qa_pms_core::error::ErrorResponse,
            crate::startup::ValidationResult,
            crate::startup::StartupValidationReport,
            search::ContextualSearchRequest,
            search::KeywordSearchRequest,
            search::UnifiedSearchResult,
            search::SearchResponse,
            search::SingleSourceSearchResponse,
            testmo::CreateTestRunRequest,
            testmo::CreateTestRunResponse,
            testmo::ErrorResponse,
            workflows::TemplatesListResponse,
            workflows::TemplateResponse,
            workflows::TemplateDetailResponse,
            workflows::StepResponse,
            workflows::CreateWorkflowRequest,
            workflows::CreateWorkflowResponse,
            workflows::WorkflowDetailResponse,
            workflows::WorkflowStepWithStatus,
            workflows::ActiveWorkflowResponse,
            workflows::WorkflowSummary,
            workflows::CompleteStepRequest,
            workflows::StepLinkRequest,
            workflows::StepActionResponse,
            workflows::WorkflowStatusResponse,
            workflows::WorkflowSummaryResponse,
            workflows::StepSummary,
            workflows::UserActiveWorkflowsResponse,
        time::TimeSessionResponse,
        time::TimeSessionsResponse,
        // Story 6.7: Historical time data schemas
        time::HistoricalStatsResponse,
        time::TicketTypeStats,
        time::TrendResponse,
        time::TrendDataResponse,
        time::UserAveragesResponse,
        time::UserAverageResponse,
        time::GapAlertsResponse,
        time::GapAlertResponse,
        reports::GenerateReportRequest,
        reports::ReportResponse,
        reports::ReportContent,
        reports::ReportStep,
        dashboard::DashboardResponse,
        dashboard::DashboardKPIs,
        dashboard::KPIMetric,
        dashboard::TrendDataPoint,
        dashboard::ActivityItem,
        alerts::AlertResponse,
        alerts::AlertsResponse,
        alerts::UnreadCountResponse,
        alerts::PatternResponse,
        alerts::PatternsResponse,
        pm_dashboard::PMDashboardResponse,
        pm_dashboard::PMSummary,
        pm_dashboard::BugsMetrics,
        pm_dashboard::EconomyMetrics,
        pm_dashboard::ComponentHealth,
        pm_dashboard::ProblematicEndpoint,
        // Epic 11: Splunk schemas
        splunk::TemplateResponse,
        splunk::TemplatesListResponse,
        splunk::CreateTemplateRequest,
        splunk::UpdateTemplateRequest,
        splunk::PrepareQueryRequest,
        splunk::PrepareQueryResponse,
        splunk::ExecuteQueryRequest,
        splunk::ExecuteQueryResponse,
        splunk::LogEntryResponse,
        splunk::QueryHistoryEntry,
        splunk::QueryHistoryResponse,
        splunk::PlaceholderInfo,
        splunk::PlaceholdersResponse,
        // Epic 12: Support schemas
        support::ErrorLogsResponse,
        support::CreateErrorRequest,
        support::UpdateStatusRequest,
        support::SuggestionsResponse,
        support::DashboardSummaryResponse,
        support::DiagnosticsResponse,
        support::KbEntriesResponse,
        support::CreateKbRequest,
        support::UpdateKbRequest,
        support::RateKbRequest,
        qa_pms_support::ErrorLog,
        qa_pms_support::ErrorStatus,
        qa_pms_support::ErrorSeverity,
        qa_pms_support::ErrorSource,
        qa_pms_support::KnowledgeBaseEntry,
        qa_pms_support::TroubleshootingSuggestion,
        qa_pms_support::SuggestionSource,
        qa_pms_support::DiagnosticResult,
        qa_pms_support::DiagnosticsReport,
        qa_pms_support::SupportDashboardSummary,
        qa_pms_support::SourceCount,
        qa_pms_support::TopError,
        // Epic 13: AI schemas
        ai::AIStatusResponse,
        ai::ProvidersResponse,
        ai::ConfigureAIRequest,
        ai::ChatRequest,
        ai::ChatMessageDto,
        ai::ChatContextDto,
        ai::TicketContextDto,
        ai::WorkflowStepContextDto,
        ai::ChatResponseDto,
        ai::TokenUsageDto,
        ai::SuggestionsRequest,
        ai::SuggestionsResponse,
        ai::SemanticSearchRequest,
        ai::SemanticSearchResponse,
        ai::GherkinRequest,
        ai::GherkinResponse,
        ai::GherkinScenarioDto,
        qa_pms_ai::ProviderModels,
        qa_pms_ai::ModelInfo,
        qa_pms_ai::ConnectionTestResult,
        qa_pms_ai::ProviderType,
        // Epic 22: Integration Health schemas
        integrations::IntegrationHealthResponse,
        integrations::IntegrationEventsResponse,
        qa_pms_integration_health::IntegrationHealth,
        qa_pms_integration_health::IntegrationEvent,
        qa_pms_integration_health::IntegrationId,
        qa_pms_integration_health::HealthStatus,
        )
    ),
    tags(
        (name = "Alerts", description = "Alert and pattern detection endpoints"),
        (name = "Dashboard", description = "Dashboard metrics endpoints"),
        (name = "health", description = "Health check endpoints"),
        (name = "Setup", description = "Setup wizard endpoints"),
        (name = "Tickets", description = "Ticket management endpoints"),
        (name = "Startup", description = "Startup validation endpoints"),
        (name = "Search", description = "Contextual search endpoints"),
        (name = "testmo", description = "Testmo integration endpoints"),
        (name = "Workflows", description = "Workflow template endpoints"),
        (name = "Time Tracking", description = "Time tracking endpoints"),
        (name = "Reports", description = "Report generation endpoints"),
        (name = "PM Dashboard", description = "PM observability dashboard endpoints"),
        (name = "Splunk", description = "Splunk query template and log endpoints"),
        (name = "Support", description = "Support portal and troubleshooting endpoints"),
        (name = "AI", description = "AI companion endpoints (BYOK)"),
        (name = "integrations", description = "Integration health monitoring endpoints")
    )
)]
pub struct ApiDoc;

/// Create the API documentation routes.
///
/// Provides `OpenAPI` JSON at `/api/v1/openapi.json`.
/// Swagger UI can be added later with proper Axum integration.
pub fn api_docs() -> Router<AppState> {
    Router::new().route("/api/v1/openapi.json", get(openapi_json))
}

/// Serve `OpenAPI` JSON specification.
async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

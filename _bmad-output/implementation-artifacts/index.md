# Implementation Artifacts Index

This directory contains all implementation story documents for the QA Intelligent PMS project, organized by epic.

## Epic 2: Setup Wizard

- **[2-1-setup-wizard-ui-shell-and-navigation.md](./2-1-setup-wizard-ui-shell-and-navigation.md)** - Setup wizard UI shell with step navigation
- **[2-2-user-profile-configuration-step.md](./2-2-user-profile-configuration-step.md)** - User profile configuration wizard step
- **[2-3-jira-integration-credentials-step.md](./2-3-jira-integration-credentials-step.md)** - Jira OAuth credentials configuration step
- **[2-4-postman-integration-credentials-step.md](./2-4-postman-integration-credentials-step.md)** - Postman API key configuration step
- **[2-5-testmo-integration-credentials-step.md](./2-5-testmo-integration-credentials-step.md)** - Testmo API credentials configuration step
- **[2-6-splunk-configuration-step-manual.md](./2-6-splunk-configuration-step-manual.md)** - Splunk manual query configuration step
- **[2-7-configuration-yaml-generation-and-validation.md](./2-7-configuration-yaml-generation-and-validation.md)** - YAML config generation with validation
- **[2-8-setup-wizard-backend-api-endpoints.md](./2-8-setup-wizard-backend-api-endpoints.md)** - Backend API endpoints for setup wizard
- **[2-9-hybrid-adaptive-layout-foundation.md](./2-9-hybrid-adaptive-layout-foundation.md)** - Hybrid adaptive layout system foundation

## Epic 3: Jira Integration

- **[3-1-jira-oauth-2-0-pkce-authentication-flow.md](./3-1-jira-oauth-2-0-pkce-authentication-flow.md)** - Jira OAuth 2.0 with PKCE authentication
- **[3-2-jira-ticket-listing-with-filters.md](./3-2-jira-ticket-listing-with-filters.md)** - Jira ticket listing with status filters
- **[3-3-jira-ticket-detail-view.md](./3-3-jira-ticket-detail-view.md)** - Jira ticket detail view component
- **[3-4-jira-ticket-status-updates.md](./3-4-jira-ticket-status-updates.md)** - Jira ticket status update functionality
- **[3-5-integration-health-check-system.md](./3-5-integration-health-check-system.md)** - Health check system for all integrations
- **[3-6-integration-status-dashboard-component.md](./3-6-integration-status-dashboard-component.md)** - Integration status dashboard UI component
- **[3-7-credential-validation-on-startup.md](./3-7-credential-validation-on-startup.md)** - Credential validation at application startup

## Epic 4: Postman & Testmo Search

- **[4-1-postman-api-client-implementation.md](./4-1-postman-api-client-implementation.md)** - Postman API client with collection search
- **[4-2-testmo-api-client-implementation.md](./4-2-testmo-api-client-implementation.md)** - Testmo API client implementation
- **[4-3-contextual-search-on-ticket-selection.md](./4-3-contextual-search-on-ticket-selection.md)** - Contextual search triggered by ticket selection
- **[4-4-search-results-display-with-source-badges.md](./4-4-search-results-display-with-source-badges.md)** - Search results display with source badges
- **[4-5-testmo-test-run-creation.md](./4-5-testmo-test-run-creation.md)** - Testmo test run creation functionality
- **[4-6-search-api-endpoints.md](./4-6-search-api-endpoints.md)** - Backend search API endpoints

## Epic 5: Workflow Engine

- **[5-1-workflow-templates-database-schema.md](./5-1-workflow-templates-database-schema.md)** - Database schema for workflow templates
- **[5-2-default-workflow-templates.md](./5-2-default-workflow-templates.md)** - Default workflow templates configuration
- **[5-3-start-workflow-from-ticket.md](./5-3-start-workflow-from-ticket.md)** - Start workflow from selected ticket
- **[5-4-workflow-step-progress-ui.md](./5-4-workflow-step-progress-ui.md)** - Workflow step progress UI component
- **[5-5-complete-workflow-step-with-notes.md](./5-5-complete-workflow-step-with-notes.md)** - Complete workflow step with notes
- **[5-6-workflow-pause-and-resume.md](./5-6-workflow-pause-and-resume.md)** - Workflow pause and resume functionality
- **[5-7-workflow-completion-and-summary.md](./5-7-workflow-completion-and-summary.md)** - Workflow completion with summary generation
- **[5-8-workflow-state-persistence.md](./5-8-workflow-state-persistence.md)** - Workflow state persistence to database

## Epic 6: Time Tracking

- **[6-1-time-tracking-database-schema.md](./6-1-time-tracking-database-schema.md)** - Database schema for time tracking
- **[6-2-automatic-timer-start-on-workflow-begin.md](./6-2-automatic-timer-start-on-workflow-begin.md)** - Automatic timer start on workflow begin
- **[6-3-real-time-timer-display.md](./6-3-real-time-timer-display.md)** - Real-time timer display component
- **[6-4-timer-pause-and-resume-controls.md](./6-4-timer-pause-and-resume-controls.md)** - Timer pause and resume controls
- **[6-5-time-per-step-tracking.md](./6-5-time-per-step-tracking.md)** - Time tracking per workflow step
- **[6-6-time-comparison-with-gap-alerts.md](./6-6-time-comparison-with-gap-alerts.md)** - Time comparison with gap alerts
- **[6-7-historical-time-data-storage.md](./6-7-historical-time-data-storage.md)** - Historical time data storage and retrieval

## Epic 7: Reporting

- **[7-1-report-generation-on-workflow-completion.md](./7-1-report-generation-on-workflow-completion.md)** - Report generation on workflow completion
- **[7-2-report-content-tests-covered-and-strategies.md](./7-2-report-content-tests-covered-and-strategies.md)** - Report content with tests and strategies
- **[7-3-report-preview-before-saving.md](./7-3-report-preview-before-saving.md)** - Report preview before saving
- **[7-4-report-export-to-markdown-and-html.md](./7-4-report-export-to-markdown-and-html.md)** - Report export to Markdown and HTML
- **[7-5-report-history-storage-and-retrieval.md](./7-5-report-history-storage-and-retrieval.md)** - Report history storage and retrieval

## Epic 8: QA Dashboard

- **[8-1-qa-dashboard-layout-and-navigation.md](./8-1-qa-dashboard-layout-and-navigation.md)** - QA dashboard layout and navigation
- **[8-2-tickets-completed-kpi-card.md](./8-2-tickets-completed-kpi-card.md)** - Tickets completed KPI card component
- **[8-3-time-metrics-kpi-cards.md](./8-3-time-metrics-kpi-cards.md)** - Time metrics KPI cards
- **[8-4-trend-visualization-chart.md](./8-4-trend-visualization-chart.md)** - Trend visualization chart component
- **[8-5-dashboard-period-filters.md](./8-5-dashboard-period-filters.md)** - Dashboard period filter controls
- **[8-6-dashboard-real-time-refresh.md](./8-6-dashboard-real-time-refresh.md)** - Dashboard real-time refresh functionality

## Epic 9: Pattern Detection

- **[9-1-time-excess-pattern-detection.md](./9-1-time-excess-pattern-detection.md)** - Time excess pattern detection algorithm
- **[9-2-consecutive-problem-detection.md](./9-2-consecutive-problem-detection.md)** - Consecutive problem detection system
- **[9-3-proactive-alert-generation.md](./9-3-proactive-alert-generation.md)** - Proactive alert generation engine
- **[9-4-alert-display-with-context.md](./9-4-alert-display-with-context.md)** - Alert display with contextual information
- **[9-5-pattern-history-tracking.md](./9-5-pattern-history-tracking.md)** - Pattern history tracking and analysis

## Epic 10: PM/PO Dashboard

- **[10-1-pm-dashboard-layout.md](./10-1-pm-dashboard-layout.md)** - PM/PO dashboard layout design
- **[10-2-bugs-discovered-vs-prevented-metrics.md](./10-2-bugs-discovered-vs-prevented-metrics.md)** - Bugs discovered vs prevented metrics
- **[10-3-economy-metrics-calculation.md](./10-3-economy-metrics-calculation.md)** - Economy metrics calculation engine
- **[10-4-component-health-visualization.md](./10-4-component-health-visualization.md)** - Component health visualization
- **[10-5-problematic-endpoints-display.md](./10-5-problematic-endpoints-display.md)** - Problematic endpoints display
- **[10-6-dashboard-export-for-meetings.md](./10-6-dashboard-export-for-meetings.md)** - Dashboard export for meetings

## Epic 11: Splunk Integration

- **[11-1-splunk-query-interface.md](./11-1-splunk-query-interface.md)** - Splunk query interface component
- **[11-2-splunk-query-templates.md](./11-2-splunk-query-templates.md)** - Pre-built Splunk query templates
- **[11-3-splunk-log-display.md](./11-3-splunk-log-display.md)** - Splunk log results display

## Epic 12: Support Portal

- **[12-1-support-dashboard.md](./12-1-support-dashboard.md)** - Support portal dashboard
- **[12-2-automatic-error-log-capture.md](./12-2-automatic-error-log-capture.md)** - Automatic error log capture system
- **[12-3-integration-diagnostic-tools.md](./12-3-integration-diagnostic-tools.md)** - Integration diagnostic tools
- **[12-4-knowledge-base-for-common-issues.md](./12-4-knowledge-base-for-common-issues.md)** - Knowledge base for common issues
- **[12-5-troubleshooting-suggestions.md](./12-5-troubleshooting-suggestions.md)** - Troubleshooting suggestions engine

## Epic 13: AI Companion

- **[13-1-ai-provider-configuration-byok.md](./13-1-ai-provider-configuration-byok.md)** - AI provider configuration with BYOK support
- **[13-2-semantic-search-enhancement.md](./13-2-semantic-search-enhancement.md)** - AI-powered semantic search enhancement
- **[13-3-gherkin-based-test-suggestions.md](./13-3-gherkin-based-test-suggestions.md)** - Gherkin-based test case suggestions
- **[13-4-mini-chatbot-ui.md](./13-4-mini-chatbot-ui.md)** - Mini chatbot UI component
- **[13-5-chatbot-contextual-awareness.md](./13-5-chatbot-contextual-awareness.md)** - Chatbot contextual awareness system
- **[13-6-ai-graceful-fallback.md](./13-6-ai-graceful-fallback.md)** - AI graceful fallback mechanism

## Epic 14: Rust Implementation Improvements

- **[14-1-graceful-shutdown-signal-handling.md](./14-1-graceful-shutdown-signal-handling.md)** - Graceful shutdown and signal handling
- **[14-2-request-id-middleware-for-correlation.md](./14-2-request-id-middleware-for-correlation.md)** - Request ID middleware for correlation
- **[14-3-prometheus-metrics-integration.md](./14-3-prometheus-metrics-integration.md)** - Prometheus metrics integration
- **[14-4-in-memory-cache-layer-with-moka.md](./14-4-in-memory-cache-layer-with-moka.md)** - In-memory cache layer with Moka
- **[14-5-rate-limiting-with-tower-governor.md](./14-5-rate-limiting-with-tower-governor.md)** - Rate limiting with Tower Governor
- **[14-6-opentelemetry-distributed-tracing.md](./14-6-opentelemetry-distributed-tracing.md)** - OpenTelemetry distributed tracing
- **[14-7-cli-admin-tool.md](./14-7-cli-admin-tool.md)** - CLI admin tool
- **[14-8-integration-tests-with-testcontainers.md](./14-8-integration-tests-with-testcontainers.md)** - Integration tests with Testcontainers

## Epic 15: Authentication & Authorization 🔴 CRITICAL

- **[epics-framework-improvements.md](../planning-artifacts/epics-framework-improvements.md#epic-15-authentication--authorization)** - Authentication & Authorization (high-level planning)
- Story 15.1: JWT Token Authentication
- Story 15.2: Auth Middleware for Axum
- Story 15.3: User Login/Logout API
- Story 15.4: Role-Based Access Control (RBAC)
- Story 15.5: Permission System
- Story 15.6: Authorization Decorators
- Story 15.7: User CRUD Operations
- Story 15.8: Password Hashing & Reset
- Story 15.9: User Profile Management
- Story 15.10: Refresh Tokens
- Story 15.11: Session Invalidation
- Story 15.12: Rate Limiting per User

## Epic 16: Reports Enhancement 🟡 HIGH

- **[epics-framework-improvements.md](../planning-artifacts/epics-framework-improvements.md#epic-16-reports-enhancement)** - Reports Enhancement (high-level planning)
- Story 16.1: Fetch Time Sessions by Workflow
- Story 16.2: Display Time in Reports
- Story 16.3: Historical Time Comparison

## Epic 17: Audit Logging 🟡 HIGH

- **[epics-framework-improvements.md](../planning-artifacts/epics-framework-improvements.md#epic-17-audit-logging)** - Audit Logging (high-level planning)
- Story 17.1: Audit Log Database Schema
- Story 17.2: Audit Service
- Story 17.3: Audit Middleware
- Story 17.4: Audit Log API
- Story 17.5: Manual Audit Triggers
- Story 17.6: Audit Log Retention
- Story 17.7: Audit Log Dashboard

## Epic 18: User Experience Improvements 🟢 MEDIUM

- **[epics-framework-improvements.md](../planning-artifacts/epics-framework-improvements.md#epic-18-user-experience-improvements)** - User Experience Improvements (high-level planning)
- Story 18.1: Customizable Dashboard Layout
- Story 18.2: Advanced Dashboard Widgets
- Story 18.3: Real-time Dashboard Updates
- Story 18.4: Global Search
- Story 18.5: Breadcrumbs Navigation
- Story 18.6: Quick Actions Menu
- Story 18.7: Mobile-First Responsive Design
- Story 18.8: Accessibility Improvements
- Story 18.9: Loading States & Skeleton Screens
- Story 18.10: User Preferences
- Story 18.11: Notifications Preferences
- Story 18.12: Onboarding Experience

## Epic 19: Advanced Features 🟢 MEDIUM

- **[epics-framework-improvements.md](../planning-artifacts/epics-framework-improvements.md#epic-19-advanced-features)** - Advanced Features (high-level planning)
- Story 19.1: Webhook Configuration
- Story 19.2: Webhook Delivery
- Story 19.3: Webhook Testing
- Story 19.4: Batch Workflows
- Story 19.5: Bulk Report Generation
- Story 19.6: Bulk Ticket Operations
- Story 19.7: Workflow Template Versioning
- Story 19.8: Report Versioning
- Story 19.9: Data Snapshots
- Story 19.10: Git Integration
- Story 19.11: CI/CD Integration
- Story 19.12: API Rate Limiting Tiers

## Epic 20: Documentation & Process 🟢 MEDIUM

- **[epics-framework-improvements.md](../planning-artifacts/epics-framework-improvements.md#epic-20-documentation--process)** - Documentation & Process (high-level planning)
- Story 20.1: Audit Existing Documentation
- Story 20.2: Establish Documentation Standards
- Story 20.3: Update Epic 8 (Dashboard & Reports) Stories
- Story 20.4: Update Epic 9 (Pattern Detection) Stories
- Story 20.5: Update Epic 10 (PM Dashboard) Stories
- Story 20.6: Update Epic 11 (Splunk) Stories
- Story 20.7: Update Epic 12 (Support) Stories
- Story 20.8: Update Epic 13 (AI) Stories
- Story 20.9: Create Developer Onboarding Guide
- Story 20.10: Create Story Creation Checklist
- Story 20.11: Automated Documentation Checks
- Story 20.12: Documentation Maintenance Process

## Epic 14: Rust Implementation Improvements (NEW)

- **[sprint-status-rust-improvements.yaml](./sprint-status-rust-improvements.yaml)** - Sprint tracking for Rust improvements
- **[epics-framework-improvements.md](../planning-artifacts/epics-framework-improvements.md)** - Framework improvements epics (15-20)

### Epic 14: Rust Implementation Improvements (Ready for Development)

#### Sprint 1: Reliability & Debugging
- **14.1** - Graceful Shutdown and Signal Handling
- **14.2** - Request ID Middleware for Correlation

#### Sprint 2: Observability - Metrics
- **14.3** - Prometheus Metrics Integration

#### Sprint 3: Performance - Caching
- **14.4** - In-Memory Cache Layer with Moka

#### Sprint 4: Security - Rate Limiting
- **14.5** - Rate Limiting with Tower Governor

#### Sprint 5: Observability - Tracing
- **14.6** - OpenTelemetry Distributed Tracing

#### Sprint 6: Usability - CLI
- **14.7** - CLI Admin Tool

#### Sprint 7: Quality - Integration Tests
- **14.8** - Integration Tests with Testcontainers

### Epic 15: Authentication & Authorization 🔴 CRITICAL (Planned)

**Priority:** 🔴 CRITICAL - Addresses authentication gaps  
**Stories:** 12 | **Estimated:** 7.5 days

#### Sprint 1: Authentication Foundation (2 days)
- **15.1** - JWT Token Authentication
- **15.2** - Auth Middleware for Axum
- **15.3** - User Login/Logout API

#### Sprint 2: Authorization (2 days)
- **15.4** - Role-Based Access Control (RBAC)
- **15.5** - Permission System
- **15.6** - Authorization Decorators

#### Sprint 3: User Management (2 days)
- **15.7** - User CRUD Operations
- **15.8** - Password Hashing & Reset
- **15.9** - User Profile Management

#### Sprint 4: Session Management (1.5 days)
- **15.10** - Refresh Tokens
- **15.11** - Session Invalidation
- **15.12** - Rate Limiting per User

### Epic 16: Reports Enhancement 🟡 HIGH (Planned)

**Priority:** 🟡 HIGH - Fix TODO in reports  
**Stories:** 3 | **Estimated:** 1 day

#### Sprint 1: Time Integration (1 day)
- **16.1** - Fetch Time Sessions by Workflow
- **16.2** - Display Time in Reports
- **16.3** - Historical Time Comparison

### Epic 17: Audit Logging 🟡 HIGH (Planned)

**Priority:** 🟡 HIGH - Track all user actions  
**Stories:** 7 | **Estimated:** 5 days

#### Sprint 1: Audit Infrastructure (2 days)
- **17.1** - Audit Log Database Schema
- **17.2** - Audit Service
- **17.3** - Audit Middleware

#### Sprint 2: Audit API & Integration (1.5 days)
- **17.4** - Audit Log API
- **17.5** - Manual Audit Triggers

#### Sprint 3: Retention & Export (1.5 days)
- **17.6** - Audit Log Retention
- **17.7** - Audit Log Dashboard

### Epic 18: User Experience Improvements 🟢 MEDIUM (Planned)

**Priority:** 🟢 MEDIUM - Better dashboards and navigation  
**Stories:** 12 | **Estimated:** 7 days

#### Sprint 1: Dashboard Enhancements (2 days)
- **18.1** - Customizable Dashboard Layout
- **18.2** - Advanced Dashboard Widgets
- **18.3** - Real-time Dashboard Updates

#### Sprint 2: Navigation & Search (1.5 days)
- **18.4** - Global Search
- **18.5** - Breadcrumbs Navigation
- **18.6** - Quick Actions Menu

#### Sprint 3: Mobile & Accessibility (1.5 days)
- **18.7** - Mobile-First Responsive Design
- **18.8** - Accessibility Improvements
- **18.9** - Loading States & Skeleton Screens

#### Sprint 4: Personalization (2 days)
- **18.10** - User Preferences
- **18.11** - Notifications Preferences
- **18.12** - Onboarding Experience

### Epic 19: Advanced Features 🟢 MEDIUM (Planned)

**Priority:** 🟢 MEDIUM - Webhooks, batch operations, versioning  
**Stories:** 12 | **Estimated:** 8 days

#### Sprint 1: Webhook System (2 days)
- **19.1** - Webhook Configuration
- **19.2** - Webhook Delivery
- **19.3** - Webhook Testing

#### Sprint 2: Batch Operations (2 days)
- **19.4** - Batch Workflows
- **19.5** - Bulk Report Generation
- **19.6** - Bulk Ticket Operations

#### Sprint 3: Versioning & History (2 days)
- **19.7** - Workflow Template Versioning
- **19.8** - Report Versioning
- **19.9** - Data Snapshots

#### Sprint 4: Advanced Integrations (2 days)
- **19.10** - Git Integration
- **19.11** - CI/CD Integration
- **19.12** - API Rate Limiting Tiers

### Epic 20: Documentation & Process 🟢 MEDIUM (Planned)

**Priority:** 🟢 MEDIUM - Address documentation gaps in epics 8-13  
**Stories:** 12 | **Estimated:** 6 days

#### Sprint 1: Documentation Review (1 day)
- **20.1** - Audit Existing Documentation
- **20.2** - Establish Documentation Standards
- **20.3** - Update Epic 8 (Dashboard & Reports) Stories

#### Sprint 2: Story Enhancement (2 days)
- **20.4** - Update Epic 9 (Pattern Detection) Stories
- **20.5** - Update Epic 10 (PM Dashboard) Stories
- **20.6** - Update Epic 11 (Splunk) Stories

#### Sprint 3: Process & Training (2 days)
- **20.7** - Update Epic 12 (Support) Stories
- **20.8** - Update Epic 13 (AI) Stories
- **20.9** - Create Developer Onboarding Guide
- **20.10** - Create Story Creation Checklist

#### Sprint 4: Continuous Documentation (1 day)
- **20.11** - Automated Documentation Checks
- **20.12** - Documentation Maintenance Process

## Configuration

- **[sprint-status.yaml](./sprint-status.yaml)** - Sprint status and story tracking configuration (Epics 1-13)
- **[sprint-status-rust-improvements.yaml](./sprint-status-rust-improvements.yaml)** - Sprint status for Epic 14
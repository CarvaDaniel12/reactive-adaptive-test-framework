```C:\Users\User\Desktop\estrategia preventiva-reativa\_bmad-output\planning-artifacts\epics.md
---
stepsCompleted: [1, 2]
inputDocuments:
  - qa-intelligent-pms/docs/01-architecture.md
  - qa-intelligent-pms/docs/02-technical-decisions.md
  - qa-intelligent-pms/docs/03-data-models.md
  - qa-intelligent-pms/docs/04-workflows.md
  - qa-intelligent-pms/docs/05-integrations.md
  - qa-intelligent-pms/docs/06-setup-guide.md
  - qa-intelligent-pms/docs/07-roadmap.md
  - qa-intelligent-pms/docs/08-interface-web.md
  - qa-intelligent-pms/docs/GUIA-USUARIO-FINAL.md
  - qa-intelligent-pms/docs/GUIA-EXPORTACAO-SPLUNK.md
  - qa-intelligent-pms/docs/ROADMAP-2026.md
  - qa-intelligent-pms/docs/STATUS-ATUAL.md
  - _bmad-output/planning-artifacts/product-brief-estrategia-preventiva-reativa-2026-01-01.md
  - _bmad-output/planning-artifacts/research/technical-rust-best-practices-research-2026-01-01.md
  - _bmad-output/planning-artifacts/prd.md
workflowType: 'epics-and-stories'
lastStep: 2
---

# Epic List - QA Intelligent PMS

## Epic 1: Project Foundation & Core Infrastructure
Development team has a working Rust/React foundation with database connectivity, ready to build features.

**FRs covered:** AR-01 through AR-12 (Architecture setup)
**NFRs addressed:** NFR-SCAL-02 (modular crates), NFR-SEC-01 (encryption foundation)

---

## Epic 2: User Configuration & Setup Wizard
QAs and DevOps can configure the framework with their credentials, and the system validates all connections - ready for daily use.

**FRs covered:** FR-CFG-01, FR-CFG-02, FR-CFG-03, FR-CFG-04, FR-CFG-05, FR-CFG-06, FR-CFG-07
**NFRs addressed:** NFR-INT-02 (startup validation), NFR-SEC-01 (encrypted storage)
**UX addressed:** UX-01, UX-02, UX-03, UX-04, UX-06, UX-07, UX-08

---

## Epic 3: Jira Integration & Ticket Management
Ana (QA) can see her Jira tickets directly in the framework, select tickets to work on, and view full ticket details without leaving the app.

**FRs covered:** FR-INT-01, FR-INT-02, FR-INT-03, FR-INT-04, FR-INT-13, FR-INT-14, FR-INT-15
**NFRs addressed:** NFR-SEC-04 (OAuth 2.0 + PKCE), NFR-PERF-01 (<2s API calls), NFR-REL-03 (retry)
**UX addressed:** UX-05, UX-09

---

## Epic 4: Postman & Testmo Search Integration
When Ana selects a ticket, the framework automatically searches for related tests in Postman and Testmo, showing direct links to relevant test cases.

**FRs covered:** FR-INT-05, FR-INT-06, FR-INT-07, FR-INT-08, FR-INT-09, FR-INT-10, FR-SRC-01, FR-SRC-02, FR-SRC-03, FR-SRC-04
**NFRs addressed:** NFR-PERF-03 (<3s search), NFR-REL-03 (retry)

---

## Epic 5: Workflow Engine & Guided Testing
Ana can follow step-by-step guided workflows based on ticket type, with automatic progress tracking, notes per step, and workflow state that persists even if she closes the browser.

**FRs covered:** FR-WRK-01, FR-WRK-02, FR-WRK-03, FR-WRK-04, FR-WRK-05, FR-WRK-06, FR-WRK-07
**NFRs addressed:** NFR-REL-01 (99.5% uptime), NFR-PERF-01 (<2s responses)
**UX addressed:** UX-01, UX-07

---

## Epic 6: Time Tracking & Estimation
Ana has automatic time tracking that starts when she begins a workflow, with real-time display, and sees her actual time vs estimates - enabling her to prove her capacity with data.

**FRs covered:** FR-TRK-01, FR-TRK-02, FR-TRK-03, FR-TRK-04, FR-TRK-05, FR-TRK-06, FR-TRK-07
**NFRs addressed:** NFR-PERF-01 (<2s for tracking operations)
**UX addressed:** UX-01

---

## Epic 7: Reporting & Documentation
When Ana completes a workflow, a professional report is automatically generated showing steps completed, time breakdown, tests covered, and strategies used - exportable for sharing.

**FRs covered:** FR-RPT-01, FR-RPT-02, FR-RPT-03, FR-RPT-04, FR-RPT-05, FR-RPT-06
**NFRs addressed:** NFR-PERF-02 (<5s for report generation)

---

## Epic 8: QA Individual Dashboard
Ana sees her personal performance dashboard with tickets completed, time metrics, identified gaps, and insights to improve her testing strategy.

**FRs covered:** FR-DSH-01, FR-DSH-05, FR-DSH-06, FR-DSH-07
**NFRs addressed:** NFR-PERF-02 (<5s dashboard load)
**UX addressed:** UX-01, UX-04

---

## Epic 9: Pattern Detection & Proactive Alerts
The framework automatically detects anomalies (excessive time, consecutive problems) and alerts Ana/Carlos proactively with actionable context.

**FRs covered:** FR-PTN-01, FR-PTN-02, FR-PTN-03, FR-PTN-04, FR-PTN-05
**NFRs addressed:** NFR-REL-02 (health checks), NFR-INT-03 (monitoring)

---

## Epic 10: PM/PO Observability Dashboard
Carlos (PM) sees a consolidated dashboard with bugs discovered vs prevented, economy metrics, component health, and can export reports for stakeholder meetings.

**FRs covered:** FR-DSH-02, FR-DSH-03, FR-DSH-04, FR-DSH-08
**NFRs addressed:** NFR-PERF-02 (<5s dashboard load)
**UX addressed:** UX-01

---

## Epic 11: Splunk Log Integration
Ana can query Splunk logs directly from the framework using pre-built templates, viewing production data without switching to Splunk UI.

**FRs covered:** FR-INT-11, FR-INT-12
**NFRs addressed:** NFR-PERF-01 (<2s for queries)

---

## Epic 12: Support Portal & Troubleshooting
Sofia (Support) can quickly diagnose and resolve QA issues using automatic error logs, diagnostic tools, and a knowledge base of common solutions.

**FRs covered:** FR-SUP-01, FR-SUP-02, FR-SUP-03, FR-SUP-04, FR-SUP-05
**NFRs addressed:** NFR-REL-04 (30-day log retention)

---

## Epic 13: AI Companion (Optional - BYOK)
QAs who provide their own AI API key get semantic search, Gherkin-based test suggestions, and a helpful mini-chatbot for framework assistance.

**FRs covered:** FR-AI-01, FR-AI-02, FR-AI-03, FR-AI-04, FR-AI-05, FR-AI-06, FR-AI-07
**UX addressed:** UX-10

---

## Epic 14: Rust Implementation Improvements
Maximize Rust potential in the project by implementing production-grade observability, performance optimizations, and developer experience improvements.

**Priority:** P0 (Critical for Production Readiness)

**Dependencies:** 
- Epic 1 (Project Foundation) - Complete ✅
- All existing 13 epics - Complete ✅

**Stories:** 8
- 14.1: Graceful Shutdown and Signal Handling
- 14.2: Request ID Middleware for Correlation
- 14.3: Prometheus Metrics Integration
- 14.4: In-Memory Cache Layer with Moka
- 14.5: Rate Limiting with Tower Governor
- 14.6: OpenTelemetry Distributed Tracing
- 14.7: CLI Admin Tool
- 14.8: Integration Tests with Testcontainers

---

## Epic 15: Authentication & Authorization
Implement comprehensive authentication and authorization system with JWT, OAuth 2.0, RBAC, MFA, audit logging, and security features.

**Priority:** P0 (Critical)

**Dependencies:** 
- Epic 1 (Project Foundation) - Complete ✅
- Epic 8 (Dashboards) - Complete ✅

**Stories:** 12
- 15.1: JWT Authentication with Refresh Tokens
- 15.2: OAuth 2.0 + PKCE (OpenID Connect)
- 15.3: Role-Based Access Control (RBAC)
- 15.4: Multi-Factor Authentication (MFA)
- 15.5: Password Policies & Reset Flows
- 15.6: Session Management & Refresh Tokens
- 15.7: API Key Authentication for Service Accounts
- 15.8: Permission System with Granular Controls
- 15.9: Admin Dashboard for User Management
- 15.10: Audit Trail for Auth Events
- 15.11: Rate Limiting for Auth Endpoints
- 15.12: Security Headers & CSRF Protection

---

## Epic 16: Reports Enhancement
Expand reporting capabilities with advanced filtering, custom templates, and automated scheduling to support deep analysis and recurring needs.

**Priority:** P1

**Dependencies:** 
- Epic 7 (Reporting & Documentation) - Complete ✅

**Stories:** 3
- 16.1: Advanced Report Filtering and Search
- 16.2: Custom Report Templates
- 16.3: Report Scheduling and Automation

---

## Epic 17: Audit Logging
Implement comprehensive audit logging system for security, compliance, and forensic analysis of all system activities.

**Priority:** P0 (Critical)

**Dependencies:** 
- Epic 15 (Authentication & Authorization) - Complete ✅

**Stories:** 7
- 17.1: Comprehensive Audit Log Storage
- 17.2: Audit Event Categories and Taxonomy
- 17.3: Audit Log Search and Filtering
- 17.4: Audit Report Generation
- 17.5: Audit Log Retention and Archival
- 17.6: Real-time Audit Monitoring Dashboard
- 17.7: Compliance Export for Audits

---

## Epic 18: User Experience Improvements
Enhance UX based on user feedback and identified gaps to increase adoption and efficiency.

**Priority:** P1

**Dependencies:** 
- Epic 7 (Reports) - Complete ✅
- Epic 15 (Authentication) - Complete ✅

**Stories:** 12
- 18.1: Onboarding Wizard Enhancement
- 18.2: Interactive Tutorials and Walkthroughs
- 18.3: Keyboard Shortcuts and Hotkeys
- 18.4: Drag-and-Drop Workflow Customization
- 18.5: Dark Mode Support
- 18.6: Responsive Mobile Improvements
- 18.7: Accessibility (WCAG 2.1 AA) Compliance
- 18.8: Performance Optimizations for Large Datasets
- 18.9: Personalized Dashboard Widgets
- 18.10: Notification Preferences and Channels
- 18.11: Offline Mode Enhancements
- 18.12: User Feedback and In-App Voting

---

## Epic 19: Advanced Features
Implement advanced features to maximize framework value and prepare for enterprise scale and AI evolution.

**Priority:** P1

**Dependencies:** 
- Epic 13 (AI Companion) - Complete ✅
- Epic 15 (Authentication) - Complete ✅

**Stories:** 12
- 19.1: Workflow Marketplace and Sharing
- 19.2: Advanced AI-Powered Test Suggestions
- 19.3: Automated Test Generation from Requirements
- 19.4: Predictive Bug Risk Scoring
- 19.5: Intelligent Test Case Prioritization
- 19.6: Cross-Team Collaboration Features
- 19.7: Multi-Tenant Support
- 19.8: Advanced RBAC with Dynamic Permissions
- 19.9: API Rate Limiting per User/Team
- 19.10: Webhook System for Integrations
- 19.11: Sandbox Environment for Safe Testing
- 19.12: Performance Analytics and Tuning

---

## Epic 20: Documentation & Process
Create comprehensive documentation and process resources to support adoption, onboarding, and knowledge sharing.

**Priority:** P1

**Dependencies:** 
- Epic 7 (Reports) - Complete ✅
- Epic 14 (Rust Improvements) - Complete ✅
- Epic 15 (Authentication) - Complete ✅

**Stories:** 12
- 20.1: Complete Developer Documentation
- 20.2: Internal Process Documentation
- 20.3: User Training Materials
- 20.4: Video Tutorials and Screencasts
- 20.5: FAQ and Knowledge Base Portal
- 20.6: Release Notes and Changelog
- 20.7: API Documentation Portal
- 20.8: Integration Guides for New Tools
- 20.9: Troubleshooting Guides
- 20.10: Best Practices and Patterns Library
- 20.11: Security Documentation
- 20.12: Documentation Versioning and Search

---

## Summary

**Total Epics:** 20
**Total Stories:** 100+ (estimated)
- Epics 1-15: 70 stories (documented in existing detailed files)
- Epics 16-20: 46 stories (to be created in next steps)

**Implementation Readiness:** All epics are defined with clear requirements, dependencies mapped, and ready for sprint planning.

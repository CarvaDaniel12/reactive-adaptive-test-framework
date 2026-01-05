# Framework Improvements Epics - QA Intelligent PMS

**Document Version:** 1.0  
**Created:** 2026-01-07  
**Author:** BMad Framework Team  
**Status:** Ready for Planning

---

## Executive Summary

This document defines **6 new epics** (15-20) to address critical gaps identified in the codebase analysis:

| Epic | Name | Priority | Stories | Est. Days | Status |
|-------|------|----------|-----------|--------|
| 15 | Authentication & Authorization | 🔴 CRITICAL | 12 | planned |
| 16 | Reports Enhancement | 🟡 HIGH | 3 | planned |
| 17 | Audit Logging | 🟡 HIGH | 5 | planned |
| 18 | User Experience Improvements | 🟢 MEDIUM | 7 | planned |
| 19 | Advanced Features | 🟢 MEDIUM | 8 | planned |
| 20 | Documentation & Process | 🟢 MEDIUM | 6 | planned |

**Total:** 41 stories, 41 estimated days

---

## Epic 15: Authentication & Authorization

**Priority:** 🔴 CRITICAL  
**Focus:** Security - Fix critical authentication gaps  
**Status:** planned

### Problem Statement

The codebase currently has **NO real authentication**. Multiple TODO comments exist:
```rust
// TODO: Get user_id from auth context
let user_id = Uuid::new_v4(); // Placeholder
```

**Affected Routes:**
- `/api/v1/splunk/*` - All routes use placeholder user IDs
- `/api/v1/workflows/*` - User filtering not enforced
- `/api/v1/reports/*` - Reports accessible by anyone

**Risk:** Any user can access, modify, or delete any other user's data.

### Stories

#### Sprint 1: Authentication Foundation (2 days)

**Story 15.1: JWT Token Authentication**
- Implement JWT token generation and validation
- Use `jsonwebtoken` crate with RS256 asymmetric signing
- Token expiration: 15 minutes (access), 7 days (refresh)
- Store public/private keys securely
- **Files to Create:** `crates/qa-pms-auth/src/lib.rs`, `crates/qa-pms-auth/src/jwt.rs`
- **Files to Modify:** `Cargo.toml`, `crates/qa-pms-config/src/lib.rs`

**Story 15.2: Auth Middleware for Axum**
- Create middleware to extract and validate JWT tokens
- Extract user claims and inject into request state
- Handle expired/invalid tokens gracefully
- Return 401 Unauthorized for missing tokens
- **Files to Create:** `crates/qa-pms-api/src/middleware/auth.rs`
- **Files to Modify:** `crates/qa-pms-api/src/app.rs`

**Story 15.3: User Login/Logout API**
- Login endpoint: validate credentials, return JWT tokens
- Logout endpoint: invalidate refresh token
- Password hashing with Argon2id
- Rate limiting on login endpoint (5 attempts/5 minutes)
- **Files to Create:** `crates/qa-pms-api/src/routes/auth.rs`
- **Files to Modify:** `crates/qa-pms-api/src/routes/mod.rs`, `crates/qa-pms-core/src/lib.rs`

#### Sprint 2: Authorization (2 days)

**Story 15.4: Role-Based Access Control (RBAC)**
- Define roles: ADMIN, QA_ENGINEER, QA_MANAGER, VIEWER
- Create roles table in database
- Assign default role on user creation
- **Files to Create:** Migrations, `crates/qa-pms-core/src/roles.rs`
- **Files to Modify:** `migrations/`, `crates/qa-pms-api/src/middleware/auth.rs`

**Story 15.5: Permission System**
- Define permissions per resource type
- Permissions: workflows:read, workflows:write, reports:read, etc.
- Role-to-permission mapping table
- Permission check utility function
- **Files to Create:** `crates/qa-pms-core/src/permissions.rs`
- **Files to Modify:** Migrations

**Story 15.6: Authorization Decorators**
- Create `#[require_role]` macro for Axum handlers
- Create `#[require_permission]` macro
- Automatically inject user context
- Return 403 Forbidden for unauthorized access
- **Files to Create:** `crates/qa-pms-api/src/middleware/authorization.rs`
- **Files to Modify:** `Cargo.toml` (add `axum-macros`)

#### Sprint 3: User Management (2 days)

**Story 15.7: User CRUD Operations**
- Create user endpoint: registration with validation
- Read user: GET /api/v1/users/:id
- Update user: PATCH /api/v1/users/:id (self or admin)
- Delete user: DELETE /api/v1/users/:id (admin only)
- **Files to Modify:** `crates/qa-pms-api/src/routes/setup.rs` (rename/expand to users.rs)

**Story 15.8: Password Hashing & Reset**
- Use `argon2` crate for password hashing
- Password reset flow: send email with reset token
- Reset token: single-use, 1-hour expiration
- Password strength validation
- **Files to Modify:** `crates/qa-pms-api/src/routes/auth.rs`

**Story 15.9: User Profile Management**
- Profile API: GET/PUT /api/v1/users/profile
- Update display name, avatar URL, preferences
- Update password endpoint with old password verification
- **Files to Create:** `crates/qa-pms-api/src/routes/profile.rs`
- **Files to Modify:** `crates/qa-pms-api/src/routes/mod.rs`, Frontend profile page

#### Sprint 4: Session Management (1.5 days)

**Story 15.10: Refresh Tokens**
- Refresh token endpoint: POST /api/v1/auth/refresh
- Validate refresh token, issue new access token
- Rotate refresh tokens on refresh
- Revoke all refresh tokens on password change
- **Files to Modify:** `crates/qa-pms-auth/src/jwt.rs`, `crates/qa-pms-api/src/routes/auth.rs`

**Story 15.11: Session Invalidation**
- Revoke specific session endpoint
- Revoke all sessions endpoint (logout everywhere)
- Track active sessions in database
- Session list in user profile
- **Files to Create:** Migrations for sessions table
- **Files to Modify:** `crates/qa-pms-api/src/routes/profile.rs`

**Story 15.12: Rate Limiting per User**
- Enhance rate limiting from Epic 14.5 to use user ID
- Different limits per role:
  - VIEWER: 50 req/min
  - QA_ENGINEER: 100 req/min
  - ADMIN: 200 req/min
- **Files to Modify:** `crates/qa-pms-api/src/rate_limit.rs`

### Dependencies

```toml
[workspace.dependencies]
jsonwebtoken = "9.2"
argon2 = "0.5"
axum-macros = "0.4"
```

### Success Metrics

- All TODOs for "user_id from auth context" removed
- 100% of protected routes have auth middleware
- Login endpoint rate-limited
- Password hashing with Argon2id
- JWT tokens with proper expiration
- Role-based permissions enforced

---

## Epic 16: Reports Enhancement

**Priority:** 🟡 HIGH  
**Focus:** Fix TODO in reports - integrate time tracking  
**Status:** planned

### Problem Statement

```rust
// qa-pms-api/src/routes/reports.rs:182
time_seconds: 0, // TODO: Get from time sessions
```

Reports don't show actual time spent per workflow step, making them less useful.

### Stories

#### Sprint 1: Time Integration (1 day)

**Story 16.1: Fetch Time Sessions by Workflow**
- Query time_sessions table for workflow ID
- Aggregate time per step
- Calculate total time
- **Files to Modify:** `crates/qa-pms-api/src/routes/reports.rs`

**Story 16.2: Display Time in Reports**
- Add time_seconds to ReportStep
- Display formatted time in report UI
- Add "Time Breakdown" section to reports
- **Files to Modify:** `crates/qa-pms-api/src/routes/reports.rs`, Frontend report components

**Story 16.3: Historical Time Comparison**
- Compare current workflow time to historical average
- Show "Faster than average" or "Slower than average"
- Highlight significant deviations (>20%)
- **Files to Modify:** `crates/qa-pms-api/src/routes/reports.rs`, Frontend

### Success Metrics

- TODO comment removed
- All reports show accurate time data
- Historical comparison displayed
- Report format tested end-to-end

---

## Epic 17: Audit Logging

**Priority:** 🟡 HIGH  
**Focus:** Security/compliance - track all user actions  
**Status:** planned

### Problem Statement

No audit log exists. Cannot track:
- Who created/modified/deleted what
- When actions occurred
- What data changed

Critical for compliance (SOC2, ISO27001, etc.) and security investigations.

### Stories

#### Sprint 1: Audit Infrastructure (2 days)

**Story 17.1: Audit Log Database Schema**
- Create audit_logs table:
  - id, user_id, action, resource_type, resource_id
  - old_value, new_value (JSONB)
  - ip_address, user_agent, timestamp
- Indexes on user_id, resource_type, timestamp
- **Files to Create:** Migration file

**Story 17.2: Audit Service**
- Create AuditService in qa-pms-core
- Methods: log_action(), query_logs()
- Async logging to minimize impact
- Background writer for performance
- **Files to Create:** `crates/qa-pms-core/src/audit.rs`

**Story 17.3: Audit Middleware**
- Auto-log all HTTP requests
- Log method, path, status, user_id
- Filter sensitive endpoints (login, password reset)
- **Files to Create:** `crates/qa-pms-api/src/middleware/audit.rs`
- **Files to Modify:** `crates/qa-pms-api/src/app.rs`

#### Sprint 2: Audit API & Integration (1.5 days)

**Story 17.4: Audit Log API**
- GET /api/v1/audit/logs - with filters
- Filters: user_id, action, resource_type, date range
- Pagination support
- Export to CSV
- **Files to Create:** `crates/qa-pms-api/src/routes/audit.rs`

**Story 17.5: Manual Audit Triggers**
- Add `audit!` macro for manual logging
- Track sensitive actions: user role change, config update
- Log workflow completions, report generation
- **Files to Modify:** Multiple route files

#### Sprint 3: Retention & Export (1.5 days)

**Story 17.6: Audit Log Retention**
- Configurable retention policy (default: 90 days)
- Automatic cleanup of old logs
- Archive to external storage (S3, GCS) option
- **Files to Modify:** `crates/qa-pms-core/src/audit.rs`, Configuration

**Story 17.7: Audit Log Dashboard**
- Visual dashboard for audit events
- Timeline view of user actions
- Filter by user, date, action type
- Export functionality
- **Files to Create:** Frontend AuditDashboardPage

### Success Metrics

- All write operations logged
- Audit API functional with filters
- Dashboard displays audit trail
- Retention policy enforced
- Export to CSV working

---

## Epic 18: User Experience Improvements

**Priority:** 🟢 MEDIUM  
**Focus:** Better dashboards, navigation, user satisfaction  
**Status:** planned

### Problem Statement

User feedback indicates need for:
- More intuitive navigation
- Better dashboard customization
- Improved mobile experience
- Keyboard shortcuts for power users

### Stories

#### Sprint 1: Dashboard Enhancements (2 days)

**Story 18.1: Customizable Dashboard Layout**
- Drag-and-drop dashboard widgets
- Save/load dashboard layouts per user
- Widget size adjustment
- Add/remove widgets (KPICards, TrendChart, etc.)
- **Files to Modify:** Frontend dashboard, create widget system

**Story 18.2: Advanced Dashboard Widgets**
- New widgets:
  - Work in Progress list
  - Upcoming deadlines
  - Team performance comparison
  - Recent alerts widget
- Widget configuration options
- **Files to Create:** Multiple widget components

**Story 18.3: Real-time Dashboard Updates**
- WebSocket connection for real-time updates
- Update KPIs without page refresh
- Show workflow progress in real-time
- New alert notifications
- **Files to Create:** `crates/qa-pms-api/src/websocket/mod.rs`

#### Sprint 2: Navigation & Search (1.5 days)

**Story 18.4: Global Search**
- Cmd+K / Ctrl+K keyboard shortcut
- Search across tickets, workflows, reports, templates
- Fuzzy matching with Fuse.js
- Quick navigation to results
- **Files to Create:** Frontend GlobalSearch component

**Story 18.5: Breadcrumbs Navigation**
- Breadcrumb trail for all pages
- Click to navigate to parent
- Display current context
- **Files to Modify:** Layout component, add breadcrumbs

**Story 18.6: Quick Actions Menu**
- Cmd+Shift+P / Ctrl+Shift+P for quick actions
- Common actions: create workflow, new report, search tickets
- Keyboard-focused selection
- **Files to Create:** Frontend QuickActions component

#### Sprint 3: Mobile & Accessibility (1.5 days)

**Story 18.7: Mobile-First Responsive Design**
- Improve mobile dashboard (stacked layout)
- Mobile-friendly workflow execution
- Touch-optimized controls
- Hamburger menu for navigation
- **Files to Modify:** Multiple frontend components

**Story 18.8: Accessibility Improvements**
- ARIA labels on all interactive elements
- Keyboard navigation support
- Screen reader compatibility
- Focus management
- High contrast mode option
- **Files to Modify:** All components, add a11y testing

**Story 18.9: Loading States & Skeleton Screens**
- Skeleton loaders for all pages
- Optimistic UI updates
- Progressive loading for charts
- Error boundaries for graceful failures
- **Files to Modify:** Frontend page components

#### Sprint 4: Personalization (2 days)

**Story 18.10: User Preferences**
- Theme selection (light/dark/auto)
- Timezone setting
- Language/locale selection
- Density setting (compact/comfortable)
- **Files to Create:** Preferences API, Frontend settings page

**Story 18.11: Notifications Preferences**
- Email notification preferences
- In-app notification settings
- Browser push notifications
- Webhook notifications for external systems
- **Files to Create:** Notification preferences table, API

**Story 18.12: Onboarding Experience**
- First-run tour for new users
- Interactive tooltips for features
- Progressive disclosure for advanced options
- Skipable but re-openable
- **Files to Create:** Onboarding system, tour components

### Success Metrics

- User satisfaction score > 4.5/5.0
- Reduced time to complete common tasks
- Mobile accessibility score > 90
- Accessibility WCAG 2.1 AA compliant
- Dashboard customization adoption > 60%

---

## Epic 19: Advanced Features

**Priority:** 🟢 MEDIUM  
**Focus:** Webhooks, batch operations, versioning  
**Status:** planned

### Stories

#### Sprint 1: Webhook System (2 days)

**Story 19.1: Webhook Configuration**
- Create webhooks table
- Webhook CRUD API
- Webhook events: workflow.created, workflow.completed, pattern.detected
- **Files to Create:** Migration, webhook service

**Story 19.2: Webhook Delivery**
- Async webhook delivery with retries
- Delivery status tracking
- Retry on failure (exponential backoff)
- Webhook signatures (HMAC)
- **Files to Create:** Webhook delivery worker

**Story 19.3: Webhook Testing**
- Test webhook endpoint
- Send test payload
- View delivery logs
- **Files to Create:** Webhook testing UI

#### Sprint 2: Batch Operations (2 days)

**Story 19.4: Batch Workflows**
- Start multiple workflows at once
- Bulk status update
- Batch CSV export
- Progress tracking for batch operations
- **Files to Create:** Batch operations API, Frontend batch UI

**Story 19.5: Bulk Report Generation**
- Generate reports for multiple workflows
- Schedule bulk reports
- Email reports on completion
- **Files to Modify:** Reports API

**Story 19.6: Bulk Ticket Operations**
- Bulk ticket status update
- Bulk ticket assignment
- Import tickets from CSV
- **Files to Modify:** Tickets API

#### Sprint 3: Versioning & History (2 days)

**Story 19.7: Workflow Template Versioning**
- Version tracking for templates
- Compare versions (diff view)
- Rollback to previous version
- Version history API
- **Files to Create:** Template versions table, versioning service

**Story 19.8: Report Versioning**
- Version history for reports
- Restore previous report versions
- Report revision notes
- **Files to Create:** Report versions table

**Story 19.9: Data Snapshots**
- Periodic data snapshots
- Backup/restore functionality
- Export snapshot to file
- Import from snapshot
- **Files to Create:** Snapshot system, CLI commands

#### Sprint 4: Advanced Integrations (2 days)

**Story 19.10: Git Integration**
- Connect workflows to Git repositories
- Link test cases to commits
- Branch-based workflow execution
- **Files to Create:** Git integration crate

**Story 19.11: CI/CD Integration**
- Trigger workflows from CI pipelines
- GitHub Actions, GitLab CI support
- Webhook handlers for CI events
- **Files to Create:** CI/CD integration service

**Story 19.12: API Rate Limiting Tiers**
- Multiple rate limit tiers
- Tier management in admin
- Usage analytics per tier
- Upgrade notifications
- **Files to Modify:** Rate limiting system

### Success Metrics

- Webhooks deliver > 99.5% reliability
- Batch operations reduce time by 80%
- Versioning enables rollbacks
- CI/CD integration adoption > 40%

---

## Epic 20: Documentation & Process

**Priority:** 🟢 MEDIUM  
**Focus:** Address documentation gaps in epics 8-13  
**Status:** planned

### Problem Statement

Epics 8-13 have **inconsistent documentation quality**:
- Some stories lack detailed acceptance criteria
- Missing user journey flows
- Incomplete testing strategies
- No clear success metrics

Example (Epic 8 vs Epic 14):
- Epic 8 Story 8.1: Basic story format, minimal details
- Epic 14 Story 14.1: Comprehensive with code examples, testing, risks

### Stories

#### Sprint 1: Documentation Review (1 day)

**Story 20.1: Audit Existing Documentation**
- Review all stories in epics 8-13
- Identify gaps: acceptance criteria, testing, examples
- Create documentation gaps report
- **Deliverable:** Markdown report

**Story 20.2: Establish Documentation Standards**
- Define story template with required sections
- Standards for: acceptance criteria (Gherkin), testing strategy, code examples
- Create documentation checklist
- **Deliverable:** Documentation template and checklist

**Story 20.3: Update Epic 8 (Dashboard & Reports) Stories**
- Enhance stories 8.1-8.6 with missing details
- Add acceptance criteria in Gherkin format
- Add testing strategies
- Add code examples where applicable
- **Files to Modify:** Update story files in `_bmad-output/implementation-artifacts/`

#### Sprint 2: Story Enhancement (2 days)

**Story 20.4: Update Epic 9 (Pattern Detection) Stories**
- Enhance stories 9.1-9.5
- Add detailed acceptance criteria
- Document pattern detection algorithms
- Add testing edge cases
- **Files to Modify:** Update story files

**Story 20.5: Update Epic 10 (PM Dashboard) Stories**
- Enhance stories 10.1-10.6
- Add business logic documentation
- Add metric calculation formulas
- Add testing scenarios
- **Files to Modify:** Update story files

**Story 20.6: Update Epic 11 (Splunk) Stories**
- Enhance stories 11.1-11.3
- Add SPL query examples
- Document Splunk integration flow
- Add error handling scenarios
- **Files to Modify:** Update story files

#### Sprint 3: Process & Training (2 days)

**Story 20.7: Update Epic 12 (Support) Stories**
- Enhance stories 12.1-12.5
- Add troubleshooting guides
- Document diagnostic flows
- Add knowledge base examples
- **Files to Modify:** Update story files

**Story 20.8: Update Epic 13 (AI) Stories**
- Enhance stories 13.1-13.6
- Add AI prompt examples
- Document BYOK configuration
- Add AI fallback scenarios
- **Files to Modify:** Update story files

**Story 20.9: Create Developer Onboarding Guide**
- Step-by-step guide for new developers
- Architecture overview
- How to run tests locally
- How to add new features
- **Deliverable:** `docs/DEVELOPER-GUIDE.md`

**Story 20.10: Create Story Creation Checklist**
- Checklist for creating new stories
- Validation steps before story approval
- Code review checklist
- **Deliverable:** `docs/STORY-CHECKLIST.md`

#### Sprint 4: Continuous Documentation (1 day)

**Story 20.11: Automated Documentation Checks**
- Pre-commit hook to validate story format
- CI check for documentation completeness
- Lint for common documentation issues
- **Files to Create:** Documentation validation script

**Story 20.12: Documentation Maintenance Process**
- Schedule for periodic documentation review
- Owner assignment per epic
- Documentation KPI tracking (completeness, accuracy)
- **Deliverable:** Documentation maintenance plan

### Success Metrics

- All stories follow new documentation standard
- Documentation completeness > 95%
- Developer onboarding time reduced by 40%
- Documentation maintenance process established

---

## Dependencies Summary

### New Workspace Dependencies

```toml
[workspace.dependencies]
# Authentication
jsonwebtoken = "9.2"
argon2 = "0.5"

# WebSockets
tokio-tungstenite = "0.21"

# Database migrations
sqlx = { version = "0.7", features = ["migrate"] } # Already exists

# Documentation
```

### New Crates

```toml
# Cargo.toml members
members = [
    # ... existing ...
    "crates/qa-pms-auth",        # NEW - Authentication crate
    "crates/qa-pms-websocket",   # NEW - WebSocket support
    "crates/qa-pms-audit",       # NEW - Audit logging (or add to core)
]
```

### Database Schema Changes

**New Tables:**
- users (user authentication, profiles)
- roles, permissions, user_roles (RBAC)
- sessions (refresh tokens)
- audit_logs (audit trail)
- webhooks, webhook_delivers (webhook system)
- template_versions (versioning)
- report_versions (versioning)
- data_snapshots (backups)
- notifications (user preferences)
- ci_integrations (CI/CD connections)

---

## Risk Assessment

| Epic | Risk | Probability | Impact | Mitigation |
|-------|-------|-------------|--------|------------|
| 15 - Auth | Breaking changes for existing users | Medium | High | Migration plan, deprecation period |
| 15 - Auth | Token vulnerabilities | Low | Critical | Security audit, penetration testing |
| 17 - Audit | Performance impact | Medium | Medium | Async logging, background writer |
| 18 - UX | Low adoption of new features | Medium | Medium | A/B testing, user feedback loops |
| 19 - Advanced | Complexity increase | High | Medium | Incremental rollout, monitoring |
| 20 - Docs | Outdated documentation | High | Medium | Automated checks, owner assignment |

---

## Success Metrics by Epic

### Epic 15 - Authentication
- 100% of routes protected
- Zero unauthenticated access vulnerabilities
- Password policies enforced
- Session timeout working

### Epic 16 - Reports
- All reports show accurate time data
- Historical comparisons displayed
- Report generation < 3 seconds

### Epic 17 - Audit Logging
- All write operations logged
- Audit query response time < 500ms
- 90-day retention enforced
- Audit export functional

### Epic 18 - UX Improvements
- User satisfaction > 4.5/5
- Mobile accessibility > 90%
- Keyboard shortcuts used by > 40% of users
- Dashboard customization > 60%

### Epic 19 - Advanced Features
- Webhook delivery > 99.5%
- Batch operations reduce time > 80%
- Versioning enables rollbacks
- CI/CD integration > 40%

### Epic 20 - Documentation
- Documentation completeness > 95%
- Developer onboarding time < 2 hours
- All stories follow standard format
- Documentation maintained quarterly

---

## Implementation Recommendations

### Priority Order

**Phase 1 (Critical - 2 weeks):**
1. Epic 15: Authentication & Security 🔴
   - MUST complete before production deployment
   - Stories 15.1-15.6 (auth foundation)

**Phase 2 (High Priority - 1.5 weeks):**
2. Epic 16: Reports Enhancement 🟡
3. Epic 17: Audit Logging 🟡
   - Can be done in parallel

**Phase 3 (User Experience - 2 weeks):**
4. Epic 18: UX Improvements 🟢
5. Epic 20: Documentation & Process 🟢
   - Start documentation review early

**Phase 4 (Advanced Features - 2 weeks):**
6. Epic 19: Advanced Features 🟢
   - Webhooks and batch operations first
   - CI/CD integration last

### Sprint Planning

**Sprint 1 (Week 1-2):** Epic 15 Sprint 1-2 (4 days) + Epic 16 (1 day)
**Sprint 2 (Week 3-4):** Epic 15 Sprint 3-4 (3.5 days) + Epic 17 Sprint 1 (2 days)
**Sprint 3 (Week 5-6):** Epic 17 Sprint 2-3 (3 days) + Epic 18 Sprint 1 (2 days)
**Sprint 4 (Week 7-8):** Epic 18 Sprint 2-4 (5 days) + Epic 20 Sprint 1 (1 day)
**Sprint 5 (Week 9-10):** Epic 20 Sprint 2-3 (4 days) + Epic 19 Sprint 1 (2 days)
**Sprint 6 (Week 11-12):** Epic 19 Sprint 2-4 (6 days) + Epic 20 Sprint 4 (2 days)

**Total:** 6 sprints, ~12 weeks, 41 stories

---

## Next Steps

1. **Review and Approve** this epic plan with stakeholders
2. **Prioritize** stories based on business needs
3. **Create Detailed Story Documents** for each story (following Epic 14 template)
4. **Update Workflow Status** to include new epics
5. **Begin Sprint 1** with Epic 15 (Authentication) as highest priority

---

## Appendix: Story Template

All new stories should follow this template (established in Epic 14):

```markdown
# Story XX.Y: Story Title

**As a** [user type]
**I want** [goal]
**So that** [benefit]

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | XX.Y |
| Epic | [Epic Name] |
| Sprint | [Sprint Number] - [Focus Area] |
| Priority | [P0/P1/P2] |
| Estimated Days | [X] |
| Dependencies | [Story IDs] |
| Status | [ready-for-dev/in-progress/done] |

---

## Technical Requirements

1. [Detailed requirement 1]
2. [Detailed requirement 2]
...

---

## Acceptance Criteria

Given/When/Then format:
- [ ] Given [context] When [action] Then [outcome]
- [ ] Given [context] When [action] Then [outcome]
...

---

## Implementation Notes

Code examples, architectural decisions, edge cases.

---

## Files to Create/Modify

| File | Changes |
|------|---------|
| path/file | Description |

---

## Testing Strategy

Unit tests, integration tests, manual tests.

---

## Success Metrics

Measurable outcomes.

---

## Context & Dependencies

Depends on, enables.
```

---

**Document End**

For questions or feedback, please contact the BMad Framework Team.
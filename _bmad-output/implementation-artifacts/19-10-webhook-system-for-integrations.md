# Story 19.10: Webhook System for Integrations

Status: ready-for-dev

## Story

**As a** Integration Developer  
**I want** a webhook system for integrations  
**So that** I can integrate with external systems via webhooks

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 19.10 |
| Epic | Epic 19: Advanced Features |
| Sprint | Sprint 3: Integrations |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Epic 3 (Jira Integration), Epic 4 (Postman & Testmo) - Complete ✅ |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create webhook system
   - Add `webhooks` table (id, url, events, secret, enabled, created_at)
   - Add `webhook_deliveries` table (webhook_id, status, payload, response, created_at)
   - Support webhook registration
   - Support webhook management

2. Implement webhook delivery
   - HTTP POST webhook delivery
   - Retry logic (exponential backoff)
   - Signature signing (HMAC)
   - Delivery status tracking

3. Support webhook events
   - Ticket events (created, updated, status changed)
   - Workflow events (started, completed, step completed)
   - Test events (test run created, test run completed)
   - Custom events (user-defined)

4. Create webhook management UI
   - Register webhooks
   - View webhook delivery history
   - Test webhooks
   - Enable/disable webhooks

---

## Acceptance Criteria

- [ ] **Given** webhook system exists  
  **When** registering webhook  
  **Then** webhook is saved and activated

- [ ] **Given** webhook system exists  
  **When** event occurs  
  **Then** webhook is delivered to registered URL

- [ ] **Given** webhook system exists  
  **When** webhook delivery fails  
  **Then** webhook is retried with exponential backoff

- [ ] **Given** webhook system exists  
  **When** viewing webhook deliveries  
  **Then** delivery history is displayed with status

---

## Tasks / Subtasks

- [ ] Task 1: Create webhooks database schema
  - [ ] 1.1: Create migration: `YYYYMMDDHHMMSS_create_webhooks_tables.sql`
  - [ ] 1.2: Define `webhooks` table
  - [ ] 1.3: Define `webhook_deliveries` table
  - [ ] 1.4: Add indexes

- [ ] Task 2: Create webhook service
  - [ ] 2.1: Create `crates/qa-pms-webhooks/Cargo.toml`
  - [ ] 2.2: Create `crates/qa-pms-webhooks/src/webhook_service.rs`
  - [ ] 2.3: Implement webhook delivery
  - [ ] 2.4: Implement retry logic

- [ ] Task 3: Create webhook API endpoints
  - [ ] 3.1: Create `crates/qa-pms-api/src/routes/webhooks.rs`
  - [ ] 3.2: POST /api/v1/webhooks - register webhook
  - [ ] 3.3: GET /api/v1/webhooks - list webhooks
  - [ ] 3.4: POST /api/v1/webhooks/:id/test - test webhook
  - [ ] 3.5: GET /api/v1/webhooks/:id/deliveries - get delivery history

- [ ] Task 4: Implement webhook event triggers
  - [ ] 4.1: Trigger webhook on ticket events
  - [ ] 4.2: Trigger webhook on workflow events
  - [ ] 4.3: Trigger webhook on test events

- [ ] Task 5: Create webhook management UI
  - [ ] 5.1: Create `frontend/src/pages/Webhooks/WebhooksPage.tsx`
  - [ ] 5.2: Create webhook registration form
  - [ ] 5.3: Display webhook delivery history
  - [ ] 5.4: Add test webhook button

---

## Files to Create

| File | Changes |
|------|---------|
| `migrations/YYYYMMDDHHMMSS_create_webhooks_tables.sql` | Create webhooks tables |
| `crates/qa-pms-webhooks/Cargo.toml` | Create webhooks crate |
| `crates/qa-pms-webhooks/src/lib.rs` | Create webhooks crate root |
| `crates/qa-pms-webhooks/src/webhook_service.rs` | Create webhook service |
| `crates/qa-pms-api/src/routes/webhooks.rs` | Create webhook API routes |
| `frontend/src/pages/Webhooks/WebhooksPage.tsx` | Create webhooks management page |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-api/src/routes/tickets.rs` | Trigger webhook on ticket events |
| `crates/qa-pms-api/src/routes/workflows.rs` | Trigger webhook on workflow events |

---

## Dev Notes

### Webhook System Architecture

**Webhook Model:**
```rust
pub struct Webhook {
    pub id: Uuid,
    pub url: String,
    pub events: Vec<String>, // ["ticket.created", "workflow.completed"]
    pub secret: String,      // HMAC signing secret
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}
```

**Webhook Delivery:**
- HTTP POST to webhook URL
- Include HMAC signature in headers
- Retry on failure (exponential backoff)
- Track delivery status

**Webhook Events:**
- `ticket.created`, `ticket.updated`, `ticket.status_changed`
- `workflow.started`, `workflow.completed`, `workflow.step_completed`
- `test.run_created`, `test.run_completed`
- Custom events (future)

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 19, Story 19.10)
- Dependency: Epic 3 (Jira Integration), Epic 4 (Postman & Testmo) - must be complete
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### File List

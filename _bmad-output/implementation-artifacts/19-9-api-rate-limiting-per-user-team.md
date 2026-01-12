# Story 19.9: API Rate Limiting per User/Team

Status: ready-for-dev

## Story

**As a** System Administrator  
**I want** API rate limiting per user/team  
**So that** I can control API usage and prevent abuse

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 19.9 |
| Epic | Epic 19: Advanced Features |
| Sprint | Sprint 3: Security & Performance |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Epic 15 (Authentication) - Complete ✅ |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Implement rate limiting middleware
   - Use Tower Governor or custom rate limiter
   - Support per-user rate limits
   - Support per-team rate limits
   - Support per-IP rate limits (optional)

2. Create rate limit configuration
   - Define rate limit policies (requests per minute/hour)
   - Configure default rate limits
   - Support custom rate limits per user/team
   - Store rate limit configuration

3. Implement rate limit tracking
   - Track request counts per user/team
   - Use Redis or in-memory store for tracking
   - Sliding window algorithm
   - Rate limit headers in responses

4. Handle rate limit exceeded
   - Return 429 Too Many Requests
   - Include Retry-After header
   - Log rate limit violations
   - Alert on repeated violations (optional)

---

## Acceptance Criteria

- [ ] **Given** rate limiting exists  
  **When** exceeding rate limit  
  **Then** 429 Too Many Requests is returned

- [ ] **Given** rate limiting exists  
  **When** rate limit is exceeded  
  **Then** Retry-After header is included

- [ ] **Given** rate limiting exists  
  **When** checking rate limit  
  **Then** rate limit is enforced per user/team

- [ ] **Given** rate limiting exists  
  **When** viewing rate limit status  
  **Then** current rate limit usage is displayed

---

## Tasks / Subtasks

- [ ] Task 1: Implement rate limiting middleware
  - [ ] 1.1: Add `tower-governor` or custom rate limiter
  - [ ] 1.2: Create rate limit middleware
  - [ ] 1.3: Extract user/team identifier from request
  - [ ] 1.4: Check rate limit before processing request

- [ ] Task 2: Create rate limit store
  - [ ] 2.1: Use Redis or in-memory store (Moka)
  - [ ] 2.2: Implement sliding window algorithm
  - [ ] 2.3: Track request counts per user/team
  - [ ] 2.4: Store rate limit state

- [ ] Task 3: Create rate limit configuration
  - [ ] 3.1: Define rate limit policies
  - [ ] 3.2: Store configuration in database or config
  - [ ] 3.3: Support custom rate limits per user/team
  - [ ] 3.4: Load configuration on startup

- [ ] Task 4: Integrate with API
  - [ ] 4.1: Add rate limit middleware to API router
  - [ ] 4.2: Add rate limit headers to responses
  - [ ] 4.3: Handle 429 responses gracefully
  - [ ] 4.4: Log rate limit violations

- [ ] Task 5: Create rate limit monitoring
  - [ ] 5.1: Create API endpoint for rate limit status
  - [ ] 5.2: Display rate limit usage in dashboard (optional)
  - [ ] 5.3: Alert on repeated violations (optional)

---

## Files to Create

| File | Changes |
|------|---------|
| `crates/qa-pms-api/src/middleware/rate_limit.rs` | Create rate limit middleware |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-api/Cargo.toml` | Add tower-governor or rate limit library |
| `crates/qa-pms-api/src/app.rs` | Add rate limit middleware to router |

---

## Dev Notes

### Rate Limiting Libraries

**Options:**
- `tower-governor` - Tower middleware for rate limiting (recommended)
- `governor` - Rate limiting library (used by tower-governor)
- Custom implementation with Redis/Moka

**Recommended: tower-governor**
- Tower middleware integration
- Redis backend support
- Sliding window algorithm

### Rate Limit Configuration

```rust
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub burst_size: u32,
}
```

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 19, Story 19.9)
- Dependency: Epic 15 (Authentication) - must be complete
- Tower Governor: https://docs.rs/tower-governor/latest/tower_governor/
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### File List

# Story 18.10: Notification Preferences and Channels

Status: ready-for-dev

## Story

**As a** User  
**I want** notification preferences and channels  
**So that** I can control how I receive notifications

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 18.10 |
| Epic | Epic 18: UX Improvements |
| Sprint | Sprint 4: Notifications |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Epic 9 (Pattern Detection & Alerts) - Complete ✅ |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create notification preferences system
   - Notification settings page
   - Per-notification-type preferences
   - Channel preferences (in-app, email, push)
   - Frequency preferences (real-time, digest, off)

2. Implement notification channels
   - In-app notifications (existing toast system)
   - Email notifications (future)
   - Push notifications (future)
   - Notification center/history

3. Store user preferences
   - Save preferences in database or localStorage
   - Per-user notification settings
   - Default preferences for new users

4. Integrate with existing alert system
   - Respect notification preferences
   - Filter notifications based on preferences
   - Respect frequency settings

---

## Acceptance Criteria

- [ ] **Given** notification preferences exist  
  **When** configuring preferences  
  **Then** preferences are saved and applied

- [ ] **Given** notification preferences exist  
  **When** disabling notification type  
  **Then** notifications of that type are not shown

- [ ] **Given** notification preferences exist  
  **When** setting digest frequency  
  **Then** notifications are batched and delivered on schedule

- [ ] **Given** notification preferences exist  
  **When** viewing notification center  
  **Then** notification history is displayed

---

## Tasks / Subtasks

- [ ] Task 1: Create notification preferences page
  - [ ] 1.1: Create `frontend/src/pages/Settings/NotificationPreferences.tsx`
  - [ ] 1.2: Add notification type toggles
  - [ ] 1.3: Add channel selection (in-app, email, push)
  - [ ] 1.4: Add frequency selection (real-time, digest, off)

- [ ] Task 2: Create notification preferences store
  - [ ] 2.1: Create `frontend/src/stores/notificationPreferencesStore.ts`
  - [ ] 2.2: Store preferences in localStorage or database
  - [ ] 2.3: Load preferences on app startup

- [ ] Task 3: Implement notification center
  - [ ] 3.1: Create `frontend/src/components/notifications/NotificationCenter.tsx`
  - [ ] 3.2: Display notification history
  - [ ] 3.3: Mark notifications as read/dismissed

- [ ] Task 4: Integrate with alert system
  - [ ] 4.1: Update alert service to respect preferences
  - [ ] 4.2: Filter notifications based on preferences
  - [ ] 4.3: Implement digest batching (future)

---

## Files to Create

| File | Changes |
|------|---------|
| `frontend/src/pages/Settings/NotificationPreferences.tsx` | Create notification preferences page |
| `frontend/src/stores/notificationPreferencesStore.ts` | Create notification preferences store |
| `frontend/src/components/notifications/NotificationCenter.tsx` | Create notification center component |

---

## Files to Modify

| File | Changes |
|------|---------|
| `frontend/src/components/alerts/AlertBell.tsx` | Integrate with notification preferences |
| `qa-intelligent-pms/crates/qa-pms-patterns/src/alerts.rs` | Respect notification preferences (future) |

---

## Dev Notes

### Notification Preferences Structure

```typescript
interface NotificationPreferences {
  alerts: {
    enabled: boolean;
    channels: ('in-app' | 'email' | 'push')[];
    frequency: 'realtime' | 'digest' | 'off';
  };
  patterns: {
    enabled: boolean;
    channels: ('in-app' | 'email' | 'push')[];
    frequency: 'realtime' | 'digest' | 'off';
  };
  // ... other notification types
}
```

### Notification Channels

**In-App (Immediate):**
- Toast notifications (existing)
- Alert bell (existing)
- Notification center (new)

**Email (Future):**
- Daily digest
- Real-time (for critical alerts)

**Push (Future):**
- Browser push notifications
- Mobile push notifications

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 18, Story 18.10)
- Dependency: Epic 9 (Pattern Detection & Alerts) - must be complete
- Alert Patterns: `qa-intelligent-pms/frontend/src/components/alerts/` (reference)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### File List

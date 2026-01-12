# Story 18.3: Keyboard Shortcuts and Hotkeys

Status: ready-for-dev

## Story

**As a** Power User  
**I want** keyboard shortcuts and hotkeys  
**So that** I can work more efficiently without using the mouse

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 18.3 |
| Epic | Epic 18: UX Improvements |
| Sprint | Sprint 2: Productivity |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | None |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create keyboard shortcut system
   - Global shortcut handler
   - Context-aware shortcuts (per page/component)
   - Shortcut registry and management
   - Support modifier keys (Ctrl, Cmd, Alt, Shift)

2. Implement common shortcuts
   - Navigation: `Ctrl+K` (command palette), `Ctrl+,` (settings), `Ctrl+1-9` (pages)
   - Actions: `Ctrl+S` (save), `Ctrl+N` (new), `Ctrl+F` (find), `Esc` (close/dismiss)
   - Workflow: `Ctrl+Enter` (complete step), `Ctrl+Shift+N` (new workflow)

3. Create shortcut display and help
   - Shortcut overlay (`?` or `Ctrl+?` to show)
   - Keyboard shortcut legend component
   - Contextual shortcuts in tooltips

4. Support customization (optional)
   - Allow users to customize shortcuts (future)
   - Store preferences in localStorage

---

## Acceptance Criteria

- [ ] **Given** keyboard shortcuts exist  
  **When** pressing Ctrl+K  
  **Then** command palette opens

- [ ] **Given** keyboard shortcuts exist  
  **When** pressing Ctrl+S on a form  
  **Then** form is saved (if saveable)

- [ ] **Given** keyboard shortcuts exist  
  **When** pressing `?` or Ctrl+?  
  **Then** keyboard shortcut legend is displayed

- [ ] **Given** keyboard shortcuts exist  
  **When** shortcuts conflict with browser defaults  
  **Then** browser defaults are prevented only when necessary

---

## Tasks / Subtasks

- [ ] Task 1: Create keyboard shortcut system
  - [ ] 1.1: Create `frontend/src/hooks/useKeyboardShortcut.ts`
  - [ ] 1.2: Implement global shortcut handler
  - [ ] 1.3: Support modifier key combinations
  - [ ] 1.4: Prevent default browser behavior when needed

- [ ] Task 2: Implement common shortcuts
  - [ ] 2.1: Add navigation shortcuts (Ctrl+K, Ctrl+1-9, Ctrl+,)
  - [ ] 2.2: Add action shortcuts (Ctrl+S, Ctrl+N, Ctrl+F, Esc)
  - [ ] 2.3: Add workflow shortcuts (Ctrl+Enter, Ctrl+Shift+N)
  - [ ] 2.4: Integrate shortcuts with pages/components

- [ ] Task 3: Create shortcut legend
  - [ ] 3.1: Create `frontend/src/components/shortcuts/ShortcutLegend.tsx`
  - [ ] 3.2: Display shortcuts grouped by category
  - [ ] 3.3: Show context-aware shortcuts
  - [ ] 3.4: Add `?` or Ctrl+? trigger

- [ ] Task 4: Add unit and integration tests

---

## Files to Create

| File | Changes |
|------|---------|
| `frontend/src/hooks/useKeyboardShortcut.ts` | Create keyboard shortcut hook |
| `frontend/src/components/shortcuts/ShortcutLegend.tsx` | Create shortcut legend component |
| `frontend/src/utils/shortcuts.ts` | Create shortcut registry and utilities |

---

## Files to Modify

| File | Changes |
|------|---------|
| `frontend/src/App.tsx` | Add global shortcut handlers |
| Pages/components | Integrate keyboard shortcuts |

---

## Dev Notes

### Keyboard Shortcut Patterns

**useKeyboardShortcut Hook:**
```typescript
export function useKeyboardShortcut(
  key: string,
  callback: () => void,
  options?: { preventDefault?: boolean; enabled?: boolean }
) {
  // Register keyboard shortcut
  // Handle modifier keys
  // Prevent default if needed
}
```

### Common Shortcuts

- `Ctrl+K` or `Cmd+K`: Command palette / search
- `Ctrl+,` or `Cmd+,`: Settings
- `Ctrl+1-9`: Navigate to page tabs
- `Ctrl+S`: Save (context-aware)
- `Ctrl+N`: New (context-aware)
- `Ctrl+F`: Find/search
- `Esc`: Close modal/dialog/dismiss
- `Ctrl+Enter`: Complete/submit (context-aware)
- `?` or `Ctrl+?`: Show keyboard shortcuts

### Project Structure Notes

**Library Options:**
- Use `react-hotkeys-hook` library for shortcut handling
- Or implement custom solution with `useEffect` and `keydown` events

**Accessibility:**
- Ensure shortcuts don't conflict with screen reader shortcuts
- Document shortcuts for accessibility

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 18, Story 18.3)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### File List

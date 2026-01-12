# Story 18.5: Dark Mode Support

Status: ready-for-dev

## Story

**As a** User  
**I want** dark mode theme  
**So that** I can work comfortably in low light conditions

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 18.5 |
| Epic | Epic 18: UX Improvements |
| Sprint | Sprint 3: Theming |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | None |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Implement dark mode theme system
   - Add dark mode color palette to Tailwind CSS
   - Support theme switching (light/dark)
   - Store theme preference in localStorage
   - Respect system preference (prefers-color-scheme)

2. Create theme provider/context
   - React Context or Zustand store for theme state
   - Theme toggle component
   - Auto-detect system preference

3. Update all components for dark mode
   - Ensure all UI components support dark mode
   - Test color contrast (WCAG compliance)
   - Update icons and images for dark mode

4. Add theme toggle UI
   - Theme toggle button in header/settings
   - Smooth theme transitions
   - Visual feedback when switching

---

## Acceptance Criteria

- [ ] **Given** dark mode exists  
  **When** enabling dark mode  
  **Then** application switches to dark theme

- [ ] **Given** dark mode exists  
  **When** theme preference is saved  
  **Then** preference persists across sessions

- [ ] **Given** dark mode exists  
  **When** system preference is dark  
  **Then** application automatically uses dark mode

- [ ] **Given** dark mode exists  
  **When** viewing in dark mode  
  **Then** all components display correctly with proper contrast

---

## Tasks / Subtasks

- [ ] Task 1: Configure Tailwind CSS for dark mode
  - [ ] 1.1: Update `tailwind.config.js` with `darkMode: 'class'`
  - [ ] 1.2: Define dark mode color palette
  - [ ] 1.3: Update Tailwind colors for dark mode variants

- [ ] Task 2: Create theme provider/store
  - [ ] 2.1: Create `frontend/src/stores/themeStore.ts` (Zustand)
  - [ ] 2.2: Implement theme state (light/dark/system)
  - [ ] 2.3: Store preference in localStorage
  - [ ] 2.4: Detect system preference (prefers-color-scheme)

- [ ] Task 3: Create theme toggle component
  - [ ] 3.1: Create `frontend/src/components/ThemeToggle.tsx`
  - [ ] 3.2: Add toggle button (light/dark/system)
  - [ ] 3.3: Integrate with theme store
  - [ ] 3.4: Add to header/settings

- [ ] Task 4: Update components for dark mode
  - [ ] 4.1: Review all components for dark mode support
  - [ ] 4.2: Add dark: variants to Tailwind classes
  - [ ] 4.3: Test color contrast (WCAG AA compliance)
  - [ ] 4.4: Update icons/images for dark mode

- [ ] Task 5: Add theme initialization
  - [ ] 5.1: Initialize theme on app startup
  - [ ] 5.2: Apply theme class to root element
  - [ ] 5.3: Handle theme changes dynamically

---

## Files to Create

| File | Changes |
|------|---------|
| `frontend/src/stores/themeStore.ts` | Create theme store |
| `frontend/src/components/ThemeToggle.tsx` | Create theme toggle component |

---

## Files to Modify

| File | Changes |
|------|---------|
| `frontend/tailwind.config.js` | Add dark mode configuration |
| `frontend/src/App.tsx` | Initialize theme on startup |
| `frontend/src/components/layout/Header.tsx` | Add theme toggle button |
| All components | Add dark: variants where needed |

---

## Dev Notes

### Tailwind CSS Dark Mode

**Configuration:**
```javascript
module.exports = {
  darkMode: 'class', // Use class strategy
  theme: {
    extend: {
      colors: {
        // Dark mode colors
      }
    }
  }
}
```

**Usage:**
```tsx
<div className="bg-white dark:bg-neutral-900 text-neutral-900 dark:text-neutral-100">
  Content
</div>
```

### Theme Store

```typescript
interface ThemeStore {
  theme: 'light' | 'dark' | 'system';
  setTheme: (theme: 'light' | 'dark' | 'system') => void;
  effectiveTheme: 'light' | 'dark';
}
```

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 18, Story 18.5)
- Tailwind CSS Docs: Dark mode configuration
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### File List

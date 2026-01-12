# Story 18.9: Personalized Dashboard Widgets

Status: ready-for-dev

## Story

**As a** User  
**I want** personalized dashboard widgets  
**So that** I can customize my dashboard to show the information most relevant to me

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 18.9 |
| Epic | Epic 18: UX Improvements |
| Sprint | Sprint 4: Personalization |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Epic 8 (QA Dashboard) - Complete ✅ |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create widget system
   - Widget registry
   - Widget component library
   - Widget configuration
   - Widget state management

2. Implement dashboard customization
   - Add/remove widgets
   - Reorder widgets (drag-and-drop)
   - Configure widget settings
   - Save dashboard layout per user

3. Create widget library
   - KPI cards widget
   - Trend chart widget
   - Recent activity widget
   - Custom metric widgets

4. Store user preferences
   - Save dashboard layout in database or localStorage
   - Per-user dashboard configuration
   - Support multiple dashboard views (future)

---

## Acceptance Criteria

- [ ] **Given** widget system exists  
  **When** adding widget to dashboard  
  **Then** widget appears on dashboard

- [ ] **Given** widget system exists  
  **When** reordering widgets  
  **Then** widget order is saved and persists

- [ ] **Given** widget system exists  
  **When** configuring widget  
  **Then** widget settings are saved and applied

- [ ] **Given** widget system exists  
  **When** removing widget  
  **Then** widget is removed and layout updates

---

## Tasks / Subtasks

- [ ] Task 1: Create widget system
  - [ ] 1.1: Create `frontend/src/components/dashboard/Widget.tsx` base component
  - [ ] 1.2: Create widget registry/types
  - [ ] 1.3: Create widget configuration UI

- [ ] Task 2: Implement dashboard customization
  - [ ] 2.1: Add widget selection UI
  - [ ] 2.2: Implement drag-and-drop widget reordering
  - [ ] 2.3: Add widget configuration modal
  - [ ] 2.4: Save dashboard layout

- [ ] Task 3: Create widget library
  - [ ] 3.1: Extract existing widgets (KPI cards, charts)
  - [ ] 3.2: Make widgets configurable
  - [ ] 3.3: Create widget preview component

- [ ] Task 4: Store user preferences
  - [ ] 4.1: Create API endpoint for dashboard layout
  - [ ] 4.2: Store layout in database or localStorage
  - [ ] 4.3: Load user dashboard on startup

---

## Files to Create

| File | Changes |
|------|---------|
| `frontend/src/components/dashboard/Widget.tsx` | Create base widget component |
| `frontend/src/components/dashboard/WidgetLibrary.tsx` | Create widget library component |
| `frontend/src/components/dashboard/WidgetConfig.tsx` | Create widget configuration component |

---

## Files to Modify

| File | Changes |
|------|---------|
| `frontend/src/pages/Dashboard/DashboardPage.tsx` | Implement widget system and customization |
| `crates/qa-pms-api/src/routes/dashboard.rs` | Add endpoint for saving dashboard layout |

---

## Dev Notes

### Widget System Architecture

**Base Widget Component:**
```typescript
interface WidgetProps {
  id: string;
  type: string;
  config: WidgetConfig;
  onUpdate?: (config: WidgetConfig) => void;
}

export function Widget({ id, type, config, onUpdate }: WidgetProps) {
  // Render widget based on type
}
```

**Widget Registry:**
```typescript
const WIDGETS = {
  kpi: KPICardWidget,
  trend: TrendChartWidget,
  activity: RecentActivityWidget,
  // ...
};
```

### Drag-and-Drop Widget Reordering

- Use dnd-kit library (already considered for Story 18.4)
- Grid layout with drag-and-drop
- Save layout on change

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 18, Story 18.9)
- Dependency: Epic 8 (QA Dashboard) - must be complete
- Dashboard Patterns: `qa-intelligent-pms/frontend/src/pages/Dashboard/` (reference)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### File List

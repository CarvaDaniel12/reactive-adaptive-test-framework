# Story 2.9: Hybrid Adaptive Layout Foundation

Status: ready-for-dev

## Story

As a user,
I want a responsive layout that adapts to my context,
So that I have the right interface for my current task.

## Acceptance Criteria

1. **Given** user has completed setup
   **When** user enters the main application
   **Then** layout includes a collapsible sidebar (64px collapsed / 240px expanded)

2. **Given** user is in the main application
   **When** user views the layout
   **Then** header with context (current ticket/dashboard) is displayed

3. **Given** user is in the main application
   **When** user views the layout
   **Then** main content area adapts to available space

4. **Given** user collapses or expands sidebar
   **When** state changes
   **Then** sidebar collapse state persists in localStorage

5. **Given** layout is rendered
   **When** styling is applied
   **Then** layout uses Tailwind CSS v4 tokens from design system

6. **Given** user presses keyboard shortcut
   **When** `Ctrl+Shift+M` is pressed
   **Then** sidebar toggles collapsed/expanded state

7. **Given** user toggles sidebar
   **When** animation plays
   **Then** smooth animation (300ms ease-in-out) occurs on sidebar toggle

## Tasks / Subtasks

- [ ] Task 1: Create MainLayout component (AC: #1, #2, #3)
  - [ ] 1.1: Create `MainLayout.tsx` in `frontend/src/layouts/`
  - [ ] 1.2: Implement three-column layout (sidebar, main, optional panel)
  - [ ] 1.3: Add responsive container with proper sizing

- [ ] Task 2: Create Sidebar component (AC: #1, #7)
  - [ ] 2.1: Create `Sidebar.tsx` in `frontend/src/components/layout/`
  - [ ] 2.2: Implement collapsed state (64px with icons)
  - [ ] 2.3: Implement expanded state (240px with labels)
  - [ ] 2.4: Add 300ms ease-in-out transition

- [ ] Task 3: Create Header component (AC: #2)
  - [ ] 3.1: Create `Header.tsx` in `frontend/src/components/layout/`
  - [ ] 3.2: Add context display area (ticket/dashboard name)
  - [ ] 3.3: Add timer display slot (for future)
  - [ ] 3.4: Add user avatar and settings dropdown

- [ ] Task 4: Create layout state store (AC: #4)
  - [ ] 4.1: Create `useLayoutStore.ts` Zustand store
  - [ ] 4.2: Add sidebarCollapsed state
  - [ ] 4.3: Persist to localStorage with Zustand persist middleware

- [ ] Task 5: Implement keyboard shortcut (AC: #6)
  - [ ] 5.1: Add global keyboard listener for Ctrl+Shift+M
  - [ ] 5.2: Toggle sidebar on shortcut press
  - [ ] 5.3: Prevent default browser behavior

- [ ] Task 6: Implement navigation items (AC: #1)
  - [ ] 6.1: Create navigation item component
  - [ ] 6.2: Add Dashboard, Tickets, Workflows, Reports, Settings items
  - [ ] 6.3: Show only icons when collapsed, icons + labels when expanded

- [ ] Task 7: Wire up routing (AC: #3)
  - [ ] 7.1: Configure React Router with MainLayout as parent
  - [ ] 7.2: Add child routes for main app sections
  - [ ] 7.3: Use Outlet for content rendering

- [ ] Task 8: Apply design system tokens (AC: #5)
  - [ ] 8.1: Use Tailwind CSS v4 color tokens
  - [ ] 8.2: Apply Inter font for UI text
  - [ ] 8.3: Apply consistent spacing scale

## Dev Notes

### Architecture Alignment

This story implements the **Hybrid Adaptive Layout Foundation** per Epic 2 requirements:

- **Location**: `frontend/src/layouts/` and `frontend/src/components/layout/`
- **State Management**: Zustand store with persist middleware
- **Styling**: Tailwind CSS v4 with OKLCH colors

### Technical Implementation Details

#### MainLayout Component Pattern

```tsx
// frontend/src/layouts/MainLayout.tsx
import { Outlet } from "react-router-dom";
import { Sidebar } from "@/components/layout/Sidebar";
import { Header } from "@/components/layout/Header";
import { useLayoutStore } from "@/stores/layoutStore";

export function MainLayout() {
  const { sidebarCollapsed } = useLayoutStore();
  
  return (
    <div className="min-h-screen bg-neutral-50 flex">
      {/* Sidebar */}
      <Sidebar />
      
      {/* Main Content Area */}
      <div className={`
        flex-1 flex flex-col min-h-screen transition-all duration-300 ease-in-out
        ${sidebarCollapsed ? "ml-16" : "ml-60"}
      `}>
        {/* Header */}
        <Header />
        
        {/* Page Content */}
        <main className="flex-1 p-6 overflow-auto">
          <Outlet />
        </main>
      </div>
    </div>
  );
}
```

#### Sidebar Component Pattern

```tsx
// frontend/src/components/layout/Sidebar.tsx
import { useLayoutStore } from "@/stores/layoutStore";
import { NavLink } from "react-router-dom";
import * as Tooltip from "@radix-ui/react-tooltip";
import {
  DashboardIcon,
  FileTextIcon,
  GearIcon,
  ListBulletIcon,
  ReaderIcon,
  ChevronLeftIcon,
  ChevronRightIcon,
} from "@radix-ui/react-icons";

const NAV_ITEMS = [
  { path: "/dashboard", icon: DashboardIcon, label: "Dashboard" },
  { path: "/tickets", icon: ListBulletIcon, label: "Tickets" },
  { path: "/workflows", icon: ReaderIcon, label: "Workflows" },
  { path: "/reports", icon: FileTextIcon, label: "Reports" },
  { path: "/settings", icon: GearIcon, label: "Settings" },
];

export function Sidebar() {
  const { sidebarCollapsed, toggleSidebar } = useLayoutStore();
  
  return (
    <aside className={`
      fixed left-0 top-0 h-screen bg-white border-r border-neutral-200
      flex flex-col transition-all duration-300 ease-in-out z-40
      ${sidebarCollapsed ? "w-16" : "w-60"}
    `}>
      {/* Logo */}
      <div className="h-16 flex items-center justify-center border-b border-neutral-200">
        {sidebarCollapsed ? (
          <span className="text-2xl font-bold text-primary-500">Q</span>
        ) : (
          <span className="text-xl font-bold text-primary-500">QA PMS</span>
        )}
      </div>
      
      {/* Navigation */}
      <nav className="flex-1 py-4">
        <Tooltip.Provider delayDuration={0}>
          <ul className="space-y-1 px-2">
            {NAV_ITEMS.map((item) => (
              <li key={item.path}>
                <Tooltip.Root>
                  <Tooltip.Trigger asChild>
                    <NavLink
                      to={item.path}
                      className={({ isActive }) => `
                        flex items-center gap-3 px-3 py-2.5 rounded-lg
                        transition-colors duration-150
                        ${isActive 
                          ? "bg-primary-50 text-primary-600" 
                          : "text-neutral-600 hover:bg-neutral-100"}
                        ${sidebarCollapsed ? "justify-center" : ""}
                      `}
                    >
                      <item.icon className="w-5 h-5 flex-shrink-0" />
                      {!sidebarCollapsed && (
                        <span className="font-medium">{item.label}</span>
                      )}
                    </NavLink>
                  </Tooltip.Trigger>
                  {sidebarCollapsed && (
                    <Tooltip.Portal>
                      <Tooltip.Content
                        side="right"
                        className="bg-neutral-900 text-white text-sm px-3 py-1.5 rounded shadow-lg"
                        sideOffset={8}
                      >
                        {item.label}
                        <Tooltip.Arrow className="fill-neutral-900" />
                      </Tooltip.Content>
                    </Tooltip.Portal>
                  )}
                </Tooltip.Root>
              </li>
            ))}
          </ul>
        </Tooltip.Provider>
      </nav>
      
      {/* Collapse Toggle */}
      <div className="p-2 border-t border-neutral-200">
        <button
          onClick={toggleSidebar}
          className="w-full flex items-center justify-center gap-2 px-3 py-2 
                     text-neutral-500 hover:text-neutral-700 hover:bg-neutral-100 
                     rounded-lg transition-colors"
          title={sidebarCollapsed ? "Expand sidebar" : "Collapse sidebar"}
        >
          {sidebarCollapsed ? (
            <ChevronRightIcon className="w-5 h-5" />
          ) : (
            <>
              <ChevronLeftIcon className="w-5 h-5" />
              <span className="text-sm">Collapse</span>
            </>
          )}
        </button>
      </div>
    </aside>
  );
}
```

#### Header Component Pattern

```tsx
// frontend/src/components/layout/Header.tsx
import { useLayoutStore } from "@/stores/layoutStore";
import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import { PersonIcon, GearIcon, ExitIcon } from "@radix-ui/react-icons";

interface HeaderProps {
  title?: string;
  subtitle?: string;
}

export function Header({ title, subtitle }: HeaderProps) {
  return (
    <header className="h-16 bg-white border-b border-neutral-200 px-6 flex items-center justify-between">
      {/* Left: Context */}
      <div>
        {title && (
          <h1 className="text-lg font-semibold text-neutral-900">{title}</h1>
        )}
        {subtitle && (
          <p className="text-sm text-neutral-500">{subtitle}</p>
        )}
      </div>
      
      {/* Center: Timer Slot (for future) */}
      <div id="header-timer-slot" />
      
      {/* Right: User Menu */}
      <DropdownMenu.Root>
        <DropdownMenu.Trigger asChild>
          <button className="flex items-center gap-2 p-2 rounded-lg hover:bg-neutral-100 transition-colors">
            <div className="w-8 h-8 bg-primary-100 text-primary-600 rounded-full flex items-center justify-center">
              <PersonIcon className="w-4 h-4" />
            </div>
          </button>
        </DropdownMenu.Trigger>
        
        <DropdownMenu.Portal>
          <DropdownMenu.Content
            className="bg-white rounded-lg shadow-lg border border-neutral-200 py-1 min-w-[160px]"
            sideOffset={8}
            align="end"
          >
            <DropdownMenu.Item className="px-3 py-2 text-sm text-neutral-600 hover:bg-neutral-100 cursor-pointer flex items-center gap-2">
              <GearIcon className="w-4 h-4" />
              Settings
            </DropdownMenu.Item>
            <DropdownMenu.Separator className="h-px bg-neutral-200 my-1" />
            <DropdownMenu.Item className="px-3 py-2 text-sm text-error-500 hover:bg-error-50 cursor-pointer flex items-center gap-2">
              <ExitIcon className="w-4 h-4" />
              Sign out
            </DropdownMenu.Item>
          </DropdownMenu.Content>
        </DropdownMenu.Portal>
      </DropdownMenu.Root>
    </header>
  );
}
```

#### Layout Store Pattern

```typescript
// frontend/src/stores/layoutStore.ts
import { create } from "zustand";
import { persist } from "zustand/middleware";

interface LayoutState {
  sidebarCollapsed: boolean;
  
  // Actions
  toggleSidebar: () => void;
  setSidebarCollapsed: (collapsed: boolean) => void;
}

export const useLayoutStore = create<LayoutState>()(
  persist(
    (set) => ({
      sidebarCollapsed: false,
      
      toggleSidebar: () => set((state) => ({ 
        sidebarCollapsed: !state.sidebarCollapsed 
      })),
      setSidebarCollapsed: (collapsed) => set({ sidebarCollapsed: collapsed }),
    }),
    { name: "qa-pms-layout" }
  )
);
```

#### Keyboard Shortcut Hook

```typescript
// frontend/src/hooks/useKeyboardShortcuts.ts
import { useEffect } from "react";
import { useLayoutStore } from "@/stores/layoutStore";

export function useKeyboardShortcuts() {
  const toggleSidebar = useLayoutStore((state) => state.toggleSidebar);
  
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ctrl+Shift+M to toggle sidebar
      if (e.ctrlKey && e.shiftKey && e.key === "M") {
        e.preventDefault();
        toggleSidebar();
      }
    };
    
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [toggleSidebar]);
}

// Use in App.tsx or MainLayout
// useKeyboardShortcuts();
```

### Project Structure Notes

Files to create:
```
frontend/src/
├── layouts/
│   └── MainLayout.tsx           # Main app layout
├── components/
│   └── layout/
│       ├── Sidebar.tsx          # Collapsible sidebar
│       ├── Header.tsx           # Top header
│       └── index.ts             # Barrel export
├── stores/
│   └── layoutStore.ts           # Layout state (add to existing)
└── hooks/
    └── useKeyboardShortcuts.ts  # Keyboard shortcuts
```

### Design System Tokens (from UX Spec)

| Token | Value | Usage |
|-------|-------|-------|
| Sidebar collapsed | 64px (w-16) | Icons only |
| Sidebar expanded | 240px (w-60) | Icons + labels |
| Header height | 64px (h-16) | Fixed header |
| Transition | 300ms ease-in-out | Sidebar animation |
| Primary | oklch(59.69% 0.228 248.64) | Active states |
| Neutral-50 | oklch(98.5% 0.002 247.86) | Background |

### Testing Notes

- Unit test sidebar toggle changes width
- Unit test keyboard shortcut triggers toggle
- Unit test layout state persists in localStorage
- Visual test: Animation is smooth

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 2.9]
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Hybrid Adaptive Layout]
- [Source: _bmad-output/planning-artifacts/architecture.md#Frontend Architecture]

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List

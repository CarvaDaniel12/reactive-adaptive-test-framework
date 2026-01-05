import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MainLayout } from "./components/layout/MainLayout";
import { ToastProvider } from "./components/ui/ToastProvider";
import { StartupCheck } from "./components/startup";
import { Home } from "./pages/Home";
import { DashboardPage } from "./pages/Dashboard";
import {
  SetupWizardLayout,
  SetupGuard,
  UserProfileStep,
  JiraStep,
  PostmanStep,
  TestmoStep,
  SplunkStep,
  SetupComplete,
} from "./pages/Setup";
import { TicketsPage, TicketDetailPage } from "./pages/Tickets";
import { WorkflowPage } from "./pages/Workflows";
import { PatternsPage } from "./pages/Patterns";
import { PMDashboardPage } from "./pages/PMDashboard";
import { SplunkPage } from "./pages/Splunk";
import { SupportPage } from "./pages/Support";

// Create a client for React Query
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 30000, // 30 seconds
      retry: 1,
    },
  },
});

function WorkflowsListPage() {
  return (
    <div className="space-y-4">
      <h2 className="text-2xl font-semibold text-neutral-900">Workflows</h2>
      <p className="text-neutral-600">Your testing workflows will appear here.</p>
    </div>
  );
}

function ReportsPage() {
  return (
    <div className="space-y-4">
      <h2 className="text-2xl font-semibold text-neutral-900">Reports</h2>
      <p className="text-neutral-600">Your testing reports will appear here.</p>
    </div>
  );
}

function SettingsPage() {
  return (
    <div className="space-y-4">
      <h2 className="text-2xl font-semibold text-neutral-900">Settings</h2>
      <p className="text-neutral-600">Application settings and integrations.</p>
    </div>
  );
}

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ToastProvider>
        <BrowserRouter>
        <Routes>
        {/* Setup Wizard Routes */}
        <Route path="/setup" element={<SetupWizardLayout />}>
          <Route index element={<Navigate to="profile" replace />} />
          <Route path="profile" element={<UserProfileStep />} />
          <Route path="jira" element={<JiraStep />} />
          <Route path="postman" element={<PostmanStep />} />
          <Route path="testmo" element={<TestmoStep />} />
          <Route path="splunk" element={<SplunkStep />} />
          <Route path="complete" element={<SetupComplete />} />
        </Route>

        {/* Main App Routes (protected by SetupGuard and StartupCheck) */}
        <Route
          path="/"
          element={
            <SetupGuard>
              <StartupCheck>
                <MainLayout />
              </StartupCheck>
            </SetupGuard>
          }
        >
          <Route index element={<DashboardPage />} />
          <Route path="home" element={<Home />} />
          <Route path="tickets" element={<TicketsPage />} />
          <Route path="tickets/:key" element={<TicketDetailPage />} />
          <Route path="workflows" element={<WorkflowsListPage />} />
          <Route path="workflows/:id" element={<WorkflowPage />} />
          <Route path="reports" element={<ReportsPage />} />
          <Route path="patterns" element={<PatternsPage />} />
          <Route path="pm-dashboard" element={<PMDashboardPage />} />
          <Route path="splunk" element={<SplunkPage />} />
          <Route path="support" element={<SupportPage />} />
          <Route path="settings" element={<SettingsPage />} />
        </Route>

        {/* Catch all - redirect to home */}
        <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
        </BrowserRouter>
      </ToastProvider>
    </QueryClientProvider>
  );
}

export default App;

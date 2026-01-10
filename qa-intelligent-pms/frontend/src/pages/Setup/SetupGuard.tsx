/**
 * Route guard that redirects to setup wizard if setup is not complete.
 *
 * Implements FR-CFG-05: Auto-redirect to wizard if config incomplete.
 */
import { Navigate } from "react-router-dom";
import { useWizardStore } from "@/stores/wizardStore";

interface SetupGuardProps {
  children: React.ReactNode;
}

export function SetupGuard({ children }: SetupGuardProps) {
  const { isComplete, _hasHydrated } = useWizardStore();

  // Wait for persisted state to hydrate to avoid false redirects
  if (!_hasHydrated) {
    return null;
  }

  if (!isComplete) {
    return <Navigate to="/setup" replace />;
  }

  return <>{children}</>;
}

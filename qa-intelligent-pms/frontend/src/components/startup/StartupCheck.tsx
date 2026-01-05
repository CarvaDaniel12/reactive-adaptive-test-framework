/**
 * Startup validation component.
 *
 * Validates all configured integrations on app startup.
 * Shows loading state during validation, blocks on critical failures,
 * and shows warning toasts for optional failures.
 */
import { useState, useEffect, type ReactNode } from "react";
import { useToast } from "@/hooks/useToast";
import { CriticalErrorScreen } from "./CriticalErrorScreen";

const API_BASE = import.meta.env.VITE_API_URL || "";

interface ValidationResult {
  integration: string;
  success: boolean;
  errorMessage: string | null;
  responseTimeMs: number | null;
  isCritical: boolean;
}

interface StartupValidationReport {
  valid: boolean;
  hasCriticalFailure: boolean;
  results: ValidationResult[];
  totalTimeMs: number;
}

interface StartupCheckProps {
  children: ReactNode;
}

type ValidationState = "loading" | "valid" | "critical-failure" | "skipped";

export function StartupCheck({ children }: StartupCheckProps) {
  const [validationState, setValidationState] =
    useState<ValidationState>("loading");
  const [report, setReport] = useState<StartupValidationReport | null>(null);
  const { toast } = useToast();

  useEffect(() => {
    async function runValidation() {
      try {
        const response = await fetch(`${API_BASE}/api/v1/startup/validate`);

        if (!response.ok) {
          // If API fails, allow app to continue (setup might not be complete)
          console.warn("Startup validation API failed, skipping check");
          setValidationState("skipped");
          return;
        }

        const data: StartupValidationReport = await response.json();
        setReport(data);

        // If no integrations configured, skip validation
        if (data.results.length === 0) {
          setValidationState("skipped");
          return;
        }

        if (data.hasCriticalFailure) {
          setValidationState("critical-failure");
          return;
        }

        // Show toast notifications for results
        data.results.forEach((result) => {
          if (result.success) {
            toast({
              title: `${capitalize(result.integration)} connected`,
              description: result.responseTimeMs
                ? `Response time: ${result.responseTimeMs}ms`
                : undefined,
              variant: "success",
            });
          } else if (!result.isCritical) {
            toast({
              title: `${capitalize(result.integration)} unavailable`,
              description: result.errorMessage || "Check your settings",
              variant: "warning",
            });
          }
        });

        setValidationState("valid");
      } catch (error) {
        console.error("Startup validation error:", error);
        // Allow app to continue on network failure
        setValidationState("skipped");
      }
    }

    runValidation();
  }, [toast]);

  if (validationState === "loading") {
    return <StartupLoadingScreen />;
  }

  if (validationState === "critical-failure" && report) {
    const criticalFailure = report.results.find(
      (r) => r.isCritical && !r.success
    );
    return (
      <CriticalErrorScreen
        integration={criticalFailure?.integration || "Unknown"}
        errorMessage={criticalFailure?.errorMessage || "Connection failed"}
      />
    );
  }

  return <>{children}</>;
}

function StartupLoadingScreen() {
  return (
    <div className="min-h-screen flex items-center justify-center bg-neutral-50">
      <div className="text-center">
        <div
          className="w-12 h-12 border-4 border-primary-200 border-t-primary-500 
                        rounded-full animate-spin mx-auto mb-4"
        />
        <h2 className="text-lg font-medium text-neutral-700">
          Validating integrations...
        </h2>
        <p className="text-sm text-neutral-500 mt-1">
          This should only take a moment
        </p>
      </div>
    </div>
  );
}

function capitalize(s: string): string {
  return s.charAt(0).toUpperCase() + s.slice(1);
}

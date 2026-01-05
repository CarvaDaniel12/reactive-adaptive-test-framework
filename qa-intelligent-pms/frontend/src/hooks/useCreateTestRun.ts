import { useMutation } from "@tanstack/react-query";
import { useToast } from "./useToast";

interface CreateTestRunRequest {
  ticketKey: string;
  caseIds: number[];
  customName?: string;
}

interface CreateTestRunResponse {
  runId: number;
  name: string;
  url: string;
  caseCount: number;
}

/**
 * Hook for creating Testmo test runs.
 *
 * Uses React Query mutation with toast notifications for success/error feedback.
 */
export function useCreateTestRun() {
  const { toast } = useToast();

  return useMutation({
    mutationFn: async (request: CreateTestRunRequest): Promise<CreateTestRunResponse> => {
      const response = await fetch("/api/v1/testmo/runs", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          ticketKey: request.ticketKey,
          caseIds: request.caseIds,
          customName: request.customName,
        }),
      });

      if (!response.ok) {
        const error = await response.json().catch(() => ({ message: "Unknown error" }));
        throw new Error(error.message || `Failed to create test run (${response.status})`);
      }

      return response.json();
    },
    onSuccess: (data) => {
      toast({
        title: "Test Run Created",
        description: `Created "${data.name}" with ${data.caseCount} test case${data.caseCount !== 1 ? "s" : ""}`,
        variant: "success",
      });
      // Open Testmo in new tab
      window.open(data.url, "_blank", "noopener,noreferrer");
    },
    onError: (error: Error) => {
      toast({
        title: "Failed to Create Test Run",
        description: error.message,
        variant: "error",
      });
    },
  });
}

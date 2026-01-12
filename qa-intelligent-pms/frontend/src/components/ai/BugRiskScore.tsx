import { useState, useCallback } from "react";
import { useMutation } from "@tanstack/react-query";
import * as Dialog from "@radix-ui/react-dialog";
import { useToast } from "@/hooks";
import type { PredictBugRiskRequest, BugRiskScore } from "@/types";

// API functions
async function predictBugRisk(request: PredictBugRiskRequest): Promise<BugRiskScore> {
  const res = await fetch("/api/v1/ai/predict-bug-risk", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(request),
  });
  if (!res.ok) {
    const error = await res.json().catch(() => ({ error: "Failed to predict bug risk" }));
    throw new Error(error.error || "Failed to predict bug risk");
  }
  return res.json();
}

interface BugRiskScoreProps {
  ticketKey: string;
  ticketTitle?: string;
}

/**
 * Bug Risk Score component for AI-powered bug prediction (Task 8).
 * 
 * Features:
 * - Predict bug risk for a ticket
 * - Display risk score with visual indicator (color-coded)
 * - Display risk level badge (Low/Medium/High/Critical)
 * - Display risk factors list with impact indicators
 * - Display predicted bug types
 * - Display confidence level
 */
export function BugRiskScore({ ticketKey }: BugRiskScoreProps) {
  const { toast } = useToast();
  const [showDialog, setShowDialog] = useState(false);
  const [riskScore, setRiskScore] = useState<BugRiskScore | null>(null);

  // Predict bug risk mutation
  const predictMutation = useMutation({
    mutationFn: predictBugRisk,
    onSuccess: (data) => {
      setRiskScore(data);
      setShowDialog(true);
      toast({
        title: "Risk Analysis Complete",
        description: `Bug risk predicted for ${ticketKey}`,
        variant: "success",
      });
    },
    onError: (error: Error) => {
      toast({
        title: "Prediction Failed",
        description: error.message,
        variant: "error",
      });
    },
  });

  // Handle analyze button click (Task 8.7)
  const handleAnalyzeClick = useCallback(() => {
    predictMutation.mutate({ ticketKey });
  }, [ticketKey, predictMutation]);

  const isLoading = predictMutation.isPending;

  // Get risk level colors (Task 8.2, 8.3)
  const getRiskLevelColors = (level: BugRiskScore["riskLevel"]) => {
    switch (level) {
      case "low":
        return {
          bg: "bg-emerald-50",
          text: "text-emerald-700",
          border: "border-emerald-200",
          badge: "bg-emerald-100 text-emerald-700",
          score: "text-emerald-600",
        };
      case "medium":
        return {
          bg: "bg-amber-50",
          text: "text-amber-700",
          border: "border-amber-200",
          badge: "bg-amber-100 text-amber-700",
          score: "text-amber-600",
        };
      case "high":
        return {
          bg: "bg-orange-50",
          text: "text-orange-700",
          border: "border-orange-200",
          badge: "bg-orange-100 text-orange-700",
          score: "text-orange-600",
        };
      case "critical":
        return {
          bg: "bg-red-50",
          text: "text-red-700",
          border: "border-red-200",
          badge: "bg-red-100 text-red-700",
          score: "text-red-600",
        };
    }
  };

  // Format risk level for display (Task 8.3)
  const formatRiskLevel = (level: BugRiskScore["riskLevel"]): string => {
    return level.charAt(0).toUpperCase() + level.slice(1);
  };

  // Format percentage
  const formatPercentage = (value: number): string => {
    return `${(value * 100).toFixed(0)}%`;
  };

  return (
    <>
      {/* Analyze Risk Button (Task 8.7) */}
      <button
        onClick={handleAnalyzeClick}
        disabled={isLoading}
        className="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium text-white 
                 bg-purple-600 rounded-lg transition-colors hover:bg-purple-700 
                 disabled:opacity-50 disabled:cursor-not-allowed shadow-sm hover:shadow-md"
      >
        {isLoading ? (
          <>
            <svg
              className="animate-spin h-4 w-4"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
            >
              <circle
                className="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                strokeWidth="4"
              />
              <path
                className="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
              />
            </svg>
            Analyzing...
          </>
        ) : (
          <>
            <svg
              className="w-4 h-4"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
              />
            </svg>
            Analyze Risk
          </>
        )}
      </button>

      {/* Risk Score Dialog (Task 8.1-8.6) */}
      {riskScore && (
        <Dialog.Root open={showDialog} onOpenChange={setShowDialog}>
          <Dialog.Portal>
            <Dialog.Overlay className="fixed inset-0 bg-black/50 animate-fade-in z-50" />
            <Dialog.Content
              className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
                       bg-white rounded-lg shadow-xl w-full max-w-2xl max-h-[90vh] overflow-y-auto z-50
                       animate-scale-in focus:outline-none"
            >
              <Dialog.Title className="text-lg font-semibold text-neutral-900 mb-4 p-6 pb-4 border-b border-neutral-200">
                Bug Risk Analysis
                <span className="ml-2 text-sm font-mono font-normal text-neutral-500">{ticketKey}</span>
              </Dialog.Title>

              <div className="p-6 space-y-6">
                {/* Risk Score and Level (Task 8.2, 8.3) */}
                <div className={`p-6 rounded-lg border-2 ${getRiskLevelColors(riskScore.riskLevel).bg} ${getRiskLevelColors(riskScore.riskLevel).border}`}>
                  <div className="flex items-center justify-between mb-4">
                    <div>
                      <div className="text-sm font-medium text-neutral-600 mb-1">Risk Score</div>
                      <div className={`text-4xl font-bold ${getRiskLevelColors(riskScore.riskLevel).score}`}>
                        {formatPercentage(riskScore.riskScore)}
                      </div>
                    </div>
                    <div className="text-right">
                      <div className="text-sm font-medium text-neutral-600 mb-1">Risk Level</div>
                      <span className={`inline-block px-4 py-2 rounded-lg text-sm font-semibold ${getRiskLevelColors(riskScore.riskLevel).badge}`}>
                        {formatRiskLevel(riskScore.riskLevel)}
                      </span>
                    </div>
                  </div>
                  {/* Confidence Level (Task 8.6) */}
                  <div className="mt-4 pt-4 border-t border-neutral-200">
                    <div className="flex items-center justify-between">
                      <span className="text-sm text-neutral-600">Confidence</span>
                      <span className="text-sm font-medium text-neutral-900">
                        {formatPercentage(riskScore.confidence)}
                      </span>
                    </div>
                    <div className="mt-2 w-full bg-neutral-200 rounded-full h-2">
                      <div
                        className="bg-primary-600 h-2 rounded-full transition-all"
                        style={{ width: formatPercentage(riskScore.confidence) }}
                      />
                    </div>
                  </div>
                </div>

                {/* Risk Factors (Task 8.4) */}
                {riskScore.riskFactors.length > 0 && (
                  <div>
                    <h3 className="text-sm font-semibold text-neutral-900 mb-3 flex items-center gap-2">
                      <svg
                        className="w-5 h-5 text-neutral-500"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                        />
                      </svg>
                      Risk Factors
                    </h3>
                    <div className="space-y-3">
                      {riskScore.riskFactors.map((factor, index) => (
                        <div
                          key={index}
                          className="p-4 bg-neutral-50 border border-neutral-200 rounded-lg"
                        >
                          <div className="flex items-start justify-between mb-2">
                            <span className="font-medium text-neutral-900">{factor.factor}</span>
                            <span className="text-sm font-semibold text-primary-600">
                              {formatPercentage(factor.impact)} impact
                            </span>
                          </div>
                          <p className="text-sm text-neutral-600">{factor.description}</p>
                        </div>
                      ))}
                    </div>
                  </div>
                )}

                {/* Predicted Bug Types (Task 8.5) */}
                {riskScore.predictedBugs.length > 0 && (
                  <div>
                    <h3 className="text-sm font-semibold text-neutral-900 mb-3 flex items-center gap-2">
                      <svg
                        className="w-5 h-5 text-neutral-500"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                        />
                      </svg>
                      Predicted Bug Types
                    </h3>
                    <div className="flex flex-wrap gap-2">
                      {riskScore.predictedBugs.map((bugType, index) => (
                        <span
                          key={index}
                          className="px-3 py-1.5 bg-warning-50 text-warning-700 text-sm font-medium rounded-md border border-warning-200"
                        >
                          {bugType}
                        </span>
                      ))}
                    </div>
                  </div>
                )}

                {/* Empty State */}
                {riskScore.riskFactors.length === 0 && riskScore.predictedBugs.length === 0 && (
                  <div className="text-center py-8 text-neutral-500">
                    <p>No risk factors or predicted bug types identified.</p>
                  </div>
                )}
              </div>

              {/* Dialog Footer */}
              <div className="flex items-center justify-end gap-3 p-6 pt-4 border-t border-neutral-200">
                <Dialog.Close asChild>
                  <button
                    className="px-4 py-2 text-sm font-medium text-neutral-700 bg-white border border-neutral-300 
                             rounded-lg hover:bg-neutral-50 transition-colors"
                  >
                    Close
                  </button>
                </Dialog.Close>
              </div>
            </Dialog.Content>
          </Dialog.Portal>
        </Dialog.Root>
      )}
    </>
  );
}

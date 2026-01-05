import { useState, useCallback, useEffect } from "react";
import { useMutation } from "@tanstack/react-query";
import * as Dialog from "@radix-ui/react-dialog";
import { useToast } from "@/hooks";
import type { GenerateTestsRequest, GenerateTestsResponse, TestCase } from "@/types";

// API functions
async function generateTests(request: GenerateTestsRequest): Promise<GenerateTestsResponse> {
  const res = await fetch("/api/v1/ai/generate-tests", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(request),
  });
  if (!res.ok) {
    const error = await res.json().catch(() => ({ error: "Failed to generate tests" }));
    throw new Error(error.error || "Failed to generate tests");
  }
  return res.json();
}

async function regenerateTests(request: GenerateTestsRequest): Promise<GenerateTestsResponse> {
  const res = await fetch("/api/v1/ai/regenerate-tests", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(request),
  });
  if (!res.ok) {
    const error = await res.json().catch(() => ({ error: "Failed to regenerate tests" }));
    throw new Error(error.error || "Failed to regenerate tests");
  }
  return res.json();
}

interface TestGeneratorProps {
  ticketKey: string;
  ticketTitle?: string;
}

interface GenerationOptions {
  includeRegression: boolean;
  includeSecurity: boolean;
  includePerformance: boolean;
  force: boolean;
}

/**
 * Test Generator component for AI-powered test case generation (Task 5).
 * 
 * Features:
 * - Generate test cases from Jira tickets
 * - Options dialog for generation parameters
 * - Display generated test cases with preview
 * - Edit functionality
 * - Save to Workflow and Testmo
 * - Regenerate with different options
 */
export function TestGenerator({ ticketKey }: TestGeneratorProps) {
  const { toast } = useToast();
  const [showOptionsDialog, setShowOptionsDialog] = useState(false);
  const [showTestCasesDialog, setShowTestCasesDialog] = useState(false);
  const [options, setOptions] = useState<GenerationOptions>({
    includeRegression: true,
    includeSecurity: false,
    includePerformance: false,
    force: false,
  });
  const [testCases, setTestCases] = useState<TestCase[]>([]);
  const [editingTestCase, setEditingTestCase] = useState<TestCase | null>(null);

  // Generate test cases mutation
  const generateMutation = useMutation({
    mutationFn: generateTests,
    onSuccess: (data) => {
      setTestCases(data.testCases);
      setShowOptionsDialog(false);
      setShowTestCasesDialog(true);
      toast({
        title: "Tests Generated",
        description: `Generated ${data.count} test cases for ${ticketKey}`,
        variant: "success",
      });
    },
    onError: (error: Error) => {
      toast({
        title: "Generation Failed",
        description: error.message,
        variant: "error",
      });
    },
  });

  // Regenerate test cases mutation
  const regenerateMutation = useMutation({
    mutationFn: regenerateTests,
    onSuccess: (data) => {
      setTestCases(data.testCases);
      toast({
        title: "Tests Regenerated",
        description: `Regenerated ${data.count} test cases`,
        variant: "success",
      });
    },
    onError: (error: Error) => {
      toast({
        title: "Regeneration Failed",
        description: error.message,
        variant: "error",
      });
    },
  });

  // Handle generate button click (Task 5.2)
  const handleGenerateClick = useCallback(() => {
    setShowOptionsDialog(true);
  }, []);

  // Handle generate with options (Task 5.3)
  const handleGenerate = useCallback(() => {
    generateMutation.mutate({
      ticketKey,
      includeRegression: options.includeRegression,
      includeSecurity: options.includeSecurity,
      includePerformance: options.includePerformance,
      force: options.force,
    });
  }, [ticketKey, options, generateMutation]);

  // Handle regenerate (Task 5.9)
  const handleRegenerate = useCallback(() => {
    regenerateMutation.mutate({
      ticketKey,
      includeRegression: options.includeRegression,
      includeSecurity: options.includeSecurity,
      includePerformance: options.includePerformance,
      force: true,
    });
  }, [ticketKey, options, regenerateMutation]);

  // Handle edit test case (Task 5.6)
  const handleEdit = useCallback((testCase: TestCase) => {
    setEditingTestCase({ ...testCase }); // Clone to allow editing
  }, []);

  // Handle save edited test case
  const handleSaveEdit = useCallback(
    (updated: TestCase) => {
      setTestCases((prev) => prev.map((tc) => (tc.id === updated.id ? updated : tc)));
      setEditingTestCase(null);
      toast({
        title: "Test Case Updated",
        description: "Test case has been updated",
        variant: "success",
      });
    },
    [toast]
  );

  // Handle save to workflow (Task 5.7)
  const handleSaveToWorkflow = useCallback(() => {
    toast({
      title: "Save to Workflow",
      description: "This feature will be implemented in a future story",
      variant: "info",
    });
  }, [toast]);

  // Handle save to Testmo (Task 5.8)
  const handleSaveToTestmo = useCallback(() => {
    toast({
      title: "Save to Testmo",
      description: "This feature will be implemented when Testmo integration is enhanced",
      variant: "info",
    });
  }, [toast]);

  const isLoading = generateMutation.isPending || regenerateMutation.isPending;

  return (
    <>
      {/* Generate Tests Button (Task 5.2) */}
      <button
        onClick={handleGenerateClick}
        disabled={isLoading}
        className="inline-flex items-center gap-2 px-4 py-2 bg-primary-600 text-white 
                 rounded-lg hover:bg-primary-700 transition-colors font-medium shadow-sm
                 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {isLoading ? (
          <>
            <svg
              className="animate-spin h-5 w-5"
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
            Generating...
          </>
        ) : (
          <>
            <svg
              className="w-5 h-5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M13 10V3L4 14h7v7l9-11h-7z"
              />
            </svg>
            Generate Tests
          </>
        )}
      </button>

      {/* Generation Options Dialog (Task 5.3) */}
      <Dialog.Root open={showOptionsDialog} onOpenChange={setShowOptionsDialog}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/50 animate-fade-in z-50" />
          <Dialog.Content
            className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
                     bg-white rounded-lg shadow-xl w-full max-w-md p-6 z-50
                     animate-scale-in focus:outline-none"
          >
            <Dialog.Title className="text-lg font-semibold text-neutral-900 mb-4">
              Generate Test Cases
            </Dialog.Title>
            <Dialog.Description className="text-sm text-neutral-600 mb-6">
              Configure options for AI-powered test case generation from ticket <span className="font-mono font-semibold">{ticketKey}</span>
            </Dialog.Description>

            <div className="space-y-4 mb-6">
              {/* Include Regression */}
              <label className="flex items-center gap-3 cursor-pointer">
                <input
                  type="checkbox"
                  checked={options.includeRegression}
                  onChange={(e) =>
                    setOptions((prev) => ({ ...prev, includeRegression: e.target.checked }))
                  }
                  className="w-5 h-5 text-primary-600 rounded border-neutral-300 focus:ring-primary-500"
                />
                <div>
                  <div className="font-medium text-neutral-900">Include Regression Tests</div>
                  <div className="text-sm text-neutral-500">
                    Generate tests to prevent bugs from recurring
                  </div>
                </div>
              </label>

              {/* Include Security */}
              <label className="flex items-center gap-3 cursor-pointer">
                <input
                  type="checkbox"
                  checked={options.includeSecurity}
                  onChange={(e) =>
                    setOptions((prev) => ({ ...prev, includeSecurity: e.target.checked }))
                  }
                  className="w-5 h-5 text-primary-600 rounded border-neutral-300 focus:ring-primary-500"
                />
                <div>
                  <div className="font-medium text-neutral-900">Include Security Tests</div>
                  <div className="text-sm text-neutral-500">
                    Generate security-focused test scenarios
                  </div>
                </div>
              </label>

              {/* Include Performance */}
              <label className="flex items-center gap-3 cursor-pointer">
                <input
                  type="checkbox"
                  checked={options.includePerformance}
                  onChange={(e) =>
                    setOptions((prev) => ({ ...prev, includePerformance: e.target.checked }))
                  }
                  className="w-5 h-5 text-primary-600 rounded border-neutral-300 focus:ring-primary-500"
                />
                <div>
                  <div className="font-medium text-neutral-900">Include Performance Tests</div>
                  <div className="text-sm text-neutral-500">
                    Generate load and stress test scenarios
                  </div>
                </div>
              </label>

              {/* Force Regeneration */}
              <label className="flex items-center gap-3 cursor-pointer">
                <input
                  type="checkbox"
                  checked={options.force}
                  onChange={(e) =>
                    setOptions((prev) => ({ ...prev, force: e.target.checked }))
                  }
                  className="w-5 h-5 text-primary-600 rounded border-neutral-300 focus:ring-primary-500"
                />
                <div>
                  <div className="font-medium text-neutral-900">Force Regeneration</div>
                  <div className="text-sm text-neutral-500">
                    Regenerate even if test cases already exist
                  </div>
                </div>
              </label>
            </div>

            <div className="flex items-center justify-end gap-3">
              <button
                onClick={() => setShowOptionsDialog(false)}
                className="px-4 py-2 text-neutral-700 border border-neutral-300 rounded-lg 
                         hover:bg-neutral-50 transition-colors"
              >
                Cancel
              </button>
              <button
                onClick={handleGenerate}
                disabled={isLoading}
                className="px-4 py-2 bg-primary-600 text-white rounded-lg 
                         hover:bg-primary-700 transition-colors disabled:opacity-50"
              >
                {isLoading ? "Generating..." : "Generate"}
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>

      {/* Test Cases Display Dialog (Task 5.5) */}
      <Dialog.Root open={showTestCasesDialog} onOpenChange={setShowTestCasesDialog}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/50 animate-fade-in z-50" />
          <Dialog.Content
            className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
                     bg-white rounded-lg shadow-xl w-full max-w-4xl max-h-[90vh] p-6 z-50
                     animate-scale-in focus:outline-none flex flex-col"
          >
            <Dialog.Title className="text-lg font-semibold text-neutral-900 mb-2">
              Generated Test Cases ({testCases.length})
            </Dialog.Title>
            <Dialog.Description className="text-sm text-neutral-600 mb-4">
              Review and edit the generated test cases for {ticketKey}
            </Dialog.Description>

            {/* Action Buttons */}
            <div className="flex items-center gap-2 mb-4">
              <button
                onClick={handleRegenerate}
                disabled={isLoading}
                className="px-3 py-1.5 text-sm bg-neutral-100 text-neutral-700 rounded-lg 
                         hover:bg-neutral-200 transition-colors disabled:opacity-50"
              >
                Regenerate
              </button>
              <button
                onClick={handleSaveToWorkflow}
                className="px-3 py-1.5 text-sm bg-primary-600 text-white rounded-lg 
                         hover:bg-primary-700 transition-colors"
              >
                Save to Workflow
              </button>
              <button
                onClick={handleSaveToTestmo}
                className="px-3 py-1.5 text-sm bg-success-600 text-white rounded-lg 
                         hover:bg-success-700 transition-colors"
              >
                Save to Testmo
              </button>
            </div>

            {/* Test Cases List */}
            <div className="flex-1 overflow-y-auto space-y-4 pr-2">
              {testCases.map((testCase) => (
                    <TestCaseCard
                      key={testCase.id}
                      testCase={testCase}
                      editedTestCase={editingTestCase}
                      onEdit={() => handleEdit(testCase)}
                      isEditing={editingTestCase?.id === testCase.id}
                      onSave={handleSaveEdit}
                      onCancel={() => setEditingTestCase(null)}
                    />
              ))}
            </div>

            <div className="flex items-center justify-end gap-3 mt-4 pt-4 border-t border-neutral-200">
              <button
                onClick={() => setShowTestCasesDialog(false)}
                className="px-4 py-2 bg-primary-600 text-white rounded-lg 
                         hover:bg-primary-700 transition-colors"
              >
                Close
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </>
  );
}

interface TestCaseCardProps {
  testCase: TestCase;
  editedTestCase: TestCase | null;
  onEdit: (testCase: TestCase) => void;
  isEditing: boolean;
  onSave: (updated: TestCase) => void;
  onCancel: () => void;
}

function TestCaseCard({
  testCase,
  editedTestCase,
  onEdit,
  isEditing,
  onSave,
  onCancel,
}: TestCaseCardProps) {
  const [edited, setEdited] = useState<TestCase>(testCase);

  // Update edited state when editing starts
  useEffect(() => {
    if (isEditing && editedTestCase) {
      setEdited(editedTestCase);
    } else if (!isEditing) {
      setEdited(testCase);
    }
  }, [isEditing, editedTestCase, testCase]);

  if (isEditing) {
    return (
      <div className="border border-neutral-200 rounded-lg p-4 bg-neutral-50">
        <div className="space-y-3">
          <input
            type="text"
            value={edited.title}
            onChange={(e) => setEdited({ ...edited, title: e.target.value })}
            className="w-full px-3 py-2 border border-neutral-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            placeholder="Test case title"
          />
          <textarea
            value={edited.description}
            onChange={(e) => setEdited({ ...edited, description: e.target.value })}
            rows={3}
            className="w-full px-3 py-2 border border-neutral-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            placeholder="Description"
          />
          <div className="flex items-center gap-2">
            <button
              onClick={() => onSave(edited)}
              className="px-3 py-1 text-sm bg-primary-600 text-white rounded hover:bg-primary-700"
            >
              Save
            </button>
            <button
              onClick={() => {
                setEdited(testCase);
                onCancel();
              }}
              className="px-3 py-1 text-sm bg-neutral-200 text-neutral-700 rounded hover:bg-neutral-300"
            >
              Cancel
            </button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="border border-neutral-200 rounded-lg p-4 hover:border-primary-300 transition-colors">
      <div className="flex items-start justify-between mb-2">
        <div className="flex-1">
          <h4 className="font-semibold text-neutral-900 mb-1">{testCase.title}</h4>
          <p className="text-sm text-neutral-600 mb-2">{testCase.description}</p>
          <div className="flex flex-wrap gap-2 mb-2">
            <span
              className={`px-2 py-0.5 text-xs font-medium rounded ${
                testCase.priority === "Critical" || testCase.priority === "P0"
                  ? "bg-error-100 text-error-700"
                  : testCase.priority === "High" || testCase.priority === "P1"
                  ? "bg-warning-100 text-warning-700"
                  : "bg-neutral-100 text-neutral-700"
              }`}
            >
              {testCase.priority}
            </span>
            <span className="px-2 py-0.5 text-xs font-medium rounded bg-primary-100 text-primary-700">
              {testCase.type}
            </span>
            {testCase.tags.map((tag) => (
              <span
                key={tag}
                className="px-2 py-0.5 text-xs font-medium rounded bg-neutral-100 text-neutral-600"
              >
                {tag}
              </span>
            ))}
          </div>
        </div>
        <button
          onClick={() => onEdit(testCase)}
          className="ml-2 px-2 py-1 text-sm text-neutral-600 hover:text-neutral-900 
                   hover:bg-neutral-100 rounded transition-colors"
        >
          Edit
        </button>
      </div>

      {/* Steps Preview */}
      {testCase.steps.length > 0 && (
        <div className="mt-3 pt-3 border-t border-neutral-100">
          <div className="text-xs font-medium text-neutral-500 mb-1">Steps:</div>
          <ol className="list-decimal list-inside space-y-1 text-sm text-neutral-700">
            {testCase.steps.slice(0, 3).map((step, idx) => (
              <li key={idx}>{step}</li>
            ))}
            {testCase.steps.length > 3 && (
              <li className="text-neutral-500 italic">
                +{testCase.steps.length - 3} more step{testCase.steps.length - 3 !== 1 ? "s" : ""}
              </li>
            )}
          </ol>
        </div>
      )}

      {/* Expected Result Preview */}
      {testCase.expectedResult && (
        <div className="mt-2 pt-2 border-t border-neutral-100">
          <div className="text-xs font-medium text-neutral-500 mb-1">Expected Result:</div>
          <p className="text-sm text-neutral-700">{testCase.expectedResult}</p>
        </div>
      )}
    </div>
  );
}

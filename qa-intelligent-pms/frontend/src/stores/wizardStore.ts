/**
 * Setup Wizard state management.
 *
 * Uses Zustand with persist middleware for localStorage persistence.
 * Implements FR-CFG-05: Wizard state persists across browser sessions.
 */
import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

/**
 * Step definition for the wizard.
 */
export interface WizardStep {
  id: number;
  key: string;
  title: string;
  description: string;
  path: string;
  optional: boolean;
}

/**
 * Wizard steps configuration.
 */
export const WIZARD_STEPS: WizardStep[] = [
  {
    id: 1,
    key: "profile",
    title: "Profile",
    description: "Configure your display name and preferences",
    path: "profile",
    optional: false,
  },
  {
    id: 2,
    key: "jira",
    title: "Jira",
    description: "Connect to your Jira instance",
    path: "jira",
    optional: false,
  },
  {
    id: 3,
    key: "postman",
    title: "Postman",
    description: "Connect to Postman for API test search",
    path: "postman",
    optional: true,
  },
  {
    id: 4,
    key: "testmo",
    title: "Testmo",
    description: "Connect to Testmo for test management",
    path: "testmo",
    optional: true,
  },
  {
    id: 5,
    key: "splunk",
    title: "Splunk",
    description: "Configure Splunk for log analysis",
    path: "splunk",
    optional: true,
  },
];

/**
 * Form data stored for each step.
 */
export interface WizardFormData {
  profile?: {
    displayName: string;
    jiraEmail: string;
    ticketStates: string[];
  };
  jira?: {
    instanceUrl: string;
    authMethod?: "api_token" | "oauth";
    // API Token auth
    email?: string;
    apiToken?: string;
    // OAuth auth
    clientId?: string;
    clientSecret?: string;
  };
  postman?: {
    apiKey: string;
    workspaceId?: string;
  };
  testmo?: {
    instanceUrl: string;
    apiKey: string;
  };
  splunk?: {
    baseUrl: string;
    defaultIndex?: string;
  };
}

/**
 * Wizard store state.
 */
interface WizardState {
  /** Current step index (1-based) */
  currentStep: number;
  /** Total number of steps */
  totalSteps: number;
  /** Array of completed step IDs */
  completedSteps: number[];
  /** Array of skipped step IDs */
  skippedSteps: number[];
  /** Form data keyed by step key */
  formData: WizardFormData;
  /** Whether setup wizard is complete */
  isComplete: boolean;
  /** Hydration flag for persisted state */
  _hasHydrated: boolean;
}

/**
 * Wizard store actions.
 */
interface WizardActions {
  /** Navigate to a specific step */
  setCurrentStep: (step: number) => void;
  /** Go to next step */
  nextStep: () => void;
  /** Go to previous step */
  prevStep: () => void;
  /** Mark a step as complete */
  markStepComplete: (stepId: number) => void;
  /** Mark a step as skipped */
  markStepSkipped: (stepId: number) => void;
  /** Save form data for a step */
  setStepData: <K extends keyof WizardFormData>(
    stepKey: K,
    data: WizardFormData[K]
  ) => void;
  /** Mark wizard as complete */
  completeWizard: () => void;
  /** Reset wizard to initial state */
  reset: () => void;
  /** Internal: mark store as hydrated */
  setHasHydrated: (value: boolean) => void;
  /** Expose hydration flag */
  hasHydrated: () => boolean;
  /** Check if a step can be navigated to */
  canNavigateToStep: (stepId: number) => boolean;
  /** Get the current step configuration */
  getCurrentStepConfig: () => WizardStep | undefined;
  /** Check if current step is valid (can proceed) */
  isCurrentStepValid: () => boolean;
}

type WizardStore = WizardState & WizardActions;

const initialState: WizardState = {
  currentStep: 1,
  totalSteps: WIZARD_STEPS.length,
  completedSteps: [],
  skippedSteps: [],
  formData: {},
  isComplete: false,
  _hasHydrated: false,
};

/**
 * Zustand store for wizard state with localStorage persistence.
 */
export const useWizardStore = create<WizardStore>()(
  persist(
    (set, get) => ({
      ...initialState,

      setCurrentStep: (step) => {
        if (step >= 1 && step <= get().totalSteps) {
          set({ currentStep: step });
        }
      },

      nextStep: () => {
        const { currentStep, totalSteps } = get();
        if (currentStep < totalSteps) {
          set({ currentStep: currentStep + 1 });
        }
      },

      prevStep: () => {
        const { currentStep } = get();
        if (currentStep > 1) {
          set({ currentStep: currentStep - 1 });
        }
      },

      markStepComplete: (stepId) =>
        set((state) => ({
          completedSteps: [...new Set([...state.completedSteps, stepId])],
          skippedSteps: state.skippedSteps.filter((id) => id !== stepId),
        })),

      markStepSkipped: (stepId) =>
        set((state) => ({
          skippedSteps: [...new Set([...state.skippedSteps, stepId])],
          completedSteps: state.completedSteps.filter((id) => id !== stepId),
        })),

      setStepData: (stepKey, data) =>
        set((state) => ({
          formData: { ...state.formData, [stepKey]: data },
        })),

      completeWizard: () => set({ isComplete: true }),

      reset: () => set({ ...initialState, _hasHydrated: true }),

      setHasHydrated: (value) => set({ _hasHydrated: value }),

      hasHydrated: () => get()._hasHydrated,

      canNavigateToStep: (stepId) => {
        const { completedSteps, skippedSteps, currentStep } = get();
        // Can navigate to current step, completed steps, skipped steps, or next uncompleted step
        if (stepId === currentStep) return true;
        if (completedSteps.includes(stepId)) return true;
        if (skippedSteps.includes(stepId)) return true;
        // Can navigate to next step after current if current is complete/skipped
        const currentIsHandled =
          completedSteps.includes(currentStep) ||
          skippedSteps.includes(currentStep);
        if (stepId === currentStep + 1 && currentIsHandled) return true;
        return false;
      },

      getCurrentStepConfig: () => {
        const { currentStep } = get();
        return WIZARD_STEPS.find((s) => s.id === currentStep);
      },

      isCurrentStepValid: () => {
        const { currentStep, formData } = get();
        const step = WIZARD_STEPS.find((s) => s.id === currentStep);
        if (!step) return false;

        // Check if step has form data
        const data = formData[step.key as keyof WizardFormData];
        if (!data) return step.optional;

        // Basic validation - non-empty required fields
        if (step.key === "profile") {
          const profile = formData.profile;
          return Boolean(
            profile?.displayName?.trim() &&
              profile?.jiraEmail?.trim() &&
              profile?.ticketStates?.length
          );
        }

        if (step.key === "jira") {
          const jira = formData.jira;
          // Valid if has instance URL and either API Token or OAuth credentials
          const hasApiToken = Boolean(jira?.email?.trim() && jira?.apiToken?.trim());
          const hasOAuth = Boolean(jira?.clientId?.trim() && jira?.clientSecret?.trim());
          return Boolean(jira?.instanceUrl?.trim() && (hasApiToken || hasOAuth));
        }

        // Optional steps are valid if they have any data or are empty
        return true;
      },
    }),
    {
      name: "qa-pms-wizard-state",
      version: 1,
      storage: createJSONStorage(() => localStorage),
      // Persist only serializable state (avoid actions + hydration flag)
      partialize: (state) => ({
        currentStep: state.currentStep,
        totalSteps: state.totalSteps,
        completedSteps: state.completedSteps,
        skippedSteps: state.skippedSteps,
        formData: state.formData,
        isComplete: state.isComplete,
      }),
      migrate: (persistedState, _fromVersion) => {
        // v0 -> v1 migration: schema changes were additive; keep what we can.
        // If anything goes wrong, fall back to a clean state.
        try {
          const s = persistedState as Partial<WizardState>;
          return {
            ...initialState,
            ...s,
            // Ensure new fields exist
            totalSteps: WIZARD_STEPS.length,
            _hasHydrated: false,
          } as WizardState;
        } catch {
          return { ...initialState, _hasHydrated: false };
        }
      },
      onRehydrateStorage: () => (state, error) => {
        // Mark hydration complete regardless of success to avoid blocking UI
        state?.setHasHydrated?.(true);
        if (error) {
          // eslint-disable-next-line no-console
          console.error("Wizard store hydration failed", error);
        }
      },
    }
  )
);

/**
 * User Profile configuration step (Step 1).
 *
 * Placeholder - will be implemented in Story 2.2.
 */
import { useState } from "react";
import { WizardStepHeader } from "@/components/wizard/WizardStepHeader";
import { WizardNavigation } from "@/components/wizard/WizardNavigation";
import { useWizardStore } from "@/stores/wizardStore";

export function UserProfileStep() {
  const { formData, setStepData } = useWizardStore();
  const profileData = formData.profile;

  const [displayName, setDisplayName] = useState(profileData?.displayName ?? "");
  const [jiraEmail, setJiraEmail] = useState(profileData?.jiraEmail ?? "");
  const [ticketStates, setTicketStates] = useState<string[]>(
    profileData?.ticketStates ?? ["Ready for QA"]
  );

  const isValid =
    displayName.trim().length > 0 &&
    jiraEmail.trim().length > 0 &&
    ticketStates.length > 0;

  const handleBeforeNext = () => {
    setStepData("profile", {
      displayName: displayName.trim(),
      jiraEmail: jiraEmail.trim(),
      ticketStates,
    });
    return true;
  };

  return (
    <div>
      <WizardStepHeader
        title="Your Profile"
        description="Configure your display name and Jira preferences"
      />

      <div className="space-y-4">
        <div>
          <label
            htmlFor="displayName"
            className="block text-sm font-medium text-neutral-700 mb-1"
          >
            Display Name
          </label>
          <input
            id="displayName"
            type="text"
            value={displayName}
            onChange={(e) => setDisplayName(e.target.value)}
            placeholder="Your name"
            className="w-full px-3 py-2 border border-neutral-300 rounded-lg
              text-neutral-900 placeholder:text-neutral-400
              focus:ring-2 focus:ring-primary-500 focus:border-primary-500
              transition-colors"
          />
        </div>

        <div>
          <label
            htmlFor="jiraEmail"
            className="block text-sm font-medium text-neutral-700 mb-1"
          >
            Jira Email/Username
          </label>
          <input
            id="jiraEmail"
            type="email"
            value={jiraEmail}
            onChange={(e) => setJiraEmail(e.target.value)}
            placeholder="your.email@company.com"
            className="w-full px-3 py-2 border border-neutral-300 rounded-lg
              text-neutral-900 placeholder:text-neutral-400
              focus:ring-2 focus:ring-primary-500 focus:border-primary-500
              transition-colors"
          />
          <p className="mt-1 text-sm text-neutral-500">
            Used to filter tickets assigned to you
          </p>
        </div>

        <div>
          <label className="block text-sm font-medium text-neutral-700 mb-1">
            Ticket States to Track
          </label>
          <div className="space-y-2">
            {["Ready for QA", "In Progress", "In Review", "Testing"].map(
              (state) => (
                <label key={state} className="flex items-center gap-2">
                  <input
                    type="checkbox"
                    checked={ticketStates.includes(state)}
                    onChange={(e) => {
                      if (e.target.checked) {
                        setTicketStates([...ticketStates, state]);
                      } else {
                        setTicketStates(ticketStates.filter((s) => s !== state));
                      }
                    }}
                    className="w-4 h-4 text-primary-500 border-neutral-300 rounded
                      focus:ring-primary-500"
                  />
                  <span className="text-sm text-neutral-700">{state}</span>
                </label>
              )
            )}
          </div>
        </div>
      </div>

      <WizardNavigation isValid={isValid} onBeforeNext={handleBeforeNext} />
    </div>
  );
}

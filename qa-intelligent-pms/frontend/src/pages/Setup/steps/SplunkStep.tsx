/**
 * Splunk configuration step (Step 5).
 *
 * Placeholder - will be fully implemented in Story 2.6.
 * Note: Splunk is manual configuration only (no API test).
 */
import { useState } from "react";
import { WizardStepHeader } from "@/components/wizard/WizardStepHeader";
import { WizardNavigation } from "@/components/wizard/WizardNavigation";
import { useWizardStore } from "@/stores/wizardStore";

export function SplunkStep() {
  const { formData, setStepData } = useWizardStore();
  const splunkData = formData.splunk;

  const [baseUrl, setBaseUrl] = useState(splunkData?.baseUrl ?? "");
  const [defaultIndex, setDefaultIndex] = useState(
    splunkData?.defaultIndex ?? ""
  );

  const handleBeforeNext = () => {
    if (baseUrl.trim()) {
      setStepData("splunk", {
        baseUrl: baseUrl.trim(),
        defaultIndex: defaultIndex.trim() || undefined,
      });
    }
    return true;
  };

  return (
    <div>
      <WizardStepHeader
        title="Splunk Configuration"
        description="Configure Splunk for log analysis (manual configuration)"
        optional
      />

      <div className="space-y-4">
        <div className="p-4 bg-warning-50 border border-warning-200 rounded-lg text-sm text-warning-800">
          <p className="font-medium">Manual Configuration</p>
          <p className="mt-1">
            Splunk connection is not tested automatically. Please verify your
            settings are correct.
          </p>
        </div>

        <div>
          <label
            htmlFor="baseUrl"
            className="block text-sm font-medium text-neutral-700 mb-1"
          >
            Splunk Base URL
          </label>
          <input
            id="baseUrl"
            type="url"
            value={baseUrl}
            onChange={(e) => setBaseUrl(e.target.value)}
            placeholder="https://splunk.your-company.com:8089"
            className="w-full px-3 py-2 border border-neutral-300 rounded-lg
              focus:ring-2 focus:ring-primary-500 focus:border-primary-500
              transition-colors"
          />
          <p className="mt-1 text-sm text-neutral-500">
            The URL to your Splunk management port
          </p>
        </div>

        <div>
          <label
            htmlFor="defaultIndex"
            className="block text-sm font-medium text-neutral-700 mb-1"
          >
            Default Index (optional)
          </label>
          <input
            id="defaultIndex"
            type="text"
            value={defaultIndex}
            onChange={(e) => setDefaultIndex(e.target.value)}
            placeholder="main"
            className="w-full px-3 py-2 border border-neutral-300 rounded-lg
              focus:ring-2 focus:ring-primary-500 focus:border-primary-500
              transition-colors"
          />
          <p className="mt-1 text-sm text-neutral-500">
            The default Splunk index to search when querying logs
          </p>
        </div>

        <div className="p-4 bg-neutral-50 rounded-lg text-sm text-neutral-600">
          <p className="font-medium mb-2">Why configure Splunk?</p>
          <ul className="list-disc list-inside space-y-1">
            <li>Search application logs during testing</li>
            <li>Correlate errors with test failures</li>
            <li>Use pre-built query templates</li>
          </ul>
        </div>
      </div>

      <WizardNavigation isValid onBeforeNext={handleBeforeNext} />
    </div>
  );
}

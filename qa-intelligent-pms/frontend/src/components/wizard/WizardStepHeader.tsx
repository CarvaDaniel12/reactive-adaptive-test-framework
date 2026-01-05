/**
 * Wizard step header component.
 *
 * Displays step title and description with consistent styling.
 */

interface WizardStepHeaderProps {
  /** Step title */
  title: string;
  /** Step description */
  description?: string;
  /** Whether this step is optional */
  optional?: boolean;
}

export function WizardStepHeader({
  title,
  description,
  optional = false,
}: WizardStepHeaderProps) {
  return (
    <div className="mb-6">
      <div className="flex items-center gap-2">
        <h2 className="text-2xl font-semibold text-neutral-900">{title}</h2>
        {optional && (
          <span className="text-xs font-medium px-2 py-0.5 bg-neutral-200 text-neutral-600 rounded">
            Optional
          </span>
        )}
      </div>
      {description && (
        <p className="mt-2 text-neutral-600">{description}</p>
      )}
    </div>
  );
}

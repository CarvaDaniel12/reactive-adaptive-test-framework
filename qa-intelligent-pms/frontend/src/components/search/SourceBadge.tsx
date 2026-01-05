import type { ReactNode } from "react";

interface SourceBadgeProps {
  source: "postman" | "testmo" | string;
  size?: "sm" | "md";
}

// SVG Icons for each source
function PostmanIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="currentColor">
      <path d="M13.527.099C6.955-.744.942 3.9.099 10.473c-.843 6.572 3.8 12.584 10.373 13.428 6.573.843 12.587-3.801 13.428-10.374C24.744 6.955 20.101.943 13.527.099zm2.471 7.485a.855.855 0 0 0-.593.25l-4.453 4.453-.307-.307-.643-.643c4.389-4.376 5.18-4.418 5.996-3.753zm-4.863 4.861l4.44-4.44a.62.62 0 1 1 .847.903l-4.699 4.125-.588-.588zm.33.694l-1.1.238a.06.06 0 0 1-.067-.032.06.06 0 0 1 .01-.073l.645-.645.512.512zm-2.803-.459l1.172-1.172.879.878-1.979.426a.074.074 0 0 1-.085-.046.073.073 0 0 1 .013-.086zm5.635 6.576c-.633.627-1.332 1.188-2.089 1.675l-.263-.329c2.122-1.626 3.512-3.653 3.512-6.332 0-.678-.088-1.338-.252-1.97l.41-.032a8.3 8.3 0 0 1 .262 2.002c0 1.86-.538 3.552-1.58 4.986zm-2.827 2.078a10.9 10.9 0 0 1-1.458.617l-.193-.39a9.5 9.5 0 0 0 1.456-.603l.195.376zm-2.175.88a10.9 10.9 0 0 1-1.534.247l-.062-.428a10.5 10.5 0 0 0 1.481-.238l.115.419z" />
    </svg>
  );
}

function TestmoIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
    </svg>
  );
}

function DefaultIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
    </svg>
  );
}

const SOURCE_CONFIG: Record<string, {
  label: string;
  bgColor: string;
  textColor: string;
  borderColor: string;
  icon: (props: { className?: string }) => ReactNode;
}> = {
  postman: {
    label: "Postman",
    bgColor: "bg-orange-100",
    textColor: "text-orange-700",
    borderColor: "border-orange-200",
    icon: PostmanIcon,
  },
  testmo: {
    label: "Testmo",
    bgColor: "bg-blue-100",
    textColor: "text-blue-700",
    borderColor: "border-blue-200",
    icon: TestmoIcon,
  },
};

/**
 * Badge component to display the source of a search result.
 * 
 * Uses distinct colors for each integration:
 * - Postman: Orange
 * - Testmo: Blue
 */
export function SourceBadge({ source, size = "md" }: SourceBadgeProps) {
  const config = SOURCE_CONFIG[source] || {
    label: source,
    bgColor: "bg-neutral-100",
    textColor: "text-neutral-700",
    borderColor: "border-neutral-200",
    icon: DefaultIcon,
  };

  const Icon = config.icon;
  const sizeClasses = size === "sm"
    ? "text-xs px-1.5 py-0.5"
    : "text-sm px-2 py-1";
  const iconSize = size === "sm" ? "w-3 h-3" : "w-4 h-4";

  return (
    <span
      className={`
        inline-flex items-center gap-1 rounded-md font-medium border
        ${config.bgColor} ${config.textColor} ${config.borderColor} ${sizeClasses}
      `}
      role="status"
      aria-label={`Source: ${config.label}`}
    >
      <Icon className={iconSize} />
      {config.label}
    </span>
  );
}

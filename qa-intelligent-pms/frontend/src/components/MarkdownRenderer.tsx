interface MarkdownRendererProps {
  /** HTML content to render */
  html: string;
  /** Whether to highlight Gherkin keywords */
  highlightGherkin?: boolean;
  /** Additional CSS classes */
  className?: string;
}

/** Gherkin keywords to highlight */
const GHERKIN_KEYWORDS = [
  "Feature:",
  "Background:",
  "Scenario:",
  "Scenario Outline:",
  "Examples:",
  "Given ",
  "When ",
  "Then ",
  "And ",
  "But ",
];

/**
 * Renders HTML content with optional Gherkin syntax highlighting.
 * Used for displaying Jira ticket descriptions and comments.
 */
export function MarkdownRenderer({
  html,
  highlightGherkin = false,
  className = "",
}: MarkdownRendererProps) {
  let processedHtml = html;

  // Highlight Gherkin keywords if enabled
  if (highlightGherkin) {
    GHERKIN_KEYWORDS.forEach((keyword) => {
      // Escape special regex characters in keyword
      const escapedKeyword = keyword.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
      const regex = new RegExp(`(${escapedKeyword})`, "g");
      processedHtml = processedHtml.replace(
        regex,
        '<span class="gherkin-keyword">$1</span>'
      );
    });
  }

  return (
    <div
      className={`markdown-content ${className}`}
      dangerouslySetInnerHTML={{ __html: processedHtml }}
    />
  );
}

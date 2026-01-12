/**
 * Integration Timeline component.
 * Displays integration events in a timeline format.
 */
import type { IntegrationEvent } from "@/hooks/useIntegrationHealth";

interface IntegrationTimelineProps {
  events: IntegrationEvent[];
}

export function IntegrationTimeline({ events }: IntegrationTimelineProps) {
  const getSeverityColor = (severity: string): string => {
    switch (severity.toLowerCase()) {
      case "critical":
        return "bg-red-100 text-red-700";
      case "high":
        return "bg-orange-100 text-orange-700";
      case "medium":
        return "bg-yellow-100 text-yellow-700";
      case "low":
        return "bg-blue-100 text-blue-700";
      default:
        return "bg-neutral-100 text-neutral-700";
    }
  };

  const formatEventType = (eventType: string): string => {
    return eventType
      .split("_")
      .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
      .join(" ");
  };

  return (
    <div className="bg-white border border-neutral-200 rounded-lg p-6">
      <h2 className="text-lg font-semibold text-neutral-900 mb-4">Recent Events</h2>

      {events.length === 0 ? (
        <div className="py-8 text-center">
          <p className="text-sm text-neutral-500">No events found</p>
        </div>
      ) : (
        <div className="space-y-4">
          {events.map((event) => (
            <div
              key={event.id}
              className="flex items-start gap-4 pb-4 border-b border-neutral-100 last:border-0 last:pb-0"
            >
              <div className="flex-shrink-0 w-2 h-2 rounded-full bg-neutral-400 mt-2" />
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2 mb-1 flex-wrap">
                  <span className="text-sm font-medium text-neutral-900">
                    {formatEventType(event.eventType)}
                  </span>
                  <span
                    className={`text-xs px-2 py-0.5 rounded capitalize ${getSeverityColor(
                      event.severity
                    )}`}
                  >
                    {event.severity}
                  </span>
                </div>
                {event.message && (
                  <p className="text-sm text-neutral-600 mb-1">{event.message}</p>
                )}
                <p className="text-xs text-neutral-500">
                  {new Date(event.occurredAt).toLocaleString()}
                </p>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

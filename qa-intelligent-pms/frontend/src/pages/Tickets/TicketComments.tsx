import { formatDistanceToNow, format } from "date-fns";
import { MarkdownRenderer } from "@/components/MarkdownRenderer";
import type { CommentInfo } from "./types";

interface TicketCommentsProps {
  comments: CommentInfo[];
}

function formatCommentDate(dateString: string): { relative: string; full: string } {
  const date = new Date(dateString);
  return {
    relative: formatDistanceToNow(date, { addSuffix: true }),
    full: format(date, "PPpp"),
  };
}

export function TicketComments({ comments }: TicketCommentsProps) {
  if (comments.length === 0) {
    return null;
  }

  return (
    <div className="bg-white border border-neutral-200 rounded-xl overflow-hidden">
      {/* Header */}
      <div className="px-6 py-4 border-b border-neutral-200 bg-neutral-50">
        <h2 className="text-lg font-semibold text-neutral-900 flex items-center gap-2">
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
              d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
            />
          </svg>
          Comments
          <span className="text-sm font-normal text-neutral-500">
            ({comments.length} {comments.length === 1 ? "comment" : "comments"})
          </span>
        </h2>
      </div>

      {/* Comments List */}
      <div className="divide-y divide-neutral-100">
        {comments.map((comment) => {
          const date = formatCommentDate(comment.createdAt);
          return (
            <div key={comment.id} className="p-6 animate-fade-in">
              {/* Comment Header */}
              <div className="flex items-center gap-3 mb-3">
                {comment.author.avatarUrl ? (
                  <img
                    src={comment.author.avatarUrl}
                    alt=""
                    className="w-8 h-8 rounded-full ring-2 ring-neutral-100"
                  />
                ) : (
                  <div className="w-8 h-8 rounded-full bg-primary-100 flex items-center justify-center ring-2 ring-neutral-100">
                    <span className="text-sm font-semibold text-primary-700">
                      {comment.author.name.charAt(0).toUpperCase()}
                    </span>
                  </div>
                )}
                <div className="flex-1 min-w-0">
                  <span className="font-medium text-neutral-900">
                    {comment.author.name}
                  </span>
                  <span
                    className="text-sm text-neutral-500 ml-2"
                    title={date.full}
                  >
                    {date.relative}
                  </span>
                </div>
              </div>

              {/* Comment Body */}
              <div className="pl-11">
                <MarkdownRenderer html={comment.bodyHtml} />
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}

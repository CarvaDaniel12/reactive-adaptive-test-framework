/** Ticket summary from API (list view) */
export interface TicketSummary {
  key: string;
  title: string;
  status: string;
  statusColor: string;
  priority: string | null;
  priorityColor: string;
  assigneeName: string | null;
  assigneeAvatar: string | null;
  updatedAt: string;
}

/** API response for ticket list */
export interface TicketListResponse {
  tickets: TicketSummary[];
  total: number;
  page: number;
  pageSize: number;
  hasMore: boolean;
  loadTimeMs?: number;
}

/** User information */
export interface UserInfo {
  name: string;
  email: string | null;
  avatarUrl: string | null;
}

/** Comment on a ticket */
export interface CommentInfo {
  id: string;
  author: UserInfo;
  bodyHtml: string;
  createdAt: string;
}

/** Attachment on a ticket */
export interface AttachmentInfo {
  id: string;
  filename: string;
  mimeType: string;
  size: number;
  sizeHuman: string;
  downloadUrl: string;
}

/** Full ticket detail from API */
export interface TicketDetail {
  key: string;
  title: string;
  descriptionHtml: string | null;
  descriptionRaw: string | null;
  status: string;
  statusColor: string;
  priority: string | null;
  priorityColor: string;
  assignee: UserInfo | null;
  reporter: UserInfo | null;
  createdAt: string;
  updatedAt: string;
  comments: CommentInfo[];
  attachments: AttachmentInfo[];
  labels: string[];
  hasGherkin: boolean;
  loadTimeMs?: number;
}

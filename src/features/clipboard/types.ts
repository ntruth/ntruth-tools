export type ClipboardKind = "text" | "image" | "file";

export interface ClipboardEntry {
  id: string;
  type: ClipboardKind;
  content: string;
  created_at: string;
  pinned: boolean;
  tags: string[];
}

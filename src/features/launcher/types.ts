export type LauncherResultType = "app" | "tool" | "command" | "file" | "web";

export interface LauncherEntry {
  id: string;
  label: string;
  description: string;
  type: LauncherResultType;
  keywords: string[];
  icon?: string;
  execute?: string;
  weight?: number;
  platforms?: Array<"windows" | "mac" | "linux">;
}

export interface LauncherResult {
  entry: LauncherEntry;
  score: number;
  badge?: string;
  meta?: string;
  data?: Record<string, unknown>;
}

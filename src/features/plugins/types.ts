export interface PluginManifest {
  id: string;
  name: string;
  version: string;
  author: string;
  summary: string;
  category: string;
  repository?: string | null;
  homepage?: string | null;
}

export interface WorkflowTrigger {
  type: string;
  value?: string | null;
}

export interface WorkflowNode {
  id: string;
  kind: string;
  label: string;
  config: Record<string, any>;
}

export interface WorkflowDefinition {
  id: string;
  name: string;
  description: string;
  trigger?: WorkflowTrigger | null;
  nodes: WorkflowNode[];
  updated_at: string;
}

export interface WorkflowRunResult {
  workflow_id: string;
  executed_at: string;
  logs: string[];
  status: string;
}

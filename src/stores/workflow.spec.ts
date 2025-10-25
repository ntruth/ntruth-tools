import { beforeEach, describe, expect, it, vi } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useWorkflowStore } from "./workflow";
import type { WorkflowDefinition, WorkflowRunResult } from "@/features/workflow/types";

const mockWorkflow: WorkflowDefinition = {
  id: "wf-test",
  name: "Test Flow",
  description: "Demo",
  trigger: { type: "manual", value: null },
  nodes: [],
  updated_at: new Date().toISOString(),
};

const mockRun: WorkflowRunResult = {
  workflow_id: "wf-test",
  executed_at: new Date().toISOString(),
  logs: ["step 1"],
  status: "success",
};

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn((cmd: string, args: Record<string, unknown>) => {
    switch (cmd) {
      case "workflow_list":
        return Promise.resolve([mockWorkflow]);
      case "workflow_save":
        return Promise.resolve({ ...mockWorkflow, ...args.workflow });
      case "workflow_run":
        return Promise.resolve(mockRun);
      default:
        return Promise.resolve(null);
    }
  }),
}));

describe("useWorkflowStore", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it("loads workflows and selects first entry", async () => {
    const store = useWorkflowStore();
    await store.fetchWorkflows();
    expect(store.state.items).toHaveLength(1);
    expect(store.state.selectedId).toBe("wf-test");
  });

  it("adds nodes into editable workflow", () => {
    const store = useWorkflowStore();
    store.editable = { ...mockWorkflow, nodes: [] };
    store.addNode("notify");
    expect(store.editable?.nodes).toHaveLength(1);
    expect(store.editable?.nodes[0].kind).toBe("notify");
  });

  it("runs workflow and stores logs", async () => {
    const store = useWorkflowStore();
    store.state.selectedId = "wf-test";
    const result = await store.run();
    expect(result?.logs).toContain("step 1");
    expect(store.state.runLog?.status).toBe("success");
  });
});

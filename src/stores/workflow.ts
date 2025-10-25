import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { reactive, ref } from "vue";
import type { WorkflowDefinition, WorkflowRunResult, WorkflowNode } from "@/features/workflow/types";

interface WorkflowState {
  items: WorkflowDefinition[];
  loading: boolean;
  selectedId: string | null;
  runLog: WorkflowRunResult | null;
}

const buildNode = (kind: string): WorkflowNode => {
  const base: Record<string, string> = {
    read_clipboard: "读取剪贴板",
    append_file: "写入文件",
    notify: "显示通知",
    capture_screen: "截图",
    pin_canvas: "固定贴图",
    custom: "自定义脚本",
  };

  const config: Record<string, unknown> = { note: "" };

  if (kind === "append_file") {
    config.path = "~/Library/Logs/unitools-workflow.txt";
    config.useClipboard = true;
  }

  if (kind === "notify") {
    config.title = "UniTools 工作流";
    config.message = "操作完成";
  }

  if (kind === "custom") {
    config.script = "";
  }

  return {
    id: `node-${Date.now()}-${Math.random().toString(16).slice(2, 6)}`,
    kind,
    label: base[kind] ?? "自定义",
    config,
  };
};

export const useWorkflowStore = defineStore("workflow", () => {
  const state = reactive<WorkflowState>({
    items: [],
    loading: false,
    selectedId: null,
    runLog: null,
  });

  const editable = ref<WorkflowDefinition | null>(null);

  const fetchWorkflows = async () => {
    state.loading = true;
    try {
      state.items = await invoke<WorkflowDefinition[]>("workflow_list");
      if (!state.selectedId && state.items.length) {
        select(state.items[0].id);
      }
    } finally {
      state.loading = false;
    }
  };

  const select = (id: string) => {
    state.selectedId = id;
    const target = state.items.find((item) => item.id === id);
    editable.value = target ? JSON.parse(JSON.stringify(target)) : null;
  };

  const addNode = (kind: string) => {
    if (!editable.value) return;
    editable.value.nodes.push(buildNode(kind));
  };

  const removeNode = (nodeId: string) => {
    if (!editable.value) return;
    editable.value.nodes = editable.value.nodes.filter((node) => node.id !== nodeId);
  };

  const updateTrigger = (type: string, value?: string) => {
    if (!editable.value) return;
    editable.value.trigger = {
      type,
      value: value ?? null,
    };
  };

  const save = async () => {
    if (!editable.value) return null;
    const result = await invoke<WorkflowDefinition>("workflow_save", {
      workflow: editable.value,
    });
    const index = state.items.findIndex((item) => item.id === result.id);
    if (index >= 0) {
      state.items.splice(index, 1, result);
    } else {
      state.items.push(result);
    }
    select(result.id);
    return result;
  };

  const run = async (id?: string) => {
    const workflowId = id ?? state.selectedId;
    if (!workflowId) return null;
    state.runLog = await invoke<WorkflowRunResult>("workflow_run", { id: workflowId });
    return state.runLog;
  };

  const create = () => {
    const template: WorkflowDefinition = {
      id: "",
      name: "新建工作流",
      description: "描述此工作流的用途",
      trigger: {
        type: "manual",
        value: null,
      },
      nodes: [],
      updated_at: new Date().toISOString(),
    };
    editable.value = template;
    state.selectedId = null;
  };

  return {
    state,
    editable,
    fetchWorkflows,
    select,
    addNode,
    removeNode,
    updateTrigger,
    save,
    run,
    create,
  };
});

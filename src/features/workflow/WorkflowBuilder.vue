<template>
  <section class="workflow">
    <aside class="sidebar">
      <header>
        <h3>自动化工作流</h3>
        <button class="primary" @click="create">新建工作流</button>
      </header>
      <nav>
        <button
          v-for="item in store.state.items"
          :key="item.id"
          :class="['nav-item', { active: item.id === store.state.selectedId }]"
          @click="() => store.select(item.id)"
        >
          <strong>{{ item.name }}</strong>
          <small>{{ formatDate(item.updated_at) }}</small>
        </button>
      </nav>
    </aside>

    <div class="canvas" v-if="store.editable">
      <header class="canvas-header">
        <div>
          <input v-model="store.editable.name" placeholder="工作流名称" />
          <textarea v-model="store.editable.description" placeholder="描述工作流的目标" />
        </div>
        <div class="trigger">
          <label for="trigger">触发器</label>
          <select
            id="trigger"
            :value="store.editable.trigger?.type ?? 'manual'"
            @change="(event) => onTriggerChange((event.target as HTMLSelectElement).value)"
          >
            <option value="manual">手动</option>
            <option value="clipboard">剪贴板</option>
            <option value="shortcut">快捷键</option>
            <option value="schedule">定时</option>
          </select>
          <input
            v-if="store.editable.trigger?.type !== 'manual'"
            :value="store.editable.trigger?.value ?? ''"
            placeholder="触发参数，如 Ctrl+Alt+S 或 cron 表达式"
            @input="(event) => onTriggerValue((event.target as HTMLInputElement).value)"
          />
        </div>
        <div class="canvas-actions">
          <button class="ghost" :disabled="!store.editable?.id" @click="() => store.run(store.editable?.id)">测试运行</button>
          <button class="primary" @click="save">保存</button>
        </div>
      </header>

      <section class="board">
        <div class="palette">
          <h4>节点库</h4>
          <button v-for="node in palette" :key="node.kind" @click="() => store.addNode(node.kind)">
            {{ node.label }}
          </button>
        </div>

        <div class="nodes">
        <article
          v-for="node in store.editable.nodes"
          :key="node.id"
          class="node"
        >
          <header>
            <span class="kind">{{ translateKind(node.kind) }}</span>
            <button class="ghost" @click="() => store.removeNode(node.id)">移除</button>
          </header>
          <input v-model="node.label" placeholder="节点名称" />
          <template v-if="node.kind === 'append_file'">
            <label class="config-label">文件路径</label>
            <input
              v-model="node.config.path"
              placeholder="~/Library/Logs/unitools-workflow.txt"
            />
            <label class="inline">
              <input type="checkbox" v-model="node.config.useClipboard" />
              若未指定内容，使用上一节点的剪贴板结果
            </label>
            <label class="config-label">追加内容</label>
            <textarea
              v-model="node.config.content"
              placeholder="留空则使用剪贴板内容"
            ></textarea>
          </template>
          <template v-else-if="node.kind === 'notify'">
            <label class="config-label">通知标题</label>
            <input v-model="node.config.title" placeholder="UniTools 工作流" />
            <label class="config-label">通知内容</label>
            <textarea
              v-model="node.config.message"
              placeholder="操作完成"
            ></textarea>
          </template>
          <template v-else-if="node.kind === 'custom'">
            <label class="config-label">脚本（shell）</label>
            <textarea
              v-model="node.config.script"
              placeholder="echo \"Hello from UniTools\""
            ></textarea>
          </template>
          <textarea
            v-model="node.config.note"
            placeholder="备注信息（可选）"
          />
        </article>

          <p v-if="!store.editable.nodes.length" class="empty">
            从左侧节点库中选择节点以构建流程。
          </p>
        </div>
      </section>

      <footer v-if="store.state.runLog" class="log">
        <h4>最近运行 · {{ formatDate(store.state.runLog.executed_at) }}</h4>
        <code v-for="log in store.state.runLog.logs" :key="log">{{ log }}</code>
      </footer>
    </div>

    <div v-else class="placeholder">
      <p>请选择已有工作流或点击“新建工作流”。</p>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed, onMounted, watch } from "vue";
import { useWorkflowStore } from "@/stores/workflow";

const store = useWorkflowStore();

const palette = computed(() => [
  { kind: "read_clipboard", label: "读取剪贴板" },
  { kind: "append_file", label: "写入文件" },
  { kind: "notify", label: "显示通知" },
  { kind: "capture_screen", label: "截图" },
  { kind: "pin_canvas", label: "固定贴图" },
  { kind: "custom", label: "自定义脚本" }
]);

const formatDate = (iso: string) => new Date(iso).toLocaleString();

const translateKind = (kind: string) => palette.value.find((item) => item.kind === kind)?.label ?? kind;

const onTriggerChange = (type: string) => {
  store.updateTrigger(type, type === "manual" ? undefined : "");
};

const onTriggerValue = (value: string) => {
  if (store.editable?.trigger) {
    store.updateTrigger(store.editable.trigger.type, value);
  }
};

const save = () => {
  void store.save();
};

const create = () => {
  store.create();
};

onMounted(() => {
  void store.fetchWorkflows();
});

watch(
  () => store.editable,
  (value) => {
    if (!value) return;
    value.nodes.forEach((node) => {
      node.config = node.config ?? {};
      if (node.config.note === undefined) node.config.note = "";
      if (node.kind === "append_file") {
        if (node.config.path === undefined) node.config.path = "~/Library/Logs/unitools-workflow.txt";
        if (node.config.useClipboard === undefined) node.config.useClipboard = true;
        if (node.config.content === undefined) node.config.content = "";
      }
      if (node.kind === "notify") {
        if (node.config.title === undefined) node.config.title = "UniTools 工作流";
        if (node.config.message === undefined) node.config.message = "操作完成";
      }
      if (node.kind === "custom" && node.config.script === undefined) {
        node.config.script = "";
      }
    });
  },
  { deep: true }
);
</script>

<style scoped>
.workflow {
  display: grid;
  grid-template-columns: 280px 1fr;
  gap: 1.25rem;
}

.sidebar {
  background: var(--color-surface);
  border-radius: 1.1rem;
  padding: 1.1rem;
  border: 1px solid rgba(15, 23, 42, 0.08);
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.sidebar header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.primary {
  border: none;
  border-radius: 0.75rem;
  background: linear-gradient(120deg, #0ea5e9 0%, #6366f1 100%);
  color: #fff;
  padding: 0.4rem 0.85rem;
  cursor: pointer;
}

.ghost {
  border: none;
  background: rgba(148, 163, 184, 0.2);
  border-radius: 0.65rem;
  padding: 0.3rem 0.6rem;
  cursor: pointer;
}

nav {
  display: grid;
  gap: 0.5rem;
}

.nav-item {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 0.15rem;
  border-radius: 0.85rem;
  padding: 0.6rem 0.75rem;
  border: 1px solid transparent;
  background: rgba(148, 163, 184, 0.1);
  cursor: pointer;
}

.nav-item.active {
  border-color: rgba(99, 102, 241, 0.5);
  background: rgba(99, 102, 241, 0.15);
}

.nav-item small {
  color: var(--color-muted);
}

.canvas {
  background: var(--color-surface);
  border-radius: 1.2rem;
  padding: 1.25rem;
  border: 1px solid rgba(15, 23, 42, 0.08);
  display: grid;
  gap: 1rem;
}

.canvas-header {
  display: grid;
  gap: 1rem;
  grid-template-columns: 1fr auto auto;
  align-items: center;
}

.canvas-header input,
.canvas-header textarea,
.trigger select,
.trigger input {
  width: 100%;
  border-radius: 0.75rem;
  border: 1px solid rgba(148, 163, 184, 0.35);
  padding: 0.55rem 0.75rem;
}

.canvas-header textarea {
  min-height: 60px;
  resize: vertical;
}

.trigger {
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
}

.canvas-actions {
  display: flex;
  gap: 0.5rem;
}

.canvas-actions button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.board {
  display: grid;
  grid-template-columns: 200px 1fr;
  gap: 1rem;
}

.palette {
  background: rgba(148, 163, 184, 0.12);
  border-radius: 1rem;
  padding: 0.9rem;
  display: grid;
  gap: 0.5rem;
}

.palette button {
  border: none;
  border-radius: 0.75rem;
  background: rgba(99, 102, 241, 0.18);
  padding: 0.45rem 0.6rem;
  cursor: pointer;
}

.nodes {
  display: grid;
  gap: 0.85rem;
}

.node {
  background: rgba(99, 102, 241, 0.08);
  border-radius: 1rem;
  padding: 0.85rem;
  display: grid;
  gap: 0.5rem;
}

.node header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.node .kind {
  font-weight: 600;
}

.node input,
.node textarea {
  border-radius: 0.6rem;
  border: 1px solid rgba(148, 163, 184, 0.3);
  padding: 0.45rem 0.6rem;
  width: 100%;
}

.node textarea {
  min-height: 60px;
}

.config-label {
  font-size: 0.85rem;
  color: var(--color-muted);
  margin-top: 0.35rem;
}

.inline {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.85rem;
  color: var(--color-muted);
}

.empty {
  color: var(--color-muted);
}

.log {
  display: grid;
  gap: 0.35rem;
  background: rgba(148, 163, 184, 0.1);
  border-radius: 1rem;
  padding: 0.8rem;
}

.log code {
  background: rgba(15, 23, 42, 0.08);
  padding: 0.4rem 0.6rem;
  border-radius: 0.6rem;
}

.placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(148, 163, 184, 0.12);
  border-radius: 1.2rem;
  color: var(--color-muted);
}

@media (max-width: 1080px) {
  .workflow {
    grid-template-columns: 1fr;
  }

  .board {
    grid-template-columns: 1fr;
  }
}
</style>

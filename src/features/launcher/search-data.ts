import { isMac, isWindows } from "@/utils/platform";
import type { LauncherEntry } from "./types";

type Platform = "mac" | "windows" | "linux";

const currentPlatform: Platform = isMac ? "mac" : isWindows ? "windows" : "linux";

const APPLICATION_ENTRIES: LauncherEntry[] = [
  {
    id: "text-editor",
    label: isMac ? "TextEdit" : "记事本",
    description: isMac ? "macOS 内置文本编辑器" : "Windows 内置文本编辑器",
    type: "app",
    keywords: isMac
      ? ["textedit", "editor", "文本", "text", "写作"]
      : ["notepad", "记事本", "txt", "editor", "文本编辑", "nano"],
    icon: "📝",
    weight: 90,
    platforms: isMac ? ["mac"] : ["windows", "linux"]
  },
  {
    id: "terminal",
    label: isMac ? "终端" : "PowerShell",
    description: isMac ? "macOS 终端" : "Windows 自动化 shell",
    type: "app",
    keywords: isMac
      ? ["terminal", "终端", "shell", "命令行"]
      : ["powershell", "pwsh", "shell", "控制台", "命令行", "terminal"],
    icon: "🛠️",
    weight: 80,
    platforms: isMac ? ["mac"] : ["windows", "linux"]
  },
  {
    id: "screenshot-tool",
    label: "截图工具",
    description: isMac ? "打开 Screenshot，捕获屏幕" : "打开 SnippingTool 进行截图",
    type: "app",
    keywords: ["截图", "screenshot", "capture", "snip"],
    icon: "✂️",
    weight: 85,
    platforms: isMac ? ["mac"] : isWindows ? ["windows"] : []
  },
  {
    id: "system-settings",
    label: "系统设置",
    description: isMac ? "打开系统设置" : "打开 Windows 设置中心",
    type: "app",
    keywords: ["settings", "系统设置", "preferences", "control"],
    icon: "⚙️",
    weight: 75,
    platforms: isMac ? ["mac"] : isWindows ? ["windows"] : []
  },
  {
    id: "unitools-docs",
    label: "UniTools 文档",
    description: "打开项目帮助中心",
    type: "command",
    keywords: ["docs", "文档", "帮助", "support"],
    execute: "https://github.com/yourusername/unitools",
    icon: "📚",
    weight: 65
  },
  {
    id: "unitools-workflow",
    label: "工作流构建器",
    description: "创建自动化工作流任务",
    type: "command",
    keywords: ["workflow", "自动化", "flow", "drag"],
    execute: "workflow",
    icon: "🔗",
    weight: 70
  }
];

export const TOOL_ENTRIES: LauncherEntry[] = [
  {
    id: "calculator",
    label: "计算器",
    description: "输入算式即可实时计算",
    type: "tool",
    keywords: ["calc", "=", "计算器"],
    icon: "🧮",
    weight: 95
  },
  {
    id: "translator",
    label: "翻译助手",
    description: "使用 `tr en 你好` 或 `translate zh hello`",
    type: "tool",
    keywords: ["translate", "tr", "翻译"],
    icon: "🌐",
    weight: 92
  }
];

export const LAUNCHER_ENTRIES: LauncherEntry[] = [...APPLICATION_ENTRIES, ...TOOL_ENTRIES].filter(
  (entry) => !entry.platforms || entry.platforms.includes(currentPlatform)
);

# 截图工具技术规格说明书 (Technical Specification)

**版本**: v1.0  
**日期**: 2024年12月17日  
**技术栈**: Tauri 2.0 + Rust + SolidJS  

---

## 目录

1. [核心性能架构](#1-核心性能架构)
2. [高清截图模块](#2-高清截图模块)
3. [标注工具链](#3-标注工具链)
4. [PIN 贴图系统](#4-pin-贴图系统)
5. [OCR 文字识别](#5-ocr-文字识别)
6. [UI 元素自动检测](#6-ui-元素自动检测)

---

## 1. 核心性能架构

### 1.1 极速唤起机制 (Hot Start Strategy)

#### 1.1.1 问题分析

传统截图工具的冷启动延迟来源：
- WebView 初始化 (~200-500ms)
- JavaScript 解析执行 (~100-300ms)
- DOM 渲染 (~50-100ms)
- 网络资源加载 (dev mode)

#### 1.1.2 解决方案：预热隐藏窗口 (Hidden Warm Window)

```
┌─────────────────────────────────────────────────────────────┐
│                    应用启动时序图                            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  App Start ──► Create Hidden Window ──► Load WebView        │
│                    │                         │              │
│                    ▼                         ▼              │
│              Position: (15000, 15000)   JS Executes         │
│              Size: 800x600              ──► Frontend Ready  │
│                    │                         │              │
│                    ▼                         ▼              │
│              Hide Window ◄──────────── Signal Backend       │
│                    │                                        │
│                    ▼                                        │
│              READY FOR INSTANT CAPTURE                      │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**关键实现点**：

1. **窗口预创建** (`tauri.conf.json`):
```json
{
  "label": "capture",
  "url": "/capture",
  "visible": true,           // 必须可见才能触发 WebView 加载
  "x": 15000, "y": 15000,    // 屏幕外位置，用户不可见
  "width": 800, "height": 600,
  "transparent": true,
  "decorations": false
}
```

2. **前端就绪信号机制**:
```rust
// Rust 端：等待前端就绪
static CAPTURE_FRONTEND_READY: AtomicBool = AtomicBool::new(false);

#[tauri::command]
pub async fn capture_frontend_ready(app: tauri::AppHandle) -> AppResult<()> {
    CAPTURE_FRONTEND_READY.store(true, Ordering::Release);
    // 尝试投递待处理帧
    try_deliver_pending_frame(&app);
    Ok(())
}
```

```typescript
// 前端：组件挂载时立即通知
onMount(async () => {
  await invoke('capture_frontend_ready')
})
```

3. **热键响应流程优化**:
```
Hotkey Press ──► Check Frontend Ready? ─── Yes ──► Hide Window
                         │                              │
                         No                             ▼
                         │                      Capture Screen (80ms)
                         ▼                              │
                    Wait with Timeout                   ▼
                         │                      Emit capture:ready
                         ▼                              │
                    Continue if ready                   ▼
                                               Show Fullscreen Window
                                                       │
                                                       ▼
                                               < 150ms Total Latency
```

#### 1.1.3 全局热键实现

使用 `tauri-plugin-global-shortcut`：

```rust
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};

fn register_global_shortcuts(app: &tauri::App) -> Result<()> {
    let capture_shortcut = Shortcut::new(
        Some(Modifiers::CONTROL | Modifiers::ALT),
        Code::KeyX
    );
    
    app.global_shortcut().on_shortcut(capture_shortcut, |app, _| {
        // 防抖：避免重复触发
        if !debounce_check(350.ms) { return; }
        
        tauri::async_runtime::spawn(async move {
            capture::init_capture(app).await;
        });
    })?;
    
    Ok(())
}
```

### 1.2 数据传输优化

#### 1.2.1 避免 Base64 开销的策略

**问题**：Base64 编码使数据膨胀 33%，且编解码消耗 CPU。

**解决方案**：文件缓存 + 路径传递

```rust
// Rust 端：写入缓存文件
let cache_dir = app.path().cache_dir()?.join("omnibox/capture");
let file_path = cache_dir.join(format!("capture_{}.png", frame_id));
std::fs::write(&file_path, &png_bytes)?;

// 发送文件路径而非 Base64
let payload = serde_json::json!({
    "path": file_path.to_str(),
    "width": width,
    "height": height,
});
app.emit_to("capture", "capture:ready", payload)?;
```

```typescript
// 前端：使用 convertFileSrc 加载本地文件
import { convertFileSrc } from '@tauri-apps/api/core'

listen<CaptureData>('capture:ready', (event) => {
  const img = new Image()
  if (event.payload.path) {
    // 零拷贝加载本地文件
    img.src = convertFileSrc(event.payload.path)
  } else {
    // 降级方案
    img.src = `data:image/png;base64,${event.payload.data}`
  }
})
```

#### 1.2.2 IPC 性能对比

| 方法 | 1080p 图像传输时间 | 内存峰值 |
|------|-------------------|----------|
| Base64 via JSON | ~120ms | 2x 图像大小 |
| File Path + convertFileSrc | ~15ms | 1x 图像大小 |
| SharedArrayBuffer (future) | ~5ms | 1x 图像大小 |

---

## 2. 高清截图模块

### 2.1 DPI/PPI 适配

#### 2.1.1 像素坐标体系

```
┌─────────────────────────────────────────────────────────────┐
│                     坐标系统说明                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Physical Pixels (物理像素)                                  │
│  └─ 屏幕实际硬件像素                                         │
│  └─ xcap 捕获的原生分辨率                                    │
│  └─ 例: 3840 x 2160 (4K 屏幕)                               │
│                                                             │
│  Logical Pixels (逻辑像素/CSS 像素)                          │
│  └─ 操作系统 DPI 缩放后的坐标                                │
│  └─ CSS/DOM 使用的单位                                       │
│  └─ 例: 1920 x 1080 (150% 缩放下)                           │
│                                                             │
│  Device Pixel Ratio (DPR)                                   │
│  └─ window.devicePixelRatio                                 │
│  └─ Physical / Logical = DPR                                │
│  └─ 例: 3840 / 1920 = 2.0                                   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

#### 2.1.2 Rust 端物理像素捕获

```rust
use xcap::Monitor;

fn capture_screen() -> Result<(Vec<u8>, u32, u32, i32, i32, u32, u32)> {
    let monitors = Monitor::all()?;
    let monitor = monitors.into_iter()
        .find(|m| m.is_primary().unwrap_or(false))
        .or_else(|| Monitor::all().ok()?.into_iter().next())
        .ok_or("No monitor found")?;

    // 获取物理坐标和尺寸
    let mon_x = monitor.x()?;           // 物理 X 坐标
    let mon_y = monitor.y()?;           // 物理 Y 坐标
    let mon_w = monitor.width()?;       // 物理宽度
    let mon_h = monitor.height()?;      // 物理高度

    // capture_image() 返回原生物理分辨率
    let img = monitor.capture_image()?;
    let (width, height) = (img.width(), img.height());
    
    // Fast PNG encoding
    let mut out = Vec::new();
    let encoder = PngEncoder::new_with_quality(
        &mut out,
        CompressionType::Fast,    // 速度优先
        FilterType::NoFilter      // 无滤波，最快
    );
    encoder.write_image(&img.into_raw(), width, height, ColorType::Rgba8)?;
    
    Ok((out, width, height, mon_x, mon_y, mon_w, mon_h))
}
```

#### 2.1.3 前端 Canvas HiDPI 渲染

**关键：Canvas 的 width/height 属性是绘图缓冲区大小，style.width/height 是显示大小**

```typescript
const drawCanvas = (image: HTMLImageElement) => {
  const canvas = canvasRef
  const ctx = canvas.getContext('2d')
  const dpr = window.devicePixelRatio || 1

  // 设置 Canvas 缓冲区为物理像素大小
  canvas.width = Math.round(window.innerWidth * dpr)
  canvas.height = Math.round(window.innerHeight * dpr)
  
  // 显示尺寸为逻辑像素
  canvas.style.width = `${window.innerWidth}px`
  canvas.style.height = `${window.innerHeight}px`

  // 缩放绘图上下文以使用逻辑像素坐标
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0)

  // 现在可以用逻辑像素坐标绑定
  ctx.drawImage(image, 0, 0, window.innerWidth, window.innerHeight)
}
```

#### 2.1.4 选区坐标转换

```typescript
// 前端选区（逻辑像素）→ 后端裁剪（物理像素）
const getSelectionInPhysicalPixels = (sel: Selection, image: HTMLImageElement) => {
  const scaleX = image.width / window.innerWidth   // 物理/逻辑
  const scaleY = image.height / window.innerHeight
  
  return {
    srcX: Math.round(sel.x * scaleX),
    srcY: Math.round(sel.y * scaleY),
    srcW: Math.round(sel.w * scaleX),
    srcH: Math.round(sel.h * scaleY),
  }
}
```

### 2.2 多屏支持

#### 2.2.1 鼠标所在屏幕检测

```rust
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, MonitorFromPoint, MONITOR_DEFAULTTONEAREST};
use windows::Win32::Graphics::Gdi::GetMonitorInfoW;

fn get_monitor_at_cursor() -> Result<Monitor> {
    unsafe {
        let mut cursor_pos = POINT::default();
        GetCursorPos(&mut cursor_pos)?;
        
        let hmonitor = MonitorFromPoint(cursor_pos, MONITOR_DEFAULTTONEAREST);
        
        // 获取监视器信息
        let mut info = MONITORINFOEXW::default();
        info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
        GetMonitorInfoW(hmonitor, &mut info.monitorInfo as *mut _)?;
        
        // 转换为 xcap Monitor
        let monitors = Monitor::all()?;
        monitors.into_iter()
            .find(|m| {
                m.x() == info.monitorInfo.rcMonitor.left &&
                m.y() == info.monitorInfo.rcMonitor.top
            })
            .ok_or("Monitor not found")
    }
}
```

#### 2.2.2 虚拟桌面坐标系

```
┌─────────────────────────────────────────────────────────────┐
│                   多屏坐标系统                               │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   ┌──────────────┐  ┌──────────────┐                       │
│   │   Monitor 1  │  │   Monitor 2  │                       │
│   │   (Primary)  │  │  (Secondary) │                       │
│   │  (0,0)       │  │ (1920,0)     │                       │
│   │  1920x1080   │  │  1920x1080   │                       │
│   └──────────────┘  └──────────────┘                       │
│                                                             │
│   虚拟桌面范围: (0,0) - (3840, 1080)                        │
│                                                             │
│   注意: 副屏可能在负坐标区域                                 │
│   例: Monitor 2 at (-1920, 0)                               │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. 标注工具链

### 3.1 技术选型：Konva.js

**选择理由**：
- 声明式 API，与 SolidJS 理念契合
- 内置 Transformer（缩放/旋转）
- 高效的 Canvas 批量渲染
- 丰富的内置形状支持
- 成熟的事件系统

**替代方案对比**：

| 库 | 优点 | 缺点 |
|----|------|------|
| Konva.js | 功能完整，文档好 | 包体积较大 (~300KB) |
| Fabric.js | 功能丰富 | 更重，面向编辑器场景 |
| 原生 Canvas | 零依赖，极致性能 | 需要自己实现所有交互 |

### 3.2 功能清单与实现

#### 3.2.1 矩形/椭圆

```typescript
// DrawManager.ts
private createRect(start: Point, end: Point): Konva.Rect {
  const x = Math.min(start.x, end.x)
  const y = Math.min(start.y, end.y)
  const width = Math.abs(end.x - start.x)
  const height = Math.abs(end.y - start.y)
  
  // Shift 键：约束为正方形
  const constrainedSize = this.isShiftDown 
    ? Math.max(width, height) 
    : null
  
  return new Konva.Rect({
    x, y,
    width: constrainedSize ?? width,
    height: constrainedSize ?? height,
    stroke: this.style.stroke,
    strokeWidth: this.style.strokeWidth,
    fill: this.style.fillEnabled ? this.style.fill : 'transparent',
    opacity: this.style.opacity,
    draggable: true,
  })
}
```

#### 3.2.2 箭头

```typescript
private createArrow(points: number[]): Konva.Arrow {
  return new Konva.Arrow({
    points,
    stroke: this.style.stroke,
    strokeWidth: this.style.strokeWidth,
    fill: this.style.arrowHeadStyle === 'filled' 
      ? this.style.stroke 
      : 'transparent',
    pointerLength: this.style.arrowPointerLength,
    pointerWidth: this.style.arrowPointerWidth,
    pointerAtBeginning: this.style.arrowMode === 'start' || this.style.arrowMode === 'both',
    pointerAtEnding: this.style.arrowMode === 'end' || this.style.arrowMode === 'both',
    lineCap: 'round',
    lineJoin: 'round',
  })
}
```

#### 3.2.3 画笔 (Freehand)

```typescript
private handlePencilMove(pointer: Point) {
  this.activePoints.push(pointer.x, pointer.y)
  
  if (this.activeNode instanceof Konva.Line) {
    this.activeNode.points(this.activePoints)
    
    // 使用 tension 创建平滑曲线
    this.activeNode.tension(0.5)
    this.activeNode.bezier(true)
    
    this.drawLayer.batchDraw()  // 批量绘制，避免频繁重绘
  }
}
```

#### 3.2.4 马赛克 (高斯模糊近似)

**实现思路**：像素化区域实现马赛克效果

```typescript
// MosaicTool.ts
export function createMosaicNode(
  bgImage: HTMLImageElement,
  rect: { x: number; y: number; width: number; height: number },
  pixelSize: number = 12
): Konva.Image {
  // 创建离屏 Canvas
  const offscreen = document.createElement('canvas')
  offscreen.width = rect.width
  offscreen.height = rect.height
  const ctx = offscreen.getContext('2d')!
  
  // 从背景图截取区域
  ctx.drawImage(
    bgImage,
    rect.x, rect.y, rect.width, rect.height,
    0, 0, rect.width, rect.height
  )
  
  // 像素化处理
  const imageData = ctx.getImageData(0, 0, rect.width, rect.height)
  const data = imageData.data
  
  for (let y = 0; y < rect.height; y += pixelSize) {
    for (let x = 0; x < rect.width; x += pixelSize) {
      // 计算区块平均颜色
      let r = 0, g = 0, b = 0, count = 0
      
      for (let dy = 0; dy < pixelSize && y + dy < rect.height; dy++) {
        for (let dx = 0; dx < pixelSize && x + dx < rect.width; dx++) {
          const idx = ((y + dy) * rect.width + (x + dx)) * 4
          r += data[idx]
          g += data[idx + 1]
          b += data[idx + 2]
          count++
        }
      }
      
      r = Math.round(r / count)
      g = Math.round(g / count)
      b = Math.round(b / count)
      
      // 填充区块
      for (let dy = 0; dy < pixelSize && y + dy < rect.height; dy++) {
        for (let dx = 0; dx < pixelSize && x + dx < rect.width; dx++) {
          const idx = ((y + dy) * rect.width + (x + dx)) * 4
          data[idx] = r
          data[idx + 1] = g
          data[idx + 2] = b
        }
      }
    }
  }
  
  ctx.putImageData(imageData, 0, 0)
  
  // 返回 Konva Image 节点
  const img = new Image()
  img.src = offscreen.toDataURL()
  
  return new Konva.Image({
    x: rect.x,
    y: rect.y,
    image: img,
    width: rect.width,
    height: rect.height,
  })
}
```

#### 3.2.5 文本输入

```typescript
private createTextNode(position: Point): Konva.Group {
  const group = new Konva.Group({
    x: position.x,
    y: position.y,
    name: 'textGroup',
    draggable: true,
  })
  
  // 背景矩形
  const bg = new Konva.Rect({
    name: 'textBg',
    visible: this.style.textBgEnabled,
    fill: this.style.textBgColor,
    opacity: this.style.textBgOpacity,
    cornerRadius: this.style.textBgRadius,
  })
  
  // 文本节点
  const text = new Konva.Text({
    name: 'textNode',
    text: '点击编辑',
    fill: this.style.stroke,
    fontSize: this.style.fontSize,
    fontFamily: this.style.fontFamily,
    padding: this.style.textPadding,
  })
  
  group.add(bg)
  group.add(text)
  
  // 双击编辑
  group.on('dblclick', () => this.startTextEditing(group))
  
  return group
}

private startTextEditing(group: Konva.Group) {
  const textNode = group.findOne('.textNode') as Konva.Text
  const rect = textNode.getClientRect()
  
  // 创建原生输入框覆盖
  const textarea = document.createElement('textarea')
  textarea.value = textNode.text()
  textarea.style.cssText = `
    position: absolute;
    left: ${rect.x}px;
    top: ${rect.y}px;
    width: ${rect.width}px;
    min-height: ${rect.height}px;
    font-size: ${textNode.fontSize()}px;
    font-family: ${textNode.fontFamily()};
    color: ${textNode.fill()};
    background: transparent;
    border: 1px dashed #4f9cff;
    outline: none;
    resize: none;
  `
  
  document.body.appendChild(textarea)
  textarea.focus()
  
  textarea.onblur = () => {
    textNode.text(textarea.value)
    this.syncTextGroupLayout(group)
    textarea.remove()
    this.commitHistory()
  }
}
```

### 3.3 撤销/重做栈

```typescript
class DrawManager {
  private undoStack: string[] = []
  private redoStack: string[] = []
  
  // 提交历史快照
  private commitHistory(isInitial = false) {
    // 序列化 drawLayer 的所有节点（排除 Transformer）
    const snapshot = this.drawLayer.toJSON()
    
    if (!isInitial) {
      this.redoStack = []  // 新操作清空重做栈
    }
    
    this.undoStack.push(snapshot)
    
    // 限制历史深度
    if (this.undoStack.length > 50) {
      this.undoStack.shift()
    }
  }
  
  undo() {
    if (this.undoStack.length <= 1) return
    
    const current = this.undoStack.pop()!
    this.redoStack.push(current)
    
    const prev = this.undoStack[this.undoStack.length - 1]
    this.restoreFromSnapshot(prev)
  }
  
  redo() {
    if (this.redoStack.length === 0) return
    
    const next = this.redoStack.pop()!
    this.undoStack.push(next)
    this.restoreFromSnapshot(next)
  }
  
  private restoreFromSnapshot(json: string) {
    // 清除现有节点
    this.drawLayer.destroyChildren()
    
    // 从 JSON 恢复
    const data = JSON.parse(json)
    data.children?.forEach((nodeData: any) => {
      const node = Konva.Node.create(nodeData)
      this.drawLayer.add(node)
    })
    
    // 重新添加 Transformer
    this.drawLayer.add(this.transformer)
    this.drawLayer.batchDraw()
  }
  
  canUndo = () => this.undoStack.length > 1
  canRedo = () => this.redoStack.length > 0
}
```

---

## 4. PIN 贴图系统

### 4.1 多窗口管理

#### 4.1.1 动态窗口创建

```rust
#[tauri::command]
pub async fn create_pin_window(
    app: tauri::AppHandle,
    image_data: String,  // Base64 PNG
    width: u32,
    height: u32,
    x: i32,
    y: i32,
) -> AppResult<()> {
    use tauri::WebviewWindowBuilder;
    use std::sync::atomic::{AtomicU32, Ordering};
    
    // 唯一窗口 ID
    static PIN_COUNTER: AtomicU32 = AtomicU32::new(0);
    let pin_id = PIN_COUNTER.fetch_add(1, Ordering::Relaxed);
    let window_label = format!("pin_{}", pin_id);
    
    // 坐标转换：捕获窗口坐标 → 屏幕坐标
    let (pos_x, pos_y) = if let Some(capture_win) = app.get_webview_window("capture") {
        let outer_pos = capture_win.outer_position()?;
        let scale = capture_win.scale_factor().unwrap_or(1.0);
        (
            (outer_pos.x as f64 / scale) + x as f64,
            (outer_pos.y as f64 / scale) + y as f64,
        )
    } else {
        (x as f64, y as f64)
    };

    // 存储 Payload（避免 URL 过长）
    PIN_PAYLOADS.lock().insert(window_label.clone(), PinPayload {
        data: image_data,
        width,
        height,
    });
    
    // 创建窗口
    WebviewWindowBuilder::new(&app, &window_label, tauri::WebviewUrl::App("/pin".into()))
        .title("Pin")
        .inner_size(width as f64, height as f64)
        .position(pos_x, pos_y)
        .decorations(false)           // 无边框
        .transparent(true)            // 透明背景
        .always_on_top(true)          // 置顶
        .skip_taskbar(true)           // 不在任务栏显示
        .resizable(false)
        .visible(true)
        .build()?;
    
    Ok(())
}
```

#### 4.1.2 窗口属性配置

| 属性 | 值 | 说明 |
|------|-----|------|
| `decorations` | false | 无标题栏/边框 |
| `transparent` | true | 透明背景 |
| `always_on_top` | true | 窗口置顶 |
| `skip_taskbar` | true | 不显示在任务栏 |
| `resizable` | false | 固定大小（可通过滚轮缩放） |

### 4.2 交互功能

#### 4.2.1 缩放 (滚轮)

```typescript
// Pin/index.tsx
const handleWheel = async (e: WheelEvent) => {
  e.preventDefault()
  
  const delta = e.deltaY > 0 ? 0.9 : 1.1  // 缩小/放大
  const newScale = Math.max(0.1, Math.min(5, currentScale() * delta))
  
  setCurrentScale(newScale)
  
  // 调整窗口大小
  const newWidth = Math.round(originalWidth * newScale)
  const newHeight = Math.round(originalHeight * newScale)
  
  await currentWindow.setSize(new LogicalSize(newWidth, newHeight))
}
```

#### 4.2.2 透明度调节 (Ctrl+滚轮)

```typescript
const handleWheel = async (e: WheelEvent) => {
  e.preventDefault()
  
  if (e.ctrlKey) {
    // 调节透明度
    const delta = e.deltaY > 0 ? -0.1 : 0.1
    const newOpacity = Math.max(0.1, Math.min(1, currentOpacity() + delta))
    setCurrentOpacity(newOpacity)
    
    // 应用到图像
    imageRef.style.opacity = newOpacity.toString()
    return
  }
  
  // 缩放逻辑...
}
```

#### 4.2.3 鼠标穿透 (Click-through)

```rust
// 前端请求穿透
#[tauri::command]
pub async fn set_pin_click_through(
    app: tauri::AppHandle,
    label: String,
    enabled: bool,
) -> AppResult<()> {
    if let Some(win) = app.get_webview_window(&label) {
        win.set_ignore_cursor_events(enabled)?;
    }
    Ok(())
}
```

```typescript
// 前端快捷键触发
onMount(() => {
  window.addEventListener('keydown', (e) => {
    if (e.key === 't' && e.ctrlKey) {
      toggleClickThrough()
    }
  })
})

const toggleClickThrough = async () => {
  const newState = !isClickThrough()
  setIsClickThrough(newState)
  await invoke('set_pin_click_through', { 
    label: currentWindow.label, 
    enabled: newState 
  })
}
```

### 4.3 内存优化

#### 4.3.1 Payload 生命周期管理

```rust
// 存储待取用的 Payload
static PIN_PAYLOADS: Lazy<Mutex<HashMap<String, PinPayload>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));

// Pin 窗口挂载时拉取并清除
#[tauri::command]
pub async fn get_pin_payload(label: String) -> AppResult<Option<PinPayload>> {
    Ok(PIN_PAYLOADS.lock().remove(&label))  // 取出即删除
}
```

#### 4.3.2 窗口关闭时清理

```rust
// 监听窗口关闭事件
app.on_window_event(|window, event| {
    if let tauri::WindowEvent::Destroyed = event {
        if window.label().starts_with("pin_") {
            // 清理残留 Payload（以防万一）
            PIN_PAYLOADS.lock().remove(window.label());
        }
    }
});
```

#### 4.3.3 大量 Pin 窗口优化建议

1. **限制最大 Pin 数量**：建议 10-20 个
2. **LRU 淘汰策略**：超出限制时关闭最旧的 Pin
3. **压缩图像**：存储时使用 JPEG 替代 PNG（可选，有损）
4. **延迟加载**：Pin 窗口可见时才加载图像

---

## 5. OCR 文字识别

### 5.1 方案选择：Windows.Media.Ocr

**优势**：
- **离线识别**：无需网络
- **秒级响应**：通常 < 200ms
- **零额外依赖**：系统内置
- **多语言支持**：英文、中文、日文等
- **无包体积增加**：使用 `windows` crate 静态绑定

**对比 Tesseract**：

| 特性 | Windows.Media.Ocr | Tesseract.js |
|------|------------------|--------------|
| 准确率 | 优秀 | 良好 |
| 速度 | < 200ms | > 1s |
| 包体积 | 0MB | ~15MB |
| 离线支持 | ✓ | ✓ |
| 平台 | Windows 10+ | 跨平台 |

### 5.2 实现代码

```rust
// ocr.rs
use windows::Graphics::Imaging::{BitmapDecoder, BitmapPixelFormat, BitmapAlphaMode, SoftwareBitmap};
use windows::Media::Ocr::OcrEngine;
use windows::Storage::Streams::{DataWriter, InMemoryRandomAccessStream};
use windows::Globalization::Language;

#[tauri::command]
pub async fn recognize_text(base64_image: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        recognize_text_sync(base64_image)
    })
    .await
    .map_err(|e| format!("OCR task join failed: {e}"))?
}

fn recognize_text_sync(base64_image: String) -> Result<String, String> {
    // 1. COM 初始化
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
    }
    
    // 2. Base64 解码
    let b64 = base64_image.split(',').last().unwrap_or(&base64_image).trim();
    let bytes = BASE64.decode(b64).map_err(|e| format!("Base64 decode failed: {e}"))?;
    
    // 3. 创建内存流
    let mem = InMemoryRandomAccessStream::new()?;
    let writer = DataWriter::CreateDataWriter(&mem)?;
    writer.WriteBytes(&bytes)?;
    writer.StoreAsync()?.get()?;
    writer.FlushAsync()?.get()?;
    mem.Seek(0)?;
    
    // 4. 解码位图
    let decoder = BitmapDecoder::CreateAsync(&mem)?.get()?;
    let mut bitmap = decoder.GetSoftwareBitmapAsync()?.get()?;
    
    // 5. 转换为 BGRA8 格式（OCR 要求）
    if bitmap.BitmapPixelFormat()? != BitmapPixelFormat::Bgra8 {
        bitmap = SoftwareBitmap::ConvertWithAlpha(
            &bitmap,
            BitmapPixelFormat::Bgra8,
            BitmapAlphaMode::Premultiplied
        )?;
    }
    
    // 6. 创建 OCR 引擎（自动检测语言）
    let engine = OcrEngine::TryCreateFromUserProfileLanguages()?;
    
    // 7. 识别
    let result = engine.RecognizeAsync(&bitmap)?.get()?;
    let lines = result.Lines()?;
    
    let mut output = String::new();
    for i in 0..lines.Size()? {
        let line = lines.GetAt(i)?;
        if !output.is_empty() { output.push('\n'); }
        output.push_str(&line.Text()?.to_string());
    }
    
    Ok(output.trim().to_string())
}
```

### 5.3 多语言 Fallback

```rust
fn recognize_with_language_fallback(bitmap: &SoftwareBitmap) -> Result<String, String> {
    // 优先使用用户配置语言
    let engine = OcrEngine::TryCreateFromUserProfileLanguages()?;
    let result = try_recognize(&engine, bitmap)?;
    if !result.is_empty() { return Ok(result); }
    
    // Fallback：尝试常用语言
    let fallback_langs = ["en-US", "zh-Hans", "zh-Hant", "ja", "ko"];
    
    for lang_tag in fallback_langs {
        if let Ok(available) = OcrEngine::AvailableRecognizerLanguages() {
            for i in 0..available.Size().unwrap_or(0) {
                if let Ok(lang) = available.GetAt(i) {
                    if lang.LanguageTag().map(|t| t.to_string()).ok() == Some(lang_tag.into()) {
                        let engine = OcrEngine::TryCreateFromLanguage(&lang)?;
                        let result = try_recognize(&engine, bitmap)?;
                        if !result.is_empty() { return Ok(result); }
                    }
                }
            }
        }
    }
    
    Ok(String::new())
}
```

---

## 6. UI 元素自动检测

### 6.1 实现思路

利用 Windows UI Automation API 获取鼠标下方 UI 元素的边界矩形。

```
┌─────────────────────────────────────────────────────────────┐
│                    UI 元素检测流程                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Mouse Position ──► UI Automation API ──► Element Rect     │
│       │                    │                    │           │
│       ▼                    ▼                    ▼           │
│  (1200, 800)      IUIAutomation::       {left: 1100,       │
│                   ElementFromPoint      top: 750,          │
│                                         right: 1400,       │
│                                         bottom: 850}       │
│                            │                    │           │
│                            ▼                    ▼           │
│                    Screen Coords        Frontend Highlight  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 6.2 Rust 实现

```rust
// automation.rs
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED};
use windows::Win32::UI::Accessibility::{CUIAutomation, IUIAutomation};
use windows::Win32::Foundation::{POINT, RECT};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[tauri::command]
pub async fn get_element_rect_at(x: i32, y: i32) -> AppResult<Option<Rect>> {
    tauri::async_runtime::spawn_blocking(move || {
        get_element_rect_blocking(x, y)
    }).await?
}

fn get_element_rect_blocking(x: i32, y: i32) -> AppResult<Option<Rect>> {
    unsafe {
        // 初始化 COM
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
        
        struct ComGuard;
        impl Drop for ComGuard {
            fn drop(&mut self) {
                unsafe { CoUninitialize() };
            }
        }
        let _guard = ComGuard;
        
        // 创建 UIAutomation 实例
        let automation: IUIAutomation = CoCreateInstance(
            &CUIAutomation,
            None,
            CLSCTX_INPROC_SERVER
        )?;
        
        // 获取指定坐标的元素
        let point = POINT { x, y };
        let element = automation.ElementFromPoint(point)?;
        
        // 获取边界矩形
        let rect: RECT = element.CurrentBoundingRectangle()?;
        
        let result = Rect {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        };
        
        if result.width() <= 0 || result.height() <= 0 {
            Ok(None)
        } else {
            Ok(Some(result))
        }
    }
}
```

### 6.3 前端高亮渲染

```typescript
// Capture/index.tsx
const [elementRect, setElementRect] = createSignal<Rect | null>(null)

// 节流请求 UI 元素
const requestElementRect = throttle(async (x: number, y: number) => {
  // 转换为屏幕坐标
  const screenX = x + windowOffset.x
  const screenY = y + windowOffset.y
  
  const rect = await invoke<Rect | null>('get_element_rect_at', { 
    x: screenX, 
    y: screenY 
  })
  
  if (rect) {
    // 转换回窗口坐标
    setElementRect({
      left: rect.left - windowOffset.x,
      top: rect.top - windowOffset.y,
      right: rect.right - windowOffset.x,
      bottom: rect.bottom - windowOffset.y,
    })
  }
}, 50)  // 50ms 节流

// 渲染高亮框
const ElementHighlight = () => {
  const rect = elementRect()
  if (!rect || status() !== 'selecting') return null
  
  return (
    <div
      class="pointer-events-none absolute border-2 border-red-500"
      style={{
        left: `${rect.left}px`,
        top: `${rect.top}px`,
        width: `${rect.right - rect.left}px`,
        height: `${rect.bottom - rect.top}px`,
      }}
    />
  )
}
```

### 6.4 性能优化

1. **节流调用**：限制 API 调用频率（建议 50-100ms）
2. **缓存结果**：相同坐标范围内无需重复请求
3. **异步执行**：`spawn_blocking` 避免阻塞 Tauri 事件循环
4. **预过滤**：可以在前端判断鼠标是否移动到新区域再请求

```typescript
// 智能请求：只在鼠标移出当前元素区域时请求
const shouldRequest = (x: number, y: number) => {
  const current = elementRect()
  if (!current) return true
  
  return x < current.left || x > current.right ||
         y < current.top || y > current.bottom
}
```

---

## 附录 A：性能指标目标

| 指标 | 目标值 | 当前实现 |
|------|--------|----------|
| 热键响应 → 窗口显示 | < 150ms | ~100ms |
| 截图捕获时间 (1080p) | < 100ms | ~80ms |
| PNG 编码时间 | < 50ms | ~30ms |
| 选区确认 → Pin 创建 | < 200ms | ~150ms |
| OCR 识别时间 | < 500ms | ~200ms |
| UI 元素检测 | < 50ms | ~30ms |

## 附录 B：依赖清单

```toml
# Cargo.toml
[dependencies]
# 截图捕获
xcap = "0.4"
image = { version = "0.24", features = ["png"] }

# Windows API
windows = { version = "0.58", features = [
    "Win32_UI_Accessibility",
    "Media_Ocr",
    "Graphics_Imaging",
    "Storage_Streams",
]}

# 剪贴板
arboard = "3"

# 异步
tokio = { version = "1", features = ["full"] }
```

```json
// package.json
{
  "dependencies": {
    "konva": "^9.0.0",
    "@tauri-apps/api": "^2.0.0"
  }
}
```

use crate::app::error::{AppError, AppResult};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use tauri::{Emitter, Manager};
use tokio::sync::Notify;
use tokio::sync::Mutex as TokioMutex;

#[derive(Clone, Serialize)]
pub struct PinPayload {
    data: String,
    width: u32,
    height: u32,
}

static PIN_PAYLOADS: Lazy<Mutex<HashMap<String, PinPayload>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone)]
struct CapturePng {
    id: u64,
    png_bytes: Vec<u8>,
    width: u32,
    height: u32,
    file_path: Option<std::path::PathBuf>,
}

static LAST_CAPTURE_PNG: Lazy<Mutex<Option<CapturePng>>> = Lazy::new(|| Mutex::new(None));

// Capture window readiness + delivery coordination.
// Problem: if the user triggers capture before the capture webview finished loading,
// showing a fullscreen transparent always-on-top window can block all global clicks.
// Solution:
// - Keep the capture window click-through until the capture frontend signals it is ready.
// - Only emit `capture:ready` to the frontend once it can receive it, or re-emit after ready.
static CAPTURE_WEBVIEW_READY: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
static CAPTURE_WEBVIEW_NOTIFY: Lazy<Notify> = Lazy::new(Notify::new);
static CAPTURE_FRAME_ID: AtomicU64 = AtomicU64::new(0);
static CAPTURE_DELIVERED_FRAME_ID: AtomicU64 = AtomicU64::new(0);
static CAPTURE_PENDING_SHOW: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

// Serialize capture init to avoid races between repeated hotkey presses.
static CAPTURE_INIT_MUTEX: Lazy<TokioMutex<()>> = Lazy::new(|| TokioMutex::new(()));

async fn wait_capture_webview_ready(timeout_ms: u64) -> bool {
    if CAPTURE_WEBVIEW_READY.load(Ordering::Acquire) {
        return true;
    }

    let _ = tokio::time::timeout(
        std::time::Duration::from_millis(timeout_ms),
        CAPTURE_WEBVIEW_NOTIFY.notified(),
    )
    .await;

    CAPTURE_WEBVIEW_READY.load(Ordering::Acquire)
}

fn show_capture_preload_window(win: &tauri::WebviewWindow) {
    // Goal: let the webview load and run JS (so it can call `capture_frontend_ready`)
    // without ever blocking user interaction with the desktop.
    // We keep it click-through, tiny, and offscreen.
    let _ = win.set_ignore_cursor_events(true);
    let _ = win.set_decorations(false);
    let _ = win.set_fullscreen(false);
    let _ = win.set_always_on_top(true);
    let _ = win.set_size(tauri::LogicalSize::new(1.0, 1.0));
    let _ = win.set_position(tauri::LogicalPosition::new(-10_000.0, -10_000.0));
    let _ = win.show();
    // Windows sometimes resets click-through on show.
    let _ = win.set_ignore_cursor_events(true);
}

fn try_deliver_last_frame(app: &tauri::AppHandle) -> bool {
    let last = LAST_CAPTURE_PNG.lock();
    let Some(last) = last.as_ref() else {
        return false;
    };

    let delivered = CAPTURE_DELIVERED_FRAME_ID.load(Ordering::Acquire);
    if delivered >= last.id {
        return false;
    }

    // Prefer file path (fast), fallback to base64.
    let payload = if let Some(p) = last.file_path.as_ref().and_then(|p| p.to_str()) {
        serde_json::json!({
            "path": p,
            "width": last.width,
            "height": last.height,
        })
    } else {
        serde_json::json!({
            "data": BASE64.encode(&last.png_bytes),
            "width": last.width,
            "height": last.height,
        })
    };

    if app.emit_to("capture", "capture:ready", payload).is_ok() {
        CAPTURE_DELIVERED_FRAME_ID.store(last.id, Ordering::Release);
        return true;
    }

    false
}

/// Called by the capture frontend on mount.
/// Marks the capture webview as ready, makes the capture window interactive,
/// and (re)delivers the latest capture frame if one is pending.
#[tauri::command]
pub async fn capture_frontend_ready(app: tauri::AppHandle) -> AppResult<()> {
    CAPTURE_WEBVIEW_READY.store(true, Ordering::Release);
    CAPTURE_WEBVIEW_NOTIFY.notify_waiters();

    // If init_capture happened before the frontend was ready, re-deliver now.
    let delivered = try_deliver_last_frame(&app);

    if let Some(win) = app.get_webview_window("capture") {
        // Ensure the capture window is always configured correctly.
        // Windows may restore default chrome after hide/show.
        let _ = win.set_decorations(false);
        let _ = win.set_always_on_top(true);

        // Only show / enable input when there is a pending capture request.
        // This prevents a transparent fullscreen window from ever blocking clicks without UI.
        let should_show = CAPTURE_PENDING_SHOW.swap(false, Ordering::AcqRel) || delivered;
        if should_show {
            let _ = win.set_fullscreen(true);
            let _ = win.show();
            let _ = win.set_decorations(false);
            let _ = win.set_fullscreen(true);
            let _ = win.set_ignore_cursor_events(false);
            let _ = win.set_focus();
        } else {
            // No active capture: make sure we never block global clicks.
            let _ = win.set_ignore_cursor_events(true);
        }
    }

    Ok(())
}

/// Start screen capture - the CORRECT sequence to avoid black screen:
/// 1. Ensure capture window is HIDDEN
/// 2. Capture the screen (while window is hidden)
/// 3. Encode image to base64
/// 4. Emit data to frontend via event
/// 5. THEN show the window (frontend will render the captured image)
#[tauri::command]
pub async fn init_capture(app: tauri::AppHandle) -> AppResult<()> {
    // Prevent overlapping captures from leaving the window in a broken state.
    let _guard = CAPTURE_INIT_MUTEX.lock().await;
    CAPTURE_PENDING_SHOW.store(false, Ordering::Release);

    // Step 1: Ensure capture window is hidden FIRST
    if let Some(win) = app.get_webview_window("capture") {
        // Safety: make the fullscreen transparent window click-through until
        // the capture frontend is ready, otherwise it may block all global clicks.
        let _ = win.set_ignore_cursor_events(true);
        // Exit fullscreen before hiding to ensure clean state
        let _ = win.set_fullscreen(false);
        let _ = win.hide();
    }
    
    // Minimal delay to ensure window is fully hidden before capture
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // Ask frontend to reset its selection/overlay state for a fresh capture.
    // This prevents cases where an old full-screen selection stays in editing mode and eats clicks.
    let _ = app.emit_to("capture", "capture:reset", serde_json::json!({}));
    
    tracing::info!("Starting screen capture...");

    // Step 2+3: Capture + encode on a blocking thread.
    // This keeps the tauri async command future `Send` (xcap monitor handles are not Send).
    let (png_bytes, width, height) = tauri::async_runtime::spawn_blocking(move || -> AppResult<(Vec<u8>, u32, u32)> {
        use image::codecs::png::{CompressionType, FilterType, PngEncoder};
        use image::{ColorType, ImageEncoder};

        let monitors = xcap::Monitor::all()
            .map_err(|e| AppError::Unknown(format!("Failed to list monitors: {e}")))?;

        // Get primary monitor (or first available)
        let monitor = monitors
            .into_iter()
            .find(|m| m.is_primary().unwrap_or(false))
            .or_else(|| xcap::Monitor::all().ok()?.into_iter().next())
            .ok_or_else(|| AppError::NotFound("No monitor found".into()))?;

        let img = monitor
            .capture_image()
            .map_err(|e| AppError::Unknown(format!("Failed to capture screen: {e}")))?;

        let width = img.width();
        let height = img.height();

        // Fast PNG encoding to improve capture hotkey responsiveness.
        // We prefer speed over compression ratio here.
        let raw = img.into_raw();
        let mut out = Vec::new();
        let encoder = PngEncoder::new_with_quality(&mut out, CompressionType::Fast, FilterType::NoFilter);
        encoder
            .write_image(&raw, width, height, ColorType::Rgba8)
            .map_err(|e| AppError::Unknown(format!("Failed to encode PNG: {e}")))?;

        Ok((out, width, height))
    })
    .await
    .map_err(|e| AppError::Unknown(format!("Capture task join failed: {e}")))??;

    tracing::info!("Captured image: {}x{}", width, height);

    let b64 = BASE64.encode(&png_bytes);

    // Persist for later crop/encode commands (e.g., pin-from-selection)
    let frame_id = CAPTURE_FRAME_ID.fetch_add(1, Ordering::Relaxed) + 1;
    // Write to cache for fast frontend load (avoid huge base64 IPC + decode).
    let mut file_path: Option<std::path::PathBuf> = None;
    if let Ok(cache_dir) = app.path().cache_dir() {
        let dir = cache_dir.join("omnibox").join("capture");
        if std::fs::create_dir_all(&dir).is_ok() {
            let p = dir.join(format!("capture_{}.png", frame_id));
            if std::fs::write(&p, &png_bytes).is_ok() {
                file_path = Some(p);
            }
        }
    }

    *LAST_CAPTURE_PNG.lock() = Some(CapturePng {
        id: frame_id,
        png_bytes,
        width,
        height,
        file_path: file_path.clone(),
    });
    
    tracing::info!("Encoded image to base64, length: {}", b64.len());

    // Step 4: Only deliver to frontend when it's ready to receive it.
    // If not ready, we'll show a click-through window and re-deliver when the frontend calls `capture_frontend_ready`.
    let mut webview_ready = wait_capture_webview_ready(1500).await;

    if webview_ready {
        // Prefer file path payload; fallback to base64.
        let payload = if let Some(p) = file_path.as_ref().and_then(|p| p.to_str()) {
            serde_json::json!({
                "path": p,
                "width": width,
                "height": height,
            })
        } else {
            serde_json::json!({
                "data": b64,
                "width": width,
                "height": height,
            })
        };

        app.emit_to("capture", "capture:ready", payload)?;
        CAPTURE_DELIVERED_FRAME_ID.store(frame_id, Ordering::Release);
        tracing::info!("Emitted capture:ready event (webview ready)");
    } else {
        tracing::warn!("Capture webview not ready yet; delaying capture:ready delivery");
    }

    // Step 5: Show capture window.
    // IMPORTANT: the capture webview may not load while the window is hidden on Windows.
    // If it's not ready yet, show a tiny offscreen click-through window to allow the webview to initialize.
    if let Some(win) = app.get_webview_window("capture") {
        // Always enforce correct config.
        let _ = win.set_decorations(false);
        let _ = win.set_always_on_top(true);

        if webview_ready {
            let _ = win.set_fullscreen(true);
            win.show()?;
            // Re-apply after show (Windows sometimes resets on show)
            let _ = win.set_decorations(false);
            let _ = win.set_fullscreen(true);

            let _ = win.set_ignore_cursor_events(false);
            let _ = win.set_focus();
            tracing::info!("Capture window shown (ready = true)");
        } else {
            // Defer fullscreen showing; when frontend becomes ready it will deliver the frame + show.
            // But we must allow the webview to load, so keep a tiny offscreen click-through window visible.
            CAPTURE_PENDING_SHOW.store(true, Ordering::Release);
            show_capture_preload_window(&win);
            tracing::warn!("Capture webview not ready; showing preload window and deferring fullscreen until frontend_ready");

            // Give it a bit longer in case it's about to become ready; if it does, we can promote immediately.
            webview_ready = wait_capture_webview_ready(3500).await;
            if webview_ready {
                // Frontend is ready now; try to deliver and show fullscreen immediately.
                let _ = try_deliver_last_frame(&app);
                CAPTURE_PENDING_SHOW.store(false, Ordering::Release);
                let _ = win.set_decorations(false);
                let _ = win.set_fullscreen(true);
                let _ = win.show();
                let _ = win.set_decorations(false);
                let _ = win.set_fullscreen(true);
                let _ = win.set_ignore_cursor_events(false);
                let _ = win.set_focus();
                tracing::info!("Capture window promoted after preload (ready = true)");
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn hide_capture_window(app: tauri::AppHandle) -> AppResult<()> {
    if let Some(win) = app.get_webview_window("capture") {
        CAPTURE_PENDING_SHOW.store(false, Ordering::Release);
        let _ = win.set_ignore_cursor_events(true);
        // Exit fullscreen before hiding to prevent Windows from getting confused
        // about window state on next show.
        let _ = win.set_fullscreen(false);
        let _ = win.hide();
    }
    Ok(())
}

#[tauri::command]
pub async fn save_capture(app: tauri::AppHandle, png_bytes: Vec<u8>) -> AppResult<()> {
    // Decode PNG -> RGBA for clipboard
    let img = image::load_from_memory(&png_bytes)
        .map_err(|e| AppError::Unknown(format!("Failed to decode PNG: {e}")))?
        .to_rgba8();
    let (w, h) = img.dimensions();
    let bytes = img.into_raw();

    // Write to clipboard
    if let Err(e) = arboard::Clipboard::new()
        .and_then(|mut clip| {
            clip.set_image(arboard::ImageData {
                width: w as usize,
                height: h as usize,
                bytes: std::borrow::Cow::Owned(bytes.clone()),
            })
        })
    {
        tracing::error!("Clipboard write failed: {e}");
    } else {
        tracing::info!("Screenshot saved to clipboard: {}x{}", w, h);
    }

    // Hide capture window
    if let Some(win) = app.get_webview_window("capture") {
        let _ = win.set_ignore_cursor_events(true);
        let _ = win.hide();
    }

    Ok(())
}

/// Save PNG to a user-selected file path (frontend picks the path).
/// `image_data` can be either raw base64 or a full data URL.
#[tauri::command]
pub async fn save_capture_file(path: String, image_data: String) -> AppResult<()> {
    let b64 = image_data
        .split(',')
        .last()
        .unwrap_or(image_data.as_str())
        .trim();

    let bytes = BASE64
        .decode(b64)
        .map_err(|e| AppError::Unknown(format!("Failed to decode base64: {e}")))?;

    std::fs::write(&path, bytes)
        .map_err(|e| AppError::Unknown(format!("Failed to write file: {e}")))?;

    Ok(())
}

/// Clipboard fallback using base64 to avoid huge JSON arrays over IPC.
/// `image_data` can be either raw base64 or a full data URL.
#[tauri::command]
pub async fn copy_capture_base64(app: tauri::AppHandle, image_data: String) -> AppResult<()> {
    let b64 = image_data
        .split(',')
        .last()
        .unwrap_or(image_data.as_str())
        .trim();

    let png_bytes = BASE64
        .decode(b64)
        .map_err(|e| AppError::Unknown(format!("Failed to decode base64: {e}")))?;

    // Reuse existing clipboard writer
    save_capture(app, png_bytes).await
}

/// Create a pin window to display a captured screenshot region
/// The window is always on top and can be dragged around
#[tauri::command]
pub async fn create_pin_window(
    app: tauri::AppHandle,
    image_data: String,  // Base64 encoded PNG
    width: u32,
    height: u32,
    x: i32,
    y: i32,
) -> AppResult<()> {
    use tauri::WebviewWindowBuilder;
    use std::sync::atomic::{AtomicU32, Ordering};
    
    // Generate unique window ID
    static PIN_COUNTER: AtomicU32 = AtomicU32::new(0);
    let pin_id = PIN_COUNTER.fetch_add(1, Ordering::Relaxed);
    let window_label = format!("pin_{}", pin_id);
    
    // Convert selection coords (capture webview coords) -> screen coords by adding capture window position.
    // This prevents pins from showing up off-screen on multi-monitor / non-zero positioned windows.
    let mut pos_x = x as f64;
    let mut pos_y = y as f64;
    if let Some(capture_win) = app.get_webview_window("capture") {
        if let (Ok(outer_pos), Ok(scale)) = (capture_win.outer_position(), capture_win.scale_factor()) {
            let s = if scale > 0.0 { scale } else { 1.0 };
            pos_x = (outer_pos.x as f64 / s) + (x as f64);
            pos_y = (outer_pos.y as f64 / s) + (y as f64);
        }
    }

    tracing::info!(
        "Creating pin window: {} at ({}, {}) size {}x{}",
        window_label,
        pos_x,
        pos_y,
        width,
        height
    );

    // Store payload for reliable retrieval (pin window pulls on mount).
    // This avoids duplicating a huge base64 string across multiple IPC paths.
    PIN_PAYLOADS.lock().insert(
        window_label.clone(),
        PinPayload {
            data: image_data,
            width,
            height,
        },
    );
    
    // Build the pin window - keep URL small; send image via event / payload pull.
    let pin_window = WebviewWindowBuilder::new(
        &app,
        &window_label,
        tauri::WebviewUrl::App("/pin".into()),
    )
    .title("Pin")
    .inner_size(width as f64, height as f64)
    .position(pos_x, pos_y)
    .decorations(false)
    .transparent(true)
    .always_on_top(true)
    .skip_taskbar(true)
    .resizable(false)
    .focused(true)
    // Show immediately; payload can arrive via event or pull-on-mount.
    .visible(true)
    .build()
    .map_err(|e| AppError::Unknown(format!("Failed to create pin window: {e}")))?;

    let _ = pin_window.set_focus();
    
    // Hide capture window after creating pin
    if let Some(win) = app.get_webview_window("capture") {
        let _ = win.set_ignore_cursor_events(true);
        let _ = win.hide();
    }
    
    tracing::info!("Pin window created successfully");
    Ok(())
}

/// Pin window pulls its payload on mount (reliable even if initial event was missed)
#[tauri::command]
pub async fn get_pin_payload(label: String) -> AppResult<Option<PinPayload>> {
    Ok(PIN_PAYLOADS.lock().remove(&label))
}

/// Close a pin window
#[tauri::command]
pub async fn close_pin_window(app: tauri::AppHandle, label: String) -> AppResult<()> {
    if let Some(win) = app.get_webview_window(&label) {
        win.close()?;
        tracing::info!("Pin window {} closed", label);
    }
    Ok(())
}

/// Create a pin window by cropping the last captured screenshot.
/// Frontend only sends selection rect in CSS pixels + viewport size; backend maps to image pixels.
#[tauri::command]
pub async fn create_pin_window_from_selection(
    app: tauri::AppHandle,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    viewport_width: u32,
    viewport_height: u32,
) -> AppResult<()> {
    // Hide the fullscreen capture window immediately to reduce perceived latency.
    if let Some(win) = app.get_webview_window("capture") {
        let _ = win.set_ignore_cursor_events(true);
        let _ = win.hide();
    }

    // Snapshot last capture bytes (avoid holding lock across heavy work)
    let last = LAST_CAPTURE_PNG
        .lock()
        .as_ref()
        .cloned()
        .ok_or_else(|| AppError::NotFound("No capture frame available".into()))?;

    let (img_w, img_h) = (last.width, last.height);
    let (vw, vh) = (
        std::cmp::max(1, viewport_width) as f64,
        std::cmp::max(1, viewport_height) as f64,
    );

    // Map CSS pixels -> image pixels
    let scale_x = img_w as f64 / vw;
    let scale_y = img_h as f64 / vh;

    let mut src_x = ((x as f64) * scale_x).round() as i64;
    let mut src_y = ((y as f64) * scale_y).round() as i64;
    let mut src_w = ((width as f64) * scale_x).round() as i64;
    let mut src_h = ((height as f64) * scale_y).round() as i64;

    // Clamp
    if src_x < 0 { src_x = 0 }
    if src_y < 0 { src_y = 0 }
    if src_w < 1 { src_w = 1 }
    if src_h < 1 { src_h = 1 }

    let max_x = img_w as i64;
    let max_y = img_h as i64;
    if src_x > max_x { src_x = max_x }
    if src_y > max_y { src_y = max_y }
    if src_x + src_w > max_x { src_w = max_x.saturating_sub(src_x) }
    if src_y + src_h > max_y { src_h = max_y.saturating_sub(src_y) }

    // Heavy work: decode PNG, crop, encode PNG, base64
    let cropped_b64 = tauri::async_runtime::spawn_blocking(move || -> AppResult<String> {
        use image::codecs::png::{CompressionType, FilterType, PngEncoder};
        use image::{ColorType, ImageEncoder};

        let img = image::load_from_memory(&last.png_bytes)
            .map_err(|e| AppError::Unknown(format!("Failed to decode last capture PNG: {e}")))?
            .to_rgba8();

        let view = image::imageops::crop_imm(
            &img,
            src_x as u32,
            src_y as u32,
            src_w as u32,
            src_h as u32,
        )
        .to_image();

        let raw = view.into_raw();
        let mut out = Vec::new();
        let encoder = PngEncoder::new_with_quality(&mut out, CompressionType::Fast, FilterType::NoFilter);
        encoder
            .write_image(&raw, src_w as u32, src_h as u32, ColorType::Rgba8)
            .map_err(|e| AppError::Unknown(format!("Failed to encode cropped PNG: {e}")))?;

        Ok(BASE64.encode(&out))
    })
    .await
    .map_err(|e| AppError::Unknown(format!("Crop task join failed: {e}")))??;

    // Reuse the existing pin creator: x/y are still capture webview coords for placement.
    create_pin_window(app, cropped_b64, width, height, x, y).await
}

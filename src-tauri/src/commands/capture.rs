use crate::app::error::{AppError, AppResult};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use tauri::{Emitter, Manager};
use tokio::sync::Mutex as TokioMutex;

#[derive(Clone, Serialize)]
pub struct PinPayload {
    data: String,
    width: u32,
    height: u32,
}

static PIN_PAYLOADS: Lazy<Mutex<HashMap<String, PinPayload>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone)]
#[allow(dead_code)]
struct CapturePng {
    id: u64,
    png_bytes: Vec<u8>,
    width: u32,
    height: u32,
    file_path: Option<std::path::PathBuf>,
}

static LAST_CAPTURE_PNG: Lazy<Mutex<Option<CapturePng>>> = Lazy::new(|| Mutex::new(None));

// Track frame IDs for delivery
static CAPTURE_FRAME_ID: AtomicU64 = AtomicU64::new(0);
static CAPTURE_DELIVERED_FRAME_ID: AtomicU64 = AtomicU64::new(0);

// Frontend ready flag - set when capture page mounts and registers listeners
static CAPTURE_FRONTEND_READY: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

// Pending frame that needs delivery when frontend becomes ready
static CAPTURE_PENDING_FRAME: Lazy<Mutex<Option<serde_json::Value>>> = Lazy::new(|| Mutex::new(None));

// Track if warmup is done
static CAPTURE_WARMED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

// Serialize capture init to avoid races between repeated hotkey presses.
static CAPTURE_INIT_MUTEX: Lazy<TokioMutex<()>> = Lazy::new(|| TokioMutex::new(()));

/// Check if capture system is ready
#[tauri::command]
pub fn is_capture_ready() -> bool {
    CAPTURE_FRONTEND_READY.load(Ordering::Acquire)
}

/// Deliver pending capture frame to frontend
fn try_deliver_pending_frame(app: &tauri::AppHandle) -> bool {
    let pending = CAPTURE_PENDING_FRAME.lock().take();
    if let Some(payload) = pending {
        if app.emit_to("capture", "capture:ready", payload).is_ok() {
            tracing::info!("Delivered pending capture frame to frontend");
            return true;
        }
    }
    false
}

/// Warm up the capture webview at app startup - show it briefly so JS loads
/// This is critical: webview JS won't execute if the window is never shown
pub async fn warmup_capture_window(app: &tauri::AppHandle) {
    if CAPTURE_WARMED.swap(true, Ordering::AcqRel) {
        return;
    }

    tracing::info!("========================================");
    tracing::info!("Warming up capture window...");
    tracing::info!("========================================");

    if let Some(win) = app.get_webview_window("capture") {
        // Window starts visible at positive off-screen coordinates per tauri.conf.json
        // Just ensure it's in the expected state for JS to load
        let _ = win.set_decorations(false);
        let _ = win.set_always_on_top(false);
        let _ = win.set_ignore_cursor_events(true);
        let _ = win.show(); // Ensure visible for webview to load
        
        tracing::info!("Capture window positioned off-screen (800x600), waiting for frontend to load...");
        
        // Wait for frontend ready signal with LONG timeout (dev mode is slow)
        // 150 iterations * 100ms = 15 seconds
        for i in 0..150 {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            
            if CAPTURE_FRONTEND_READY.load(Ordering::Acquire) {
                tracing::info!("✓ Capture frontend ready during warmup at iteration {} ({}ms)", i, i * 100);
                break;
            }
            if i % 30 == 29 {
                tracing::info!("Still waiting for capture frontend... ({}s)", (i + 1) / 10);
            }
        }
        
        // Hide it after warmup
        let _ = win.hide();
        let frontend_ready = CAPTURE_FRONTEND_READY.load(Ordering::Acquire);
        
        tracing::info!("========================================");
        if frontend_ready {
            tracing::info!("✓ CAPTURE SYSTEM READY - You can use Ctrl+Alt+X now!");
        } else {
            tracing::warn!("⚠ Capture frontend not ready after 15s warmup");
            tracing::info!("  (This is normal in dev mode - capture will still work when triggered)");
        }
        tracing::info!("========================================");
    } else {
        tracing::error!("Capture window not found during warmup!");
    }
}

/// Called by the capture frontend on mount.
/// Marks the frontend as ready and delivers any pending capture frame.
#[tauri::command]
pub async fn capture_frontend_ready(app: tauri::AppHandle) -> AppResult<()> {
    tracing::info!("Capture frontend ready signal received");
    CAPTURE_FRONTEND_READY.store(true, Ordering::Release);

    // Deliver any pending frame
    let delivered = try_deliver_pending_frame(&app);
    if delivered {
        tracing::info!("Delivered pending frame on frontend ready");
    }

    Ok(())
}

/// Start screen capture - simplified and robust flow:
/// 1. Wait for frontend ready (critical!)
/// 2. Hide capture window
/// 3. Capture screen to PNG
/// 4. Show capture window covering the screen
/// 5. Emit capture data to frontend
#[tauri::command]
pub async fn init_capture(app: tauri::AppHandle) -> AppResult<()> {
    // Prevent overlapping captures
    let _guard = match CAPTURE_INIT_MUTEX.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            tracing::warn!("Capture init skipped: another capture is in progress");
            return Ok(());
        }
    };

    tracing::info!("=== Starting screen capture ===");
    
    // Check if frontend is ready
    let mut frontend_ready = CAPTURE_FRONTEND_READY.load(Ordering::Acquire);
    tracing::info!("Frontend ready status: {}", frontend_ready);
    
    // If not ready, wait with timeout (useful in dev mode where loading is slow)
    if !frontend_ready {
        tracing::info!("Frontend not ready, waiting with extended timeout...");
        
        // Show capture window to trigger JS loading if not already loaded
        if let Some(win) = app.get_webview_window("capture") {
            let _ = win.set_position(tauri::PhysicalPosition::new(15000i32, 15000i32));
            let _ = win.show();
        }
        
        // Wait for frontend ready with 10 second timeout
        for i in 0..100 {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            if CAPTURE_FRONTEND_READY.load(Ordering::Acquire) {
                tracing::info!("✓ Frontend became ready after {}ms", (i + 1) * 100);
                frontend_ready = true;
                break;
            }
            if i % 20 == 19 {
                tracing::info!("Still waiting for frontend... ({}s)", (i + 1) / 10);
            }
        }
        
        // Hide window after waiting
        if let Some(win) = app.get_webview_window("capture") {
            let _ = win.hide();
        }
        
        // Check result
        if !frontend_ready {
            tracing::error!("Frontend not ready after 10s, proceeding anyway (may fail)");
        }
    }

    // Step 1: Hide capture window first (must be hidden during capture)
    if let Some(win) = app.get_webview_window("capture") {
        let _ = win.set_fullscreen(false);
        let _ = win.hide();
        tracing::info!("Capture window hidden for capture");
    }

    // Small delay to ensure window is fully hidden
    tokio::time::sleep(std::time::Duration::from_millis(80)).await;

    // Step 2: Capture screen
    tracing::info!("Capturing screen...");
    let (png_bytes, width, height, mon_x, mon_y, mon_w, mon_h) = 
        tauri::async_runtime::spawn_blocking(move || -> AppResult<(Vec<u8>, u32, u32, i32, i32, u32, u32)> {
            use image::codecs::png::{CompressionType, FilterType, PngEncoder};
            use image::{ColorType, ImageEncoder};

            let monitors = xcap::Monitor::all()
                .map_err(|e| AppError::Unknown(format!("Failed to list monitors: {e}")))?;

            let monitor = monitors
                .into_iter()
                .find(|m| m.is_primary().unwrap_or(false))
                .or_else(|| xcap::Monitor::all().ok()?.into_iter().next())
                .ok_or_else(|| AppError::NotFound("No monitor found".into()))?;

            // Get monitor geometry
            let mon_x = monitor.x().unwrap_or(0);
            let mon_y = monitor.y().unwrap_or(0);
            let mon_w = monitor.width().unwrap_or(1920);
            let mon_h = monitor.height().unwrap_or(1080);

            let img = monitor
                .capture_image()
                .map_err(|e| AppError::Unknown(format!("Failed to capture screen: {e}")))?;

            let width = img.width();
            let height = img.height();

            // Fast PNG encoding
            let raw = img.into_raw();
            let mut out = Vec::new();
            let encoder = PngEncoder::new_with_quality(&mut out, CompressionType::Fast, FilterType::NoFilter);
            encoder
                .write_image(&raw, width, height, ColorType::Rgba8)
                .map_err(|e| AppError::Unknown(format!("Failed to encode PNG: {e}")))?;

            Ok((out, width, height, mon_x, mon_y, mon_w, mon_h))
        })
        .await
        .map_err(|e| AppError::Unknown(format!("Capture task join failed: {e}")))??;

    tracing::info!("Captured image: {}x{}, monitor: ({}, {}) {}x{}", 
        width, height, mon_x, mon_y, mon_w, mon_h);

    // Store frame for later use
    let frame_id = CAPTURE_FRAME_ID.fetch_add(1, Ordering::Relaxed) + 1;
    
    // Write to cache file for fast frontend load
    let mut file_path: Option<std::path::PathBuf> = None;
    if let Ok(cache_dir) = app.path().cache_dir() {
        let dir = cache_dir.join("omnibox").join("capture");
        if std::fs::create_dir_all(&dir).is_ok() {
            let p = dir.join(format!("capture_{}.png", frame_id));
            if std::fs::write(&p, &png_bytes).is_ok() {
                file_path = Some(p.clone());
                tracing::info!("Saved capture to cache: {:?}", p);
            }
        }
    }

    *LAST_CAPTURE_PNG.lock() = Some(CapturePng {
        id: frame_id,
        png_bytes: png_bytes.clone(),
        width,
        height,
        file_path: file_path.clone(),
    });

    // Build payload - include monitor position for coordinate conversion
    // Always send base64 data for reliability (convertFileSrc can have issues)
    let payload = serde_json::json!({
        "data": BASE64.encode(&png_bytes),
        "width": width,
        "height": height,
        "monitorX": mon_x,
        "monitorY": mon_y,
    });

    // Also save to file for debugging (optional)
    if let Some(p) = file_path.as_ref() {
        tracing::info!("Capture also saved to: {:?}", p);
    }

    // Step 3: Emit capture data BEFORE showing window
    // This ensures frontend has the image ready to render
    *CAPTURE_PENDING_FRAME.lock() = Some(payload.clone());
    
    if app.emit_to("capture", "capture:ready", payload.clone()).is_ok() {
        CAPTURE_DELIVERED_FRAME_ID.store(frame_id, Ordering::Release);
        *CAPTURE_PENDING_FRAME.lock() = None;
        tracing::info!("Emitted capture:ready event");
    } else {
        tracing::warn!("Failed to emit capture:ready event");
    }

    // Small delay to let frontend process the event
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // Step 4: Show capture window - use PHYSICAL pixels to cover screen exactly
    if let Some(win) = app.get_webview_window("capture") {
        // Reset window state
        let _ = win.set_decorations(false);
        let _ = win.set_always_on_top(true);
        let _ = win.set_skip_taskbar(true);
        
        // Position window to cover the monitor using physical coordinates
        // DO NOT use fullscreen mode - it has bugs on Windows
        let _ = win.set_position(tauri::PhysicalPosition::new(mon_x, mon_y));
        let _ = win.set_size(tauri::PhysicalSize::new(mon_w, mon_h));

        // Show the window
        win.show()?;
        
        // Re-apply settings after show (Windows can reset them)
        let _ = win.set_decorations(false);
        let _ = win.set_always_on_top(true);
        let _ = win.set_ignore_cursor_events(false);
        
        // Multiple focus attempts - Windows transparent windows need this
        let _ = win.set_focus();
        
        // Spawn a background task to retry focus (Windows can be stubborn)
        let win_clone = win.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(50));
            let _ = win_clone.set_focus();
            std::thread::sleep(std::time::Duration::from_millis(100));
            let _ = win_clone.set_focus();
        });

        // Log actual window state
        let actual_pos = win.outer_position().ok();
        let actual_size = win.outer_size().ok();
        tracing::info!("Capture window shown: pos={:?} size={:?}", actual_pos, actual_size);
    }

    tracing::info!("=== Capture init complete ===");
    Ok(())
}

#[tauri::command]
pub async fn hide_capture_window(app: tauri::AppHandle) -> AppResult<()> {
    if let Some(win) = app.get_webview_window("capture") {
        let _ = win.set_ignore_cursor_events(true);
        let _ = win.set_fullscreen(false);
        let _ = win.hide();
        tracing::info!("Capture window hidden");
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

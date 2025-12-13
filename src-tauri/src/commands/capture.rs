use crate::app::error::{AppError, AppResult};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use image::{DynamicImage, ImageBuffer, Rgba};
use std::io::Cursor;
use tauri::{Emitter, Manager};

/// Start screen capture - the CORRECT sequence to avoid black screen:
/// 1. Ensure capture window is HIDDEN
/// 2. Capture the screen (while window is hidden)
/// 3. Encode image to base64
/// 4. Emit data to frontend via event
/// 5. THEN show the window (frontend will render the captured image)
#[tauri::command]
pub async fn init_capture(app: tauri::AppHandle) -> AppResult<()> {
    // Step 1: Ensure capture window is hidden FIRST
    if let Some(win) = app.get_webview_window("capture") {
        let _ = win.hide();
    }
    
    // Small delay to ensure window is fully hidden before capture
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    
    tracing::info!("Starting screen capture...");
    
    // Step 2: Capture the screen while window is hidden
    let monitors = xcap::Monitor::all()
        .map_err(|e| AppError::Unknown(format!("Failed to list monitors: {e}")))?;
    
    // Get primary monitor (or first available)
    let monitor = monitors
        .into_iter()
        .find(|m| m.is_primary().unwrap_or(false))
        .or_else(|| xcap::Monitor::all().ok()?.into_iter().next())
        .ok_or_else(|| AppError::NotFound("No monitor found".into()))?;
    
    let mon_width = monitor.width().unwrap_or(1920);
    let mon_height = monitor.height().unwrap_or(1080);
    tracing::info!("Capturing monitor: {}x{}", mon_width, mon_height);
    
    let img = monitor
        .capture_image()
        .map_err(|e| AppError::Unknown(format!("Failed to capture screen: {e}")))?;

    let width = img.width();
    let height = img.height();
    
    tracing::info!("Captured image: {}x{}", width, height);

    // Step 3: Encode to PNG base64
    let mut buf = Cursor::new(Vec::new());
    let dyn_img = DynamicImage::ImageRgba8(
        ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, img.into_raw())
            .ok_or_else(|| AppError::Unknown("Invalid image buffer".to_string()))?
    );
    dyn_img
        .write_to(&mut buf, image::ImageFormat::Png)
        .map_err(|e| AppError::Unknown(format!("Failed to encode PNG: {e}")))?;

    let b64 = BASE64.encode(buf.into_inner());
    
    tracing::info!("Encoded image to base64, length: {}", b64.len());

    // Step 4: Emit data to frontend via event BEFORE showing window
    app.emit_to(
        "capture",
        "capture:ready",
        serde_json::json!({
            "data": b64,
            "width": width,
            "height": height,
        }),
    )?;
    
    tracing::info!("Emitted capture:ready event");

    // Step 5: Show capture window AFTER data is sent
    if let Some(win) = app.get_webview_window("capture") {
        win.show()?;
        win.set_focus()?;
        tracing::info!("Capture window shown");
    }

    Ok(())
}

#[tauri::command]
pub async fn hide_capture_window(app: tauri::AppHandle) -> AppResult<()> {
    if let Some(win) = app.get_webview_window("capture") {
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
        let _ = win.hide();
    }

    Ok(())
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
    
    tracing::info!("Creating pin window: {} at ({}, {}) size {}x{}", window_label, x, y, width, height);
    
    // Build the pin window - keep URL small; send image via event
    let pin_window = WebviewWindowBuilder::new(
        &app,
        &window_label,
        tauri::WebviewUrl::App("/pin".into()),
    )
    .title("Pin")
    .inner_size(width as f64, height as f64)
    .position(x as f64, y as f64)
    .decorations(false)
    .transparent(true)
    .always_on_top(true)
    .skip_taskbar(true)
    .resizable(false)
    .focused(true)
    .visible(false)
    .build()
    .map_err(|e| AppError::Unknown(format!("Failed to create pin window: {e}")))?;

    // Give frontend a moment to mount and start listening
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // Send image data to pin window
    app.emit_to(
        &window_label,
        "pin:set_image",
        serde_json::json!({
            "data": image_data,
            "width": width,
            "height": height,
        }),
    )?;

    // Show after payload is sent
    let _ = pin_window.show();
    let _ = pin_window.set_focus();
    
    // Hide capture window after creating pin
    if let Some(win) = app.get_webview_window("capture") {
        let _ = win.hide();
    }
    
    tracing::info!("Pin window created successfully");
    Ok(())
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

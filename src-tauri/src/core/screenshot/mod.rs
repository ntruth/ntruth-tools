//! High-performance Screenshot Engine
//!
//! This module provides optimized screenshot capture capabilities with:
//! - Multi-monitor support with automatic cursor position detection
//! - Physical pixel capture for HiDPI displays
//! - Fast PNG encoding with minimal compression
//! - Memory-efficient buffer management

use crate::app::error::{AppError, AppResult};
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::{ColorType, ImageEncoder, RgbaImage};
use std::sync::Arc;
use parking_lot::RwLock;

/// Monitor information for multi-screen support
#[derive(Debug, Clone, serde::Serialize)]
pub struct MonitorInfo {
    pub id: String,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f64,
    pub is_primary: bool,
}

/// Captured screenshot data
#[derive(Debug, Clone)]
pub struct CaptureResult {
    pub png_bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub monitor: MonitorInfo,
}

/// Screenshot engine with caching and optimization
pub struct ScreenshotEngine {
    /// Cached monitor list (refreshed on demand)
    monitors_cache: Arc<RwLock<Option<Vec<MonitorInfo>>>>,
    /// Pre-allocated buffer for PNG encoding (reduces allocations)
    encode_buffer: Arc<RwLock<Vec<u8>>>,
}

impl Default for ScreenshotEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenshotEngine {
    pub fn new() -> Self {
        Self {
            monitors_cache: Arc::new(RwLock::new(None)),
            encode_buffer: Arc::new(RwLock::new(Vec::with_capacity(8 * 1024 * 1024))), // 8MB pre-alloc
        }
    }

    /// Refresh the monitor cache
    pub fn refresh_monitors(&self) -> AppResult<Vec<MonitorInfo>> {
        let monitors = xcap::Monitor::all()
            .map_err(|e| AppError::Unknown(format!("Failed to enumerate monitors: {e}")))?;

        let info: Vec<MonitorInfo> = monitors
            .into_iter()
            .enumerate()
            .map(|(idx, m)| MonitorInfo {
                id: format!("monitor_{}", idx),
                name: m.name().unwrap_or_else(|_| format!("Display {}", idx + 1)),
                x: m.x().unwrap_or(0),
                y: m.y().unwrap_or(0),
                width: m.width().unwrap_or(1920),
                height: m.height().unwrap_or(1080),
                scale_factor: m.scale_factor().unwrap_or(1.0) as f64,
                is_primary: m.is_primary().unwrap_or(false),
            })
            .collect();

        *self.monitors_cache.write() = Some(info.clone());
        Ok(info)
    }

    /// Get all monitors (uses cache if available)
    pub fn get_monitors(&self) -> AppResult<Vec<MonitorInfo>> {
        if let Some(cached) = self.monitors_cache.read().clone() {
            return Ok(cached);
        }
        self.refresh_monitors()
    }

    /// Get monitor at cursor position
    #[cfg(windows)]
    pub fn get_monitor_at_cursor(&self) -> AppResult<MonitorInfo> {
        use windows::Win32::Foundation::POINT;
        use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, MonitorFromPoint, MONITOR_DEFAULTTONEAREST, MONITORINFOEXW};
        use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

        unsafe {
            let mut cursor_pos = POINT::default();
            GetCursorPos(&mut cursor_pos)
                .map_err(|e| AppError::Unknown(format!("GetCursorPos failed: {e}")))?;

            let hmonitor = MonitorFromPoint(cursor_pos, MONITOR_DEFAULTTONEAREST);

            let mut info = MONITORINFOEXW::default();
            info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
            if !GetMonitorInfoW(hmonitor, &mut info.monitorInfo as *mut _ as *mut _).as_bool() {
                return Err(AppError::Unknown("GetMonitorInfoW failed".into()));
            }

            let rc = info.monitorInfo.rcMonitor;

            // Find matching monitor in our cache
            let monitors = self.get_monitors()?;
            monitors
                .into_iter()
                .find(|m| m.x == rc.left && m.y == rc.top)
                .ok_or_else(|| AppError::NotFound("Monitor not found".into()))
        }
    }

    #[cfg(not(windows))]
    pub fn get_monitor_at_cursor(&self) -> AppResult<MonitorInfo> {
        // Fallback: return primary monitor
        self.get_monitors()?
            .into_iter()
            .find(|m| m.is_primary)
            .or_else(|| self.get_monitors().ok()?.into_iter().next())
            .ok_or_else(|| AppError::NotFound("No monitor found".into()))
    }

    /// Capture a specific monitor
    pub fn capture_monitor(&self, monitor_info: &MonitorInfo) -> AppResult<CaptureResult> {
        let monitors = xcap::Monitor::all()
            .map_err(|e| AppError::Unknown(format!("Failed to list monitors: {e}")))?;

        let monitor = monitors
            .into_iter()
            .find(|m| {
                m.x().unwrap_or(0) == monitor_info.x && m.y().unwrap_or(0) == monitor_info.y
            })
            .ok_or_else(|| AppError::NotFound("Target monitor not found".into()))?;

        let img = monitor
            .capture_image()
            .map_err(|e| AppError::Unknown(format!("Failed to capture screen: {e}")))?;

        let width = img.width();
        let height = img.height();
        let raw = img.into_raw();

        // Fast PNG encoding with pre-allocated buffer
        let png_bytes = self.encode_png_fast(&raw, width, height)?;

        Ok(CaptureResult {
            png_bytes,
            width,
            height,
            monitor: monitor_info.clone(),
        })
    }

    /// Capture the primary monitor
    pub fn capture_primary(&self) -> AppResult<CaptureResult> {
        let monitors = self.get_monitors()?;
        let primary = monitors
            .iter()
            .find(|m| m.is_primary)
            .or_else(|| monitors.first())
            .cloned()
            .ok_or_else(|| AppError::NotFound("No monitor found".into()))?;

        self.capture_monitor(&primary)
    }

    /// Capture the monitor under the cursor
    pub fn capture_at_cursor(&self) -> AppResult<CaptureResult> {
        let monitor = self.get_monitor_at_cursor()?;
        self.capture_monitor(&monitor)
    }

    /// Fast PNG encoding optimized for speed over compression ratio
    fn encode_png_fast(&self, raw: &[u8], width: u32, height: u32) -> AppResult<Vec<u8>> {
        let mut buffer = self.encode_buffer.write();
        buffer.clear();

        let encoder = PngEncoder::new_with_quality(
            &mut *buffer,
            CompressionType::Fast,   // Fastest compression
            FilterType::NoFilter,    // No filtering for speed
        );

        encoder
            .write_image(raw, width, height, ColorType::Rgba8)
            .map_err(|e| AppError::Unknown(format!("PNG encoding failed: {e}")))?;

        Ok(buffer.clone())
    }

    /// Crop and encode a region from raw RGBA data
    pub fn crop_and_encode(
        &self,
        raw: &[u8],
        full_width: u32,
        full_height: u32,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> AppResult<Vec<u8>> {
        // Validate bounds
        if x + width > full_width || y + height > full_height {
            return Err(AppError::Unknown("Crop region out of bounds".into()));
        }

        let img = RgbaImage::from_raw(full_width, full_height, raw.to_vec())
            .ok_or_else(|| AppError::Unknown("Failed to create image from raw data".into()))?;

        let cropped = image::imageops::crop_imm(&img, x, y, width, height).to_image();
        let cropped_raw = cropped.into_raw();

        self.encode_png_fast(&cropped_raw, width, height)
    }
}

/// Global screenshot engine instance
static SCREENSHOT_ENGINE: once_cell::sync::Lazy<ScreenshotEngine> =
    once_cell::sync::Lazy::new(ScreenshotEngine::new);

/// Get the global screenshot engine
pub fn get_engine() -> &'static ScreenshotEngine {
    &SCREENSHOT_ENGINE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = ScreenshotEngine::new();
        assert!(engine.monitors_cache.read().is_none());
    }
}

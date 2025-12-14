use crate::app::error::{AppError, AppResult};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn width(&self) -> i32 {
        self.right - self.left
    }

    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }

    pub fn is_empty(&self) -> bool {
        self.width() <= 0 || self.height() <= 0
    }
}

/// Get UI element bounding rectangle at a given screen point.
///
/// Performance notes:
/// - Runs on a blocking thread via `spawn_blocking` to avoid blocking Tauri's async runtime.
/// - On Windows, uses UI Automation (COM) `IUIAutomation::ElementFromPoint`.
/// - Returns `Ok(None)` when no element or empty rect.
#[tauri::command]
pub async fn get_element_rect_at(x: i32, y: i32) -> AppResult<Option<Rect>> {
    tauri::async_runtime::spawn_blocking(move || get_element_rect_at_blocking(x, y))
        .await
        .map_err(|e| AppError::Unknown(format!("UIA join error: {e}")))?
}

#[cfg(windows)]
fn get_element_rect_at_blocking(x: i32, y: i32) -> AppResult<Option<Rect>> {
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER,
        COINIT_APARTMENTTHREADED,
    };
    use windows::Win32::UI::Accessibility::{CUIAutomation, IUIAutomation};
    use windows::Win32::Foundation::{POINT, RECT};

    unsafe {
        // UIAutomation requires COM on the calling thread.
        // We initialize per-call on a blocking thread (cheap enough for moderate polling).
        // If you need very high frequency polling, wrap this into a long-lived worker thread
        // and reuse the IUIAutomation instance.
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)
            .ok()
            .map_err(|e| AppError::Unknown(format!("CoInitializeEx failed: {e}")))?;

        struct CoGuard;
        impl Drop for CoGuard {
            fn drop(&mut self) {
                unsafe { CoUninitialize() };
            }
        }
        let _guard = CoGuard;

        let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER)
            .map_err(|e| AppError::Unknown(format!("CoCreateInstance(CUIAutomation) failed: {e}")))?;

        let point = POINT { x, y };
        let element = automation
            .ElementFromPoint(point)
            .map_err(|e| AppError::Unknown(format!("ElementFromPoint failed: {e}")))?;

        let rect: RECT = element
            .CurrentBoundingRectangle()
            .map_err(|e| AppError::Unknown(format!("CurrentBoundingRectangle failed: {e}")))?;

        let out = Rect {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        };

        if out.is_empty() {
            Ok(None)
        } else {
            Ok(Some(out))
        }
    }
}

#[cfg(not(windows))]
fn get_element_rect_at_blocking(_x: i32, _y: i32) -> AppResult<Option<Rect>> {
    // Non-Windows stub
    Ok(None)
}

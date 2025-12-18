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

    /// Check if a point is inside this rect
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.left && x < self.right && y >= self.top && y < self.bottom
    }
}

// Thread-local storage for UIA context (COM objects are apartment-threaded)
#[cfg(windows)]
thread_local! {
    static UIA_AUTOMATION: std::cell::RefCell<Option<windows::Win32::UI::Accessibility::IUIAutomation>> = std::cell::RefCell::new(None);
}

/// Initialize UI Automation context on current thread
#[cfg(windows)]
fn ensure_uia_context() -> AppResult<windows::Win32::UI::Accessibility::IUIAutomation> {
    use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED};
    use windows::Win32::UI::Accessibility::{CUIAutomation, IUIAutomation};

    UIA_AUTOMATION.with(|cell| {
        let mut opt = cell.borrow_mut();
        if let Some(ref automation) = *opt {
            return Ok(automation.clone());
        }

        unsafe {
            // Initialize COM if not already done on this thread
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

            let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER)
                .map_err(|e| AppError::Unknown(format!("CoCreateInstance(CUIAutomation) failed: {e}")))?;

            *opt = Some(automation.clone());
            Ok(automation)
        }
    })
}

/// Get UI element bounding rectangle at a given screen point.
///
/// Performance notes:
/// - Runs on a blocking thread via `spawn_blocking` to avoid blocking Tauri's async runtime.
/// - On Windows, uses UI Automation (COM) `IUIAutomation::ElementFromPoint`.
/// - Caches the IUIAutomation instance for repeated calls.
/// - Returns `Ok(None)` when no element or empty rect.
#[tauri::command]
pub async fn get_element_rect_at(x: i32, y: i32) -> AppResult<Option<Rect>> {
    tauri::async_runtime::spawn_blocking(move || get_element_rect_at_blocking(x, y))
        .await
        .map_err(|e| AppError::Unknown(format!("UIA join error: {e}")))?
}

#[cfg(windows)]
fn get_element_rect_at_blocking(x: i32, y: i32) -> AppResult<Option<Rect>> {
    use windows::Win32::Foundation::POINT;

    let automation = ensure_uia_context()?;

    unsafe {
        let point = POINT { x, y };
        
        // Get element at point
        let element = match automation.ElementFromPoint(point) {
            Ok(el) => el,
            Err(e) => {
                // Some points may not have UI elements (e.g., desktop background)
                tracing::trace!("ElementFromPoint({}, {}) failed: {}", x, y, e);
                return Ok(None);
            }
        };

        // Get bounding rectangle
        let rect = match element.CurrentBoundingRectangle() {
            Ok(r) => r,
            Err(e) => {
                tracing::trace!("CurrentBoundingRectangle failed: {}", e);
                return Ok(None);
            }
        };

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

/// Get detailed UI element information at a given screen point
#[derive(Debug, Clone, serde::Serialize)]
pub struct ElementInfo {
    pub rect: Rect,
    pub name: Option<String>,
    pub control_type: Option<String>,
    pub class_name: Option<String>,
}

#[tauri::command]
pub async fn get_element_info_at(x: i32, y: i32) -> AppResult<Option<ElementInfo>> {
    tauri::async_runtime::spawn_blocking(move || get_element_info_at_blocking(x, y))
        .await
        .map_err(|e| AppError::Unknown(format!("UIA join error: {e}")))?
}

#[cfg(windows)]
fn get_element_info_at_blocking(x: i32, y: i32) -> AppResult<Option<ElementInfo>> {
    use windows::Win32::Foundation::POINT;

    let automation = ensure_uia_context()?;

    unsafe {
        let point = POINT { x, y };
        
        let element = match automation.ElementFromPoint(point) {
            Ok(el) => el,
            Err(_) => return Ok(None),
        };

        let rect = match element.CurrentBoundingRectangle() {
            Ok(r) => Rect {
                left: r.left,
                top: r.top,
                right: r.right,
                bottom: r.bottom,
            },
            Err(_) => return Ok(None),
        };

        if rect.is_empty() {
            return Ok(None);
        }

        // Get optional properties
        let name = element.CurrentName().ok().map(|s| s.to_string());
        let class_name = element.CurrentClassName().ok().map(|s| s.to_string());
        let control_type = element.CurrentLocalizedControlType().ok().map(|s| s.to_string());

        Ok(Some(ElementInfo {
            rect,
            name,
            control_type,
            class_name,
        }))
    }
}

#[cfg(not(windows))]
fn get_element_info_at_blocking(_x: i32, _y: i32) -> AppResult<Option<ElementInfo>> {
    Ok(None)
}

/// Batch query multiple points for element rects (more efficient for polling)
#[tauri::command]
pub async fn get_element_rects_batch(points: Vec<(i32, i32)>) -> AppResult<Vec<Option<Rect>>> {
    tauri::async_runtime::spawn_blocking(move || {
        points
            .into_iter()
            .map(|(x, y)| get_element_rect_at_blocking(x, y).ok().flatten())
            .collect()
    })
    .await
    .map_err(|e| AppError::Unknown(format!("UIA batch join error: {e}")))
}

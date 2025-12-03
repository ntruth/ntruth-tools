// Clipboard manager module
pub mod types;
pub mod storage;
pub mod filter;
pub mod monitor;
pub mod window;

pub use types::{ClipboardContent, ImageFormat};
pub use storage::{ClipboardStorage, ClipboardHistoryItem};
pub use filter::ContentFilter;
pub use monitor::ClipboardMonitor;
pub use window::ClipboardWindowManager;

pub struct ClipboardManager;

impl ClipboardManager {
    pub fn new() -> Self {
        Self
    }
}

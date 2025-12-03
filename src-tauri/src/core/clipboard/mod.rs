// Clipboard manager module
pub mod types;
pub mod storage;
pub mod filter;

pub use types::{ClipboardContent, ImageFormat};
pub use storage::{ClipboardStorage, ClipboardHistoryItem};
pub use filter::ContentFilter;

// TODO: Implement monitor.rs for clipboard monitoring
// TODO: Implement paste.rs for paste simulation

pub struct ClipboardManager;

impl ClipboardManager {
    pub fn new() -> Self {
        Self
    }
}

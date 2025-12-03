pub mod macos;
pub mod windows;

// Platform-specific functionality
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "windows")]
pub use windows::*;

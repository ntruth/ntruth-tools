pub mod app;
pub mod commands;
pub mod core;
pub mod platform;
pub mod storage;
pub mod utils;

// Search engines
#[cfg(windows)]
pub mod app_indexer;
#[cfg(windows)]
pub mod everything_service;

// Legacy - can be removed after migration
#[cfg(windows)]
pub mod everything;

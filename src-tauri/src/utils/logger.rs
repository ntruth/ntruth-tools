// Logger utilities
use std::path::Path;
use tracing::Level;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize the logger with file output and rotation
/// Note: For production, consider using tracing_appender::rolling for better performance
pub fn init_logger(log_dir: Option<&Path>, level: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    // Determine log level from parameter or environment variable
    let log_level = level
        .or_else(|| std::env::var("RUST_LOG").ok().as_deref())
        .unwrap_or("info");

    // Create filter
    let filter = EnvFilter::try_new(log_level)?;

    // Create formatter for console
    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_line_number(true)
        .with_level(true);

    // If log directory is specified, set up file logging
    if let Some(log_path) = log_dir {
        // Ensure log directory exists
        std::fs::create_dir_all(log_path)?;

        // Create log file path
        let log_file = log_path.join("omnibox.log");

        // Create file appender
        // Note: Using basic file appender for simplicity. For high-throughput apps,
        // consider using tracing_appender::rolling::RollingFileAppender
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)?;

        let file_layer = fmt::layer()
            .with_writer(std::sync::Arc::new(std::sync::Mutex::new(file)))
            .with_ansi(false)
            .with_target(true)
            .with_line_number(true);

        // Combine console and file output
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .with(file_layer)
            .init();
    } else {
        // Console only
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .init();
    }

    Ok(())
}

/// Initialize a simple logger for development
pub fn init_simple_logger() {
    let _ = init_logger(None, Some("debug"));
}

/// Get log level from string
pub fn parse_level(level: &str) -> Level {
    match level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" | "warning" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    }
}

/// Rotate log files (keep last N files)
pub fn rotate_logs(log_dir: &Path, keep_count: usize) -> std::io::Result<()> {
    let log_file = log_dir.join("omnibox.log");

    if !log_file.exists() {
        return Ok(());
    }

    // Check file size (rotate if > 10MB)
    let metadata = std::fs::metadata(&log_file)?;
    if metadata.len() < 10 * 1024 * 1024 {
        return Ok(());
    }

    // Rotate existing logs
    for i in (1..keep_count).rev() {
        let old_file = log_dir.join(format!("omnibox.log.{}", i));
        let new_file = log_dir.join(format!("omnibox.log.{}", i + 1));
        if old_file.exists() {
            let _ = std::fs::rename(&old_file, &new_file);
        }
    }

    // Move current log to .1
    let backup = log_dir.join("omnibox.log.1");
    std::fs::rename(&log_file, &backup)?;

    // Delete oldest log if exceeding keep_count
    let oldest = log_dir.join(format!("omnibox.log.{}", keep_count + 1));
    if oldest.exists() {
        std::fs::remove_file(oldest)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_level() {
        assert!(matches!(parse_level("debug"), Level::DEBUG));
        assert!(matches!(parse_level("info"), Level::INFO));
        assert!(matches!(parse_level("warn"), Level::WARN));
        assert!(matches!(parse_level("error"), Level::ERROR));
    }
}


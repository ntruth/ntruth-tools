// Logger utilities
// TODO: Implement logger

use tracing_subscriber;

pub fn init_logger() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
}

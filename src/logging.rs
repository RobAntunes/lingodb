//! Structured logging for Lingo
//! 
//! This module provides structured logging using the `tracing` crate,
//! with support for different log levels, structured fields, and spans.

use tracing::{Level, Metadata};
use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use crate::config::env::EnvConfig;

/// Initialize the logging system
/// 
/// This should be called once at program startup.
/// The log level can be controlled via the LINGO_LOG_LEVEL environment variable.
pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    let config = EnvConfig::from_env();
    
    // Create env filter from config
    let filter = match config.log_level.as_str() {
        "trace" => EnvFilter::new("trace"),
        "debug" => EnvFilter::new("debug"),
        "info" => EnvFilter::new("info"),
        "warn" => EnvFilter::new("warn"),
        "error" => EnvFilter::new("error"),
        _ => EnvFilter::new("info"),
    };
    
    // Add RUST_LOG support as fallback
    let filter = filter.add_directive(
        std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "info".to_string())
            .parse()?
    );
    
    // Build the subscriber
    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(config.debug_mode)
        .with_thread_names(config.debug_mode)
        .with_file(config.debug_mode)
        .with_line_number(config.debug_mode);
    
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .try_init()?;
    
    Ok(())
}

/// Initialize logging for tests
#[cfg(test)]
pub fn init_test_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .with_test_writer()
        .try_init();
}

// Re-export commonly used tracing macros
pub use tracing::{debug, error, info, trace, warn};
pub use tracing::{debug_span, error_span, info_span, trace_span, warn_span};
pub use tracing::{event, span};

/// Log query execution with structured fields
#[macro_export]
macro_rules! log_query {
    ($level:expr, $query:expr, $($field:tt)*) => {
        tracing::event!(
            $level,
            query = %$query,
            category = "query",
            $($field)*
        );
    };
}

/// Log database operations with structured fields
#[macro_export]
macro_rules! log_db_op {
    ($level:expr, $operation:expr, $($field:tt)*) => {
        tracing::event!(
            $level,
            operation = %$operation,
            category = "database",
            $($field)*
        );
    };
}

/// Log performance metrics
#[macro_export]
macro_rules! log_perf {
    ($operation:expr, $duration_ms:expr, $($field:tt)*) => {
        tracing::event!(
            tracing::Level::INFO,
            operation = %$operation,
            duration_ms = $duration_ms,
            category = "performance",
            $($field)*
        );
    };
}

/// Create a span for tracking operations
#[macro_export]
macro_rules! lingo_span {
    ($name:expr) => {
        tracing::info_span!($name)
    };
    ($name:expr, $($field:tt)*) => {
        tracing::info_span!($name, $($field)*)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_logging_macros() {
        init_test_logging();
        
        // Test basic logging
        info!("Test info message");
        debug!("Test debug message");
        warn!("Test warning");
        error!("Test error");
        
        // Test structured logging
        info!(user_id = 42, action = "login", "User logged in");
        
        // Test custom macros
        log_query!(Level::INFO, "SELECT * FROM nodes", result_count = 10);
        log_db_op!(Level::DEBUG, "open", path = "/tmp/test.lingo");
        log_perf!("query_execution", 125, node_count = 1000);
    }
    
    #[test]
    fn test_spans() {
        init_test_logging();
        
        let span = lingo_span!("test_operation", id = 123);
        let _guard = span.enter();
        
        info!("Inside span");
        debug!(nested = true, "Nested operation");
    }
}
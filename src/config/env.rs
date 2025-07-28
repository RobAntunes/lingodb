//! Environment variable configuration for Lingo
//! 
//! This module provides environment-based configuration with sensible defaults.

use std::path::PathBuf;
use std::env;

/// Environment variable names
pub mod vars {
    /// Database file path
    pub const DATABASE_PATH: &str = "LINGO_DATABASE_PATH";
    
    /// Log level (trace, debug, info, warn, error)
    pub const LOG_LEVEL: &str = "LINGO_LOG_LEVEL";
    
    /// Maximum cache size in MB
    pub const CACHE_SIZE_MB: &str = "LINGO_CACHE_SIZE_MB";
    
    /// Maximum query timeout in seconds
    pub const QUERY_TIMEOUT_SECS: &str = "LINGO_QUERY_TIMEOUT_SECS";
    
    /// Enable performance profiling
    pub const ENABLE_PROFILING: &str = "LINGO_ENABLE_PROFILING";
    
    /// Data directory for storing databases
    pub const DATA_DIR: &str = "LINGO_DATA_DIR";
    
    /// Maximum result nodes (overrides security limit)
    pub const MAX_RESULT_NODES: &str = "LINGO_MAX_RESULT_NODES";
    
    /// Enable debug mode
    pub const DEBUG_MODE: &str = "LINGO_DEBUG";
}

/// Runtime configuration from environment
#[derive(Debug, Clone)]
pub struct EnvConfig {
    /// Database file path
    pub database_path: Option<PathBuf>,
    
    /// Log level
    pub log_level: String,
    
    /// Cache size in MB
    pub cache_size_mb: usize,
    
    /// Query timeout in seconds
    pub query_timeout_secs: u64,
    
    /// Enable profiling
    pub enable_profiling: bool,
    
    /// Data directory
    pub data_dir: PathBuf,
    
    /// Maximum result nodes
    pub max_result_nodes: usize,
    
    /// Debug mode
    pub debug_mode: bool,
}

impl Default for EnvConfig {
    fn default() -> Self {
        Self {
            database_path: None,
            log_level: "info".to_string(),
            cache_size_mb: 100,
            query_timeout_secs: 30,
            enable_profiling: false,
            data_dir: default_data_dir(),
            max_result_nodes: 10_000,
            debug_mode: false,
        }
    }
}

impl EnvConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();
        
        // Database path
        if let Ok(path) = env::var(vars::DATABASE_PATH) {
            config.database_path = Some(PathBuf::from(path));
        }
        
        // Log level
        if let Ok(level) = env::var(vars::LOG_LEVEL) {
            config.log_level = level.to_lowercase();
        }
        
        // Cache size
        if let Ok(size) = env::var(vars::CACHE_SIZE_MB) {
            if let Ok(size_mb) = size.parse::<usize>() {
                config.cache_size_mb = size_mb;
            }
        }
        
        // Query timeout
        if let Ok(timeout) = env::var(vars::QUERY_TIMEOUT_SECS) {
            if let Ok(secs) = timeout.parse::<u64>() {
                config.query_timeout_secs = secs;
            }
        }
        
        // Profiling
        config.enable_profiling = env::var(vars::ENABLE_PROFILING)
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false);
        
        // Data directory
        if let Ok(dir) = env::var(vars::DATA_DIR) {
            config.data_dir = PathBuf::from(dir);
        }
        
        // Max result nodes
        if let Ok(max) = env::var(vars::MAX_RESULT_NODES) {
            if let Ok(max_nodes) = max.parse::<usize>() {
                config.max_result_nodes = max_nodes;
            }
        }
        
        // Debug mode
        config.debug_mode = env::var(vars::DEBUG_MODE)
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false);
        
        config
    }
    
    /// Get the default database path
    pub fn default_database_path(&self) -> PathBuf {
        self.database_path.clone().unwrap_or_else(|| {
            self.data_dir.join("english.lingo")
        })
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate log level
        match self.log_level.as_str() {
            "trace" | "debug" | "info" | "warn" | "error" => {},
            _ => return Err(format!("Invalid log level: {}", self.log_level)),
        }
        
        // Validate cache size
        if self.cache_size_mb == 0 {
            return Err("Cache size must be greater than 0".to_string());
        }
        
        // Validate timeout
        if self.query_timeout_secs == 0 {
            return Err("Query timeout must be greater than 0".to_string());
        }
        
        // Validate max nodes
        if self.max_result_nodes == 0 {
            return Err("Max result nodes must be greater than 0".to_string());
        }
        
        Ok(())
    }
}

/// Get default data directory
fn default_data_dir() -> PathBuf {
    if let Some(data_dir) = dirs::data_dir() {
        data_dir.join("lingo")
    } else {
        PathBuf::from("./data")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_default_config() {
        let config = EnvConfig::default();
        assert_eq!(config.log_level, "info");
        assert_eq!(config.cache_size_mb, 100);
        assert_eq!(config.query_timeout_secs, 30);
        assert!(!config.enable_profiling);
        assert!(!config.debug_mode);
    }
    
    #[test]
    fn test_env_config() {
        // Set test environment variables
        env::set_var(vars::LOG_LEVEL, "debug");
        env::set_var(vars::CACHE_SIZE_MB, "200");
        env::set_var(vars::ENABLE_PROFILING, "true");
        env::set_var(vars::DEBUG_MODE, "1");
        
        let config = EnvConfig::from_env();
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.cache_size_mb, 200);
        assert!(config.enable_profiling);
        assert!(config.debug_mode);
        
        // Clean up
        env::remove_var(vars::LOG_LEVEL);
        env::remove_var(vars::CACHE_SIZE_MB);
        env::remove_var(vars::ENABLE_PROFILING);
        env::remove_var(vars::DEBUG_MODE);
    }
    
    #[test]
    fn test_validation() {
        let mut config = EnvConfig::default();
        assert!(config.validate().is_ok());
        
        config.log_level = "invalid".to_string();
        assert!(config.validate().is_err());
        
        config.log_level = "info".to_string();
        config.cache_size_mb = 0;
        assert!(config.validate().is_err());
    }
}
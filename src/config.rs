//! Configuration and initialization for Lingo

pub mod env;

use crate::storage::{Database, MemoryMappedDatabase};
use crate::core::error::Result;
use std::path::PathBuf;
use self::env::EnvConfig;

/// Lingo configuration
#[derive(Debug, Clone)]
pub struct LingoConfig {
    /// Path to the database file
    pub database_path: PathBuf,
    /// Whether to use the standard English database
    pub use_standard_english: bool,
    /// Whether to allow auto-discovery of morphemes
    pub enable_auto_discovery: bool,
    /// Maximum database size in MB
    pub max_database_size_mb: usize,
}

impl Default for LingoConfig {
    fn default() -> Self {
        let env_config = EnvConfig::from_env();
        Self {
            database_path: env_config.default_database_path(),
            use_standard_english: true,
            enable_auto_discovery: true,
            max_database_size_mb: env_config.cache_size_mb,
        }
    }
}

impl LingoConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        let env_config = EnvConfig::from_env();
        Self {
            database_path: env_config.default_database_path(),
            use_standard_english: env_config.database_path.is_none(),
            enable_auto_discovery: env_config.debug_mode,
            max_database_size_mb: env_config.cache_size_mb,
        }
    }
}

/// Initialize Lingo with configuration
pub struct LingoInit;

impl LingoInit {
    /// Initialize or load a Lingo database
    pub fn initialize(config: &LingoConfig) -> Result<Database> {
        if config.database_path.exists() {
            // Load existing database
            println!("📂 Loading existing database: {:?}", config.database_path);
            Ok(MemoryMappedDatabase::open(&config.database_path)?)
        } else if config.use_standard_english {
            // Look for pre-built English database
            let english_db_path = PathBuf::from("english.lingo");
            if english_db_path.exists() {
                println!("📂 Loading pre-built English database");
                Ok(MemoryMappedDatabase::open(&english_db_path)?)
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Pre-built English database not found. Please provide a database file."
                ).into());
            }
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Database file not found. Please provide a database file."
            ).into());
        }
    }
}

/// Default database paths for different platforms
pub fn default_database_path() -> PathBuf {
    if let Some(data_dir) = dirs::data_dir() {
        data_dir.join("lingo").join("english.lingo")
    } else {
        PathBuf::from("english.lingo")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_initialize_with_missing_file() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.lingo");
        
        let config = LingoConfig {
            database_path: db_path.clone(),
            use_standard_english: false, // Don't try to use standard English
            ..Default::default()
        };
        
        // Should fail because file doesn't exist
        let result = LingoInit::initialize(&config);
        assert!(result.is_err());
    }
}
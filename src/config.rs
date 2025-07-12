//! Configuration and initialization for Lingo

use crate::data::DatabaseSeeder;
use crate::storage::{Database, MemoryMappedDatabase};
use crate::core::error::Result;
use std::path::{Path, PathBuf};

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
        Self {
            database_path: PathBuf::from("lingo.db"),
            use_standard_english: true,
            enable_auto_discovery: true,
            max_database_size_mb: 100,
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
            // Create standard English database
            println!("🌱 Creating standard English database...");
            Self::create_standard_database(&config.database_path)?;
            Ok(MemoryMappedDatabase::open(&config.database_path)?)
        } else {
            // Create empty database
            println!("📝 Creating empty database...");
            Self::create_empty_database(&config.database_path)?;
            Ok(MemoryMappedDatabase::open(&config.database_path)?)
        }
    }
    
    /// Create standard English database
    fn create_standard_database(path: &Path) -> Result<()> {
        let mut seeder = DatabaseSeeder::new();
        seeder.seed_english()?;
        seeder.build(path.to_str().unwrap())?;
        Ok(())
    }
    
    /// Create empty database
    fn create_empty_database(path: &Path) -> Result<()> {
        let mut builder = crate::storage::DatabaseBuilder::new();
        builder.build(path.to_str().unwrap())?;
        Ok(())
    }
    
    /// Clear and reseed database
    pub fn reseed_database(path: &Path, seeder_fn: impl FnOnce(&mut DatabaseSeeder) -> Result<()>) -> Result<()> {
        // Delete existing file
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        
        // Create new seeder and apply custom seeding
        let mut seeder = DatabaseSeeder::new();
        seeder_fn(&mut seeder)?;
        seeder.build(path.to_str().unwrap())?;
        
        Ok(())
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
    fn test_initialize_standard() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.lingo");
        
        let config = LingoConfig {
            database_path: db_path.clone(),
            use_standard_english: true,
            ..Default::default()
        };
        
        let _db = LingoInit::initialize(&config).unwrap();
        assert!(db_path.exists());
    }
}
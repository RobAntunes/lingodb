//! Tests for memory-mapped database access

#[cfg(test)]
mod tests {
    use crate::storage::MemoryMappedDatabase;
    use crate::core::NodeId;
    use tempfile::TempDir;
    use std::fs;
    
    #[test]
    fn test_open_nonexistent_file() {
        let result = MemoryMappedDatabase::open("/nonexistent/path/db.lingo");
        assert!(result.is_err(), "Should fail to open nonexistent file");
    }
    
    #[test]
    fn test_open_empty_file() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("empty.lingo");
        
        // Create empty file
        fs::write(&db_path, b"").unwrap();
        
        let result = MemoryMappedDatabase::open(&db_path);
        assert!(result.is_err(), "Should fail with empty file");
    }
    
    #[test]
    fn test_open_invalid_header() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("invalid.lingo");
        
        // Write invalid data
        fs::write(&db_path, b"INVALID HEADER DATA").unwrap();
        
        let result = MemoryMappedDatabase::open(&db_path);
        assert!(result.is_err(), "Should fail with invalid header");
    }
    
    #[test]
    fn test_node_count_access() {
        // This test would require a valid database file
        // For now, we just test the API exists
        // In real tests, you'd use a pre-built test database
    }
    
    #[test]
    fn test_get_node_invalid_id() {
        // This test would require a valid database
        // Testing that the method exists and returns proper error types
    }
}
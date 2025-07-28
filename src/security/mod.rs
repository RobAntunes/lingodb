//! Security utilities for Lingo
//! 
//! This module provides security-related functionality including:
//! - Path validation to prevent directory traversal attacks
//! - Input sanitization for queries
//! - Size limits to prevent DoS attacks

use std::path::{Path, PathBuf};
use crate::core::error::{LingoError, Result};

/// Maximum allowed file size (100MB)
pub const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024;

/// Maximum query string length (10KB)
pub const MAX_QUERY_LENGTH: usize = 10 * 1024;

/// Maximum number of nodes in a query result
pub const MAX_RESULT_NODES: usize = 10_000;

/// Validates and canonicalizes a file path to prevent directory traversal attacks
/// 
/// # Arguments
/// * `path` - The path to validate
/// * `allowed_dirs` - Optional list of allowed base directories
/// 
/// # Returns
/// * `Ok(PathBuf)` - The canonicalized safe path
/// * `Err(LingoError)` - If the path is invalid or attempts directory traversal
/// 
/// # Example
/// ```
/// use lingo::security::validate_path;
/// 
/// let safe_path = validate_path("../../../etc/passwd", Some(&["/var/data"]))?;
/// // Returns error - path traversal attempt
/// ```
pub fn validate_path<P: AsRef<Path>>(
    path: P,
    allowed_dirs: Option<&[PathBuf]>
) -> Result<PathBuf> {
    let path = path.as_ref();
    
    // Check for suspicious patterns
    let path_str = path.to_string_lossy();
    if path_str.contains("..") || path_str.contains("~") {
        return Err(LingoError::SecurityError {
            message: "Path contains suspicious patterns".to_string()
        });
    }
    
    // Canonicalize the path (resolves symlinks and normalizes)
    let canonical = path.canonicalize()
        .map_err(|e| LingoError::SecurityError {
            message: format!("Failed to canonicalize path: {}", e)
        })?;
    
    // If allowed directories are specified, ensure the path is within them
    if let Some(allowed) = allowed_dirs {
        let is_allowed = allowed.iter().any(|dir| {
            if let Ok(canonical_dir) = dir.canonicalize() {
                canonical.starts_with(&canonical_dir)
            } else {
                false
            }
        });
        
        if !is_allowed {
            return Err(LingoError::SecurityError {
                message: "Path is outside allowed directories".to_string()
            });
        }
    }
    
    // Check file size to prevent loading huge files
    if let Ok(metadata) = std::fs::metadata(&canonical) {
        if metadata.len() > MAX_FILE_SIZE {
            return Err(LingoError::SecurityError {
                message: format!("File size exceeds maximum allowed size of {} MB", 
                    MAX_FILE_SIZE / (1024 * 1024))
            });
        }
    }
    
    Ok(canonical)
}

/// Validates a query string for safety
/// 
/// # Arguments
/// * `query` - The query string to validate
/// 
/// # Returns
/// * `Ok(&str)` - The validated query string
/// * `Err(LingoError)` - If the query is invalid
pub fn validate_query(query: &str) -> Result<&str> {
    // Check length
    if query.len() > MAX_QUERY_LENGTH {
        return Err(LingoError::SecurityError {
            message: format!("Query exceeds maximum length of {} bytes", MAX_QUERY_LENGTH)
        });
    }
    
    // Check for null bytes
    if query.contains('\0') {
        return Err(LingoError::SecurityError {
            message: "Query contains null bytes".to_string()
        });
    }
    
    // Check for control characters
    if query.chars().any(|c| c.is_control() && c != '\n' && c != '\r' && c != '\t') {
        return Err(LingoError::SecurityError {
            message: "Query contains invalid control characters".to_string()
        });
    }
    
    Ok(query)
}

/// Validates a node limit value
pub fn validate_limit(limit: usize) -> Result<usize> {
    if limit == 0 {
        return Err(LingoError::SecurityError {
            message: "Limit must be greater than 0".to_string()
        });
    }
    
    if limit > MAX_RESULT_NODES {
        return Err(LingoError::SecurityError {
            message: format!("Limit exceeds maximum of {} nodes", MAX_RESULT_NODES)
        });
    }
    
    Ok(limit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_validate_path_traversal() {
        // Test directory traversal attempts
        assert!(validate_path("../../../etc/passwd", None).is_err());
        assert!(validate_path("./../../sensitive.db", None).is_err());
        assert!(validate_path("~/.ssh/id_rsa", None).is_err());
    }
    
    #[test]
    fn test_validate_path_allowed_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let allowed = vec![temp_dir.path().to_path_buf()];
        
        // Create a test file
        let test_file = temp_dir.path().join("test.lingo");
        std::fs::write(&test_file, b"test").unwrap();
        
        // Should succeed for files within allowed directory
        assert!(validate_path(&test_file, Some(&allowed)).is_ok());
        
        // Should fail for files outside allowed directory
        assert!(validate_path("/etc/passwd", Some(&allowed)).is_err());
    }
    
    #[test]
    fn test_validate_query() {
        // Valid queries
        assert!(validate_query("SELECT * FROM nodes").is_ok());
        assert!(validate_query("technical").is_ok());
        
        // Invalid queries
        assert!(validate_query("query\0with\0nulls").is_err());
        assert!(validate_query(&"x".repeat(MAX_QUERY_LENGTH + 1)).is_err());
    }
    
    #[test]
    fn test_validate_limit() {
        assert!(validate_limit(10).is_ok());
        assert!(validate_limit(MAX_RESULT_NODES).is_ok());
        assert!(validate_limit(0).is_err());
        assert!(validate_limit(MAX_RESULT_NODES + 1).is_err());
    }
}
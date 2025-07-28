//! Error types for the Lingo database

use crate::core::NodeId;
use std::io;
use thiserror::Error;

/// Main error type for Lingo database operations
#[derive(Debug, Error)]
pub enum LingoError {
    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    
    /// Database format error
    #[error("Invalid database format: {0}")]
    InvalidFormat(String),
    
    /// Version mismatch
    #[error("Unsupported database version: {major}.{minor}")]
    UnsupportedVersion { major: u16, minor: u16 },
    
    /// Corrupted data
    #[error("Corrupted database: {0}")]
    CorruptedData(String),
    
    /// Node not found
    #[error("Node not found: {0}")]
    NodeNotFound(NodeId),
    
    /// Word not found
    #[error("Word not found: {0}")]
    WordNotFound(String),
    
    /// Invalid UTF-8 in string table
    #[error("Invalid UTF-8 in string table")]
    InvalidUtf8,
    
    /// Index out of bounds
    #[error("Index out of bounds: {index} >= {max}")]
    IndexOutOfBounds { index: usize, max: usize },
    
    /// Checksum mismatch
    #[error("Checksum mismatch in {section}")]
    ChecksumMismatch { section: String },
    
    /// Query compilation error
    #[error("Query compilation error: {0}")]
    QueryCompilation(String),
    
    /// Execution error
    #[error("Query execution error: {0}")]
    Execution(String),
    
    /// Memory allocation error
    #[error("Memory allocation failed: {0}")]
    MemoryAllocation(String),
    
    /// Feature not implemented
    #[error("Feature not implemented: {0}")]
    NotImplemented(String),
    
    /// Build error
    #[error("Build error: {0}")]
    Build(#[from] BuildError),
    
    /// File format error
    #[error("File format error: {0}")]
    FileFormat(String),
    
    /// Database error
    #[error("Database error: {0}")]
    Database(String),
    
    /// Invalid node ID
    #[error("Invalid node ID: {0:?}")]
    InvalidNodeId(NodeId),
    
    /// Security error
    #[error("Security error: {message}")]
    SecurityError { message: String },
}

/// Result type alias for Lingo operations
pub type Result<T> = std::result::Result<T, LingoError>;

/// Query-specific errors
#[derive(Debug, Error)]
pub enum QueryError {
    /// Empty stack during execution
    #[error("Empty stack")]
    EmptyStack,
    
    /// Stack overflow
    #[error("Stack overflow")]
    StackOverflow,
    
    /// Invalid operation
    #[error("Invalid operation: {0:?}")]
    InvalidOperation(crate::core::bytecode::SlangOp),
    
    /// Missing required index
    #[error("Required index not available: {0}")]
    MissingIndex(String),
    
    /// Invalid operand
    #[error("Invalid operand for operation")]
    InvalidOperand,
}

/// Database building errors
#[derive(Debug, Error)]
pub enum BuildError {
    /// Duplicate node ID
    #[error("Duplicate node ID: {0}")]
    DuplicateNode(NodeId),
    
    /// Invalid connection
    #[error("Invalid connection: {reason}")]
    InvalidConnection { reason: String },
    
    /// String too long
    #[error("String too long: {length} > {max}")]
    StringTooLong { length: usize, max: usize },
    
    /// Too many nodes
    #[error("Too many nodes: {count} > {max}")]
    TooManyNodes { count: usize, max: usize },
    
    /// Invalid coordinate
    #[error("Invalid coordinate: {reason}")]
    InvalidCoordinate { reason: String },
}
//! Storage layer for the Lingo database.
//!
//! This module provides the core storage functionality for Lingo databases,
//! including file format definitions, memory-mapped access, and database building.
//! The storage layer is designed for efficiency, with zero-copy reads and
//! compact data structures optimized for mobile and embedded deployments.
//!
//! # Architecture
//!
//! The storage system consists of several components:
//!
//! - **File Format**: Binary format with header, nodes, connections, and indices
//! - **Memory Mapping**: Zero-copy access through OS memory mapping
//! - **String Table**: Deduplicated string storage with variable-length encoding
//! - **Database Builder**: API for constructing new databases
//!
//! # File Format
//!
//! Lingo databases use a custom binary format optimized for read performance:
//!
//! ```text
//! [Header]
//! [Node Data]         // Array of 60-byte nodes
//! [Connections]       // Orthogonal connections
//! [String Table]      // Deduplicated strings
//! [Spatial Index]     // Octree for 3D queries
//! [Vertical Index]    // Layer traversal index
//! ```
//!
//! # Examples
//!
//! ## Reading a database
//!
//! ```rust,no_run
//! use lingo::storage::Database;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let db = Database::open("english.lingo")?;
//! println!("Database has {} nodes", db.node_count());
//! # Ok(())
//! # }
//! ```
//!
//! ## Building a database
//!
//! ```rust,no_run
//! use lingo::storage::DatabaseBuilder;
//! use lingo::core::{LinguisticNode, NodeId, Layer, Coordinate3D};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut builder = DatabaseBuilder::new();
//!
//! // Add nodes
//! let node = LinguisticNode::new(
//!     NodeId(1),
//!     Layer::Words,
//!     Coordinate3D::new(0.5, 0.3, 0.55)
//! );
//! builder.add_node(node)?;
//!
//! // Build and save
//! builder.build("output.lingo")?;
//! # Ok(())
//! # }
//! ```

pub mod file_format;
mod mmap;
mod string_table;
mod builder;

pub use file_format::{LingoFileHeader, FileFormatFlags};
pub use mmap::MemoryMappedDatabase;
pub use string_table::StringTable;
pub use builder::DatabaseBuilder;

// Re-export commonly used types
pub use self::mmap::Database;

// Re-export for plugins module
pub use self::mmap::MemoryMappedDatabase as LingoDatabase;
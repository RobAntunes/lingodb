//! Query execution engine for the Lingo database.
//!
//! This module provides the SLANG bytecode interpreter that executes
//! compiled queries against a Lingo database. The engine is designed
//! for high performance with a stack-based virtual machine architecture.
//!
//! # Architecture
//!
//! The execution engine consists of:
//!
//! - **LingoExecutor**: The main bytecode interpreter
//! - **NodeSet**: Efficient result set management
//! - **Execution Stack**: Intermediate result storage
//! - **Registers**: 16 general-purpose storage locations
//!
//! # Performance Features
//!
//! - Stack-based VM for efficient execution
//! - Zero-copy access to database structures
//! - Automatic result deduplication
//! - Execution metrics tracking
//!
//! # Examples
//!
//! ## Basic query execution
//!
//! ```rust,no_run
//! use lingo::{LingoExecutor, QueryBuilder};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut executor = LingoExecutor::new();
//! executor.load_database("english.lingo")?;
//!
//! let query = QueryBuilder::find("example")
//!     .similar()
//!     .limit(10)
//!     .compile();
//!
//! let result = executor.execute(&query)?;
//! println!("Found {} results in {:?}", 
//!     result.nodes.len(), 
//!     result.execution_time
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Batch query processing
//!
//! ```rust,no_run
//! use lingo::{LingoExecutor, QueryBuilder};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut executor = LingoExecutor::new();
//! executor.load_database("data.lingo")?;
//!
//! // Reuse executor for multiple queries
//! let words = vec!["happy", "sad", "excited"];
//! for word in words {
//!     let query = QueryBuilder::find(word)
//!         .similar_threshold(0.75)
//!         .compile();
//!     
//!     let result = executor.execute(&query)?;
//!     println!("{}: {} similar words", word, result.nodes.len());
//! }
//! # Ok(())
//! # }
//! ```

pub mod executor;

pub use executor::{LingoExecutor, QueryResult, NodeSet, ExecutionStats};
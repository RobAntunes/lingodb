//! Query building and compilation for the Lingo database.
//!
//! This module provides a fluent API for constructing linguistic queries
//! that compile to optimized SLANG bytecode. The query system enables
//! complex navigation through the 3D linguistic space with operations
//! for similarity search, layer traversal, and relationship following.
//!
//! # Architecture
//!
//! The query system has two main components:
//!
//! 1. **Query Builder**: Fluent API for constructing queries
//! 2. **Query Compiler**: Converts operations to SLANG bytecode
//!
//! # Query Operations
//!
//! - **Loading**: Start with a word or node ID
//! - **Similarity**: Find semantically similar nodes
//! - **Navigation**: Move between linguistic layers
//! - **Connections**: Follow typed relationships
//! - **Filtering**: Refine results by properties
//!
//! # Examples
//!
//! ## Finding similar words
//!
//! ```rust
//! use lingo::query::QueryBuilder;
//!
//! let query = QueryBuilder::find("happy")
//!     .similar_threshold(0.85)
//!     .limit(10)
//!     .compile();
//! ```
//!
//! ## Exploring concept hierarchies
//!
//! ```rust
//! use lingo::query::QueryBuilder;
//!
//! let query = QueryBuilder::find("algorithm")
//!     .layer_up_n(2)      // To concepts
//!     .follow_connection() // Related concept
//!     .layer_down_n(2)    // Back to words
//!     .compile();
//! ```
//!
//! ## Cross-linguistic connections
//!
//! ```rust
//! use lingo::query::{QueryBuilder, FilterCriteria};
//! use lingo::core::node::EtymologyOrigin;
//!
//! let query = QueryBuilder::find("democracy")
//!     .similar()
//!     .filter(FilterCriteria::Etymology(EtymologyOrigin::Greek))
//!     .compile();
//! ```

mod builder;

pub use builder::{QueryBuilder, CompiledQuery, Operation, FilterCriteria, SortCriteria};
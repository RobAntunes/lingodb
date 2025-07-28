// Copyright 2025 Roberto Antunes
//
// Licensed under the Functional Source License, Version 1.1 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://github.com/RobAntunes/lingodb/blob/main/LICENSE
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Lingo Database
//! 
//! Revolutionary 3D spatial linguistic database with orthogonal connections.
//! 
//! Lingo is a high-performance linguistic database that represents language as a 3D spatial structure,
//! enabling novel approaches to natural language processing, semantic search, and linguistic analysis.
//! The database organizes linguistic elements across seven hierarchical layers, from individual letters
//! to abstract domains, with rich cross-layer connections that capture morphological, semantic, and
//! etymological relationships.
//! 
//! ## Features
//! 
//! - **7-Layer Linguistic Hierarchy**: Organizes language from letters to domains
//! - **3D Spatial Indexing**: Uses octree structures for efficient spatial queries
//! - **Cross-Domain Orthogonal Connections**: Rich relationship modeling between concepts
//! - **SLANG Bytecode**: Ahead-of-time query compilation for optimal performance
//! - **Single-File Database**: Optimized format for mobile and embedded deployment
//! - **Runtime Learning**: Dynamic discovery of analogies and relationships
//! 
//! ## Quick Start
//! 
//! ```rust,no_run
//! use lingo::{QueryBuilder, LingoExecutor};
//! 
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create an executor and load a database
//! let mut executor = LingoExecutor::new();
//! executor.load_database("english.lingo")?;
//! 
//! // Build and execute a query
//! let query = QueryBuilder::find("technical")
//!     .similar_threshold(0.8)
//!     .layer_up()
//!     .limit(10)
//!     .compile();
//! 
//! let result = executor.execute(&query)?;
//! println!("Found {} related concepts", result.nodes.len());
//! # Ok(())
//! # }
//! ```
//! 
//! ## Core Concepts
//! 
//! ### Linguistic Layers
//! 
//! The database organizes language into seven distinct layers:
//! 
//! 1. **Letters** (Layer 0): Individual characters and digraphs
//! 2. **Phonemes** (Layer 1): Sound units and pronunciation patterns
//! 3. **Morphemes** (Layer 2): Meaningful word parts (roots, prefixes, suffixes)
//! 4. **Words** (Layer 3): Complete lexical units
//! 5. **Phrases** (Layer 4): Multi-word expressions and idioms
//! 6. **Concepts** (Layer 5): Abstract semantic representations
//! 7. **Domains** (Layer 6): High-level knowledge categories
//! 
//! ### Spatial Organization
//! 
//! Each linguistic node exists at a specific 3D coordinate:
//! - **X-axis**: Semantic similarity within a layer
//! - **Y-axis**: Etymology and language origin
//! - **Z-axis**: Linguistic layer (0.0 to 1.0)
//! 
//! ### Orthogonal Connections
//! 
//! Nodes are connected through typed relationships:
//! - **Hypernymy/Hyponymy**: Parent-child semantic relationships
//! - **Meronymy**: Part-whole relationships
//! - **Analogy**: Structural or functional similarities
//! - **Etymology**: Historical linguistic connections
//! - **Morphological**: Word formation relationships
//! 
//! ## Architecture
//! 
//! The crate is organized into several modules:
//! 
//! - [`core`]: Core data structures and types
//! - [`storage`]: Database storage and file format
//! - [`query`]: Query building and compilation
//! - [`engine`]: Query execution engine
//! - [`index`]: Spatial and vertical indexing
//! - [`discovery`]: Runtime learning and analogy discovery
//! 
//! ## Performance Considerations
//! 
//! - **Memory-Mapped Access**: Zero-copy reads for optimal performance
//! - **Packed Structures**: 60-byte nodes for cache efficiency
//! - **Spatial Indexing**: O(log n) lookups using octree structures
//! - **Query Compilation**: Queries compile to efficient bytecode
//! 
//! ## Examples
//! 
//! ### Finding Similar Words
//! 
//! ```rust,no_run
//! # use lingo::{QueryBuilder, LingoExecutor};
//! # let mut executor = LingoExecutor::new();
//! let query = QueryBuilder::find("viral")
//!     .similar_threshold(0.85)
//!     .limit(5)
//!     .compile();
//! ```
//! 
//! ### Exploring Concept Hierarchies
//! 
//! ```rust,no_run
//! # use lingo::{QueryBuilder, LingoExecutor};
//! # let mut executor = LingoExecutor::new();
//! let query = QueryBuilder::find("computer")
//!     .layer_up_n(2)  // Move to concept layer
//!     .follow_connection()  // Follow strongest connection
//!     .layer_down()  // Back to words
//!     .limit(20)
//!     .compile();
//! ```
//! 
//! ### Cross-Domain Connections
//! 
//! ```rust,no_run
//! # use lingo::{QueryBuilder, LingoExecutor, core::ConnectionType};
//! # let mut executor = LingoExecutor::new();
//! let query = QueryBuilder::find("metaphor")
//!     .follow_connection_type(ConnectionType::Analogy)
//!     .spatial_neighbors(0.3)
//!     .compile();
//! ```

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

pub mod core;
pub mod storage;
pub mod index;
pub mod query;
pub mod engine;
pub mod discovery;
pub mod data;
pub mod config;
pub mod ffi;
pub mod plugins;
pub mod morphology;
pub mod security;
pub mod logging;
// pub mod mirroring; // Temporarily disabled due to compilation issues

// WebAssembly bindings (only compiled for WASM target)
#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub use crate::core::{LinguisticNode, OrthogonalConnection, Coordinate3D};
pub use crate::query::QueryBuilder;
pub use crate::storage::Database;
pub use crate::engine::LingoExecutor;
pub use crate::plugins::{PluginPipeline, Plugin, FunctionExtractor};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Maximum supported file format version
pub const MAX_FILE_VERSION: (u16, u16) = (1, 0);
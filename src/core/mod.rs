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

//! Core data structures and types for the Lingo database.
//!
//! This module contains the fundamental building blocks of the Lingo system:
//! 
//! - **Nodes**: Linguistic elements with spatial positions and properties
//! - **Connections**: Typed relationships between nodes
//! - **Coordinates**: 3D spatial positioning system
//! - **Types**: Common type definitions and identifiers
//! - **Bytecode**: SLANG virtual machine instructions
//! - **Errors**: Error types and result definitions
//!
//! # Key Concepts
//!
//! ## Linguistic Nodes
//!
//! Each [`LinguisticNode`] represents a single linguistic element (letter, word, concept, etc.)
//! positioned in 3D space. Nodes are packed into exactly 60 bytes for cache efficiency.
//!
//! ## Spatial Organization
//!
//! The 3D coordinate system maps linguistic properties to spatial dimensions:
//! - X-axis: Semantic similarity
//! - Y-axis: Etymology/origin
//! - Z-axis: Linguistic layer (0.0-1.0)
//!
//! ## Orthogonal Connections
//!
//! [`OrthogonalConnection`]s link nodes across layers and domains, capturing
//! relationships like hypernymy, meronymy, etymology, and analogy.
//!
//! # Examples
//!
//! ```rust
//! use lingo::core::{LinguisticNode, NodeId, Layer, Coordinate3D};
//!
//! // Create a word node
//! let node = LinguisticNode::new(
//!     NodeId(100),
//!     Layer::Words,
//!     Coordinate3D::new(0.5, 0.3, 0.55)
//! );
//!
//! assert_eq!(node.layer, Layer::Words);
//! ```

mod node;
mod connection;
mod coordinate;
mod types;

pub use node::{LinguisticNode, NodeFlags, Layer, EtymologyOrigin, MorphemeType};
pub use connection::{OrthogonalConnection, ConnectionType, ContextMask};
pub use coordinate::{Coordinate3D, BoundingBox3D};
pub use types::{NodeId, PhonemeId, Vector3D};

/// SLANG bytecode operations
pub mod bytecode;

/// Error types
pub mod error;

#[cfg(test)]
mod tests;
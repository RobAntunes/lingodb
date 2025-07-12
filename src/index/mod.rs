//! Indexing structures for efficient queries

pub mod octree;
pub mod vertical;

pub use octree::{SpatialIndex, OctreeBuilder, OctreeNode};
pub use vertical::VerticalIndex;
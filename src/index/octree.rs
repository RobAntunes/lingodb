//! Octree spatial indexing for 3D linguistic space
//! 
//! This module implements a space-partitioning octree that divides
//! the 3D coordinate space into eight octants recursively, enabling
//! O(log n) spatial queries for finding linguistic neighbors.

use crate::core::{Coordinate3D, BoundingBox3D, NodeId};
use std::collections::HashMap;

/// Maximum depth of the octree (limits subdivision)
const MAX_OCTREE_DEPTH: u8 = 10;

/// Maximum nodes per octree leaf before subdivision
const MAX_NODES_PER_LEAF: usize = 16;

/// Octree node for spatial partitioning (64 bytes)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OctreeNode {
    /// Spatial boundaries of this node
    pub bounds: BoundingBox3D,
    /// Child node indices (0 = null)
    /// Order: -x-y-z, +x-y-z, -x+y-z, +x+y-z, -x-y+z, +x-y+z, -x+y+z, +x+y+z
    pub children: [u32; 8],
    /// Number of linguistic nodes in this octant
    pub node_count: u16,
    /// Offset to node array (for leaf nodes)
    pub node_offset: u32,
    /// Tree depth at this node
    pub depth: u8,
    /// Node flags
    pub flags: OctreeFlags,
}

bitflags::bitflags! {
    /// Flags for octree nodes
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct OctreeFlags: u8 {
        /// This is a leaf node
        const IS_LEAF = 0b00000001;
        /// Node has been optimized
        const OPTIMIZED = 0b00000010;
        /// Node contains cross-layer connections
        const CROSS_LAYER = 0b00000100;
    }
}

/// Octree builder for constructing the spatial index
#[derive(Debug)]
pub struct OctreeBuilder {
    /// All octree nodes
    nodes: Vec<OctreeNode>,
    /// Node buckets (leaf node -> contained node IDs)
    node_buckets: HashMap<u32, Vec<NodeId>>,
    /// Node positions for building
    positions: HashMap<NodeId, Coordinate3D>,
}

impl OctreeBuilder {
    /// Create a new octree builder
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            node_buckets: HashMap::new(),
            positions: HashMap::new(),
        }
    }
    
    /// Add a node to be indexed
    pub fn add_node(&mut self, node_id: NodeId, position: Coordinate3D) {
        self.positions.insert(node_id, position);
    }
    
    /// Build the octree
    pub fn build(mut self) -> SpatialIndex {
        // Create root node covering entire space
        let root_bounds = BoundingBox3D::new(
            Coordinate3D::zero(),
            Coordinate3D::new(1.0, 1.0, 1.0),
        );
        
        // Collect all nodes
        let all_nodes: Vec<(NodeId, Coordinate3D)> = 
            self.positions.iter().map(|(&id, &pos)| (id, pos)).collect();
        
        // Build tree recursively
        let root_index = self.build_node(root_bounds, &all_nodes, 0);
        
        SpatialIndex {
            root_index,
            nodes: self.nodes,
            node_buckets: self.node_buckets,
        }
    }
    
    /// Recursively build an octree node
    fn build_node(
        &mut self,
        bounds: BoundingBox3D,
        nodes: &[(NodeId, Coordinate3D)],
        depth: u8,
    ) -> u32 {
        let node_index = self.nodes.len() as u32;
        
        // Check if we should create a leaf node
        if nodes.len() <= MAX_NODES_PER_LEAF || depth >= MAX_OCTREE_DEPTH {
            // Create leaf node
            let octree_node = OctreeNode {
                bounds,
                children: [0; 8],
                node_count: nodes.len() as u16,
                node_offset: node_index, // Will be used as bucket key
                depth,
                flags: OctreeFlags::IS_LEAF,
            };
            
            // Store node IDs in bucket
            let node_ids: Vec<NodeId> = nodes.iter().map(|(id, _)| *id).collect();
            self.node_buckets.insert(node_index, node_ids);
            
            self.nodes.push(octree_node);
            return node_index;
        }
        
        // Subdivide space into 8 octants
        let center = bounds.center();
        let mut octant_nodes: [Vec<(NodeId, Coordinate3D)>; 8] = Default::default();
        
        // Distribute nodes into octants
        for &(node_id, pos) in nodes {
            let octant = Self::get_octant(center, pos);
            octant_nodes[octant].push((node_id, pos));
        }
        
        // Create internal node
        let octree_node = OctreeNode {
            bounds,
            children: [0; 8],
            node_count: nodes.len() as u16,
            node_offset: 0,
            depth,
            flags: OctreeFlags::empty(),
        };
        
        self.nodes.push(octree_node);
        
        // Recursively build child nodes
        for (i, child_nodes) in octant_nodes.into_iter().enumerate() {
            if !child_nodes.is_empty() {
                let child_bounds = Self::get_octant_bounds(bounds, i);
                let child_index = self.build_node(child_bounds, &child_nodes, depth + 1);
                self.nodes[node_index as usize].children[i] = child_index;
            }
        }
        
        node_index
    }
    
    /// Determine which octant a point belongs to
    fn get_octant(center: Coordinate3D, point: Coordinate3D) -> usize {
        let mut octant = 0;
        if point.x >= center.x { octant |= 1; }
        if point.y >= center.y { octant |= 2; }
        if point.z >= center.z { octant |= 4; }
        octant
    }
    
    /// Get the bounds of a specific octant
    fn get_octant_bounds(parent: BoundingBox3D, octant: usize) -> BoundingBox3D {
        let center = parent.center();
        let min = parent.min;
        let max = parent.max;
        
        let new_min = Coordinate3D::new(
            if octant & 1 == 0 { min.x } else { center.x },
            if octant & 2 == 0 { min.y } else { center.y },
            if octant & 4 == 0 { min.z } else { center.z },
        );
        
        let new_max = Coordinate3D::new(
            if octant & 1 == 0 { center.x } else { max.x },
            if octant & 2 == 0 { center.y } else { max.y },
            if octant & 4 == 0 { center.z } else { max.z },
        );
        
        BoundingBox3D::new(new_min, new_max)
    }
}

/// Spatial index using octree structure
#[derive(Debug)]
pub struct SpatialIndex {
    /// Root node index
    root_index: u32,
    /// All octree nodes
    nodes: Vec<OctreeNode>,
    /// Node buckets for leaf nodes
    node_buckets: HashMap<u32, Vec<NodeId>>,
}

impl SpatialIndex {
    /// Create a new empty spatial index
    pub fn new() -> Self {
        Self {
            root_index: 0,
            nodes: Vec::new(),
            node_buckets: HashMap::new(),
        }
    }
    
    /// Get the octree nodes for serialization
    pub fn nodes(&self) -> &[OctreeNode] {
        &self.nodes
    }
    
    /// Get the root index
    pub fn root_index(&self) -> u32 {
        self.root_index
    }
    
    /// Find all nodes within a given radius of a center point
    pub fn find_within_radius(&self, center: Coordinate3D, radius: f32) -> Vec<NodeId> {
        let mut results = Vec::new();
        
        if self.nodes.is_empty() {
            return results;
        }
        
        let search_bounds = BoundingBox3D::from_center_radius(center, radius);
        self.search_recursive(self.root_index, center, radius, search_bounds, &mut results);
        
        results
    }
    
    /// Find K nearest neighbors to a point
    pub fn find_k_nearest(&self, center: Coordinate3D, k: usize) -> Vec<(NodeId, f32)> {
        // Start with a small radius and expand as needed
        let mut radius = 0.1;
        let mut results = Vec::new();
        
        while results.len() < k && radius <= 2.0 {
            results.clear();
            let candidates = self.find_within_radius(center, radius);
            
            // Calculate distances and sort
            for node_id in candidates {
                // Note: In real implementation, we'd need access to actual positions
                // This is a placeholder
                results.push((node_id, 0.0));
            }
            
            radius *= 2.0;
        }
        
        // Sort by distance and take top K
        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        results.truncate(k);
        
        results
    }
    
    /// Find neighbors within threshold distance
    pub fn find_neighbors(&self, center: Coordinate3D, threshold: f32) -> Vec<NodeId> {
        self.find_within_radius(center, threshold)
    }
    
    /// Recursive search through octree
    fn search_recursive(
        &self,
        node_index: u32,
        center: Coordinate3D,
        radius: f32,
        search_bounds: BoundingBox3D,
        results: &mut Vec<NodeId>,
    ) {
        let node = &self.nodes[node_index as usize];
        
        // Check if node bounds intersect search sphere
        if !node.bounds.intersects_sphere(center, radius) {
            return;
        }
        
        // If this is a leaf node, check all contained nodes
        if node.flags.contains(OctreeFlags::IS_LEAF) {
            if let Some(node_ids) = self.node_buckets.get(&node_index) {
                // In real implementation, we'd check actual distances here
                results.extend(node_ids);
            }
            return;
        }
        
        // Recursively search children
        for &child_index in &node.children {
            if child_index != 0 {
                self.search_recursive(child_index, center, radius, search_bounds, results);
            }
        }
    }
    
    /// Get statistics about the octree
    pub fn stats(&self) -> OctreeStats {
        let mut stats = OctreeStats::default();
        
        if !self.nodes.is_empty() {
            self.gather_stats(self.root_index, &mut stats);
        }
        
        stats
    }
    
    fn gather_stats(&self, node_index: u32, stats: &mut OctreeStats) {
        let node = &self.nodes[node_index as usize];
        
        stats.total_nodes += 1;
        stats.max_depth = stats.max_depth.max(node.depth);
        
        if node.flags.contains(OctreeFlags::IS_LEAF) {
            stats.leaf_nodes += 1;
            stats.total_items += node.node_count as usize;
            stats.max_items_per_leaf = stats.max_items_per_leaf.max(node.node_count as usize);
        } else {
            stats.internal_nodes += 1;
            for &child_index in &node.children {
                if child_index != 0 {
                    self.gather_stats(child_index, stats);
                }
            }
        }
    }
}

/// Statistics about the octree structure
#[derive(Debug, Default)]
pub struct OctreeStats {
    /// Total number of octree nodes
    pub total_nodes: usize,
    /// Number of internal nodes
    pub internal_nodes: usize,
    /// Number of leaf nodes
    pub leaf_nodes: usize,
    /// Maximum tree depth
    pub max_depth: u8,
    /// Total items indexed
    pub total_items: usize,
    /// Maximum items in a single leaf
    pub max_items_per_leaf: usize,
}

impl Default for SpatialIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_octree_construction() {
        let mut builder = OctreeBuilder::new();
        
        // Add some test nodes
        builder.add_node(NodeId(1), Coordinate3D::new(0.1, 0.1, 0.1));
        builder.add_node(NodeId(2), Coordinate3D::new(0.9, 0.9, 0.9));
        builder.add_node(NodeId(3), Coordinate3D::new(0.5, 0.5, 0.5));
        
        let index = builder.build();
        let stats = index.stats();
        
        assert!(stats.total_nodes > 0);
        assert_eq!(stats.total_items, 3);
    }
    
    #[test]
    fn test_octant_calculation() {
        let center = Coordinate3D::new(0.5, 0.5, 0.5);
        
        assert_eq!(OctreeBuilder::get_octant(center, Coordinate3D::new(0.0, 0.0, 0.0)), 0);
        assert_eq!(OctreeBuilder::get_octant(center, Coordinate3D::new(1.0, 0.0, 0.0)), 1);
        assert_eq!(OctreeBuilder::get_octant(center, Coordinate3D::new(0.0, 1.0, 0.0)), 2);
        assert_eq!(OctreeBuilder::get_octant(center, Coordinate3D::new(1.0, 1.0, 1.0)), 7);
    }
}
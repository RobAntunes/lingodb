//! Memory-mapped database access with zero-copy reads

use crate::core::{
    NodeId, LinguisticNode, OrthogonalConnection, Coordinate3D,
    error::{LingoError, Result},
};
use crate::storage::{LingoFileHeader, StringTable};
use crate::security::validate_path;
use crate::logging::{debug, info, warn, trace};
// TODO: Add octree header when fully implemented
// use crate::index::octree::OctreeHeader;
use memmap2::{Mmap, MmapOptions};
use std::fs::File;
use std::path::Path;
use std::slice;
use std::mem;

/// Section offsets within the database
#[derive(Debug, Clone, Copy)]
struct SectionOffsets {
    /// Start of nodes section
    nodes_start: usize,
    /// Start of connections section  
    connections_start: usize,
    /// Start of string table
    strings_start: usize,
    /// Start of octree index
    octree_start: usize,
    /// End of file
    file_end: usize,
}

/// Memory-mapped Lingo database for efficient zero-copy access
pub struct MemoryMappedDatabase {
    /// Memory-mapped file
    mmap: Mmap,
    /// Section offsets
    offsets: SectionOffsets,
    /// File header (cached)
    header: LingoFileHeader,
}

impl MemoryMappedDatabase {
    /// Open a Lingo database file
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().display().to_string();
        info!(path = %path_str, "Opening Lingo database");
        
        // Validate path for security
        let safe_path = validate_path(path.as_ref(), None)?;
        debug!(validated_path = ?safe_path, "Path validated");
        
        // Open file
        let file = File::open(&safe_path)
            .map_err(|e| {
                warn!(path = %path_str, error = %e, "Failed to open database file");
                LingoError::Io(e)
            })?;
            
        // Memory map the file
        let mmap = unsafe {
            MmapOptions::new()
                .map(&file)
                .map_err(|e| LingoError::Io(e))?
        };
        
        // Validate minimum size
        if mmap.len() < mem::size_of::<LingoFileHeader>() {
            return Err(LingoError::FileFormat("File too small".to_string()));
        }
        
        // Read and validate header
        let header = Self::read_header(&mmap)?;
        
        // Calculate section offsets
        let offsets = Self::calculate_offsets(&header, mmap.len())?;
        
        Ok(Self {
            mmap,
            offsets,
            header,
        })
    }
    
    /// Read and validate file header
    fn read_header(mmap: &Mmap) -> Result<LingoFileHeader> {
        // SAFETY: We've already validated the size
        let header_bytes = &mmap[..mem::size_of::<LingoFileHeader>()];
        let header: LingoFileHeader = unsafe {
            std::ptr::read_unaligned(header_bytes.as_ptr() as *const LingoFileHeader)
        };
        
        // Validate magic number
        if &header.magic != b"LINGO1.0" {
            return Err(LingoError::FileFormat("Invalid magic number".to_string()));
        }
        
        // Validate version
        if header.version_major != 1 || header.version_minor != 0 {
            return Err(LingoError::UnsupportedVersion {
                major: header.version_major,
                minor: header.version_minor,
            });
        }
        
        Ok(header)
    }
    
    /// Calculate section offsets
    fn calculate_offsets(header: &LingoFileHeader, file_size: usize) -> Result<SectionOffsets> {
        // Use the actual offsets from the header
        let nodes_start = header.node_array_offset as usize;
        let connections_start = header.connection_array_offset as usize;
        let strings_start = header.string_table_offset as usize;
        let octree_start = header.octree_offset as usize;
        
        // Validate offsets
        if nodes_start + header.node_array_size as usize > file_size {
            return Err(LingoError::FileFormat(
                format!("Node array extends past file end")
            ));
        }
        
        if connections_start + header.connection_array_size as usize > file_size {
            return Err(LingoError::FileFormat(
                format!("Connection array extends past file end")
            ));
        }
        
        if strings_start + header.string_table_size as usize > file_size {
            return Err(LingoError::FileFormat(
                format!("String table extends past file end")
            ));
        }
        
        Ok(SectionOffsets {
            nodes_start,
            connections_start,
            strings_start,
            octree_start,
            file_end: file_size,
        })
    }
    
    /// Get file header
    pub fn header(&self) -> &LingoFileHeader {
        &self.header
    }
    
    /// Get total number of nodes
    pub fn node_count(&self) -> usize {
        self.header.node_count as usize
    }
    
    /// Get total number of connections
    pub fn connection_count(&self) -> usize {
        self.header.connection_count as usize
    }
    
    /// Get a node by ID
    pub fn get_node(&self, id: NodeId) -> Result<&LinguisticNode> {
        if id.0 == 0 {
            return Err(LingoError::InvalidNodeId(id));
        }
        let index = (id.0 - 1) as usize; // Node IDs start from 1
        if index >= self.node_count() {
            return Err(LingoError::InvalidNodeId(id));
        }
        
        // Calculate offset
        let offset = self.offsets.nodes_start + index * mem::size_of::<LinguisticNode>();
        
        // SAFETY: We've validated the bounds
        unsafe {
            let ptr = self.mmap[offset..].as_ptr() as *const LinguisticNode;
            Ok(&*ptr)
        }
    }
    
    /// Get all nodes as a slice
    pub fn nodes(&self) -> &[LinguisticNode] {
        let count = self.node_count();
        if count == 0 {
            return &[];
        }
        
        // SAFETY: We've validated the bounds during construction
        unsafe {
            slice::from_raw_parts(
                self.mmap[self.offsets.nodes_start..].as_ptr() as *const LinguisticNode,
                count
            )
        }
    }
    
    /// Get a connection by index
    pub fn get_connection(&self, index: usize) -> Result<&OrthogonalConnection> {
        if index >= self.connection_count() {
            return Err(LingoError::Database("Connection index out of bounds".to_string()));
        }
        
        // Calculate offset
        let offset = self.offsets.connections_start + index * mem::size_of::<OrthogonalConnection>();
        
        // SAFETY: We've validated the bounds
        unsafe {
            let ptr = self.mmap[offset..].as_ptr() as *const OrthogonalConnection;
            Ok(&*ptr)
        }
    }
    
    /// Get all connections as a slice
    pub fn connections(&self) -> &[OrthogonalConnection] {
        let count = self.connection_count();
        if count == 0 {
            return &[];
        }
        
        // SAFETY: We've validated the bounds during construction
        unsafe {
            slice::from_raw_parts(
                self.mmap[self.offsets.connections_start..].as_ptr() as *const OrthogonalConnection,
                count
            )
        }
    }
    
    /// Get connections for a specific node
    pub fn get_node_connections(&self, node_id: NodeId) -> Result<&[OrthogonalConnection]> {
        let node = self.get_node(node_id)?;
        
        let start = node.connections_offset as usize;
        let count = node.connections_count as usize;
        
        if start + count > self.connection_count() {
            return Err(LingoError::Database("Invalid connection range".to_string()));
        }
        
        Ok(&self.connections()[start..start + count])
    }
    
    /// Get the string table
    pub fn string_table(&self) -> Result<StringTable> {
        let start = self.offsets.strings_start;
        let end = self.offsets.octree_start;
        let data = &self.mmap[start..end];
        
        StringTable::from_bytes(data)
    }
    
    /// Get a string from the string table
    pub fn get_string(&self, offset: u32, length: u16) -> Result<&str> {
        let start = self.offsets.strings_start + offset as usize;
        let end = start + length as usize;
        
        if end > self.offsets.octree_start {
            return Err(LingoError::Database("String offset out of bounds".to_string()));
        }
        
        let bytes = &self.mmap[start..end];
        std::str::from_utf8(bytes)
            .map_err(|_| LingoError::Database("Invalid UTF-8 in string table".to_string()))
    }
    
    /// Get the word for a node
    pub fn get_node_word(&self, node_id: NodeId) -> Result<&str> {
        let node = self.get_node(node_id)?;
        self.get_string(node.word_offset, node.word_length)
    }
    
    /// Get octree header if present
    pub fn octree_header(&self) -> Option<()> {
        if self.header.octree_size == 0 {
            return None;
        }
        
        // TODO: Return actual octree header when implemented
        Some(())
    }
    
    /// Find nodes by word (linear search for now)
    pub fn find_nodes_by_word(&self, word: &str) -> Vec<NodeId> {
        let mut results = Vec::new();
        
        for i in 0..self.node_count() {
            let node_id = NodeId(i as u32 + 1); // Node IDs start from 1
            if let Ok(node) = self.get_node(node_id) {
                if let Ok(node_word) = self.get_string(node.word_offset, node.word_length) {
                    if node_word == word {
                        results.push(node_id);
                    }
                }
            }
        }
        
        results
    }
    
    /// Find similar nodes by position (using octree if available)
    pub fn find_similar_nodes(&self, position: Coordinate3D, radius: f32, limit: Option<usize>) -> Vec<NodeId> {
        // TODO: Use octree index when fully implemented
        // For now, do linear search
        let mut candidates = Vec::new();
        
        for (i, node) in self.nodes().iter().enumerate() {
            let dist_sq = Self::distance_squared(position, node.position);
            if dist_sq <= radius * radius {
                candidates.push((NodeId(i as u32), dist_sq));
            }
        }
        
        // Sort by distance
        candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Apply limit
        if let Some(limit) = limit {
            candidates.truncate(limit);
        }
        
        candidates.into_iter().map(|(id, _)| id).collect()
    }
    
    /// Calculate squared distance between two positions
    fn distance_squared(a: Coordinate3D, b: Coordinate3D) -> f32 {
        let dx = a.x - b.x;
        let dy = a.y - b.y; 
        let dz = a.z - b.z;
        dx * dx + dy * dy + dz * dz
    }
}

/// Database type alias
pub type Database = MemoryMappedDatabase;
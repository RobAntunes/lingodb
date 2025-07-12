//! Database builder for creating Lingo files

use crate::core::{
    Coordinate3D, Layer, LinguisticNode, NodeId, OrthogonalConnection,
    ConnectionType, NodeFlags, EtymologyOrigin, MorphemeType,
    error::{Result, BuildError},
};
use crate::storage::{LingoFileHeader, StringTable, FileFormatFlags};
use crate::index::octree::{OctreeBuilder, OctreeNode};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// Builder for creating Lingo database files
pub struct DatabaseBuilder {
    /// Nodes to be written
    nodes: Vec<LinguisticNode>,
    /// Node ID to index mapping
    node_index: HashMap<NodeId, usize>,
    /// Connections grouped by source node
    connections: HashMap<NodeId, Vec<OrthogonalConnection>>,
    /// String table
    string_table: StringTable,
    /// Metadata
    language_code: String,
    model_version: String,
}

impl DatabaseBuilder {
    /// Create a new database builder
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            node_index: HashMap::new(),
            connections: HashMap::new(),
            string_table: StringTable::new(),
            language_code: "en-US".to_string(),
            model_version: "1.0.0".to_string(),
        }
    }
    
    /// Set language code (e.g., "en-US")
    pub fn set_language(&mut self, code: &str) -> &mut Self {
        self.language_code = code.to_string();
        self
    }
    
    /// Set model version
    pub fn set_model_version(&mut self, version: &str) -> &mut Self {
        self.model_version = version.to_string();
        self
    }
    
    /// Add a node to the database
    pub fn add_node(
        &mut self,
        word: &str,
        layer: Layer,
        position: Coordinate3D,
    ) -> Result<NodeId> {
        let node_id = NodeId((self.nodes.len() + 1) as u32);
        
        // Check for duplicate
        if self.node_index.contains_key(&node_id) {
            return Err(BuildError::DuplicateNode(node_id).into());
        }
        
        // Add string to table
        let word_offset = self.string_table.add_string(word)?;
        let word_length = word.len() as u16;
        
        // Create node
        let mut node = LinguisticNode::new(node_id, layer, position);
        node.word_offset = word_offset;
        node.word_length = word_length;
        
        // Store node
        let index = self.nodes.len();
        self.nodes.push(node);
        self.node_index.insert(node_id, index);
        
        Ok(node_id)
    }
    
    /// Add a node with full properties
    pub fn add_node_full(
        &mut self,
        word: &str,
        layer: Layer,
        position: Coordinate3D,
        etymology: EtymologyOrigin,
        morpheme_type: MorphemeType,
        flags: NodeFlags,
    ) -> Result<NodeId> {
        let node_id = self.add_node(word, layer, position)?;
        
        // Update node properties
        let node = &mut self.nodes[self.node_index[&node_id]];
        node.etymology_origin = etymology;
        node.morpheme_type = morpheme_type;
        node.flags = flags;
        
        Ok(node_id)
    }
    
    /// Add a connection between nodes
    pub fn add_connection(
        &mut self,
        source: NodeId,
        target: NodeId,
        connection_type: ConnectionType,
        strength: f32,
    ) -> Result<()> {
        // Validate nodes exist
        if !self.node_index.contains_key(&source) {
            return Err(BuildError::InvalidConnection {
                reason: format!("Source node {} not found", source),
            }.into());
        }
        if !self.node_index.contains_key(&target) {
            return Err(BuildError::InvalidConnection {
                reason: format!("Target node {} not found", target),
            }.into());
        }
        
        // Create connection
        let connection = OrthogonalConnection::new(target, connection_type, strength);
        
        // Store connection
        self.connections
            .entry(source)
            .or_insert_with(Vec::new)
            .push(connection);
        
        Ok(())
    }
    
    /// Build the database and write to file
    pub fn build<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        
        // Sort nodes by ID for efficient lookup
        self.nodes.sort_by_key(|n| n.id);
        
        // Update connection offsets
        self.update_connection_offsets()?;
        
        // Build spatial index
        let octree_nodes = self.build_octree()?;
        
        // Create header
        let mut header = self.create_header()?;
        
        // Update header with octree info
        header.octree_offset = header.connection_array_offset + header.connection_array_size;
        header.octree_size = (octree_nodes.len() * std::mem::size_of::<OctreeNode>()) as u64;
        header.file_size = header.octree_offset + header.octree_size;
        
        // Write header
        self.write_header(&mut writer, &header)?;
        
        // Write string table
        self.write_string_table(&mut writer)?;
        
        // Write nodes
        self.write_nodes(&mut writer)?;
        
        // Write connections
        self.write_connections(&mut writer)?;
        
        // Write octree
        self.write_octree(&mut writer, &octree_nodes)?;
        
        // TODO: Write vertical index
        
        writer.flush()?;
        Ok(())
    }
    
    /// Update connection offsets in nodes
    fn update_connection_offsets(&mut self) -> Result<()> {
        let mut current_offset = 0u32;
        
        for node in &mut self.nodes {
            let node_id = node.id;
            if let Some(connections) = self.connections.get(&node_id) {
                node.connections_offset = current_offset;
                node.connections_count = connections.len() as u16;
                current_offset += connections.len() as u32;
            }
        }
        
        Ok(())
    }
    
    /// Create file header
    fn create_header(&self) -> Result<LingoFileHeader> {
        let mut header = LingoFileHeader::new();
        
        // Set metadata
        header.node_count = self.nodes.len() as u32;
        header.connection_count = self.connections.values().map(|v| v.len()).sum::<usize>() as u32;
        
        // Set timestamp
        header.creation_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Set language code
        let lang_bytes = self.language_code.as_bytes();
        let len = lang_bytes.len().min(8);
        header.language_code[..len].copy_from_slice(&lang_bytes[..len]);
        
        // Set model version
        let model_bytes = self.model_version.as_bytes();
        let len = model_bytes.len().min(16);
        header.model_version[..len].copy_from_slice(&model_bytes[..len]);
        
        // Calculate section offsets
        let mut current_offset = 512u64; // After header
        
        // String table
        header.string_table_offset = current_offset;
        header.string_table_size = self.string_table.size() as u64;
        current_offset += header.string_table_size;
        
        // Align to 8 bytes
        current_offset = (current_offset + 7) & !7;
        
        // Node array
        header.node_array_offset = current_offset;
        header.node_array_size = (self.nodes.len() * std::mem::size_of::<LinguisticNode>()) as u64;
        current_offset += header.node_array_size;
        
        // Connection array
        header.connection_array_offset = current_offset;
        let total_connections: usize = self.connections.values().map(|v| v.len()).sum();
        header.connection_array_size = (total_connections * std::mem::size_of::<OrthogonalConnection>()) as u64;
        current_offset += header.connection_array_size;
        
        // File size
        header.file_size = current_offset;
        
        // Set flags
        header.format_flags = FileFormatFlags::empty();
        
        Ok(header)
    }
    
    /// Write header to file
    fn write_header(&self, writer: &mut BufWriter<File>, header: &LingoFileHeader) -> Result<()> {
        let header_bytes = unsafe {
            std::slice::from_raw_parts(
                header as *const _ as *const u8,
                std::mem::size_of::<LingoFileHeader>()
            )
        };
        writer.write_all(header_bytes)?;
        Ok(())
    }
    
    /// Write string table
    fn write_string_table(&self, writer: &mut BufWriter<File>) -> Result<()> {
        writer.write_all(self.string_table.as_bytes())?;
        
        // Pad to 8-byte alignment
        let padding = (8 - (self.string_table.size() % 8)) % 8;
        if padding > 0 {
            writer.write_all(&vec![0u8; padding])?;
        }
        
        Ok(())
    }
    
    /// Write nodes
    fn write_nodes(&self, writer: &mut BufWriter<File>) -> Result<()> {
        for node in &self.nodes {
            let node_bytes = unsafe {
                std::slice::from_raw_parts(
                    node as *const _ as *const u8,
                    std::mem::size_of::<LinguisticNode>()
                )
            };
            writer.write_all(node_bytes)?;
        }
        Ok(())
    }
    
    /// Write connections
    fn write_connections(&self, writer: &mut BufWriter<File>) -> Result<()> {
        // Write connections in node order
        for node in &self.nodes {
            let node_id = node.id;
            if let Some(connections) = self.connections.get(&node_id) {
                for connection in connections {
                    let conn_bytes = unsafe {
                        std::slice::from_raw_parts(
                            connection as *const _ as *const u8,
                            std::mem::size_of::<OrthogonalConnection>()
                        )
                    };
                    writer.write_all(conn_bytes)?;
                }
            }
        }
        Ok(())
    }
    
    /// Build octree spatial index
    fn build_octree(&self) -> Result<Vec<OctreeNode>> {
        let mut octree_builder = OctreeBuilder::new();
        
        // Add all nodes to the octree
        for node in &self.nodes {
            octree_builder.add_node(node.id, node.position);
        }
        
        // Build the octree
        let spatial_index = octree_builder.build();
        
        // Extract octree nodes for serialization
        Ok(spatial_index.nodes().to_vec())
    }
    
    /// Write octree index
    fn write_octree(&self, writer: &mut BufWriter<File>, octree_nodes: &[OctreeNode]) -> Result<()> {
        for node in octree_nodes {
            let node_bytes = unsafe {
                std::slice::from_raw_parts(
                    node as *const _ as *const u8,
                    std::mem::size_of::<OctreeNode>()
                )
            };
            writer.write_all(node_bytes)?;
        }
        Ok(())
    }
}

impl Default for DatabaseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_database_builder() {
        let mut builder = DatabaseBuilder::new();
        
        // Add some nodes
        let tech_id = builder.add_node(
            "technical",
            Layer::Words,
            Coordinate3D::new(0.5, 0.8, 0.525)
        ).unwrap();
        
        let technology_id = builder.add_node(
            "technology", 
            Layer::Words,
            Coordinate3D::new(0.52, 0.8, 0.525)
        ).unwrap();
        
        // Add connection
        builder.add_connection(
            tech_id,
            technology_id,
            ConnectionType::Derivation,
            0.9
        ).unwrap();
        
        // Build database
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.lingo");
        
        builder.build(&db_path).unwrap();
        
        // Verify file exists and has content
        let metadata = std::fs::metadata(&db_path).unwrap();
        assert!(metadata.len() > 512); // At least header size
    }
}
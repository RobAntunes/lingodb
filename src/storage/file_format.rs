//! Lingo database file format specification

use bitflags::bitflags;

/// Magic bytes for Lingo files
pub const MAGIC_BYTES: &[u8; 8] = b"LINGO1.0";

/// Lingo file header (512 bytes total)
#[repr(C)]
pub struct LingoFileHeader {
    // Magic & Version (16 bytes)
    /// Magic bytes "LINGO1.0"
    pub magic: [u8; 8],
    /// Major version
    pub version_major: u16,
    /// Minor version
    pub version_minor: u16,
    /// Format feature flags
    pub format_flags: FileFormatFlags,
    
    // File Layout (32 bytes)
    /// Total file size
    pub file_size: u64,
    /// Total linguistic nodes
    pub node_count: u32,
    /// Total orthogonal connections
    pub connection_count: u32,
    /// Maximum octree depth
    pub octree_depth: u8,
    /// Number of layers (7)
    pub layer_count: u8,
    /// Compression algorithm used
    pub compression_type: CompressionType,
    /// Padding
    _padding1: u8,
    
    // Section Offsets (64 bytes)
    /// Offset to string table
    pub string_table_offset: u64,
    /// Size of string table
    pub string_table_size: u64,
    
    /// Offset to node array
    pub node_array_offset: u64,
    /// Size of node array
    pub node_array_size: u64,
    
    /// Offset to connection array
    pub connection_array_offset: u64,
    /// Size of connection array
    pub connection_array_size: u64,
    
    /// Offset to spatial index
    pub octree_offset: u64,
    /// Size of spatial index
    pub octree_size: u64,
    
    // Index Offsets (32 bytes)
    /// Offset to vertical mappings
    pub vertical_index_offset: u64,
    /// Size of vertical mappings
    pub vertical_index_size: u64,
    
    /// Offset to cache optimization hints
    pub cache_hints_offset: u64,
    /// Size of cache hints
    pub cache_hints_size: u64,
    
    // Checksums (32 bytes)
    /// Header integrity check
    pub header_checksum: u64,
    /// Data integrity check
    pub data_checksum: u64,
    /// String table integrity
    pub string_checksum: u64,
    /// Index integrity
    pub index_checksum: u64,
    
    // Metadata (64 bytes)
    /// Unix timestamp of creation
    pub creation_timestamp: u64,
    /// ISO language code "en-US"
    pub language_code: [u8; 8],
    /// Semantic model version
    pub model_version: [u8; 16],
    /// Build information
    pub build_info: [u8; 32],
    
    // Reserved (64 bytes)
    /// Reserved for future use
    pub reserved: [u8; 64],
    
    // Padding to 512 bytes
    _padding2: [u8; 216],
}

// Ensure header is exactly 512 bytes
const _: () = assert!(std::mem::size_of::<LingoFileHeader>() == 512);

impl LingoFileHeader {
    /// Create a new header with default values
    pub fn new() -> Self {
        Self {
            magic: *MAGIC_BYTES,
            version_major: 1,
            version_minor: 0,
            format_flags: FileFormatFlags::empty(),
            
            file_size: 0,
            node_count: 0,
            connection_count: 0,
            octree_depth: 0,
            layer_count: 7,
            compression_type: CompressionType::None,
            _padding1: 0,
            
            string_table_offset: 512, // Right after header
            string_table_size: 0,
            node_array_offset: 0,
            node_array_size: 0,
            connection_array_offset: 0,
            connection_array_size: 0,
            octree_offset: 0,
            octree_size: 0,
            
            vertical_index_offset: 0,
            vertical_index_size: 0,
            cache_hints_offset: 0,
            cache_hints_size: 0,
            
            header_checksum: 0,
            data_checksum: 0,
            string_checksum: 0,
            index_checksum: 0,
            
            creation_timestamp: 0,
            language_code: [0; 8],
            model_version: [0; 16],
            build_info: [0; 32],
            
            reserved: [0; 64],
            _padding2: [0; 216],
        }
    }
    
    /// Validate magic bytes and version
    pub fn validate(&self) -> Result<(), String> {
        if &self.magic != MAGIC_BYTES {
            return Err("Invalid magic bytes".to_string());
        }
        
        // Copy fields to avoid unaligned references
        let version_major = self.version_major;
        let layer_count = self.layer_count;
        
        if version_major > crate::MAX_FILE_VERSION.0 {
            return Err(format!(
                "Unsupported major version: {} > {}",
                version_major,
                crate::MAX_FILE_VERSION.0
            ));
        }
        
        if layer_count != 7 {
            return Err(format!(
                "Invalid layer count: {} (expected 7)",
                layer_count
            ));
        }
        
        Ok(())
    }
}

impl Default for LingoFileHeader {
    fn default() -> Self {
        Self::new()
    }
}

bitflags! {
    /// File format feature flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FileFormatFlags: u32 {
        /// String table is compressed
        const STRING_COMPRESSION = 0x00000001;
        /// Includes phonetic index
        const HAS_PHONETIC_INDEX = 0x00000002;
        /// Includes etymology index
        const HAS_ETYMOLOGY_INDEX = 0x00000004;
        /// Includes learning data
        const HAS_LEARNING_DATA = 0x00000008;
        /// Optimized for mobile
        const MOBILE_OPTIMIZED = 0x00000010;
        /// Includes cache hints
        const HAS_CACHE_HINTS = 0x00000020;
        /// Debug symbols included
        const DEBUG_SYMBOLS = 0x00000040;
    }
}

/// Compression algorithm
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    /// No compression
    None = 0,
    /// LZ4 compression
    Lz4 = 1,
    /// Zstandard compression
    Zstd = 2,
    /// Dictionary compression
    Dictionary = 3,
}

/// Section identifiers for checksums
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionId {
    /// File header
    Header,
    /// String table
    Strings,
    /// Node array
    Nodes,
    /// Connection array
    Connections,
    /// Spatial index
    Octree,
    /// Vertical index
    VerticalIndex,
    /// Cache hints
    CacheHints,
}
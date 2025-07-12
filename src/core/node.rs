//! Core linguistic node structure

use crate::core::{Coordinate3D, NodeId};
use bitflags::bitflags;

/// Main linguistic node structure representing a single element in the database.
///
/// Each node occupies exactly 60 bytes and represents a linguistic element at a specific
/// layer in the hierarchy. Nodes contain spatial positioning information, linguistic
/// properties, and pointers to related nodes and connections.
///
/// # Memory Layout
///
/// The structure is packed for cache efficiency with the following layout:
/// - Position and identity: 28 bytes
/// - Linguistic properties: 16 bytes
/// - Index pointers: 16 bytes
///
/// # Examples
///
/// ```rust
/// use lingo::core::{LinguisticNode, NodeId, Layer, Coordinate3D};
///
/// // Create a word node
/// let node = LinguisticNode::new(
///     NodeId(42),
///     Layer::Words,
///     Coordinate3D::new(0.5, 0.3, 0.55)
/// );
///
/// assert_eq!(node.layer, Layer::Words);
/// assert_eq!(node.id, NodeId(42));
/// ```
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct LinguisticNode {
    /// 3D spatial position (12 bytes)
    pub position: Coordinate3D,
    
    // Node identity (16 bytes)
    /// Unique node identifier (4 bytes)
    pub id: NodeId,
    /// Linguistic layer (1 byte)
    pub layer: Layer,
    /// Offset into string table (4 bytes)
    pub word_offset: u32,
    /// String length (2 bytes)
    pub word_length: u16,
    /// Node flags (1 byte)
    pub flags: NodeFlags,
    /// Alignment padding (4 bytes)
    _padding: [u8; 4],
    
    // Linguistic properties (16 bytes)
    /// Etymology origin (1 byte)
    pub etymology_origin: EtymologyOrigin,
    /// Compressed phoneme pattern (8 bytes)
    pub phonetic_signature: u64,
    /// Type of morpheme (1 byte)
    pub morpheme_type: MorphemeType,
    /// Productivity score 0-65535 (2 bytes)
    pub productivity_score: u16,
    /// Word frequency ranking (4 bytes)
    pub frequency_rank: u32,
    
    // Index pointers (16 bytes)
    /// Offset to children array (4 bytes)
    pub children_offset: u32,
    /// Number of children (2 bytes)
    pub children_count: u16,
    /// Offset to orthogonal connections (4 bytes)
    pub connections_offset: u32,
    /// Number of connections (2 bytes)
    pub connections_count: u16,
    /// Octree bucket ID (4 bytes)
    pub spatial_bucket: u32,
}

// Ensure the struct is exactly 60 bytes
const _: () = assert!(std::mem::size_of::<LinguisticNode>() == 60);

impl LinguisticNode {
    /// Creates a new linguistic node with default properties.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the node
    /// * `layer` - Linguistic layer where the node resides
    /// * `position` - 3D spatial coordinate
    ///
    /// # Returns
    ///
    /// A new `LinguisticNode` with the specified ID, layer, and position.
    /// All other fields are initialized to default values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::core::{LinguisticNode, NodeId, Layer, Coordinate3D};
    ///
    /// let node = LinguisticNode::new(
    ///     NodeId(100),
    ///     Layer::Morphemes,
    ///     Coordinate3D::new(0.2, 0.4, 0.375)
    /// );
    ///
    /// assert_eq!(node.frequency_rank, u32::MAX); // Default: unranked
    /// assert!(node.flags.is_empty()); // Default: no flags
    /// ```
    pub fn new(id: NodeId, layer: Layer, position: Coordinate3D) -> Self {
        Self {
            position,
            id,
            layer,
            word_offset: 0,
            word_length: 0,
            flags: NodeFlags::empty(),
            _padding: [0; 4],
            etymology_origin: EtymologyOrigin::Unknown,
            phonetic_signature: 0,
            morpheme_type: MorphemeType::Root,
            productivity_score: 0,
            frequency_rank: u32::MAX,
            children_offset: 0,
            children_count: 0,
            connections_offset: 0,
            connections_count: 0,
            spatial_bucket: 0,
        }
    }
}

/// Represents one of the seven linguistic layers in the database hierarchy.
///
/// Each layer represents a different level of linguistic abstraction, from
/// individual characters at the bottom to abstract knowledge domains at the top.
/// Layers are mapped to specific Z-coordinate ranges in 3D space.
///
/// # Layer Descriptions
///
/// - **Letters** (0): Individual characters, digraphs, and graphemes
/// - **Phonemes** (1): Sound units and pronunciation patterns
/// - **Morphemes** (2): Meaningful word parts like roots, prefixes, and suffixes
/// - **Words** (3): Complete lexical units
/// - **Phrases** (4): Multi-word expressions, idioms, and collocations
/// - **Concepts** (5): Abstract semantic representations
/// - **Domains** (6): High-level knowledge categories
///
/// # Examples
///
/// ```rust
/// use lingo::core::node::Layer;
///
/// let word_layer = Layer::Words;
/// let (z_min, z_max) = word_layer.z_range();
/// assert!(z_min < z_max);
/// assert_eq!(word_layer.z_center(), (z_min + z_max) / 2.0);
/// ```
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Layer {
    /// Individual characters, digraphs (Layer 0)
    Letters = 0,
    /// Sound units, pronunciation (Layer 1)
    Phonemes = 1,
    /// Meaningful word parts (Layer 2)
    Morphemes = 2,
    /// Complete words (Layer 3)
    Words = 3,
    /// Multi-word expressions (Layer 4)
    Phrases = 4,
    /// Abstract semantic concepts (Layer 5)
    Concepts = 5,
    /// Knowledge domains (Layer 6)
    Domains = 6,
}

impl Layer {
    /// Returns the Z-coordinate range for this layer.
    ///
    /// Each layer occupies a specific range of Z-coordinates in the 3D space,
    /// ensuring spatial separation between different linguistic levels.
    ///
    /// # Returns
    ///
    /// A tuple `(min_z, max_z)` representing the inclusive range of Z-coordinates
    /// for nodes in this layer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::core::node::Layer;
    ///
    /// let (min, max) = Layer::Words.z_range();
    /// assert_eq!((min, max), (0.45, 0.60));
    /// ```
    pub fn z_range(&self) -> (f32, f32) {
        match self {
            Layer::Letters => (0.0, 0.15),
            Layer::Phonemes => (0.15, 0.30),
            Layer::Morphemes => (0.30, 0.45),
            Layer::Words => (0.45, 0.60),
            Layer::Phrases => (0.60, 0.75),
            Layer::Concepts => (0.75, 0.90),
            Layer::Domains => (0.90, 1.0),
        }
    }
    
    /// Returns the center Z-coordinate for this layer.
    ///
    /// This is useful for positioning new nodes at the default depth for a layer.
    ///
    /// # Returns
    ///
    /// The midpoint of the layer's Z-coordinate range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::core::node::Layer;
    ///
    /// let center = Layer::Concepts.z_center();
    /// assert_eq!(center, 0.825); // (0.75 + 0.90) / 2
    /// ```
    pub fn z_center(&self) -> f32 {
        let (min, max) = self.z_range();
        (min + max) / 2.0
    }
}

bitflags! {
    /// Bit flags representing various properties of a linguistic node.
    ///
    /// These flags encode boolean properties efficiently in a single byte,
    /// allowing for fast filtering and querying of nodes based on their
    /// characteristics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::core::node::NodeFlags;
    ///
    /// let mut flags = NodeFlags::IS_TERMINAL | NodeFlags::IS_FREQUENT;
    /// assert!(flags.contains(NodeFlags::IS_TERMINAL));
    /// assert!(!flags.contains(NodeFlags::IS_ARCHAIC));
    ///
    /// // Add more flags
    /// flags.insert(NodeFlags::IS_TECHNICAL);
    /// assert!(flags.contains(NodeFlags::IS_TECHNICAL));
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct NodeFlags: u8 {
        /// End of word/concept
        const IS_TERMINAL = 0b00000001;
        /// Has spelling/pronunciation variants
        const HAS_VARIANTS = 0b00000010;
        /// Can form new words
        const IS_PRODUCTIVE = 0b00000100;
        /// Borrowed from another language
        const IS_BORROWED = 0b00001000;
        /// Historical/obsolete
        const IS_ARCHAIC = 0b00010000;
        /// Technical terminology
        const IS_TECHNICAL = 0b00100000;
        /// Added through learning
        const IS_LEARNED = 0b01000000;
        /// High-frequency usage
        const IS_FREQUENT = 0b10000000;
    }
}

/// Classifies the etymological origin of a linguistic element.
///
/// Etymology origins affect the Y-coordinate positioning of nodes,
/// grouping words from similar linguistic backgrounds together in space.
/// This enables efficient queries for words from specific language families
/// or historical periods.
///
/// # Coordinate Mapping
///
/// Each origin maps to a base Y-coordinate that influences the node's
/// vertical position in 3D space, creating etymological clusters.
///
/// # Examples
///
/// ```rust
/// use lingo::core::node::EtymologyOrigin;
///
/// let origin = EtymologyOrigin::Latin;
/// let y_base = origin.base_y_coordinate();
/// assert_eq!(y_base, 0.4);
/// ```
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtymologyOrigin {
    /// Germanic origin
    Germanic = 0,
    /// Latin origin
    Latin = 1,
    /// Greek origin
    Greek = 2,
    /// French origin
    French = 3,
    /// Arabic origin
    Arabic = 4,
    /// Sanskrit origin
    Sanskrit = 5,
    /// Chinese origin
    Chinese = 6,
    /// Japanese origin
    Japanese = 7,
    /// Modern coinage (20th+ century)
    Modern = 8,
    /// Unknown origin
    Unknown = 255,
}

impl EtymologyOrigin {
    /// Returns the base Y-coordinate for nodes with this etymological origin.
    ///
    /// The Y-axis represents etymological relationships, with related language
    /// families positioned closer together. This base coordinate is used as a
    /// starting point for positioning nodes, with individual variations based
    /// on other factors.
    ///
    /// # Returns
    ///
    /// A float between 0.0 and 1.0 representing the base Y position.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::core::node::EtymologyOrigin;
    ///
    /// // Germanic and French origins are relatively close
    /// let germanic = EtymologyOrigin::Germanic.base_y_coordinate();
    /// let french = EtymologyOrigin::French.base_y_coordinate();
    /// assert!((germanic - french).abs() < 0.3);
    /// ```
    pub fn base_y_coordinate(&self) -> f32 {
        match self {
            EtymologyOrigin::Germanic => 0.0,
            EtymologyOrigin::French => 0.2,
            EtymologyOrigin::Latin => 0.4,
            EtymologyOrigin::Arabic => 0.6,
            EtymologyOrigin::Greek => 0.8,
            EtymologyOrigin::Sanskrit => 0.5,
            EtymologyOrigin::Chinese => 0.7,
            EtymologyOrigin::Japanese => 0.75,
            EtymologyOrigin::Modern => 1.0,
            EtymologyOrigin::Unknown => 0.5,
        }
    }
}

/// Classifies the type of morpheme for nodes in the morpheme layer.
///
/// Morpheme types determine how word parts combine and their relative
/// importance in word formation. This affects both spatial positioning
/// and connection strengths in morphological relationships.
///
/// # Types
///
/// - **Root**: Core meaning-bearing unit
/// - **Prefix**: Attached before roots (e.g., "un-", "pre-")
/// - **Suffix**: Attached after roots (e.g., "-ing", "-ness")
/// - **Infix**: Inserted within roots (rare in English)
/// - **Circumfix**: Surrounds roots (e.g., German "ge-...-t")
/// - **Compound**: Elements in compound words
///
/// # Examples
///
/// ```rust
/// use lingo::core::node::MorphemeType;
///
/// let root = MorphemeType::Root;
/// assert_eq!(root.composition_weight(), 0.6); // Highest weight
/// ```
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorphemeType {
    /// Root morpheme
    Root = 0,
    /// Prefix
    Prefix = 1,
    /// Suffix
    Suffix = 2,
    /// Infix
    Infix = 3,
    /// Circumfix
    Circumfix = 4,
    /// Compound element
    Compound = 5,
}

impl MorphemeType {
    /// Returns the composition weight for this morpheme type.
    ///
    /// The weight determines the morpheme's relative importance when
    /// calculating word compositions and morphological relationships.
    /// Roots have the highest weight as they carry core meaning.
    ///
    /// # Returns
    ///
    /// A weight between 0.0 and 1.0, where higher values indicate
    /// greater compositional importance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::core::node::MorphemeType;
    ///
    /// assert!(MorphemeType::Root.composition_weight() >
    ///         MorphemeType::Suffix.composition_weight());
    /// ```
    pub fn composition_weight(&self) -> f32 {
        match self {
            MorphemeType::Root => 0.6,
            MorphemeType::Prefix => 0.2,
            MorphemeType::Suffix => 0.2,
            MorphemeType::Infix => 0.3,
            MorphemeType::Circumfix => 0.3,
            MorphemeType::Compound => 0.5,
        }
    }
}

// Implement Debug manually due to packed struct
impl std::fmt::Debug for LinguisticNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Copy fields to avoid unaligned references
        let id = self.id;
        let layer = self.layer;
        let position = self.position;
        let etymology = self.etymology_origin;
        let flags = self.flags;
        
        f.debug_struct("LinguisticNode")
            .field("id", &id)
            .field("layer", &layer)
            .field("position", &position)
            .field("etymology", &etymology)
            .field("flags", &flags)
            .finish()
    }
}
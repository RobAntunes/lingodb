//! Orthogonal connection structure for cross-domain relationships

use crate::core::{NodeId, Vector3D};
use bitflags::bitflags;

/// Orthogonal connection between nodes (20 bytes packed)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct OrthogonalConnection {
    /// Target node ID (4 bytes)
    pub target_node: NodeId,
    /// Connection strength 0-65535 (2 bytes)
    pub strength: u16,
    /// Type of relationship (1 byte)
    pub connection_type: ConnectionType,
    /// Context flags (1 byte)
    pub context_mask: ContextMask,
    /// 3D direction vector (12 bytes)
    pub transformation_vector: Vector3D,
}

// Ensure the struct is exactly 20 bytes
const _: () = assert!(std::mem::size_of::<OrthogonalConnection>() == 20);

impl OrthogonalConnection {
    /// Create a new connection
    pub fn new(
        target_node: NodeId,
        connection_type: ConnectionType,
        strength: f32,
    ) -> Self {
        Self {
            target_node,
            strength: (strength.clamp(0.0, 1.0) * 65535.0) as u16,
            connection_type,
            context_mask: ContextMask::empty(),
            transformation_vector: Vector3D::zero(),
        }
    }
    
    /// Get normalized strength value (0.0-1.0)
    pub fn strength_normalized(&self) -> f32 {
        self.strength as f32 / 65535.0
    }
    
    /// Set strength from normalized value
    pub fn set_strength_normalized(&mut self, strength: f32) {
        self.strength = (strength.clamp(0.0, 1.0) * 65535.0) as u16;
    }
}

/// Type of semantic/linguistic connection
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    /// Similar meaning
    Synonymy = 0,
    /// Opposite meaning
    Antonymy = 1,
    /// More general concept
    Hypernymy = 2,
    /// More specific concept
    Hyponymy = 3,
    /// Part-whole relationship
    Meronymy = 4,
    /// Morphological derivation
    Derivation = 5,
    /// Historical relationship
    Etymology = 6,
    /// Sound similarity
    Phonetic = 7,
    /// Cross-domain similarity
    Analogy = 8,
    /// Frequently co-occurring
    Collocation = 9,
    /// Cause-effect relationship
    Causation = 10,
    /// Discovered through usage
    Learned = 11,
    /// Cross-domain lexical bridge
    LexicalBridge = 12,
    /// Morphological pattern similarity
    MorphologicalPattern = 13,
}

impl ConnectionType {
    /// Check if this connection type is bidirectional
    pub fn is_bidirectional(&self) -> bool {
        matches!(self, 
            ConnectionType::Synonymy | 
            ConnectionType::Antonymy | 
            ConnectionType::Phonetic |
            ConnectionType::Analogy |
            ConnectionType::Collocation
        )
    }
    
    /// Get the inverse connection type (if applicable)
    pub fn inverse(&self) -> Option<ConnectionType> {
        match self {
            ConnectionType::Hypernymy => Some(ConnectionType::Hyponymy),
            ConnectionType::Hyponymy => Some(ConnectionType::Hypernymy),
            ConnectionType::Meronymy => None, // Part-whole doesn't have simple inverse
            _ => {
                if self.is_bidirectional() {
                    Some(*self)
                } else {
                    None
                }
            }
        }
    }
}

bitflags! {
    /// Context mask for domain-specific connections
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ContextMask: u8 {
        /// Medical domain
        const MEDICAL = 0b00000001;
        /// Business domain
        const BUSINESS = 0b00000010;
        /// Technical domain
        const TECHNICAL = 0b00000100;
        /// Academic domain
        const ACADEMIC = 0b00001000;
        /// Casual context
        const CASUAL = 0b00010000;
        /// Formal context
        const FORMAL = 0b00100000;
        /// Archaic usage
        const ARCHAIC = 0b01000000;
        /// Regional variation
        const REGIONAL = 0b10000000;
    }
}

impl ContextMask {
    /// Check if contexts are compatible
    pub fn is_compatible_with(&self, other: ContextMask) -> bool {
        // Empty mask is compatible with everything
        if self.is_empty() || other.is_empty() {
            return true;
        }
        
        // Check for any overlap
        !(*self & other).is_empty()
    }
    
    /// Get domain masks only
    pub fn domains_only(&self) -> ContextMask {
        *self & (ContextMask::MEDICAL | 
                ContextMask::BUSINESS | 
                ContextMask::TECHNICAL | 
                ContextMask::ACADEMIC)
    }
    
    /// Get register masks only  
    pub fn register_only(&self) -> ContextMask {
        *self & (ContextMask::CASUAL | 
                ContextMask::FORMAL | 
                ContextMask::ARCHAIC | 
                ContextMask::REGIONAL)
    }
}

// Implement Debug manually due to packed struct
impl std::fmt::Debug for OrthogonalConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Copy fields to avoid unaligned references
        let target = self.target_node;
        let conn_type = self.connection_type;
        let strength = self.strength_normalized();
        let context = self.context_mask;
        
        f.debug_struct("OrthogonalConnection")
            .field("target", &target)
            .field("type", &conn_type)
            .field("strength", &strength)
            .field("context", &context)
            .finish()
    }
}

/// Connection discovery metadata
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscoveryMethod {
    /// Pre-computed from corpus analysis
    Precomputed,
    /// Discovered through cross-domain lexical analysis
    CrossDomainLexical,
    /// Found via morphological pattern matching
    MorphologicalPattern,
    /// Etymological relationship mining
    EtymologicalMining,
    /// Semantic field analysis
    SemanticField,
    /// Runtime user interaction
    RuntimeLearning,
    /// Analogy pattern detection
    AnalogyDetection,
}
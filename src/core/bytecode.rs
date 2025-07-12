//! SLANG bytecode operations for query execution

/// SLANG bytecode operation codes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlangOp {
    // Node Operations (0-15)
    /// Load node by string
    LoadNode = 0,
    /// Load node by ID
    LoadNodeId = 1,
    /// Get current node set
    GetCurrent = 2,
    /// Set current node set
    SetCurrent = 3,
    
    // Layer Operations (16-31)
    /// Move up N layers
    LayerUp = 16,
    /// Move down N layers
    LayerDown = 17,
    /// Set to specific layer
    LayerSet = 18,
    /// Filter by layer
    LayerFilter = 19,
    
    // Tree Operations (32-47)
    /// Follow children
    TreeForward = 32,
    /// Go to parents
    TreeBackward = 33,
    /// Follow specific path
    TreePath = 34,
    /// Common cached path
    TreeCommonPath = 35,
    
    // Orthogonal Operations (48-63)
    /// Follow Nth strongest connection
    FollowConnection = 48,
    /// Follow specific connection type
    FollowConnectionType = 49,
    /// Get bidirectional connections
    Bidirectional = 50,
    /// Explore connection neighborhood
    ConnectionNeighborhood = 51,
    
    // Spatial Operations (64-79)
    /// Find spatial neighbors
    SpatialNeighbors = 64,
    /// Within radius
    SpatialRadius = 65,
    /// Constrain to layer
    SpatialLayer = 66,
    /// Find cluster center
    SpatialCluster = 67,
    
    // Search Operations (80-95)
    /// Semantic similarity search
    FindSimilar = 80,
    /// Phonetic similarity
    FindPhonetic = 81,
    /// Same etymology
    FindEtymological = 82,
    /// Morphological patterns
    FindMorphological = 83,
    /// Conceptual relationships
    FindConceptual = 84,
    
    // Analysis Operations (96-111)
    /// Full linguistic analysis
    AnalyzeAll = 96,
    /// Phonetic analysis
    AnalyzePhonetic = 97,
    /// Etymology trace
    AnalyzeEtymology = 98,
    /// Morphological decomposition
    AnalyzeMorphology = 99,
    /// Semantic analysis
    AnalyzeSemantic = 100,
    
    // Pattern Operations (112-127)
    /// Trace derivation pattern
    PatternTrace = 112,
    /// Group by pattern
    PatternCluster = 113,
    /// Predict new formations
    PatternPredict = 114,
    /// Learn from interaction
    PatternLearn = 115,
    
    // Result Operations (128-143)
    /// Filter results
    Filter = 128,
    /// Sort results
    Sort = 129,
    /// Limit result count
    Limit = 130,
    /// Remove duplicates
    Deduplicate = 131,
    
    // Control Operations (144-159)
    /// Conditional branch
    Branch = 144,
    /// Loop operation
    Loop = 145,
    /// Call subroutine
    Call = 146,
    /// Return from subroutine
    Return = 147,
    
    // Data Operations (160-175)
    /// Push to stack
    Push = 160,
    /// Pop from stack
    Pop = 161,
    /// Store in variable
    Store = 162,
    /// Load from variable
    Load = 163,
    
    // Special Operations (240-255)
    /// No operation
    Nop = 240,
    /// End execution
    Halt = 255,
}

/// SLANG bytecode instruction (12 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct SlangInstruction {
    /// Operation code (1 byte)
    pub opcode: SlangOp,
    /// Operation flags (1 byte)
    pub flags: u8,
    /// First operand (2 bytes)
    pub operand1: u16,
    /// Second operand (4 bytes)
    pub operand2: u32,
    /// Third operand (4 bytes)
    pub operand3: u32,
}

// Ensure the struct is exactly 12 bytes
const _: () = assert!(std::mem::size_of::<SlangInstruction>() == 12);

impl SlangInstruction {
    /// Create a new instruction
    pub fn new(opcode: SlangOp) -> Self {
        Self {
            opcode,
            flags: 0,
            operand1: 0,
            operand2: 0,
            operand3: 0,
        }
    }
    
    /// Create instruction with one operand
    pub fn with_operand1(opcode: SlangOp, operand1: u16) -> Self {
        Self {
            opcode,
            flags: 0,
            operand1,
            operand2: 0,
            operand3: 0,
        }
    }
    
    /// Create instruction with two operands
    pub fn with_operand2(opcode: SlangOp, operand1: u16, operand2: u32) -> Self {
        Self {
            opcode,
            flags: 0,
            operand1,
            operand2,
            operand3: 0,
        }
    }
    
    /// Create instruction with all operands
    pub fn with_all_operands(
        opcode: SlangOp,
        flags: u8,
        operand1: u16,
        operand2: u32,
        operand3: u32,
    ) -> Self {
        Self {
            opcode,
            flags,
            operand1,
            operand2,
            operand3,
        }
    }
}

// Implement Debug manually due to packed struct
impl std::fmt::Debug for SlangInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Copy fields to avoid unaligned references
        let opcode = self.opcode;
        let flags = self.flags;
        let operand1 = self.operand1;
        let operand2 = self.operand2;
        let operand3 = self.operand3;
        
        write!(f, "{:?}", opcode)?;
        
        if flags != 0 {
            write!(f, " flags={:#04x}", flags)?;
        }
        if operand1 != 0 {
            write!(f, " op1={}", operand1)?;
        }
        if operand2 != 0 {
            write!(f, " op2={}", operand2)?;
        }
        if operand3 != 0 {
            write!(f, " op3={}", operand3)?;
        }
        
        Ok(())
    }
}

/// Flags for instruction execution
pub mod instruction_flags {
    /// Has limit parameter
    pub const HAS_LIMIT: u8 = 0x01;
    /// Inverse operation
    pub const INVERSE: u8 = 0x02;
    /// Case insensitive
    pub const CASE_INSENSITIVE: u8 = 0x04;
    /// Include self in results
    pub const INCLUDE_SELF: u8 = 0x08;
}
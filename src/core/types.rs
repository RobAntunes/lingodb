//! Core type definitions for the Lingo database

use std::fmt;

/// Unique identifier for a linguistic node
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct NodeId(pub u32);

impl NodeId {
    /// Invalid node ID constant
    pub const INVALID: Self = Self(0);
    
    /// Check if this is a valid node ID
    pub fn is_valid(&self) -> bool {
        self.0 != 0
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node#{}", self.0)
    }
}

/// Phoneme identifier
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhonemeId(pub u16);

/// 3D transformation vector for connections
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D {
    /// X-axis transformation
    pub dx: f32,
    /// Y-axis transformation  
    pub dy: f32,
    /// Z-axis transformation
    pub dz: f32,
}

impl Vector3D {
    /// Create a new vector
    pub fn new(dx: f32, dy: f32, dz: f32) -> Self {
        Self { dx, dy, dz }
    }
    
    /// Zero vector
    pub fn zero() -> Self {
        Self { dx: 0.0, dy: 0.0, dz: 0.0 }
    }
    
    /// Calculate magnitude
    pub fn magnitude(&self) -> f32 {
        (self.dx * self.dx + self.dy * self.dy + self.dz * self.dz).sqrt()
    }
    
    /// Normalize the vector
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            Self {
                dx: self.dx / mag,
                dy: self.dy / mag,
                dz: self.dz / mag,
            }
        } else {
            Self::zero()
        }
    }
}
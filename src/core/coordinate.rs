//! 3D coordinate system for spatial linguistic relationships

use std::ops::{Add, Sub, Mul, Div};

/// 3D coordinate in linguistic space
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate3D {
    /// Phonetic/Acoustic similarity space (0.0-1.0)
    /// 0.0 = consonant-heavy, 1.0 = vowel-heavy
    pub x: f32,
    
    /// Etymology/Origin space (0.0-1.0)  
    /// 0.0 = Germanic, 0.3 = Latin, 0.7 = Greek, 1.0 = Modern
    pub y: f32,
    
    /// Abstraction level (0.0-1.0)
    /// Determined by layer: 0.0 = letters, 1.0 = domains
    pub z: f32,
}

impl Coordinate3D {
    /// Create a new coordinate
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: x.clamp(0.0, 1.0),
            y: y.clamp(0.0, 1.0),
            z: z.clamp(0.0, 1.0),
        }
    }
    
    /// Zero coordinate
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
    
    /// Calculate Euclidean distance to another coordinate
    #[inline]
    pub fn distance(&self, other: Coordinate3D) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
    
    /// Calculate squared distance (faster, no sqrt)
    #[inline]
    pub fn distance_squared(&self, other: Coordinate3D) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
    
    /// Linear interpolation between two coordinates
    pub fn lerp(&self, other: Coordinate3D, t: f32) -> Coordinate3D {
        Coordinate3D {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
        }
    }
    
    /// Clamp all components to [0, 1] range
    pub fn clamp(&self) -> Coordinate3D {
        Coordinate3D::new(self.x, self.y, self.z)
    }
}

/// 3D bounding box for spatial queries
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox3D {
    /// Minimum coordinates
    pub min: Coordinate3D,
    /// Maximum coordinates
    pub max: Coordinate3D,
}

impl BoundingBox3D {
    /// Create a new bounding box
    pub fn new(min: Coordinate3D, max: Coordinate3D) -> Self {
        Self { min, max }
    }
    
    /// Create a bounding box from a center and radius
    pub fn from_center_radius(center: Coordinate3D, radius: f32) -> Self {
        let offset = Coordinate3D::new(radius, radius, radius);
        Self {
            min: (center - offset).clamp(),
            max: (center + offset).clamp(),
        }
    }
    
    /// Check if a point is inside the bounding box
    #[inline]
    pub fn contains(&self, point: Coordinate3D) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }
    
    /// Check if two bounding boxes intersect
    pub fn intersects(&self, other: &BoundingBox3D) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }
    
    /// Check if a sphere intersects this bounding box
    pub fn intersects_sphere(&self, center: Coordinate3D, radius: f32) -> bool {
        let closest_x = center.x.clamp(self.min.x, self.max.x);
        let closest_y = center.y.clamp(self.min.y, self.max.y);
        let closest_z = center.z.clamp(self.min.z, self.max.z);
        
        let distance_sq = (center.x - closest_x).powi(2) +
                         (center.y - closest_y).powi(2) +
                         (center.z - closest_z).powi(2);
        
        distance_sq <= radius * radius
    }
    
    /// Get the center of the bounding box
    pub fn center(&self) -> Coordinate3D {
        Coordinate3D {
            x: (self.min.x + self.max.x) / 2.0,
            y: (self.min.y + self.max.y) / 2.0,
            z: (self.min.z + self.max.z) / 2.0,
        }
    }
    
    /// Get the size of the bounding box
    pub fn size(&self) -> Coordinate3D {
        self.max - self.min
    }
}

// Arithmetic operations for Coordinate3D
impl Add for Coordinate3D {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Coordinate3D {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Coordinate3D {
    type Output = Self;
    
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f32> for Coordinate3D {
    type Output = Self;
    
    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_coordinate_distance() {
        let c1 = Coordinate3D::new(0.0, 0.0, 0.0);
        let c2 = Coordinate3D::new(1.0, 1.0, 1.0);
        
        let distance = c1.distance(c2);
        assert!((distance - 1.732).abs() < 0.001); // √3 ≈ 1.732
    }
    
    #[test]
    fn test_bounding_box_contains() {
        let bbox = BoundingBox3D::new(
            Coordinate3D::new(0.2, 0.2, 0.2),
            Coordinate3D::new(0.8, 0.8, 0.8)
        );
        
        assert!(bbox.contains(Coordinate3D::new(0.5, 0.5, 0.5)));
        assert!(!bbox.contains(Coordinate3D::new(0.1, 0.5, 0.5)));
    }
    
    #[test]
    fn test_coordinate_clamping() {
        let c = Coordinate3D::new(1.5, -0.5, 0.5);
        assert_eq!(c.x, 1.0);
        assert_eq!(c.y, 0.0);
        assert_eq!(c.z, 0.5);
    }
}
//! Adaptive spatial architecture for LingoDB
//! 
//! Instead of enforcing rigid semantic consistency, this module provides
//! an adaptive system that:
//! 1. Learns from existing patterns
//! 2. Finds optimal positions for new morphemes
//! 3. Allows the space to evolve naturally
//! 4. Maintains local coherence without global rigidity

use super::{LinguisticNode, NodeFlags, Layer, EtymologyOrigin, MorphemeType, Coordinate3D, NodeId};
use std::collections::HashMap;

/// Adaptive spatial manager that learns from existing patterns
pub struct AdaptiveSpatialManager {
    /// Learned spatial patterns
    spatial_patterns: SpatialPatterns,
    /// Flexibility parameters
    pub flexibility: FlexibilityParams,
    /// Historical positions for pattern learning
    pub position_history: Vec<(String, Coordinate3D, MorphemeType)>,
}

/// Learned patterns from existing morpheme positions
#[derive(Debug, Clone)]
pub struct SpatialPatterns {
    /// Average positions by morpheme type
    type_centroids: HashMap<MorphemeType, Coordinate3D>,
    /// Etymology clustering patterns
    etymology_clusters: HashMap<EtymologyOrigin, Vec<Coordinate3D>>,
    /// Semantic gradient vectors (learned from oppositions)
    gradient_vectors: Vec<GradientVector>,
    /// Local density maps
    density_field: DensityField,
}

impl SpatialPatterns {
    /// Create a new empty spatial patterns instance
    pub fn new() -> Self {
        Self {
            type_centroids: HashMap::new(),
            etymology_clusters: HashMap::new(),
            gradient_vectors: Vec::new(),
            density_field: DensityField {
                density_grid: vec![vec![vec![0.0; 20]; 20]; 20],
                resolution: 20,
            },
        }
    }
}

/// A learned semantic gradient
#[derive(Debug, Clone)]
pub struct GradientVector {
    pub start_concept: String,
    pub end_concept: String,
    pub vector: Coordinate3D,
    pub consistency_score: f32,
    pub sample_count: usize,
}

/// Spatial density information
#[derive(Debug, Clone)]
pub struct DensityField {
    /// Grid-based density measurements
    density_grid: Vec<Vec<Vec<f32>>>,
    /// Resolution of the grid
    resolution: usize,
}

/// Parameters controlling adaptation flexibility
#[derive(Debug, Clone)]
pub struct FlexibilityParams {
    /// How much to weight existing patterns vs new positions (0.0-1.0)
    pub pattern_weight: f32,
    /// Minimum distance between morphemes
    pub min_separation: f32,
    /// How much to allow deviation from type centroids
    pub type_deviation: f32,
    /// Whether to allow pattern evolution
    pub allow_drift: bool,
    /// Learning rate for pattern updates
    pub learning_rate: f32,
}

impl Default for FlexibilityParams {
    fn default() -> Self {
        Self {
            pattern_weight: 0.7,      // 70% existing patterns, 30% new
            min_separation: 0.01,     // Minimum distance between morphemes
            type_deviation: 0.2,      // Allow 20% deviation from type centers
            allow_drift: true,        // Let patterns evolve
            learning_rate: 0.1,       // Gradual adaptation
        }
    }
}

impl AdaptiveSpatialManager {
    /// Create a new adaptive spatial manager
    pub fn new() -> Self {
        Self {
            spatial_patterns: SpatialPatterns::new(),
            flexibility: FlexibilityParams::default(),
            position_history: Vec::new(),
        }
    }

    /// Learn patterns from existing morphemes
    pub fn learn_from_database(&mut self, morphemes: &[(String, LinguisticNode)]) {
        // Clear and rebuild patterns
        self.position_history.clear();
        
        // Collect positions by type
        let mut by_type: HashMap<MorphemeType, Vec<Coordinate3D>> = HashMap::new();
        let mut by_etymology: HashMap<EtymologyOrigin, Vec<Coordinate3D>> = HashMap::new();
        
        for (word, node) in morphemes {
            let pos = node.position;
            by_type.entry(node.morpheme_type).or_default().push(pos);
            
            // Infer etymology from flags (simplified)
            let etymology = if node.flags.contains(NodeFlags::IS_TECHNICAL) {
                EtymologyOrigin::Modern
            } else {
                EtymologyOrigin::Unknown
            };
            by_etymology.entry(etymology).or_default().push(pos);
            
            self.position_history.push((word.clone(), pos, node.morpheme_type));
        }
        
        // Calculate centroids
        for (morph_type, positions) in by_type {
            if let Some(centroid) = calculate_centroid(&positions) {
                self.spatial_patterns.type_centroids.insert(morph_type, centroid);
            }
        }
        
        // Store etymology clusters
        self.spatial_patterns.etymology_clusters = by_etymology;
        
        // Learn gradient patterns
        self.learn_gradient_patterns();
        
        // Build density field
        self.spatial_patterns.density_field = self.build_density_field();
    }

    /// Find optimal position for a new morpheme
    pub fn find_optimal_position(
        &self,
        morpheme: &str,
        morph_type: MorphemeType,
        etymology: EtymologyOrigin,
        semantic_hints: &[SemanticHint],
    ) -> Coordinate3D {
        // Start with base position from type
        let mut position = self.get_type_base_position(morph_type);
        
        // Adjust for etymology clustering
        position = self.adjust_for_etymology(position, etymology);
        
        // Apply semantic hints (e.g., "similar to X", "opposite of Y")
        for hint in semantic_hints {
            position = self.apply_semantic_hint(position, hint);
        }
        
        // Ensure minimum separation from existing morphemes
        position = self.ensure_separation(position);
        
        // Add controlled randomness for variety
        position = self.add_controlled_noise(position, morph_type);
        
        position
    }

    /// Update patterns when new morphemes are added (if drift is allowed)
    pub fn adapt_to_new_morpheme(
        &mut self,
        morpheme: &str,
        position: Coordinate3D,
        morph_type: MorphemeType,
    ) {
        if !self.flexibility.allow_drift {
            return;
        }
        
        // Add to history
        self.position_history.push((morpheme.to_string(), position, morph_type));
        
        // Update type centroid with learning rate
        if let Some(centroid) = self.spatial_patterns.type_centroids.get_mut(&morph_type) {
            let delta = Coordinate3D {
                x: (position.x - centroid.x) * self.flexibility.learning_rate,
                y: (position.y - centroid.y) * self.flexibility.learning_rate,
                z: (position.z - centroid.z) * self.flexibility.learning_rate,
            };
            
            centroid.x += delta.x;
            centroid.y += delta.y;
            centroid.z += delta.z;
        }
        
        // Update density field
        self.update_density_at(position);
    }

    /// Check if a new morpheme would disrupt existing patterns
    pub fn assess_disruption(
        &self,
        position: Coordinate3D,
        morph_type: MorphemeType,
    ) -> DisruptionAssessment {
        let mut assessment = DisruptionAssessment::default();
        
        // Check distance from type centroid
        if let Some(centroid) = self.spatial_patterns.type_centroids.get(&morph_type) {
            let distance = calculate_distance(&position, centroid);
            assessment.centroid_deviation = distance;
            assessment.within_normal_range = distance < self.flexibility.type_deviation;
        }
        
        // Check density at position
        assessment.local_density = self.get_density_at(position);
        assessment.is_overcrowded = assessment.local_density > 0.8;
        
        // Check gradient consistency
        assessment.gradient_consistency = self.check_gradient_consistency(position);
        
        assessment
    }

    // Private helper methods

    fn get_type_base_position(&self, morph_type: MorphemeType) -> Coordinate3D {
        self.spatial_patterns.type_centroids.get(&morph_type).copied()
            .unwrap_or_else(|| {
                // Fallback positions if no centroid learned yet
                match morph_type {
                    MorphemeType::Prefix => Coordinate3D { x: 0.2, y: 0.5, z: 0.37 },
                    MorphemeType::Suffix => Coordinate3D { x: 0.8, y: 0.5, z: 0.37 },
                    MorphemeType::Root => Coordinate3D { x: 0.5, y: 0.5, z: 0.37 },
                    _ => Coordinate3D { x: 0.5, y: 0.5, z: 0.37 },
                }
            })
    }

    fn adjust_for_etymology(&self, mut position: Coordinate3D, etymology: EtymologyOrigin) -> Coordinate3D {
        if let Some(cluster_positions) = self.spatial_patterns.etymology_clusters.get(&etymology) {
            if let Some(cluster_centroid) = calculate_centroid(cluster_positions) {
                // Move slightly toward etymology cluster
                let weight = 0.3; // 30% influence from etymology
                position.x = position.x * (1.0 - weight) + cluster_centroid.x * weight;
                position.y = position.y * (1.0 - weight) + cluster_centroid.y * weight;
            }
        }
        position
    }

    fn apply_semantic_hint(&self, mut position: Coordinate3D, hint: &SemanticHint) -> Coordinate3D {
        match hint {
            SemanticHint::SimilarTo(target) => {
                // Move toward the target
                if let Some((_, target_pos, _)) = self.position_history.iter()
                    .find(|(word, _, _)| word == target) {
                    let weight = 0.4; // 40% influence
                    position.x = position.x * (1.0 - weight) + target_pos.x * weight;
                    position.y = position.y * (1.0 - weight) + target_pos.y * weight;
                }
            }
            SemanticHint::OppositeTo(target) => {
                // Find the target and move in opposite direction
                if let Some((_, target_pos, _)) = self.position_history.iter()
                    .find(|(word, _, _)| word == target) {
                    // Use learned gradient vectors if available
                    if let Some(gradient) = self.find_relevant_gradient(target) {
                        position.x = target_pos.x + gradient.vector.x;
                        position.y = target_pos.y + gradient.vector.y;
                        position.z = target_pos.z + gradient.vector.z;
                    }
                }
            }
            SemanticHint::Between(word1, word2) => {
                // Position between two concepts
                let pos1 = self.position_history.iter()
                    .find(|(w, _, _)| w == word1).map(|(_, p, _)| p);
                let pos2 = self.position_history.iter()
                    .find(|(w, _, _)| w == word2).map(|(_, p, _)| p);
                
                if let (Some(p1), Some(p2)) = (pos1, pos2) {
                    position.x = (p1.x + p2.x) / 2.0;
                    position.y = (p1.y + p2.y) / 2.0;
                    position.z = (p1.z + p2.z) / 2.0;
                }
            }
        }
        position
    }

    fn ensure_separation(&self, mut position: Coordinate3D) -> Coordinate3D {
        let mut iterations = 0;
        let max_iterations = 10;
        
        while iterations < max_iterations {
            let mut too_close = false;
            let mut repulsion = Coordinate3D { x: 0.0, y: 0.0, z: 0.0 };
            
            for (_, pos, _) in &self.position_history {
                let distance = calculate_distance(&position, pos);
                if distance < self.flexibility.min_separation {
                    too_close = true;
                    // Calculate repulsion vector
                    if distance > 0.0 {
                        let factor = (self.flexibility.min_separation - distance) / distance;
                        repulsion.x += (position.x - pos.x) * factor;
                        repulsion.y += (position.y - pos.y) * factor;
                        repulsion.z += (position.z - pos.z) * factor;
                    }
                }
            }
            
            if !too_close {
                break;
            }
            
            // Apply repulsion
            position.x += repulsion.x * 0.1;
            position.y += repulsion.y * 0.1;
            position.z += repulsion.z * 0.1;
            
            iterations += 1;
        }
        
        position
    }

    fn add_controlled_noise(&self, mut position: Coordinate3D, morph_type: MorphemeType) -> Coordinate3D {
        // Add small random variation based on morpheme type
        let noise_scale = match morph_type {
            MorphemeType::Prefix | MorphemeType::Suffix => 0.02,
            MorphemeType::Root => 0.05, // More variation for roots
            _ => 0.03,
        };
        
        position.x += (rand() - 0.5) * noise_scale;
        position.y += (rand() - 0.5) * noise_scale;
        // Keep Z constant for layer
        
        position
    }

    fn learn_gradient_patterns(&mut self) {
        // This would analyze opposition pairs and learn gradient vectors
        // For now, simplified implementation
        self.spatial_patterns.gradient_vectors.clear();
        
        // Example: Learn from known oppositions
        let oppositions = vec![
            ("in", "out"),
            ("up", "down"),
            ("pre", "post"),
            ("micro", "macro"),
        ];
        
        for (word1, word2) in oppositions {
            if let (Some((_, pos1, _)), Some((_, pos2, _))) = (
                self.position_history.iter().find(|(w, _, _)| w == word1),
                self.position_history.iter().find(|(w, _, _)| w == word2),
            ) {
                let vector = Coordinate3D {
                    x: pos2.x - pos1.x,
                    y: pos2.y - pos1.y,
                    z: pos2.z - pos1.z,
                };
                
                self.spatial_patterns.gradient_vectors.push(GradientVector {
                    start_concept: word1.to_string(),
                    end_concept: word2.to_string(),
                    vector,
                    consistency_score: 1.0, // Would be calculated from samples
                    sample_count: 1,
                });
            }
        }
    }

    fn build_density_field(&self) -> DensityField {
        let resolution = 20; // 20x20x20 grid
        let mut grid = vec![vec![vec![0.0; resolution]; resolution]; resolution];
        
        // Calculate density based on morpheme positions
        for (_, pos, _) in &self.position_history {
            let ix = ((pos.x * resolution as f32) as usize).min(resolution - 1);
            let iy = ((pos.y * resolution as f32) as usize).min(resolution - 1);
            let iz = ((pos.z * resolution as f32) as usize).min(resolution - 1);
            
            // Add Gaussian influence around the position
            for di in 0..3 {
                for dj in 0..3 {
                    for dk in 0..3 {
                        let i = (ix + di).saturating_sub(1).min(resolution - 1);
                        let j = (iy + dj).saturating_sub(1).min(resolution - 1);
                        let k = (iz + dk).saturating_sub(1).min(resolution - 1);
                        
                        let dist = ((di as f32 - 1.0).powi(2) + 
                                   (dj as f32 - 1.0).powi(2) + 
                                   (dk as f32 - 1.0).powi(2)).sqrt();
                        
                        grid[i][j][k] += (-dist * dist).exp();
                    }
                }
            }
        }
        
        DensityField { density_grid: grid, resolution }
    }

    fn update_density_at(&mut self, position: Coordinate3D) {
        // Update density field incrementally
        // Simplified for now
    }

    fn get_density_at(&self, position: Coordinate3D) -> f32 {
        let grid = &self.spatial_patterns.density_field.density_grid;
        let res = self.spatial_patterns.density_field.resolution;
        
        let ix = ((position.x * res as f32) as usize).min(res - 1);
        let iy = ((position.y * res as f32) as usize).min(res - 1);
        let iz = ((position.z * res as f32) as usize).min(res - 1);
        
        grid[ix][iy][iz]
    }

    fn check_gradient_consistency(&self, position: Coordinate3D) -> f32 {
        // Check if position maintains gradient patterns
        // Returns 0.0-1.0 consistency score
        1.0 // Simplified
    }

    fn find_relevant_gradient(&self, concept: &str) -> Option<&GradientVector> {
        self.spatial_patterns.gradient_vectors.iter()
            .find(|g| g.start_concept == concept || g.end_concept == concept)
    }

    /// Post-upload calibration: globally optimize spatial positions
    pub fn calibrate_spatial_layout(&mut self, iterations: usize) -> CalibrationResults {
        println!("🔧 Starting spatial calibration...");
        
        let mut results = CalibrationResults {
            initial_disruption_score: self.calculate_global_disruption_score(),
            final_disruption_score: 0.0,
            iterations_completed: 0,
            morphemes_repositioned: 0,
            convergence_achieved: false,
        };

        for iteration in 0..iterations {
            let improvement = self.calibration_iteration();
            results.morphemes_repositioned += improvement.repositioned_count;
            
            if improvement.max_improvement < 0.001 {
                results.convergence_achieved = true;
                results.iterations_completed = iteration + 1;
                break;
            }
            
            if (iteration + 1) % 10 == 0 {
                println!("  Iteration {}/{}: {:.3} avg improvement", 
                    iteration + 1, iterations, improvement.max_improvement);
            }
        }
        
        results.final_disruption_score = self.calculate_global_disruption_score();
        results.iterations_completed = iterations.min(results.iterations_completed);
        
        println!("✓ Calibration complete: {:.2}% disruption reduction", 
            (results.initial_disruption_score - results.final_disruption_score) / results.initial_disruption_score * 100.0);
        
        results
    }

    /// Single calibration iteration using local optimization
    fn calibration_iteration(&mut self) -> CalibrationIteration {
        let mut repositioned_count = 0;
        let mut max_improvement: f32 = 0.0;
        
        // Get all morphemes from position history
        let morphemes_to_optimize: Vec<_> = self.position_history.clone();
        
        for (morpheme, current_pos, morph_type) in morphemes_to_optimize {
            // Calculate current disruption at this position
            let current_score = self.calculate_position_score(current_pos, morph_type);
            
            // Try local adjustments
            let candidates = self.generate_position_candidates(current_pos, morph_type);
            let mut best_pos = current_pos;
            let mut best_score = current_score;
            
            for candidate_pos in candidates {
                let candidate_score = self.calculate_position_score(candidate_pos, morph_type);
                if candidate_score > best_score {
                    best_pos = candidate_pos;
                    best_score = candidate_score;
                }
            }
            
            // If we found a better position, update it
            if best_score > current_score + 0.01 {
                self.update_morpheme_position(&morpheme, best_pos, morph_type);
                repositioned_count += 1;
                max_improvement = max_improvement.max(best_score - current_score);
            }
        }
        
        CalibrationIteration {
            repositioned_count,
            max_improvement,
        }
    }

    /// Calculate global disruption score across all morphemes
    pub fn calculate_global_disruption_score(&self) -> f32 {
        let mut total_disruption = 0.0;
        let mut count = 0;
        
        for (_, position, morph_type) in &self.position_history {
            let assessment = self.assess_disruption(*position, *morph_type);
            if !assessment.within_normal_range || assessment.is_overcrowded {
                total_disruption += 1.0;
            }
            count += 1;
        }
        
        if count > 0 { total_disruption / count as f32 } else { 0.0 }
    }

    /// Calculate quality score for a position
    fn calculate_position_score(&self, position: Coordinate3D, morph_type: MorphemeType) -> f32 {
        let assessment = self.assess_disruption(position, morph_type);
        
        let mut score = 1.0;
        
        // Penalty for overcrowding
        if assessment.is_overcrowded {
            score -= 0.5;
        }
        
        // Penalty for being outside normal range
        if !assessment.within_normal_range {
            score -= 0.3;
        }
        
        // Reward for good separation
        score += self.calculate_separation_bonus(position);
        
        // Reward for pattern consistency
        score += self.check_gradient_consistency(position) * 0.2;
        
        score.max(0.0)
    }

    /// Generate candidate positions for optimization
    fn generate_position_candidates(&self, center: Coordinate3D, _morph_type: MorphemeType) -> Vec<Coordinate3D> {
        let mut candidates = Vec::new();
        let step_size = 0.05;
        
        // Generate positions in a small radius around current position
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                for dz in [-1, 0, 1] {
                    if dx == 0 && dy == 0 && dz == 0 { continue; }
                    
                    let candidate = Coordinate3D {
                        x: (center.x + dx as f32 * step_size).clamp(0.0, 1.0),
                        y: (center.y + dy as f32 * step_size).clamp(0.0, 1.0),
                        z: (center.z + dz as f32 * step_size).clamp(0.0, 1.0),
                    };
                    candidates.push(candidate);
                }
            }
        }
        
        candidates
    }

    /// Update morpheme position in history
    fn update_morpheme_position(&mut self, morpheme: &str, new_pos: Coordinate3D, morph_type: MorphemeType) {
        // Find and update in position history
        for (hist_morpheme, hist_pos, hist_type) in &mut self.position_history {
            if hist_morpheme == morpheme && *hist_type == morph_type {
                *hist_pos = new_pos;
                break;
            }
        }
        
        // Update spatial patterns - add to existing centroids
        let positions = vec![new_pos];
        if let Some(centroid) = calculate_centroid(&positions) {
            self.spatial_patterns.type_centroids.insert(morph_type, centroid);
        }
    }

    /// Calculate separation bonus for good spacing
    fn calculate_separation_bonus(&self, position: Coordinate3D) -> f32 {
        let mut min_distance = f32::MAX;
        
        for (_, other_pos, _) in &self.position_history {
            let distance = ((position.x - other_pos.x).powi(2) + 
                           (position.y - other_pos.y).powi(2) + 
                           (position.z - other_pos.z).powi(2)).sqrt();
            min_distance = min_distance.min(distance);
        }
        
        // Bonus for maintaining good separation
        if min_distance > self.flexibility.min_separation * 2.0 {
            0.2
        } else if min_distance > self.flexibility.min_separation {
            0.1
        } else {
            0.0
        }
    }
}

/// Semantic hints for positioning new morphemes
#[derive(Debug, Clone)]
pub enum SemanticHint {
    /// Position similar to another morpheme
    SimilarTo(String),
    /// Position opposite to another morpheme
    OppositeTo(String),
    /// Position between two morphemes
    Between(String, String),
}

/// Assessment of how much a new morpheme would disrupt existing patterns
#[derive(Debug, Default)]
pub struct DisruptionAssessment {
    /// Distance from expected type centroid
    pub centroid_deviation: f32,
    /// Whether position is within normal range
    pub within_normal_range: bool,
    /// Density at proposed position
    pub local_density: f32,
    /// Whether area is overcrowded
    pub is_overcrowded: bool,
    /// Consistency with learned gradients
    pub gradient_consistency: f32,
}

/// Results of spatial calibration
#[derive(Debug, Clone)]
pub struct CalibrationResults {
    pub initial_disruption_score: f32,
    pub final_disruption_score: f32,
    pub iterations_completed: usize,
    pub morphemes_repositioned: usize,
    pub convergence_achieved: bool,
}

/// Results of a single calibration iteration
#[derive(Debug, Clone)]
struct CalibrationIteration {
    repositioned_count: usize,
    max_improvement: f32,
}

// Helper functions

fn calculate_centroid(positions: &[Coordinate3D]) -> Option<Coordinate3D> {
    if positions.is_empty() {
        return None;
    }
    
    let sum = positions.iter().fold(
        Coordinate3D { x: 0.0, y: 0.0, z: 0.0 },
        |acc, pos| Coordinate3D {
            x: acc.x + pos.x,
            y: acc.y + pos.y,
            z: acc.z + pos.z,
        }
    );
    
    let n = positions.len() as f32;
    Some(Coordinate3D {
        x: sum.x / n,
        y: sum.y / n,
        z: sum.z / n,
    })
}

fn calculate_distance(a: &Coordinate3D, b: &Coordinate3D) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let dz = b.z - a.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn rand() -> f32 {
    // Simple pseudo-random for demonstration
    // In production, use a proper RNG
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(1))
        .subsec_nanos();
    (nanos % 1000) as f32 / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_adaptive_spatial_manager_creation() {
        let manager = AdaptiveSpatialManager::new();
        assert_eq!(manager.position_history.len(), 0);
        assert!(manager.spatial_patterns.type_centroids.is_empty());
    }
    
    #[test]
    fn test_find_optimal_position() {
        let mut manager = AdaptiveSpatialManager::new();
        
        // Add some history
        manager.position_history.push((
            "tech".to_string(),
            Coordinate3D::new(0.5, 0.8, 0.375),
            MorphemeType::Root
        ));
        
        let pos = manager.find_optimal_position(
            "technology",
            MorphemeType::Root,
            EtymologyOrigin::Greek,
            &[]  // Empty semantic hints
        );
        
        // Should be in morpheme layer (approximately)
        assert!((pos.z - 0.375).abs() < 0.01);
        // Should have reasonable X,Y coordinates
        assert!(pos.x >= 0.0 && pos.x <= 1.0);
        assert!(pos.y >= 0.0 && pos.y <= 1.0);
    }
    
    #[test]
    fn test_learn_patterns() {
        let mut manager = AdaptiveSpatialManager::new();
        
        // Add some morphemes to learn from
        manager.position_history.push((
            "bio".to_string(),
            Coordinate3D::new(0.3, 0.8, 0.375),
            MorphemeType::Root
        ));
        manager.position_history.push((
            "geo".to_string(),
            Coordinate3D::new(0.35, 0.82, 0.375),
            MorphemeType::Root
        ));
        
        // Convert to format expected by learn_from_database
        let morphemes: Vec<(String, LinguisticNode)> = vec![];
        manager.learn_from_database(&morphemes);
        
        // Should have updated patterns (though empty input means no change)
        assert!(manager.spatial_patterns.type_centroids.is_empty());
    }
    
    #[test]
    fn test_flexibility_params() {
        let manager = AdaptiveSpatialManager::new();
        
        // Check default flexibility values
        assert!(manager.flexibility.pattern_weight >= 0.0);
        assert!(manager.flexibility.pattern_weight <= 1.0);
        assert!(manager.flexibility.min_separation > 0.0);
        assert!(manager.flexibility.type_deviation > 0.0);
        assert!(manager.flexibility.learning_rate > 0.0);
        assert!(manager.flexibility.learning_rate <= 1.0);
    }
}
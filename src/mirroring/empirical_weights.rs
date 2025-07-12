// Copyright 2025 Roberto Antunes
//
// Licensed under the Functional Source License, Version 1.1 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://github.com/RobAntunes/lingodb/blob/main/LICENSE
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Empirical weight calculation for morpheme composition
//! 
//! This module analyzes the actual database to determine optimal weights
//! for morpheme composition based on spatial distributions and co-occurrence patterns.

use std::collections::HashMap;
use crate::core::{Layer, MorphemeType, NodeId, Coordinate3D};
use crate::core::error::LingoError;
use crate::storage::LingoDatabase;
use crate::engine::LingoExecutor;
use crate::query::QueryBuilder;
use super::CompositionWeights;

/// Calculator for empirically determining composition weights
pub struct EmpiricalWeightCalculator<'a> {
    db: &'a LingoDatabase,
}

impl<'a> EmpiricalWeightCalculator<'a> {
    pub fn new(db: &'a LingoDatabase) -> Self {
        Self { db }
    }
    
    /// Calculate weights based on database analysis
    pub fn calculate_weights(&self, executor: &mut LingoExecutor) -> Result<CompositionWeights, LingoError> {
        println!("Calculating empirical weights from database...");
        
        // Step 1: Analyze morpheme type distributions
        let morpheme_type_weights = self.analyze_morpheme_distributions(executor)?;
        
        // Step 2: Analyze spatial coherence patterns
        let spatial_weight = self.analyze_spatial_coherence(executor)?;
        
        // Step 3: Analyze morphological consistency
        let morphological_weight = self.analyze_morphological_consistency(executor)?;
        
        // Step 4: Analyze semantic clustering
        let semantic_weight = self.analyze_semantic_clustering(executor)?;
        
        Ok(CompositionWeights {
            morpheme_type_weights,
            spatial_weight,
            morphological_weight,
            semantic_weight,
        })
    }
    
    /// Analyze the distribution of morpheme types and their importance
    fn analyze_morpheme_distributions(
        &self, 
        executor: &mut LingoExecutor
    ) -> Result<HashMap<MorphemeType, f32>, LingoError> {
        let mut type_counts: HashMap<MorphemeType, usize> = HashMap::new();
        let mut type_positions: HashMap<MorphemeType, Vec<Coordinate3D>> = HashMap::new();
        
        // Sample morphemes from the database
        let sample_words = vec![
            "manager", "developer", "organizing", "created", "transformation",
            "interactive", "processing", "analyzer", "builder", "coordinator"
        ];
        
        for word in sample_words {
            // Try to find the word
            if let Ok(word_result) = executor.execute(&QueryBuilder::find(word).compile()) {
                if let Some(word_id) = word_result.nodes.as_slice().first() {
                    // Get morphemes by going layer down
                    if let Ok(morpheme_result) = executor.execute(
                        &QueryBuilder::load(*word_id).layer_down().compile()
                    ) {
                        for morpheme_id in morpheme_result.nodes.as_slice() {
                            if let Ok(morpheme_node) = self.db.get_node(*morpheme_id) {
                                // Infer morpheme type from the morpheme text
                                if let Ok(morpheme_text) = self.db.get_node_word(*morpheme_id) {
                                    let morph_type = self.infer_morpheme_type(&morpheme_text, word);
                                    
                                    *type_counts.entry(morph_type).or_insert(0) += 1;
                                    type_positions.entry(morph_type)
                                        .or_insert_with(Vec::new)
                                        .push(morpheme_node.position);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Calculate weights based on frequency and spatial variance
        let total_morphemes: usize = type_counts.values().sum();
        let mut weights = HashMap::new();
        
        for (morph_type, count) in type_counts {
            let frequency_weight = count as f32 / total_morphemes as f32;
            
            // Calculate spatial variance for this type
            let spatial_variance = if let Some(positions) = type_positions.get(&morph_type) {
                self.calculate_spatial_variance(positions)
            } else {
                1.0
            };
            
            // Weight = frequency * (1 / variance) to favor consistent positioning
            let weight = frequency_weight * (1.0 / (1.0 + spatial_variance));
            weights.insert(morph_type, weight);
        }
        
        // Normalize weights
        let sum: f32 = weights.values().sum();
        if sum > 0.0 {
            for weight in weights.values_mut() {
                *weight /= sum;
            }
        }
        
        Ok(weights)
    }
    
    /// Analyze how well morphemes maintain spatial coherence when composed
    fn analyze_spatial_coherence(
        &self,
        executor: &mut LingoExecutor
    ) -> Result<f32, LingoError> {
        let mut coherence_scores = Vec::new();
        
        // Test words with known decompositions
        let test_pairs = vec![
            ("manager", vec!["manage", "er"]),
            ("developer", vec!["develop", "er"]),
            ("organization", vec!["organize", "ation"]),
        ];
        
        for (word, expected_morphemes) in test_pairs {
            if let Ok(word_result) = executor.execute(&QueryBuilder::find(word).compile()) {
                if let Some(word_id) = word_result.nodes.as_slice().first() {
                    if let Ok(word_node) = self.db.get_node(*word_id) {
                        // Get actual morpheme positions
                        let mut morpheme_positions = Vec::new();
                        
                        for morpheme in expected_morphemes {
                            if let Ok(morph_result) = executor.execute(
                                &QueryBuilder::find(morpheme).layer(Layer::Morphemes).compile()
                            ) {
                                if let Some(morph_id) = morph_result.nodes.as_slice().first() {
                                    if let Ok(morph_node) = self.db.get_node(*morph_id) {
                                        morpheme_positions.push(morph_node.position);
                                    }
                                }
                            }
                        }
                        
                        if !morpheme_positions.is_empty() {
                            // Calculate composed position
                            let composed = self.weighted_average_position(&morpheme_positions);
                            
                            // Calculate distance from actual word position
                            let distance = euclidean_distance(word_node.position, composed);
                            let coherence = 1.0 / (1.0 + distance);
                            coherence_scores.push(coherence);
                        }
                    }
                }
            }
        }
        
        // Return average coherence as the spatial weight
        if coherence_scores.is_empty() {
            Ok(0.3) // Default
        } else {
            Ok(coherence_scores.iter().sum::<f32>() / coherence_scores.len() as f32)
        }
    }
    
    /// Analyze morphological consistency patterns
    fn analyze_morphological_consistency(
        &self,
        _executor: &mut LingoExecutor
    ) -> Result<f32, LingoError> {
        // For now, return a reasonable default
        // In a full implementation, this would analyze:
        // - How consistently morphemes combine
        // - Violation rates of morphological rules
        // - Success rates of round-trip decomposition/composition
        Ok(0.5)
    }
    
    /// Analyze semantic clustering patterns
    fn analyze_semantic_clustering(
        &self,
        _executor: &mut LingoExecutor
    ) -> Result<f32, LingoError> {
        // For now, return a reasonable default
        // In a full implementation, this would analyze:
        // - How tightly words cluster by semantic domain
        // - Correlation between spatial distance and semantic similarity
        Ok(0.2)
    }
    
    /// Infer morpheme type from its form and position in word
    fn infer_morpheme_type(&self, morpheme: &str, word: &str) -> MorphemeType {
        if word.ends_with(morpheme) && morpheme != word {
            match morpheme {
                "er" | "or" | "ist" | "ant" | "ian" => MorphemeType::AgentSuffix,
                "ize" | "fy" | "ate" | "en" => MorphemeType::VerbSuffix,
                "ed" | "ing" | "s" => MorphemeType::TenseSuffix,
                _ => MorphemeType::Suffix,
            }
        } else if word.starts_with(morpheme) && morpheme != word {
            MorphemeType::Prefix
        } else {
            MorphemeType::Root
        }
    }
    
    /// Calculate spatial variance for a set of positions
    fn calculate_spatial_variance(&self, positions: &[Coordinate3D]) -> f32 {
        if positions.len() < 2 {
            return 0.0;
        }
        
        // Calculate centroid
        let centroid = self.weighted_average_position(positions);
        
        // Calculate variance as average squared distance from centroid
        let variance: f32 = positions.iter()
            .map(|pos| euclidean_distance(*pos, centroid).powi(2))
            .sum::<f32>() / positions.len() as f32;
        
        variance
    }
    
    /// Calculate weighted average position
    fn weighted_average_position(&self, positions: &[Coordinate3D]) -> Coordinate3D {
        let sum_x: f32 = positions.iter().map(|p| p.x).sum();
        let sum_y: f32 = positions.iter().map(|p| p.y).sum();
        let sum_z: f32 = positions.iter().map(|p| p.z).sum();
        let count = positions.len() as f32;
        
        Coordinate3D {
            x: sum_x / count,
            y: sum_y / count,
            z: sum_z / count,
        }
    }
}

/// Calculate Euclidean distance between two 3D points
fn euclidean_distance(a: Coordinate3D, b: Coordinate3D) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
}
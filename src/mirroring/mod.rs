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

//! Mirroring decompose module - bidirectional morphological composition
//! 
//! This module implements the revolutionary mirroring decompose architecture
//! that enables bidirectional morphological analysis, pattern synthesis,
//! and oppositional reasoning.

use std::collections::HashMap;
use std::sync::Arc;
use lru::LruCache;
use serde::{Serialize, Deserialize};

use crate::core::{LinguisticNode, NodeId, Layer, Coordinate3D, MorphemeType};
use crate::core::error::LingoError;
use crate::storage::LingoDatabase;
use crate::engine::LingoExecutor;
use crate::query::QueryBuilder;
use crate::morphology::{MorphemeAnalysis, decompose_word_to_morphemes, 
                        calculate_composed_position, preprocess_text};

mod empirical_weights;
mod composition;
mod opposition;
mod synthesis;

pub use empirical_weights::EmpiricalWeightCalculator;
pub use composition::Composer;
pub use opposition::{OppositionEngine, MirrorType, EtymologicalMirrorEngine, EtymologicalMirror, EtymologyProfile, 
                      EtymologyFamily, RoleType, NegationType, BorrowingType};
pub use synthesis::{SynthesisEngine, SynthesisResult};

/// Main mirroring decomposer that handles bidirectional morphological analysis
pub struct MirroringDecomposer {
    db: Arc<LingoDatabase>,
    executor: LingoExecutor,
    morpheme_patterns: HashMap<String, MorphemePattern>,
    opposition_vectors: HashMap<String, Vec<String>>,
    synthesis_cache: LruCache<String, Vec<SynthesisResult>>,
    /// Empirically calculated weights for morpheme composition
    composition_weights: CompositionWeights,
}

#[derive(Debug, Clone)]
pub struct MorphemePattern {
    pub pattern_type: PatternType,
    pub components: Vec<MorphemeComponent>,
    pub mirror_patterns: Vec<String>,
    pub generative_slots: Vec<GenerativeSlot>,
}

#[derive(Debug, Clone)]
pub struct MorphemeComponent {
    pub morpheme: String,
    pub morpheme_type: MorphemeType,
    pub position: Option<Coordinate3D>,
    pub empirical_weight: f32,
}

#[derive(Debug, Clone)]
pub struct GenerativeSlot {
    pub slot_type: SlotType,
    pub compatible_morphemes: Vec<String>,
    pub position_constraints: PositionConstraints,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternType {
    Agent,      // -er, -ant, -ist patterns
    Action,     // -ize, -fy, -ate patterns  
    Negation,   // un-, dis-, de- patterns
    Intensifier, // super-, ultra-, hyper- patterns
    Temporal,   // pre-, post-, re- patterns
    Relational, // inter-, intra-, trans- patterns
}

#[derive(Debug, Clone)]
pub enum SlotType {
    Prefix,
    Root,
    Suffix,
    Infix,
}

#[derive(Debug, Clone)]
pub struct PositionConstraints {
    pub min_position: Coordinate3D,
    pub max_position: Coordinate3D,
    pub preferred_region: Coordinate3D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionWeights {
    /// Weights learned from the database distribution
    pub morpheme_type_weights: HashMap<MorphemeType, f32>,
    /// Spatial coherence weight
    pub spatial_weight: f32,
    /// Morphological consistency weight
    pub morphological_weight: f32,
    /// Semantic similarity weight
    pub semantic_weight: f32,
}

impl Default for CompositionWeights {
    fn default() -> Self {
        // Start with reasonable defaults, then learn empirically
        let mut morpheme_type_weights = HashMap::new();
        morpheme_type_weights.insert(MorphemeType::Root, 0.6);
        morpheme_type_weights.insert(MorphemeType::Prefix, 0.2);
        morpheme_type_weights.insert(MorphemeType::Suffix, 0.2);
        morpheme_type_weights.insert(MorphemeType::AgentSuffix, 0.25);
        morpheme_type_weights.insert(MorphemeType::VerbSuffix, 0.25);
        morpheme_type_weights.insert(MorphemeType::TenseSuffix, 0.15);
        
        Self {
            morpheme_type_weights,
            spatial_weight: 0.3,
            morphological_weight: 0.5,
            semantic_weight: 0.2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MirrorPair {
    pub original: String,
    pub mirror: String,
    pub mirror_type: MirrorType,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub round_trip_success: bool,
    pub spatial_consistency_score: f32,
    pub morpheme_coherence: f32,
    pub alternative_compositions: Vec<String>,
}

impl MirroringDecomposer {
    /// Create a new mirroring decomposer
    pub fn new(database: Arc<LingoDatabase>) -> Result<Self, LingoError> {
        let mut executor = LingoExecutor::new();
        
        // Clone the Arc to get a reference for the executor
        let db_path = "english.lingo"; // This should be configurable
        executor.load_database(db_path)?;
        
        Ok(Self {
            db: database,
            executor,
            morpheme_patterns: HashMap::new(),
            opposition_vectors: HashMap::new(),
            synthesis_cache: LruCache::new(std::num::NonZeroUsize::new(1000).unwrap()),
            composition_weights: CompositionWeights::default(),
        })
    }
    
    /// Learn empirical weights from the database
    pub fn learn_empirical_weights(&mut self) -> Result<(), LingoError> {
        let calculator = EmpiricalWeightCalculator::new(&self.db);
        self.composition_weights = calculator.calculate_weights(&mut self.executor)?;
        Ok(())
    }
    
    /// Forward decomposition - break word into morphemes
    pub fn decompose(&mut self, word: &str) -> Vec<MorphemeAnalysis> {
        decompose_word_to_morphemes(word, &self.db, &mut self.executor)
    }
    
    /// Reverse composition - build words from morphemes
    pub fn compose(&mut self, morphemes: &[String]) -> Vec<String> {
        let composer = Composer::new(&self.db, &self.composition_weights);
        composer.compose_from_morphemes(morphemes, &mut self.executor)
    }
    
    /// Find mirror patterns (opposites) using the new etymological engine
    pub fn find_mirrors(&mut self, word: &str) -> Vec<MirrorPair> {
        // Use the new EtymologicalMirrorEngine for advanced mirror discovery
        let etymological_engine = EtymologicalMirrorEngine::new(self.db.clone());
        
        match etymological_engine.discover_etymological_mirrors(word) {
            Ok(etymological_mirrors) => {
                etymological_mirrors.into_iter()
                    .map(|etym_mirror| MirrorPair {
                        original: etym_mirror.original,
                        mirror: etym_mirror.mirror,
                        mirror_type: etym_mirror.mirror_type,
                        confidence: etym_mirror.confidence,
                    })
                    .collect()
            },
            Err(_) => {
                // Fallback to legacy method if new engine fails
                self.find_mirrors_legacy(word)
            }
        }
    }
    
    /// Legacy mirror finding method for backward compatibility
    fn find_mirrors_legacy(&mut self, word: &str) -> Vec<MirrorPair> {
        let decomposition = self.decompose(word);
        
        let mut mirrors = Vec::new();
        
        for morpheme_analysis in &decomposition {
            let opposite_morphemes = {
                let opposition_engine = OppositionEngine::new(&self.db);
                opposition_engine.find_opposite_morphemes(morpheme_analysis)
            };
            
            for opposite_set in opposite_morphemes {
                let mirror_candidates = self.compose(&opposite_set);
                
                for mirror_word in mirror_candidates {
                    let mirror_type = {
                        let opposition_engine = OppositionEngine::new(&self.db);
                        opposition_engine.classify_mirror_type(
                            morpheme_analysis, 
                            &opposite_set
                        )
                    };
                    
                    let confidence = {
                        let mirror_ratio = opposite_set.len() as f32 / decomposition.len() as f32;
                        let exists_bonus = if let Ok(result) = self.executor.execute(
                            &QueryBuilder::find(&mirror_word).compile()
                        ) {
                            if !result.nodes.is_empty() { 0.3 } else { 0.0 }
                        } else {
                            0.0
                        };
                        (mirror_ratio * 0.7 + exists_bonus).min(1.0)
                    };
                    
                    mirrors.push(MirrorPair {
                        original: word.to_string(),
                        mirror: mirror_word,
                        mirror_type,
                        confidence,
                    });
                }
            }
        }
        
        mirrors
    }
    
    /// Synthesize new functional expressions
    pub fn synthesize_functions(
        &mut self, 
        pattern_type: PatternType, 
        base_morphemes: &[String]
    ) -> Vec<SynthesisResult> {
        // Check cache first
        let cache_key = format!("{:?}:{}", pattern_type, base_morphemes.join(","));
        if let Some(cached) = self.synthesis_cache.get(&cache_key) {
            return cached.clone();
        }
        
        let synthesis_engine = SynthesisEngine::new(&self.db, &self.composition_weights);
        let results = synthesis_engine.synthesize(pattern_type, base_morphemes, &mut self.executor);
        
        // Cache results
        self.synthesis_cache.put(cache_key, results.clone());
        
        results
    }
    
    /// Validate decomposition through round-trip testing
    pub fn validate_decomposition_quality(&mut self, word: &str) -> ValidationResult {
        // Step 1: Decompose the word
        let morphemes = self.decompose(word);
        
        // Step 2: Recompose from morphemes
        let morpheme_strings: Vec<String> = morphemes.iter()
            .map(|m| m.surface_form.clone())
            .collect();
        let recomposed_candidates = {
            let composer = Composer::new(&self.db, &self.composition_weights);
            composer.compose_from_morphemes(&morpheme_strings, &mut self.executor)
        };
        
        // Step 3: Check if original word is in candidates
        let round_trip_success = recomposed_candidates.contains(&word.to_string());
        
        // Step 4: Analyze spatial consistency
        let spatial_consistency_score = if let Ok(word_result) = self.executor.execute(
            &QueryBuilder::find(word).compile()
        ) {
            if let Some(word_node_id) = word_result.nodes.as_slice().first() {
                if let Ok(word_node) = self.db.get_node(*word_node_id) {
                    let composed_position = calculate_composed_position(&morphemes);
                    let distance = euclidean_distance(word_node.position, composed_position);
                    1.0 / (1.0 + distance)
                } else {
                    0.0
                }
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        // Step 5: Calculate morpheme coherence
        let morpheme_coherence = self.calculate_morpheme_coherence(&morphemes);
        
        ValidationResult {
            round_trip_success,
            spatial_consistency_score,
            morpheme_coherence,
            alternative_compositions: recomposed_candidates,
        }
    }
    
    /// Calculate confidence for a mirror pair
    fn calculate_mirror_confidence(
        &mut self,
        original_morphemes: &[MorphemeAnalysis],
        mirror_morphemes: &[String],
        mirror_word: &str
    ) -> f32 {
        // Base confidence on:
        // 1. How many morphemes were successfully mirrored
        // 2. Whether the mirror word exists in the database
        // 3. Spatial coherence between original and mirror
        
        let mirror_ratio = mirror_morphemes.len() as f32 / original_morphemes.len() as f32;
        
        let exists_bonus = if let Ok(result) = self.executor.execute(
            &QueryBuilder::find(mirror_word).compile()
        ) {
            if !result.nodes.is_empty() { 0.3 } else { 0.0 }
        } else {
            0.0
        };
        
        (mirror_ratio * 0.7 + exists_bonus).min(1.0)
    }
    
    /// Calculate morpheme coherence score
    fn calculate_morpheme_coherence(&self, morphemes: &[MorphemeAnalysis]) -> f32 {
        if morphemes.len() < 2 {
            return 1.0;
        }
        
        let mut total_coherence = 0.0;
        let mut pairs = 0;
        
        // Check pairwise coherence
        for i in 0..morphemes.len() {
            for j in (i+1)..morphemes.len() {
                let coherence = self.calculate_pairwise_coherence(&morphemes[i], &morphemes[j]);
                total_coherence += coherence;
                pairs += 1;
            }
        }
        
        if pairs > 0 {
            total_coherence / pairs as f32
        } else {
            1.0
        }
    }
    
    /// Calculate coherence between two morphemes
    fn calculate_pairwise_coherence(
        &self,
        morph1: &MorphemeAnalysis,
        morph2: &MorphemeAnalysis
    ) -> f32 {
        // Check if morpheme types are compatible
        let type_compatibility = match (&morph1.morpheme_type, &morph2.morpheme_type) {
            (MorphemeType::Root, MorphemeType::AgentSuffix) => 0.9,
            (MorphemeType::Root, MorphemeType::VerbSuffix) => 0.9,
            (MorphemeType::Prefix, MorphemeType::Root) => 0.8,
            (MorphemeType::Root, MorphemeType::Suffix) => 0.8,
            _ => 0.5,
        };
        
        // Check spatial coherence if positions available
        let spatial_coherence = if let (Some(pos1), Some(pos2)) = (morph1.position, morph2.position) {
            let distance = euclidean_distance(pos1, pos2);
            1.0 / (1.0 + distance)
        } else {
            0.5
        };
        
        (type_compatibility + spatial_coherence) / 2.0
    }
}

/// Calculate Euclidean distance between two 3D points
fn euclidean_distance(a: Coordinate3D, b: Coordinate3D) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mirroring_decomposer_creation() {
        // This test would require a test database
        // Skipping for now as it needs full setup
    }
}
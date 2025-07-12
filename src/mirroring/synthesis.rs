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

//! Synthesis engine for generating new functional expressions

use std::collections::HashSet;
use crate::core::{LinguisticNode, NodeId, Layer, Coordinate3D, MorphemeType};
use crate::storage::LingoDatabase;
use crate::engine::LingoExecutor;
use crate::query::QueryBuilder;
use crate::plugins::function_extraction::{FunctionalPrimitive, ActionType, TemporalAspect};
use super::{CompositionWeights, PatternType};

#[derive(Debug, Clone)]
pub struct SynthesisResult {
    pub generated_word: String,
    pub predicted_function: FunctionalPrimitive,
    pub morpheme_composition: Vec<String>,
    pub confidence: f32,
    pub spatial_position: Coordinate3D,
}

/// Engine for synthesizing new functional expressions
pub struct SynthesisEngine<'a> {
    db: &'a LingoDatabase,
    weights: &'a CompositionWeights,
}

impl<'a> SynthesisEngine<'a> {
    pub fn new(db: &'a LingoDatabase, weights: &'a CompositionWeights) -> Self {
        Self { db, weights }
    }
    
    /// Synthesize new functional expressions based on pattern type
    pub fn synthesize(
        &self,
        pattern_type: PatternType,
        base_morphemes: &[String],
        executor: &mut LingoExecutor
    ) -> Vec<SynthesisResult> {
        let mut results = Vec::new();
        
        // Find compatible morphemes for this pattern
        let compatible_morphemes = self.find_compatible_morphemes(&pattern_type, base_morphemes, executor);
        
        for morpheme_set in compatible_morphemes {
            // Try different combinations
            let combinations = self.generate_combinations(&morpheme_set, &pattern_type);
            
            for combination in combinations {
                if let Some(result) = self.attempt_synthesis(&combination, &pattern_type, executor) {
                    results.push(result);
                }
            }
        }
        
        // Sort by confidence
        results.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        
        // Limit to top results
        results.truncate(10);
        
        results
    }
    
    /// Find morphemes compatible with the pattern type
    fn find_compatible_morphemes(
        &self,
        pattern_type: &PatternType,
        base_morphemes: &[String],
        _executor: &mut LingoExecutor
    ) -> Vec<Vec<String>> {
        let mut compatible_sets = Vec::new();
        
        match pattern_type {
            PatternType::Agent => {
                // For agent patterns, find roots that can take agent suffixes
                let agent_suffixes = vec!["er", "or", "ist", "ant", "ian"];
                
                for base in base_morphemes {
                    for suffix in &agent_suffixes {
                        let mut set = vec![base.clone()];
                        set.push(suffix.to_string());
                        compatible_sets.push(set);
                    }
                }
            }
            
            PatternType::Action => {
                // For action patterns, find roots that can take verb suffixes
                let verb_suffixes = vec!["ize", "fy", "ate"];
                
                for base in base_morphemes {
                    for suffix in &verb_suffixes {
                        let mut set = vec![base.clone()];
                        set.push(suffix.to_string());
                        compatible_sets.push(set);
                    }
                }
            }
            
            PatternType::Negation => {
                // For negation patterns, add negation prefixes
                let negation_prefixes = vec!["un", "dis", "non", "de"];
                
                for base in base_morphemes {
                    for prefix in &negation_prefixes {
                        let mut set = vec![prefix.to_string()];
                        set.push(base.clone());
                        compatible_sets.push(set);
                    }
                }
            }
            
            PatternType::Intensifier => {
                // For intensifier patterns, add intensifying prefixes
                let intensifier_prefixes = vec!["super", "ultra", "hyper", "mega"];
                
                for base in base_morphemes {
                    for prefix in &intensifier_prefixes {
                        let mut set = vec![prefix.to_string()];
                        set.push(base.clone());
                        compatible_sets.push(set);
                    }
                }
            }
            
            _ => {
                // For other patterns, return base morphemes as-is
                compatible_sets.push(base_morphemes.to_vec());
            }
        }
        
        compatible_sets
    }
    
    /// Generate different ordering combinations
    fn generate_combinations(&self, morphemes: &[String], pattern_type: &PatternType) -> Vec<Vec<String>> {
        let mut combinations = Vec::new();
        
        // For now, just use the natural order based on pattern type
        match pattern_type {
            PatternType::Agent | PatternType::Action => {
                // Root + Suffix order
                combinations.push(morphemes.to_vec());
            }
            PatternType::Negation | PatternType::Intensifier => {
                // Prefix + Root order
                combinations.push(morphemes.to_vec());
            }
            _ => {
                combinations.push(morphemes.to_vec());
            }
        }
        
        combinations
    }
    
    /// Attempt to synthesize a word from morphemes
    fn attempt_synthesis(
        &self,
        morphemes: &[String],
        pattern_type: &PatternType,
        executor: &mut LingoExecutor
    ) -> Option<SynthesisResult> {
        // Compose the word
        let composed_word = self.compose_morphemes(morphemes);
        
        // Calculate spatial position
        let spatial_position = self.calculate_synthetic_position(morphemes, executor);
        
        // Predict the functional primitive
        let predicted_function = self.predict_function(
            &composed_word,
            morphemes,
            pattern_type,
            spatial_position
        );
        
        // Calculate confidence
        let confidence = self.calculate_synthesis_confidence(
            &composed_word,
            morphemes,
            pattern_type,
            executor
        );
        
        // Only return if confidence is above threshold
        if confidence > 0.3 {
            Some(SynthesisResult {
                generated_word: composed_word,
                predicted_function,
                morpheme_composition: morphemes.to_vec(),
                confidence,
                spatial_position,
            })
        } else {
            None
        }
    }
    
    /// Compose morphemes into a word
    fn compose_morphemes(&self, morphemes: &[String]) -> String {
        // Simple concatenation for now
        // In a full implementation, this would apply morphophonological rules
        morphemes.join("")
    }
    
    /// Calculate spatial position for synthetic word
    fn calculate_synthetic_position(
        &self,
        morphemes: &[String],
        executor: &mut LingoExecutor
    ) -> Coordinate3D {
        let mut positions = Vec::new();
        let mut weights = Vec::new();
        
        // Get positions of constituent morphemes
        for morpheme in morphemes {
            if let Ok(result) = executor.execute(
                &QueryBuilder::find(morpheme).layer(Layer::Morphemes).compile()
            ) {
                if let Some(node_id) = result.nodes.as_slice().first() {
                    if let Ok(node) = self.db.get_node(*node_id) {
                        positions.push(node.position);
                        
                        // Use empirical weights if available
                        let morph_type = self.infer_morpheme_type(morpheme);
                        let weight = self.weights.morpheme_type_weights
                            .get(&morph_type)
                            .copied()
                            .unwrap_or(0.5);
                        weights.push(weight);
                    }
                }
            }
        }
        
        if positions.is_empty() {
            // Default position
            Coordinate3D { x: 0.5, y: 0.5, z: 3.0 }
        } else {
            // Weighted average
            let total_weight: f32 = weights.iter().sum();
            let weighted_x: f32 = positions.iter().zip(&weights)
                .map(|(pos, w)| pos.x * w).sum::<f32>() / total_weight;
            let weighted_y: f32 = positions.iter().zip(&weights)
                .map(|(pos, w)| pos.y * w).sum::<f32>() / total_weight;
            let weighted_z: f32 = positions.iter().zip(&weights)
                .map(|(pos, w)| pos.z * w).sum::<f32>() / total_weight;
            
            Coordinate3D {
                x: weighted_x,
                y: weighted_y,
                z: 3.0, // Force to words layer
            }
        }
    }
    
    /// Predict functional primitive from synthesis
    fn predict_function(
        &self,
        _word: &str,
        _morphemes: &[String],
        pattern_type: &PatternType,
        position: Coordinate3D
    ) -> FunctionalPrimitive {
        match pattern_type {
            PatternType::Agent => {
                FunctionalPrimitive::Agency {
                    actor: LinguisticNode::new(
                        NodeId(0), // Placeholder
                        Layer::Words,
                        position
                    ),
                    capability_level: 0.7, // Based on pattern confidence
                    responsibility_scope: Vec::new(),
                }
            }
            
            PatternType::Action => {
                FunctionalPrimitive::Action {
                    verb: LinguisticNode::new(
                        NodeId(0),
                        Layer::Words,
                        position
                    ),
                    transformation_type: ActionType::Transformative,
                    intensity: 0.6,
                    temporal_aspect: TemporalAspect::Present,
                }
            }
            
            _ => {
                // Default to action for other patterns
                FunctionalPrimitive::Action {
                    verb: LinguisticNode::new(
                        NodeId(0),
                        Layer::Words,
                        position
                    ),
                    transformation_type: ActionType::Mechanical,
                    intensity: 0.5,
                    temporal_aspect: TemporalAspect::Present,
                }
            }
        }
    }
    
    /// Calculate confidence for synthesis
    fn calculate_synthesis_confidence(
        &self,
        word: &str,
        morphemes: &[String],
        pattern_type: &PatternType,
        executor: &mut LingoExecutor
    ) -> f32 {
        let mut confidence = 0.0;
        
        // Check if word already exists (lower confidence for existing words)
        if let Ok(result) = executor.execute(&QueryBuilder::find(word).compile()) {
            if !result.nodes.is_empty() {
                confidence += 0.2; // Exists but not novel
            } else {
                confidence += 0.4; // Novel synthesis
            }
        }
        
        // Check morpheme compatibility
        let compatibility = self.check_morpheme_compatibility(morphemes);
        confidence += compatibility * 0.3;
        
        // Pattern match bonus
        let pattern_bonus = match pattern_type {
            PatternType::Agent => 0.3, // High confidence for agent patterns
            PatternType::Action => 0.25,
            _ => 0.2,
        };
        confidence += pattern_bonus;
        
        confidence.min(1.0)
    }
    
    /// Check if morphemes are compatible
    fn check_morpheme_compatibility(&self, morphemes: &[String]) -> f32 {
        // Simple heuristics for now
        if morphemes.len() == 2 {
            let first = &morphemes[0];
            let second = &morphemes[1];
            
            // Check for valid prefix-root or root-suffix combinations
            if self.is_valid_prefix(first) || self.is_valid_suffix(second) {
                return 0.8;
            }
        }
        
        0.5 // Default compatibility
    }
    
    fn is_valid_prefix(&self, morpheme: &str) -> bool {
        let prefixes = ["un", "dis", "re", "pre", "post", "anti", "de", "over", "under",
                       "super", "ultra", "hyper", "mega"];
        prefixes.contains(&morpheme)
    }
    
    fn is_valid_suffix(&self, morpheme: &str) -> bool {
        let suffixes = ["er", "or", "ist", "ant", "ian", "ize", "fy", "ate", "en",
                       "ed", "ing", "s", "tion", "sion", "ment", "ness", "ity"];
        suffixes.contains(&morpheme)
    }
    
    fn infer_morpheme_type(&self, morpheme: &str) -> MorphemeType {
        if self.is_valid_prefix(morpheme) {
            MorphemeType::Prefix
        } else if self.is_valid_suffix(morpheme) {
            match morpheme {
                "er" | "or" | "ist" | "ant" | "ian" => MorphemeType::AgentSuffix,
                "ize" | "fy" | "ate" | "en" => MorphemeType::VerbSuffix,
                "ed" | "ing" | "s" => MorphemeType::TenseSuffix,
                _ => MorphemeType::Suffix,
            }
        } else {
            MorphemeType::Root
        }
    }
}
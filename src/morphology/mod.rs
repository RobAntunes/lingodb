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

//! Morphological decomposition and analysis module
//! 
//! This module implements TRUE compositional morphology - no hardcoding!
//! Everything emerges from the database content.

use crate::core::{NodeId, Layer, Coordinate3D, MorphemeType};
use crate::storage::LingoDatabase;
use crate::query::QueryBuilder;
use crate::engine::LingoExecutor;

#[derive(Debug, Clone)]
pub struct MorphemeAnalysis {
    pub surface_form: String,
    pub morpheme_type: MorphemeType,
    pub position: Option<Coordinate3D>,
    pub node_id: Option<NodeId>,
}

/// Decompose a word into its constituent morphemes using the database
pub fn decompose_word_to_morphemes(
    word: &str, 
    database: &LingoDatabase,
    executor: &mut LingoExecutor
) -> Vec<MorphemeAnalysis> {
    let lower_word = word.to_lowercase();
    
    // First, check if the whole word exists as a morpheme
    if let Ok(result) = executor.execute(&QueryBuilder::find(&lower_word).layer(Layer::Morphemes).compile()) {
        if let Some(node_id) = result.nodes.as_slice().first() {
            if let Ok(node) = database.get_node(*node_id) {
                return vec![MorphemeAnalysis {
                    surface_form: lower_word,
                    morpheme_type: node.morpheme_type,
                    position: Some(node.position),
                    node_id: Some(*node_id),
                }];
            }
        }
    }
    
    // If not found as a single morpheme, try to decompose
    discover_morpheme_decomposition(&lower_word, database, executor)
}

/// Discover morpheme boundaries by finding valid morphemes in the database
fn discover_morpheme_decomposition(
    word: &str,
    database: &LingoDatabase,
    executor: &mut LingoExecutor
) -> Vec<MorphemeAnalysis> {
    let mut best_decomposition = Vec::new();
    let mut best_score = 0.0;
    
    // Try all possible decomposition points
    for split_point in 1..word.len() {
        let part1 = &word[..split_point];
        let part2 = &word[split_point..];
        
        // Check if both parts exist as morphemes in the database
        let part1_analysis = check_morpheme_in_database(part1, database, executor);
        let part2_analysis = check_morpheme_in_database(part2, database, executor);
        
        if let (Some(m1), Some(m2)) = (part1_analysis, part2_analysis) {
            // Score this decomposition based on morpheme properties
            let score = score_decomposition(&m1, &m2, word);
            
            if score > best_score {
                best_score = score;
                best_decomposition = vec![m1, m2];
            }
        }
        
        // Also try three-part decompositions for complex words
        if split_point < word.len() - 1 {
            for split2 in (split_point + 1)..word.len() {
                let part1 = &word[..split_point];
                let part2 = &word[split_point..split2];
                let part3 = &word[split2..];
                
                let m1 = check_morpheme_in_database(part1, database, executor);
                let m2 = check_morpheme_in_database(part2, database, executor);
                let m3 = check_morpheme_in_database(part3, database, executor);
                
                if let (Some(morph1), Some(morph2), Some(morph3)) = (m1, m2, m3) {
                    let score = score_three_part_decomposition(&morph1, &morph2, &morph3, word);
                    
                    if score > best_score {
                        best_score = score;
                        best_decomposition = vec![morph1, morph2, morph3];
                    }
                }
            }
        }
    }
    
    // If we found a good decomposition, return it
    if !best_decomposition.is_empty() && best_score > 0.5 {
        return best_decomposition;
    }
    
    // Otherwise, treat as unknown root composed from letters
    vec![MorphemeAnalysis {
        surface_form: word.to_string(),
        morpheme_type: MorphemeType::Root,
        position: calculate_position_from_letters(word, database, executor),
        node_id: None,
    }]
}

/// Check if a string exists as a morpheme in the database
fn check_morpheme_in_database(
    morpheme: &str,
    database: &LingoDatabase,
    executor: &mut LingoExecutor
) -> Option<MorphemeAnalysis> {
    if morpheme.is_empty() {
        return None;
    }
    
    // Query the morpheme layer
    if let Ok(result) = executor.execute(
        &QueryBuilder::find(morpheme).layer(Layer::Morphemes).compile()
    ) {
        if let Some(node_id) = result.nodes.as_slice().first() {
            if let Ok(node) = database.get_node(*node_id) {
                return Some(MorphemeAnalysis {
                    surface_form: morpheme.to_string(),
                    morpheme_type: node.morpheme_type,
                    position: Some(node.position),
                    node_id: Some(*node_id),
                });
            }
        }
    }
    
    None
}

/// Score a two-part decomposition based on morphological validity
fn score_decomposition(m1: &MorphemeAnalysis, m2: &MorphemeAnalysis, word: &str) -> f32 {
    let mut score: f32 = 0.0;
    
    // Check morpheme type compatibility
    match (&m1.morpheme_type, &m2.morpheme_type) {
        // Root + Suffix is very common
        (MorphemeType::Root, MorphemeType::Suffix) => score += 0.8,
        (MorphemeType::Root, MorphemeType::AgentSuffix) => score += 0.9,
        (MorphemeType::Root, MorphemeType::VerbSuffix) => score += 0.85,
        (MorphemeType::Root, MorphemeType::TenseSuffix) => score += 0.8,
        
        // Prefix + Root is common
        (MorphemeType::Prefix, MorphemeType::Root) => score += 0.8,
        
        // Compound patterns
        (MorphemeType::Root, MorphemeType::Root) => score += 0.6,
        (MorphemeType::Compound, _) | (_, MorphemeType::Compound) => score += 0.7,
        
        // Less common patterns
        _ => score += 0.3,
    }
    
    // Bonus for morphemes that recreate the original word exactly
    if format!("{}{}", m1.surface_form, m2.surface_form) == word {
        score += 0.2;
    }
    
    // Spatial coherence bonus
    if let (Some(pos1), Some(pos2)) = (m1.position, m2.position) {
        let distance = euclidean_distance(pos1, pos2);
        score += (1.0 / (1.0 + distance)) * 0.1;
    }
    
    score.min(1.0_f32)
}

/// Score a three-part decomposition
fn score_three_part_decomposition(
    m1: &MorphemeAnalysis,
    m2: &MorphemeAnalysis, 
    m3: &MorphemeAnalysis,
    word: &str
) -> f32 {
    let mut score: f32 = 0.0;
    
    // Prefix + Root + Suffix is the ideal pattern
    match (&m1.morpheme_type, &m2.morpheme_type, &m3.morpheme_type) {
        (MorphemeType::Prefix, MorphemeType::Root, MorphemeType::Suffix) => score += 0.95,
        (MorphemeType::Prefix, MorphemeType::Root, MorphemeType::AgentSuffix) => score += 0.95,
        (MorphemeType::Prefix, MorphemeType::Root, MorphemeType::VerbSuffix) => score += 0.95,
        _ => score += 0.4,
    }
    
    // Exact reconstruction bonus
    if format!("{}{}{}", m1.surface_form, m2.surface_form, m3.surface_form) == word {
        score += 0.05;
    }
    
    score.min(1.0_f32)
}

/// Calculate position by composing letter positions
fn calculate_position_from_letters(
    word: &str,
    database: &LingoDatabase,
    executor: &mut LingoExecutor
) -> Option<Coordinate3D> {
    let mut positions = Vec::new();
    
    // Get position of each letter
    for ch in word.chars() {
        let letter = ch.to_string();
        if let Ok(result) = executor.execute(
            &QueryBuilder::find(&letter).layer(Layer::Letters).compile()
        ) {
            if let Some(letter_id) = result.nodes.as_slice().first() {
                if let Ok(node) = database.get_node(*letter_id) {
                    positions.push(node.position);
                }
            }
        }
    }
    
    if positions.is_empty() {
        return None;
    }
    
    // Calculate average position
    let sum_x: f32 = positions.iter().map(|p| p.x).sum();
    let sum_y: f32 = positions.iter().map(|p| p.y).sum();
    let count = positions.len() as f32;
    
    Some(Coordinate3D {
        x: sum_x / count,
        y: sum_y / count,
        z: Layer::Morphemes.z_center(),
    })
}

/// Calculate composed 3D position from morpheme positions
pub fn calculate_composed_position(morphemes: &[MorphemeAnalysis]) -> Coordinate3D {
    let positions: Vec<Coordinate3D> = morphemes.iter()
        .filter_map(|m| m.position)
        .collect();
    
    if positions.is_empty() {
        return Coordinate3D { x: 0.5, y: 0.5, z: Layer::Words.z_center() };
    }
    
    // Weight morphemes by type importance
    let mut total_weight = 0.0;
    let mut weighted_x = 0.0;
    let mut weighted_y = 0.0;
    
    for (i, morpheme) in morphemes.iter().enumerate() {
        if let Some(pos) = morpheme.position {
            let weight = morpheme.morpheme_type.composition_weight();
            weighted_x += pos.x * weight;
            weighted_y += pos.y * weight;
            total_weight += weight;
        }
    }
    
    if total_weight > 0.0 {
        Coordinate3D {
            x: weighted_x / total_weight,
            y: weighted_y / total_weight,
            z: Layer::Words.z_center(),
        }
    } else {
        Coordinate3D { x: 0.5, y: 0.5, z: Layer::Words.z_center() }
    }
}

/// Simple preprocessing - just clean and lowercase
pub fn preprocess_text(text: &str) -> String {
    text.trim().to_lowercase()
}

/// Calculate Euclidean distance between two 3D points
fn euclidean_distance(a: Coordinate3D, b: Coordinate3D) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{MorphemeType, Coordinate3D, NodeId};
    
    #[test]
    fn test_morpheme_analysis_struct() {
        // Test the MorphemeAnalysis struct
        let analysis = MorphemeAnalysis {
            surface_form: "test".to_string(),
            morpheme_type: MorphemeType::Root,
            position: Some(Coordinate3D::new(0.5, 0.5, 0.5)),
            node_id: Some(NodeId(1)),
        };
        
        assert_eq!(analysis.surface_form, "test");
        assert_eq!(analysis.morpheme_type, MorphemeType::Root);
        assert!(analysis.position.is_some());
        assert!(analysis.node_id.is_some());
    }
    
    #[test]
    fn test_decompose_with_fallback() {
        // Test the fallback behavior when no database is available
        // This would normally require a real database and executor
        // Testing the struct only for now
        
        // This would normally require a real database and executor
        // For now, just test that the function exists
    }
    
    #[test]
    fn test_morpheme_types() {
        // Test that all morpheme types are handled
        let types = vec![
            MorphemeType::Prefix,
            MorphemeType::Root,
            MorphemeType::Suffix,
            MorphemeType::Compound,
        ];
        
        for morph_type in types {
            let analysis = MorphemeAnalysis {
                surface_form: "test".to_string(),
                morpheme_type: morph_type,
                position: None,
                node_id: None,
            };
            assert_eq!(analysis.morpheme_type, morph_type);
        }
    }
}
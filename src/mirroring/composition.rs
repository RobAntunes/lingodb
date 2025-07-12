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

//! Morpheme composition engine
//! 
//! Handles the reverse process of decomposition - building words from morphemes

use std::collections::HashSet;
use crate::core::{Layer, MorphemeType, Coordinate3D};
use crate::storage::LingoDatabase;
use crate::engine::LingoExecutor;
use crate::query::QueryBuilder;
use crate::morphology::MorphemeAnalysis;
use super::CompositionWeights;

/// Handles composition of morphemes into words
pub struct Composer<'a> {
    db: &'a LingoDatabase,
    weights: &'a CompositionWeights,
}

impl<'a> Composer<'a> {
    pub fn new(db: &'a LingoDatabase, weights: &'a CompositionWeights) -> Self {
        Self { db, weights }
    }
    
    /// Compose morphemes into possible words
    pub fn compose_from_morphemes(
        &self,
        morphemes: &[String],
        executor: &mut LingoExecutor
    ) -> Vec<String> {
        let mut candidates = Vec::new();
        
        // Generate permutations that make linguistic sense
        let valid_orderings = self.generate_valid_orderings(morphemes);
        
        for ordering in valid_orderings {
            let composed = self.attempt_composition(&ordering);
            
            // Validate the composed word
            if self.validate_composed_word(&composed, executor) {
                candidates.push(composed);
            }
        }
        
        // Calculate confidences first to avoid multiple borrows
        let mut candidates_with_conf: Vec<(String, f32)> = candidates.into_iter()
            .map(|word| {
                let conf = self.calculate_composition_confidence(&word, morphemes);
                (word, conf)
            })
            .collect();
        
        // Sort by confidence/likelihood
        candidates_with_conf.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Extract just the words
        let candidates: Vec<String> = candidates_with_conf.into_iter()
            .map(|(word, _)| word)
            .collect();
        
        candidates
    }
    
    /// Generate linguistically valid orderings of morphemes
    fn generate_valid_orderings(&self, morphemes: &[String]) -> Vec<Vec<String>> {
        let mut orderings = Vec::new();
        
        // Classify morphemes by type
        let mut prefixes = Vec::new();
        let mut roots = Vec::new();
        let mut suffixes = Vec::new();
        
        for morpheme in morphemes {
            let morph_type = self.classify_morpheme(morpheme);
            match morph_type {
                MorphemeType::Prefix => prefixes.push(morpheme.clone()),
                MorphemeType::Root => roots.push(morpheme.clone()),
                MorphemeType::Suffix | MorphemeType::AgentSuffix | 
                MorphemeType::VerbSuffix | MorphemeType::TenseSuffix => {
                    suffixes.push(morpheme.clone())
                },
                _ => roots.push(morpheme.clone()), // Default to root
            }
        }
        
        // Generate valid combinations: prefix* + root+ + suffix*
        if !roots.is_empty() {
            // Simple case: one of each type
            let mut ordering = Vec::new();
            ordering.extend(prefixes.clone());
            ordering.extend(roots.clone());
            ordering.extend(suffixes.clone());
            orderings.push(ordering);
            
            // Try without prefix
            if !prefixes.is_empty() {
                let mut no_prefix = Vec::new();
                no_prefix.extend(roots.clone());
                no_prefix.extend(suffixes.clone());
                orderings.push(no_prefix);
            }
            
            // Try without suffix
            if !suffixes.is_empty() {
                let mut no_suffix = Vec::new();
                no_suffix.extend(prefixes.clone());
                no_suffix.extend(roots.clone());
                orderings.push(no_suffix);
            }
        }
        
        orderings
    }
    
    /// Attempt to compose morphemes into a word
    fn attempt_composition(&self, morphemes: &[String]) -> String {
        let mut composed = String::new();
        
        for (i, morpheme) in morphemes.iter().enumerate() {
            if i == 0 {
                // First morpheme - just add it
                composed.push_str(morpheme);
            } else {
                // Apply morphophonological rules for subsequent morphemes
                let (stem_adjustment, morpheme_adjustment) = self.get_morphophonological_adjustments(
                    &composed,
                    morpheme
                );
                
                // Apply stem adjustment if needed
                if stem_adjustment > 0 {
                    composed.truncate(composed.len() - stem_adjustment);
                }
                
                // Add the adjusted morpheme
                composed.push_str(&morpheme_adjustment);
            }
        }
        
        composed
    }
    
    /// Get morphophonological adjustments for stem and morpheme
    fn get_morphophonological_adjustments(
        &self,
        stem: &str,
        morpheme: &str
    ) -> (usize, String) {
        // Returns (stem_chars_to_remove, adjusted_morpheme)
        
        // Rule 1: e-deletion before -ing, -er, -ed
        if stem.ends_with('e') && (morpheme == "ing" || morpheme == "er" || morpheme == "ed") {
            return (1, morpheme.to_string()); // Remove 'e' from stem
        }
        
        // Rule 2: Consonant doubling
        if self.should_double_consonant(stem, morpheme) {
            let last_char = stem.chars().last().unwrap();
            let mut adjusted = String::new();
            adjusted.push(last_char);
            adjusted.push_str(morpheme);
            return (0, adjusted); // No stem change, double consonant in morpheme
        }
        
        // Rule 3: y → i before certain suffixes
        if stem.ends_with('y') && stem.len() > 1 {
            let chars: Vec<char> = stem.chars().collect();
            if chars.len() >= 2 && !self.is_vowel(chars[chars.len()-2]) {
                if morpheme == "es" || morpheme == "ed" || morpheme == "er" || morpheme == "est" {
                    let mut adjusted = String::from("i");
                    adjusted.push_str(morpheme);
                    return (1, adjusted); // Remove 'y' from stem, add 'i' to morpheme
                }
            }
        }
        
        // Default: no changes
        (0, morpheme.to_string())
    }
    
    
    /// Check if consonant should be doubled
    fn should_double_consonant(&self, stem: &str, morpheme: &str) -> bool {
        if stem.len() < 3 {
            return false;
        }
        
        let chars: Vec<char> = stem.chars().collect();
        let len = chars.len();
        
        // Check CVC pattern and morpheme starts with vowel
        if !self.is_vowel(chars[len-1]) && 
           self.is_vowel(chars[len-2]) && 
           !self.is_vowel(chars[len-3]) &&
           morpheme.chars().next().map(|c| self.is_vowel(c)).unwrap_or(false) {
            return true;
        }
        
        false
    }
    
    /// Check if character is a vowel
    fn is_vowel(&self, c: char) -> bool {
        matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
    }
    
    /// Validate that a composed word is linguistically valid
    fn validate_composed_word(&self, word: &str, executor: &mut LingoExecutor) -> bool {
        // Check basic constraints
        if word.len() < 2 || word.len() > 30 {
            return false;
        }
        
        // Check if it exists in the database
        if let Ok(result) = executor.execute(&QueryBuilder::find(word).compile()) {
            if !result.nodes.is_empty() {
                return true; // Word exists
            }
        }
        
        // Even if not in database, check if it follows morphological rules
        self.follows_morphological_rules(word)
    }
    
    /// Check if word follows basic morphological rules
    fn follows_morphological_rules(&self, word: &str) -> bool {
        // No triple consonants (except specific cases)
        let chars: Vec<char> = word.chars().collect();
        for i in 0..chars.len().saturating_sub(2) {
            if !self.is_vowel(chars[i]) && 
               !self.is_vowel(chars[i+1]) && 
               !self.is_vowel(chars[i+2]) {
                // Exception for compounds like "worthwhile"
                if i > 0 && self.is_compound_boundary(i, &chars) {
                    continue;
                }
                return false;
            }
        }
        
        true
    }
    
    /// Check if position might be a compound boundary
    fn is_compound_boundary(&self, pos: usize, chars: &[char]) -> bool {
        // Simple heuristic: look for common compound patterns
        pos > 2 && pos < chars.len() - 2
    }
    
    /// Calculate confidence for a composed word
    fn calculate_composition_confidence(
        &self,
        word: &str,
        original_morphemes: &[String]
    ) -> f32 {
        let mut confidence = 0.0;
        
        // For now, we'll check existence separately in the caller
        // to avoid needing executor here
        
        // Check morphological plausibility
        if self.follows_morphological_rules(word) {
            confidence += 0.3;
        }
        
        // Check if all morphemes are represented
        let morpheme_coverage = original_morphemes.iter()
            .filter(|m| word.contains(m.as_str()))
            .count() as f32 / original_morphemes.len() as f32;
        confidence += morpheme_coverage * 0.2;
        
        confidence.min(1.0)
    }
    
    /// Classify a morpheme by its form
    fn classify_morpheme(&self, morpheme: &str) -> MorphemeType {
        // Common prefixes
        let prefixes = ["un", "dis", "re", "pre", "post", "anti", "de", "over", "under"];
        if prefixes.contains(&morpheme) {
            return MorphemeType::Prefix;
        }
        
        // Agent suffixes
        if matches!(morpheme, "er" | "or" | "ist" | "ant" | "ian") {
            return MorphemeType::AgentSuffix;
        }
        
        // Verb suffixes
        if matches!(morpheme, "ize" | "fy" | "ate" | "en") {
            return MorphemeType::VerbSuffix;
        }
        
        // Tense suffixes
        if matches!(morpheme, "ed" | "ing" | "s") {
            return MorphemeType::TenseSuffix;
        }
        
        // Other suffixes
        if matches!(morpheme, "tion" | "sion" | "ment" | "ness" | "ity" | "able" | "ible") {
            return MorphemeType::Suffix;
        }
        
        // Default to root
        MorphemeType::Root
    }
}
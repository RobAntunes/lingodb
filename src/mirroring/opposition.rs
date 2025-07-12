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

//! Opposition engine for finding morphological and semantic opposites

use std::collections::HashMap;
use std::sync::Arc;
use crate::core::{Coordinate3D, MorphemeType, EtymologyOrigin, LinguisticNode, NodeId, Layer};
use crate::storage::LingoDatabase;
use crate::morphology::MorphemeAnalysis;
use crate::engine::LingoExecutor;
use crate::query::QueryBuilder;
use lru::LruCache;

#[derive(Debug, Clone, PartialEq)]
pub enum MirrorType {
    EtymologicalOpposite {
        root_family: EtymologyFamily,
        semantic_distance: f32,
    },
    FunctionalOpposite {
        role_inversion: RoleType,
        domain_context: String,
    },
    MorphologicalOpposite {
        valid_negation: NegationType,
        productivity_score: f32,
    },
    CrossLinguisticMirror {
        source_etymology: EtymologyOrigin,
        target_etymology: EtymologyOrigin, 
        borrowing_pattern: BorrowingType,
    },
    SpatialOpposite {
        vector_opposition: Coordinate3D,
        clustering_confidence: f32,
    },
    // Legacy types for compatibility
    Negation,
    Reversal,
    Complementary,
    Gradable,
    Directional,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EtymologyFamily {
    Latin,
    Greek,
    Germanic,
    Romance,
    IndoEuropean,
    Semitic,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RoleType {
    Agent,
    Patient,
    Actor,
    Target,
    Creator,
    Destroyer,
    Teacher,
    Student,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NegationType {
    Prefix(String),        // un-, dis-, non-
    Suffix(String),        // -less
    Circumfix(String, String), // ge-...-t
    LexicalReplacement,    // good -> bad
}

#[derive(Debug, Clone, PartialEq)]
pub enum BorrowingType {
    Direct,
    Calque,
    Semantic,
    Phonetic,
}

#[derive(Debug, Clone)]
pub struct EtymologyProfile {
    pub word: String,
    pub primary_etymology: EtymologyOrigin,
    pub morpheme_etymologies: Vec<EtymologyData>,
    pub semantic_position: Coordinate3D,
    pub root_concepts: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EtymologyData {
    pub morpheme: String,
    pub origin: EtymologyOrigin,
    pub root_meaning: String,
    pub semantic_field: String,
    pub historical_development: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EtymologicalMirror {
    pub original: String,
    pub mirror: String,
    pub mirror_type: MirrorType,
    pub confidence: f32,
    pub linguistic_evidence: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum FunctionalRole {
    Agent { domain: String, capability_type: String },
    Action { transformation_type: String, intensity: f32 },
    State { polarity: f32, stability: f32 },
    Relation { directionality: String, symmetry: bool },
}

#[derive(Debug, Clone)]
pub struct OppositionVector {
    pub semantic_direction: Coordinate3D,
    pub strength: f32,
    pub context: String,
}

#[derive(Debug, Clone)]
pub struct ValidatedMirror {
    pub mirror: EtymologicalMirror,
    pub validation_score: f32,
    pub real_word_check: bool,
    pub morphological_validity: bool,
    pub etymological_consistency: bool,
}

#[derive(Debug, Clone)]
pub struct SpatialOpposite {
    pub word: String,
    pub position: Coordinate3D,
    pub distance: f32,
    pub clustering_strength: f32,
}

pub struct EtymologyGraph {
    pub oppositions: HashMap<String, Vec<String>>,
    pub semantic_fields: HashMap<String, Vec<String>>,
    pub historical_relations: HashMap<String, Vec<String>>,
}

/// Advanced etymological mirror engine for discovering linguistic opposites
pub struct EtymologicalMirrorEngine {
    db: Arc<LingoDatabase>,
    etymology_graphs: HashMap<EtymologyOrigin, EtymologyGraph>,
    opposition_vectors: HashMap<String, Vec<OppositionVector>>,
    validated_mirrors: HashMap<String, Vec<ValidatedMirror>>,
    spatial_opposition_cache: LruCache<String, Vec<SpatialOpposite>>,
}

impl EtymologicalMirrorEngine {
    pub fn new(database: Arc<LingoDatabase>) -> Self {
        let mut etymology_graphs = HashMap::new();
        
        // Initialize etymology graphs with known oppositions
        etymology_graphs.insert(EtymologyOrigin::Latin, EtymologyGraph {
            oppositions: Self::build_latin_oppositions(),
            semantic_fields: Self::build_latin_semantic_fields(),
            historical_relations: HashMap::new(),
        });
        
        etymology_graphs.insert(EtymologyOrigin::Greek, EtymologyGraph {
            oppositions: Self::build_greek_oppositions(),
            semantic_fields: Self::build_greek_semantic_fields(),
            historical_relations: HashMap::new(),
        });
        
        etymology_graphs.insert(EtymologyOrigin::Germanic, EtymologyGraph {
            oppositions: Self::build_germanic_oppositions(),
            semantic_fields: Self::build_germanic_semantic_fields(),
            historical_relations: HashMap::new(),
        });
        
        Self {
            db: database,
            etymology_graphs,
            opposition_vectors: HashMap::new(),
            validated_mirrors: HashMap::new(),
            spatial_opposition_cache: LruCache::new(std::num::NonZeroUsize::new(1000).unwrap()),
        }
    }
    
    fn build_latin_oppositions() -> HashMap<String, Vec<String>> {
        let mut oppositions = HashMap::new();
        
        // Latin root families with opposites
        oppositions.insert("create".to_string(), vec!["destroy".to_string()]);
        oppositions.insert("construct".to_string(), vec!["destruct".to_string()]);
        oppositions.insert("produce".to_string(), vec!["reduce".to_string()]);
        oppositions.insert("educate".to_string(), vec!["seduce".to_string()]);
        oppositions.insert("conduct".to_string(), vec!["abduct".to_string()]);
        
        oppositions
    }
    
    fn build_greek_oppositions() -> HashMap<String, Vec<String>> {
        let mut oppositions = HashMap::new();
        
        // Greek root families
        oppositions.insert("dialogue".to_string(), vec!["monologue".to_string()]);
        oppositions.insert("prologue".to_string(), vec!["epilogue".to_string()]);
        oppositions.insert("synthesis".to_string(), vec!["analysis".to_string()]);
        oppositions.insert("antithesis".to_string(), vec!["hypothesis".to_string()]);
        
        oppositions
    }
    
    fn build_germanic_oppositions() -> HashMap<String, Vec<String>> {
        let mut oppositions = HashMap::new();
        
        // Germanic strong verb patterns
        oppositions.insert("build".to_string(), vec!["break".to_string()]);
        oppositions.insert("grow".to_string(), vec!["shrink".to_string()]);
        oppositions.insert("rise".to_string(), vec!["fall".to_string()]);
        
        oppositions
    }
    
    fn build_latin_semantic_fields() -> HashMap<String, Vec<String>> {
        let mut fields = HashMap::new();
        fields.insert("creation".to_string(), vec!["create".to_string(), "construct".to_string(), "produce".to_string()]);
        fields.insert("destruction".to_string(), vec!["destroy".to_string(), "destruct".to_string(), "reduce".to_string()]);
        fields
    }
    
    fn build_greek_semantic_fields() -> HashMap<String, Vec<String>> {
        let mut fields = HashMap::new();
        fields.insert("discourse".to_string(), vec!["dialogue".to_string(), "monologue".to_string()]);
        fields.insert("analysis".to_string(), vec!["synthesis".to_string(), "analysis".to_string()]);
        fields
    }
    
    fn build_germanic_semantic_fields() -> HashMap<String, Vec<String>> {
        let mut fields = HashMap::new();
        fields.insert("construction".to_string(), vec!["build".to_string(), "make".to_string()]);
        fields.insert("destruction".to_string(), vec!["break".to_string(), "destroy".to_string()]);
        fields
    }
    
    /// REPLACE: Simple prefix generation WITH: True etymological analysis
    pub fn discover_etymological_mirrors(&self, word: &str) -> Result<Vec<EtymologicalMirror>, crate::core::error::LingoError> {
        let mut mirrors = Vec::new();
        
        // Step 1: Get the word's etymological profile
        let etymology_profile = self.analyze_etymology_profile(word)?;
        
        // Step 2: Find opposites within same etymology family
        let family_opposites = self.find_family_opposites(&etymology_profile);
        mirrors.extend(family_opposites);
        
        // Step 3: Find cross-linguistic opposites
        let cross_linguistic = self.find_cross_linguistic_opposites(&etymology_profile);
        mirrors.extend(cross_linguistic);
        
        // Step 4: Find spatial opposites in 3D semantic space
        let spatial_opposites = self.find_spatial_opposites(word);
        mirrors.extend(spatial_opposites);
        
        // Step 5: Find functional role opposites
        let functional_opposites = self.find_functional_opposites(word);
        mirrors.extend(functional_opposites);
        
        // Step 6: Validate all mirrors for linguistic authenticity
        let validated_mirrors = mirrors.into_iter()
            .filter(|mirror| self.validate_mirror_authenticity(mirror))
            .collect();
            
        Ok(validated_mirrors)
    }
    
    fn analyze_etymology_profile(&self, word: &str) -> Result<EtymologyProfile, crate::core::error::LingoError> {
        // Create a temporary executor for database queries
        let mut executor = crate::engine::LingoExecutor::new();
        executor.load_database("english.lingo")?;
        
        // Query the database for deep etymological analysis
        let word_result = executor.execute(
            &QueryBuilder::find(word)
                .layer(Layer::Words)
                .compile()
        )?;
        
        let word_node_id = word_result.nodes.as_slice().first()
            .ok_or_else(|| crate::core::error::LingoError::WordNotFound("Word not found".to_string()))?;
        
        let word_node = self.db.get_node(*word_node_id)?;
        
        // Get morphological breakdown to roots
        let morpheme_result = executor.execute(
            &QueryBuilder::load(*word_node_id)
                .layer_down() // Go to morphemes
                .compile()
        )?;
        
        // Extract etymology data for each morpheme
        let etymology_data = morpheme_result.nodes.as_slice().iter()
            .map(|&node_id| {
                let morpheme_node = self.db.get_node(node_id).unwrap();
                EtymologyData {
                    morpheme: self.get_surface_form(&morpheme_node),
                    origin: morpheme_node.etymology_origin,
                    root_meaning: self.extract_root_meaning(&morpheme_node),
                    semantic_field: self.extract_semantic_field(&morpheme_node),
                    historical_development: self.trace_historical_development(&morpheme_node),
                }
            })
            .collect();
        
        Ok(EtymologyProfile {
            word: word.to_string(),
            primary_etymology: word_node.etymology_origin,
            morpheme_etymologies: etymology_data,
            semantic_position: word_node.position,
            root_concepts: self.extract_root_concepts(morpheme_result.nodes.as_slice()),
        })
    }
    
    fn find_family_opposites(&self, profile: &EtymologyProfile) -> Vec<EtymologicalMirror> {
        let mut opposites = Vec::new();
        
        // Find words with same etymological origin but opposite meaning
        if let Some(etymology_graph) = self.etymology_graphs.get(&profile.primary_etymology) {
            for (root_word, opposite_candidates) in &etymology_graph.oppositions {
                if profile.word.contains(root_word) {
                    for opposite_candidate in opposite_candidates {
                        let semantic_distance = self.calculate_semantic_distance(
                            &profile.word, 
                            opposite_candidate
                        );
                        
                        // Look for etymological clues of opposition
                        if self.has_etymological_opposition_markers(profile, opposite_candidate) {
                            opposites.push(EtymologicalMirror {
                                original: profile.word.clone(),
                                mirror: opposite_candidate.clone(),
                                mirror_type: MirrorType::EtymologicalOpposite {
                                    root_family: self.determine_etymology_family(profile.primary_etymology),
                                    semantic_distance,
                                },
                                confidence: self.calculate_etymology_confidence(profile, opposite_candidate),
                                linguistic_evidence: self.gather_linguistic_evidence(profile, opposite_candidate),
                            });
                        }
                    }
                }
            }
        }
        
        opposites
    }
    
    fn find_spatial_opposites(&self, word: &str) -> Vec<EtymologicalMirror> {
        // Check cache first
        if let Some(cached) = self.spatial_opposition_cache.get(word) {
            return cached.iter().map(|spatial_opp| EtymologicalMirror {
                original: word.to_string(),
                mirror: spatial_opp.word.clone(),
                mirror_type: MirrorType::SpatialOpposite {
                    vector_opposition: self.calculate_opposition_vector(spatial_opp.position),
                    clustering_confidence: spatial_opp.clustering_strength,
                },
                confidence: self.calculate_spatial_confidence(spatial_opp.distance),
                linguistic_evidence: vec![
                    format!("Spatial distance: {:.3}", spatial_opp.distance),
                    format!("Clustering strength: {:.3}", spatial_opp.clustering_strength),
                ],
            }).collect();
        }
        
        // Create executor for spatial queries
        let mut executor = crate::engine::LingoExecutor::new();
        if executor.load_database("english.lingo").is_err() {
            return Vec::new();
        }
        
        // Get word position
        let word_result = executor.execute(
            &QueryBuilder::find(word)
                .layer(Layer::Words)
                .compile()
        );
        
        if let Ok(result) = word_result {
            if let Some(&word_node_id) = result.nodes.as_slice().first() {
                if let Ok(word_node) = self.db.get_node(word_node_id) {
                    return self.find_spatial_opposites_for_position(word, word_node.position);
                }
            }
        }
        
        Vec::new()
    }
    
    fn find_spatial_opposites_for_position(&self, word: &str, word_position: Coordinate3D) -> Vec<EtymologicalMirror> {
        // Find the opposite point in 3D semantic space
        let opposite_point = Coordinate3D {
            x: 1.0 - word_position.x,  // Flip X axis
            y: 1.0 - word_position.y,  // Flip Y axis  
            z: word_position.z,        // Keep abstraction level
        };
        
        // Create executor for spatial radius queries
        let mut executor = crate::engine::LingoExecutor::new();
        if executor.load_database("english.lingo").is_err() {
            return Vec::new();
        }
        
        // Find words clustered around the opposite point using a spatial query
        let spatial_result = executor.execute(
            &QueryBuilder::spatial_radius_from_point(opposite_point, 0.2)
                .layer(Layer::Words)
                .compile()
        );
        
        if let Ok(result) = spatial_result {
            result.nodes.as_slice().iter()
                .filter_map(|&candidate_id| self.db.get_node(candidate_id).ok())
                .map(|candidate| EtymologicalMirror {
                    original: word.to_string(),
                    mirror: self.get_surface_form(&candidate),
                    mirror_type: MirrorType::SpatialOpposite {
                        vector_opposition: self.calculate_opposition_vector_between(
                            word_position, 
                            candidate.position
                        ),
                        clustering_confidence: self.calculate_clustering_confidence(&candidate),
                    },
                    confidence: self.calculate_spatial_confidence_between(&word_position, &candidate.position),
                    linguistic_evidence: vec![
                        format!("Spatial distance: {:.3}", word_position.distance(candidate.position)),
                        format!("Opposition vector: {:?}", opposite_point),
                    ],
                })
                .collect()
        } else {
            Vec::new()
        }
    }
    
    fn find_cross_linguistic_opposites(&self, profile: &EtymologyProfile) -> Vec<EtymologicalMirror> {
        let mut cross_linguistic = Vec::new();
        
        match profile.primary_etymology {
            EtymologyOrigin::Latin => {
                cross_linguistic.extend(self.find_germanic_opposites_for_latin(&profile.word));
                cross_linguistic.extend(self.find_greek_opposites_for_latin(&profile.word));
            },
            EtymologyOrigin::Greek => {
                cross_linguistic.extend(self.find_latin_opposites_for_greek(&profile.word));
                cross_linguistic.extend(self.find_germanic_opposites_for_greek(&profile.word));
            },
            EtymologyOrigin::Germanic => {
                cross_linguistic.extend(self.find_latin_opposites_for_germanic(&profile.word));
                cross_linguistic.extend(self.find_greek_opposites_for_germanic(&profile.word));
            },
            _ => {},
        }
        
        cross_linguistic
    }
    
    fn find_functional_opposites(&self, word: &str) -> Vec<EtymologicalMirror> {
        let mut opposites = Vec::new();
        
        let functional_role = self.determine_functional_role(word);
        
        match functional_role {
            FunctionalRole::Agent { domain, capability_type } => {
                opposites.extend(self.find_opposite_agents(&domain, &capability_type));
            },
            FunctionalRole::Action { transformation_type, intensity } => {
                opposites.extend(self.find_reverse_actions(&transformation_type, intensity));
            },
            FunctionalRole::State { polarity, stability } => {
                opposites.extend(self.find_opposite_states(polarity, stability));
            },
            _ => {},
        }
        
        opposites
    }
    
    fn validate_mirror_authenticity(&self, mirror: &EtymologicalMirror) -> bool {
        // 1. Check if mirror word actually exists in database
        let mirror_exists = self.word_exists_in_database(&mirror.mirror);
        
        // 2. Check if the opposition makes semantic sense
        let semantic_validity = self.validate_semantic_opposition(&mirror.original, &mirror.mirror);
        
        // 3. Check morphological validity (no impossible combinations)
        let morphological_validity = self.validate_morphological_possibility(&mirror.mirror);
        
        // 4. Check etymological consistency
        let etymological_validity = match &mirror.mirror_type {
            MirrorType::EtymologicalOpposite { root_family, .. } => {
                self.validate_etymological_consistency(&mirror.original, &mirror.mirror, root_family)
            },
            MirrorType::MorphologicalOpposite { valid_negation, .. } => {
                self.validate_negation_pattern(&mirror.original, &mirror.mirror, valid_negation)
            },
            _ => true,
        };
        
        mirror_exists && semantic_validity && morphological_validity && etymological_validity
    }
    
    // Helper methods for implementation
    fn get_surface_form(&self, node: &LinguisticNode) -> String {
        self.db.get_string(node.word_offset, node.word_length)
            .unwrap_or_default().to_string()
    }
    
    fn extract_root_meaning(&self, _node: &LinguisticNode) -> String {
        "unknown".to_string() // Placeholder - would extract from semantic annotations
    }
    
    fn extract_semantic_field(&self, _node: &LinguisticNode) -> String {
        "general".to_string() // Placeholder - would extract from semantic categories
    }
    
    fn trace_historical_development(&self, _node: &LinguisticNode) -> Vec<String> {
        Vec::new() // Placeholder - would trace etymology chain
    }
    
    fn extract_root_concepts(&self, _nodes: &[NodeId]) -> Vec<String> {
        Vec::new() // Placeholder - would extract conceptual roots
    }
    
    fn calculate_semantic_distance(&self, word1: &str, word2: &str) -> f32 {
        // Placeholder - would calculate semantic similarity using embeddings
        if word1.len() == word2.len() { 0.5 } else { 0.8 }
    }
    
    fn has_etymological_opposition_markers(&self, _profile: &EtymologyProfile, _candidate: &str) -> bool {
        true // Placeholder - would check for linguistic opposition markers
    }
    
    fn determine_etymology_family(&self, origin: EtymologyOrigin) -> EtymologyFamily {
        match origin {
            EtymologyOrigin::Latin | EtymologyOrigin::French => EtymologyFamily::Romance,
            EtymologyOrigin::Greek => EtymologyFamily::Greek,
            EtymologyOrigin::Germanic => EtymologyFamily::Germanic,
            _ => EtymologyFamily::IndoEuropean,
        }
    }
    
    fn calculate_etymology_confidence(&self, _profile: &EtymologyProfile, _candidate: &str) -> f32 {
        0.8 // Placeholder - would calculate based on etymological evidence
    }
    
    fn gather_linguistic_evidence(&self, profile: &EtymologyProfile, candidate: &str) -> Vec<String> {
        vec![
            format!("Etymology: {:?}", profile.primary_etymology),
            format!("Candidate: {}", candidate),
        ]
    }
    
    fn calculate_opposition_vector(&self, position: Coordinate3D) -> Coordinate3D {
        Coordinate3D {
            x: 1.0 - position.x,
            y: 1.0 - position.y,
            z: position.z,
        }
    }
    
    fn calculate_opposition_vector_between(&self, pos1: Coordinate3D, pos2: Coordinate3D) -> Coordinate3D {
        Coordinate3D {
            x: pos2.x - pos1.x,
            y: pos2.y - pos1.y,
            z: pos2.z - pos1.z,
        }
    }
    
    fn calculate_clustering_confidence(&self, _node: &LinguisticNode) -> f32 {
        0.7 // Placeholder - would calculate based on local word density
    }
    
    fn calculate_spatial_confidence(&self, distance: f32) -> f32 {
        1.0 / (1.0 + distance)
    }
    
    fn calculate_spatial_confidence_between(&self, pos1: &Coordinate3D, pos2: &Coordinate3D) -> f32 {
        let distance = pos1.distance(*pos2);
        1.0 / (1.0 + distance)
    }
    
    fn word_exists_in_database(&self, word: &str) -> bool {
        let mut executor = crate::engine::LingoExecutor::new();
        if executor.load_database("english.lingo").is_err() {
            return false;
        }
        
        let result = executor.execute(
            &QueryBuilder::find(word).compile()
        );
        
        result.map_or(false, |r| !r.nodes.is_empty())
    }
    
    fn validate_semantic_opposition(&self, _word1: &str, _word2: &str) -> bool {
        true // Placeholder - would check semantic opposition validity
    }
    
    fn validate_morphological_possibility(&self, _word: &str) -> bool {
        true // Placeholder - would check morphological constraints
    }
    
    fn validate_etymological_consistency(&self, _original: &str, _mirror: &str, _family: &EtymologyFamily) -> bool {
        true // Placeholder - would validate etymological relationships
    }
    
    fn validate_negation_pattern(&self, _original: &str, _mirror: &str, _negation: &NegationType) -> bool {
        true // Placeholder - would validate negation morphology
    }
    
    fn determine_functional_role(&self, word: &str) -> FunctionalRole {
        // Simple heuristics - would be more sophisticated in practice
        if word.ends_with("er") || word.ends_with("or") {
            FunctionalRole::Agent {
                domain: "general".to_string(),
                capability_type: "action".to_string(),
            }
        } else if word.ends_with("ize") || word.ends_with("ate") {
            FunctionalRole::Action {
                transformation_type: "causative".to_string(),
                intensity: 0.8,
            }
        } else {
            FunctionalRole::State {
                polarity: 0.5,
                stability: 0.7,
            }
        }
    }
    
    fn find_germanic_opposites_for_latin(&self, _word: &str) -> Vec<EtymologicalMirror> {
        Vec::new() // Placeholder
    }
    
    fn find_greek_opposites_for_latin(&self, _word: &str) -> Vec<EtymologicalMirror> {
        Vec::new() // Placeholder
    }
    
    fn find_latin_opposites_for_greek(&self, _word: &str) -> Vec<EtymologicalMirror> {
        Vec::new() // Placeholder
    }
    
    fn find_germanic_opposites_for_greek(&self, _word: &str) -> Vec<EtymologicalMirror> {
        Vec::new() // Placeholder
    }
    
    fn find_latin_opposites_for_germanic(&self, _word: &str) -> Vec<EtymologicalMirror> {
        Vec::new() // Placeholder
    }
    
    fn find_greek_opposites_for_germanic(&self, _word: &str) -> Vec<EtymologicalMirror> {
        Vec::new() // Placeholder
    }
    
    fn find_opposite_agents(&self, _domain: &str, _capability: &str) -> Vec<EtymologicalMirror> {
        Vec::new() // Placeholder
    }
    
    fn find_reverse_actions(&self, _transformation: &str, _intensity: f32) -> Vec<EtymologicalMirror> {
        Vec::new() // Placeholder
    }
    
    fn find_opposite_states(&self, _polarity: f32, _stability: f32) -> Vec<EtymologicalMirror> {
        Vec::new() // Placeholder
    }
}

// Legacy OppositionEngine for backward compatibility
pub struct OppositionEngine<'a> {
    engine: EtymologicalMirrorEngine,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> OppositionEngine<'a> {
    pub fn new(db: &'a LingoDatabase) -> Self {
        let engine = EtymologicalMirrorEngine::new(Arc::new((*db).clone()));
        Self {
            engine,
            _phantom: std::marker::PhantomData,
        }
    }
    
    pub fn find_opposite_morphemes(&self, morpheme: &MorphemeAnalysis) -> Vec<Vec<String>> {
        // Legacy compatibility - convert new system to old format
        if let Ok(mirrors) = self.engine.discover_etymological_mirrors(&morpheme.surface_form) {
            mirrors.into_iter()
                .map(|mirror| vec![mirror.mirror])
                .collect()
        } else {
            Vec::new()
        }
    }
    
    pub fn classify_mirror_type(&self, _original: &MorphemeAnalysis, _opposite_morphemes: &[String]) -> MirrorType {
        MirrorType::Gradable // Legacy default
    }
}
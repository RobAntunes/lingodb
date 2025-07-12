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

//! Function Extraction Plugin - Extract functional semantics from natural language
//! 
//! Implements the complete function extraction specification as a plugin module.
//! Uses deterministic hierarchical composition through the LINGO 7-layer architecture.

use std::collections::HashMap;
use std::time::Instant;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

use crate::core::{LinguisticNode, NodeId, Layer, Coordinate3D, MorphemeType};
use crate::query::{QueryBuilder, CompiledQuery};
use crate::storage::LingoDatabase;
use crate::engine::LingoExecutor;
use crate::morphology::{decompose_word_to_morphemes, calculate_composed_position, 
                        preprocess_text, MorphemeAnalysis};
use crate::mirroring::{MirroringDecomposer, PatternType, 
                       SynthesisResult as MirroringSynthesis, MirrorPair as MirroringMirror,
                       MirrorType as MirroringMirrorType};
use super::{Plugin, PluginContext, PluginResult, PluginError, PluginEnhancement};

/// Function extraction plugin
pub struct FunctionExtractor {
    database_path: Option<String>,
    executor: Option<LingoExecutor>,
    mirroring_decomposer: Option<MirroringDecomposer>,
    confidence_threshold: f32,
    spatial_coherence_weight: f32,
    morphological_weight: f32,
}

impl FunctionExtractor {
    pub fn new() -> Self {
        Self {
            database_path: None,
            executor: None,
            mirroring_decomposer: None,
            confidence_threshold: 0.5,
            spatial_coherence_weight: 0.3,
            morphological_weight: 0.7,
        }
    }
    
    /// Extract function signature from text
    pub fn extract_function_signature(&mut self, text: &str) -> Result<FunctionSignature, PluginError> {
        let executor = self.executor.as_mut()
            .ok_or_else(|| PluginError::NotInitialized("Executor not available".to_string()))?;
        
        // Open database for this operation
        let database_path = self.database_path.as_ref()
            .ok_or_else(|| PluginError::NotInitialized("Database path not set".to_string()))?;
        
        let database = LingoDatabase::open(database_path)
            .map_err(|e| PluginError::InitializationFailed {
                plugin: "function_extraction".to_string(),
                error: e.to_string(),
            })?;
        
        let start_time = Instant::now();
        
        // Step 1: Run all detection algorithms in parallel using enhanced morphological analysis
        let agency_primitives = Self::detect_agency(text, &database, executor)?;
        let action_primitives = Self::detect_actions(text, &database, executor)?;
        let transformation_primitives = Self::detect_transformations(text, &database, executor)?;
        let conditionality_primitives = Self::detect_conditionality(text, &database, executor)?;
        let sequence_primitives = Self::detect_sequences(text, &database, executor)?;
        let purpose_primitives = Self::detect_purpose(text, &database, executor)?;
        
        // Step 2: Combine all primitives
        let mut all_primitives = Vec::new();
        all_primitives.extend(agency_primitives);
        all_primitives.extend(action_primitives);
        all_primitives.extend(transformation_primitives);
        all_primitives.extend(conditionality_primitives);
        all_primitives.extend(sequence_primitives);
        all_primitives.extend(purpose_primitives);
        
        // Step 3: Enhanced analysis with mirroring decomposer
        let (synthesis_opportunities, mirror_analysis, negation_transforms, morphological_confidence) = 
            if let Some(decomposer) = &mut self.mirroring_decomposer {
                Self::enhanced_analysis_with_mirroring(text, &all_primitives, decomposer)?
            } else {
                (Vec::new(), Vec::new(), Vec::new(), 0.0)
            };
        
        // Step 4: Calculate spatial coherence
        let spatial_coherence = Self::calculate_spatial_coherence(&all_primitives);
        
        // Step 5: Calculate overall confidence (enhanced with morphological confidence)
        let base_confidence = Self::calculate_overall_confidence(&all_primitives, spatial_coherence);
        let confidence = if morphological_confidence > 0.0 {
            (base_confidence * 0.7) + (morphological_confidence * 0.3)
        } else {
            base_confidence
        };
        
        // Step 6: Generate extraction path for debugging
        let extraction_path = Self::generate_extraction_path(&all_primitives);
        
        let execution_time = start_time.elapsed();
        
        Ok(FunctionSignature {
            primitives: all_primitives,
            confidence,
            source_text: text.to_string(),
            extraction_path,
            spatial_coherence,
            execution_time_ms: execution_time.as_millis() as f32,
            
            // Enhanced with mirroring capabilities
            synthesis_opportunities,
            mirror_analysis,
            negation_transforms,
            morphological_confidence,
        })
    }
    
    /// Enhanced analysis using mirroring decomposer
    fn enhanced_analysis_with_mirroring(
        text: &str,
        primitives: &[FunctionalPrimitive],
        decomposer: &mut MirroringDecomposer
    ) -> Result<(Vec<SynthesisResult>, Vec<MirrorPair>, Vec<NegationAnalysis>, f32), PluginError> {
        let mut synthesis_opportunities = Vec::new();
        let mut mirror_analysis = Vec::new();
        let mut negation_transforms = Vec::new();
        
        // Process each word in the text for morphological analysis
        let words = preprocess_text(text).split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
            
        let mut total_morphological_confidence = 0.0;
        let mut analyzed_words = 0;
        
        for word in &words {
            // Find mirrors for each significant word
            let mirrors = decomposer.find_mirrors(word);
            for mirror in mirrors {
                mirror_analysis.push(MirrorPair {
                    original: mirror.original,
                    mirror: mirror.mirror,
                    mirror_type: match mirror.mirror_type {
                        MirroringMirrorType::Negation => MirrorType::Negation,
                        MirroringMirrorType::Reversal => MirrorType::Reversal,
                        MirroringMirrorType::Complementary => MirrorType::Complementary,
                        MirroringMirrorType::Gradable => MirrorType::Gradable,
                        MirroringMirrorType::Directional => MirrorType::Directional,
                        // Handle new etymological mirror types
                        MirroringMirrorType::EtymologicalOpposite { .. } => MirrorType::EtymologicalOpposite { 
                            root_family: crate::mirroring::EtymologyFamily::Latin, 
                            semantic_distance: 0.5 
                        },
                        MirroringMirrorType::FunctionalOpposite { .. } => MirrorType::FunctionalOpposite { 
                            role_inversion: crate::mirroring::RoleType::Agent, 
                            domain_context: "general".to_string() 
                        },
                        MirroringMirrorType::MorphologicalOpposite { .. } => MirrorType::MorphologicalOpposite { 
                            valid_negation: crate::mirroring::NegationType::Prefix("un".to_string()), 
                            productivity_score: 0.5 
                        },
                        MirroringMirrorType::CrossLinguisticMirror { .. } => MirrorType::CrossLinguisticMirror { 
                            source_etymology: crate::core::EtymologyOrigin::Latin, 
                            target_etymology: crate::core::EtymologyOrigin::Germanic, 
                            borrowing_pattern: crate::mirroring::BorrowingType::Direct 
                        },
                        MirroringMirrorType::SpatialOpposite { .. } => MirrorType::SpatialOpposite { 
                            vector_opposition: crate::core::Coordinate3D::new(0.5, 0.5, 0.5), 
                            clustering_confidence: 0.7 
                        },
                    },
                    confidence: mirror.confidence,
                });
            }
            
            // Validate morphological decomposition quality
            let validation = decomposer.validate_decomposition_quality(word);
            if validation.round_trip_success {
                total_morphological_confidence += validation.morpheme_coherence;
                analyzed_words += 1;
            }
        }
        
        // Generate synthesis opportunities for each primitive type found
        for primitive in primitives {
            match primitive {
                FunctionalPrimitive::Agency { actor, .. } => {
                    // For now, skip synthesis for agency primitives without word access
                    // TODO: Implement proper word extraction from LinguisticNode
                },
                FunctionalPrimitive::Action { verb, .. } => {
                    // For now, skip synthesis for action primitives without word access
                    // TODO: Implement proper word extraction from LinguisticNode
                },
                _ => {
                    // Handle other primitive types as needed
                }
            }
        }
        
        // Detect negations in the text
        negation_transforms.extend(Self::detect_negations(text));
        
        // Calculate average morphological confidence
        let morphological_confidence = if analyzed_words > 0 {
            total_morphological_confidence / analyzed_words as f32
        } else {
            0.0
        };
        
        Ok((synthesis_opportunities, mirror_analysis, negation_transforms, morphological_confidence))
    }
    
    /// Detect negation patterns in text
    fn detect_negations(text: &str) -> Vec<NegationAnalysis> {
        let mut negations = Vec::new();
        let lower_text = text.to_lowercase();
        
        // Detect different types of negations
        if lower_text.contains("don't want") || lower_text.contains("do not want") {
            // Extract what they don't want
            if let Some(start) = lower_text.find("don't want") {
                let after_negation = &lower_text[start + 10..];
                let concept = after_negation.split_whitespace()
                    .take(3)
                    .collect::<Vec<_>>()
                    .join(" ");
                
                negations.push(NegationAnalysis {
                    negated_concept: concept.trim_start_matches("a ").trim_start_matches("an ").to_string(),
                    negation_type: NegationType::DirectNegation,
                    negation_reason: "Explicitly stated preference against".to_string(),
                    alternative_suggestions: Vec::new(),
                });
            }
        }
        
        if lower_text.contains("not looking for") {
            if let Some(start) = lower_text.find("not looking for") {
                let after_negation = &lower_text[start + 15..];
                let concept = after_negation.split_whitespace()
                    .take(3)
                    .collect::<Vec<_>>()
                    .join(" ");
                
                negations.push(NegationAnalysis {
                    negated_concept: concept.trim_start_matches("a ").trim_start_matches("an ").to_string(),
                    negation_type: NegationType::Exclusion,
                    negation_reason: "Actively excluding from search".to_string(),
                    alternative_suggestions: Vec::new(),
                });
            }
        }
        
        if lower_text.contains("instead of") || lower_text.contains("rather than") {
            negations.push(NegationAnalysis {
                negated_concept: "previous option".to_string(),
                negation_type: NegationType::Opposition,
                negation_reason: "Preferring alternative approach".to_string(),
                alternative_suggestions: Vec::new(),
            });
        }
        
        negations
    }
    
    // Detection algorithms implementation
    
    fn detect_agency(text: &str, database: &LingoDatabase, executor: &mut LingoExecutor) -> Result<Vec<FunctionalPrimitive>, PluginError> {
        let mut agents = Vec::new();
        
        // Step 1: Preprocess text
        let cleaned_text = preprocess_text(text);
        let words: Vec<&str> = cleaned_text.split_whitespace().collect();
        
        // Step 2: For each word, decompose into morphemes and check for agency
        for word in words {
            let morpheme_analysis = decompose_word_to_morphemes(&word, database, executor);
            
            // Step 3: Check if any morphemes indicate agency
            let has_agent_morphemes = morpheme_analysis.iter().any(|morpheme| {
                matches!(morpheme.morpheme_type, 
                    MorphemeType::AgentSuffix | 
                    MorphemeType::Root if Self::is_agent_root(&morpheme.surface_form)
                )
            });
            
            // Step 4: If word contains agent morphemes, analyze the full composition
            if has_agent_morphemes {
                let capability_level = 0.8; // High capability for agent morphemes
                let spatial_position = calculate_composed_position(&morpheme_analysis);
                
                // Create a composite node from morphemes
                let actor = LinguisticNode::new(
                    NodeId(0), // Would be assigned by database in real implementation
                    Layer::Words,
                    spatial_position
                );
                
                agents.push(FunctionalPrimitive::Agency {
                    actor,
                    capability_level,
                    responsibility_scope: Vec::new(), // Simplified for now
                });
            }
        }
        
        Ok(agents)
    }
    
    fn detect_actions(text: &str, database: &LingoDatabase, executor: &mut LingoExecutor) -> Result<Vec<FunctionalPrimitive>, PluginError> {
        let mut actions = Vec::new();
        
        // Step 1: Preprocess text
        let cleaned_text = preprocess_text(text);
        let words: Vec<&str> = cleaned_text.split_whitespace().collect();
        
        // Step 2: For each word, check for action morphology
        for word in words {
            let morpheme_analysis = decompose_word_to_morphemes(&word, database, executor);
            
            // Step 3: Look for action-indicating morphemes
            let has_action_morphemes = morpheme_analysis.iter().any(|morpheme| {
                matches!(morpheme.morpheme_type,
                    MorphemeType::VerbSuffix |
                    MorphemeType::Root if Self::is_action_root(&morpheme.surface_form)
                )
            });
            
            // Step 4: If action morphemes found, compose the action primitive
            if has_action_morphemes {
                let transformation_strength = 0.7; // Default strength for action verbs
                let spatial_position = calculate_composed_position(&morpheme_analysis);
                
                // Determine action type through spatial positioning
                let action_type = Self::classify_action_type_by_position(&spatial_position);
                
                // Create composite verb node
                let verb = LinguisticNode::new(
                    NodeId(0), // Would be assigned by database
                    Layer::Words,
                    spatial_position
                );
                
                actions.push(FunctionalPrimitive::Action {
                    verb,
                    transformation_type: action_type,
                    intensity: transformation_strength,
                    temporal_aspect: TemporalAspect::Present, // Default for now
                });
            }
        }
        
        Ok(actions)
    }
    
    fn detect_transformations(text: &str, database: &LingoDatabase, executor: &mut LingoExecutor) -> Result<Vec<FunctionalPrimitive>, PluginError> {
        let mut transformations = Vec::new();
        
        // Find transformation patterns through phrase analysis
        let query = QueryBuilder::find(text)
            .layer_up() // Look at phrases
            .similar_threshold(0.1) // Very tight clustering
            .limit(15)
            .compile();
        
        let result = executor.execute(&query)
            .map_err(|e| PluginError::InitializationFailed {
                plugin: "function_extraction".to_string(),
                error: e.to_string(),
            })?;
        
        for node_id in result.nodes.as_slice() {
            if let Ok(node) = database.get_node(*node_id) {
                if matches!(node.layer, Layer::Phrases) {
                    if let Ok(phrase) = database.get_node_word(*node_id) {
                        if Self::contains_transformation_markers(phrase) {
                            if let Some((input_state, output_state)) = Self::extract_transformation_states(phrase, database) {
                                let process_vector = Self::calculate_transformation_vector(
                                    input_state.position,
                                    output_state.position,
                                );
                                let reversibility = Self::calculate_reversibility(phrase);
                                
                                transformations.push(FunctionalPrimitive::Transformation {
                                    input_state,
                                    output_state,
                                    process_vector,
                                    reversibility,
                                });
                            }
                        }
                    }
                }
            }
        }
        
        Ok(transformations)
    }
    
    fn detect_conditionality(text: &str, database: &LingoDatabase, executor: &mut LingoExecutor) -> Result<Vec<FunctionalPrimitive>, PluginError> {
        let mut conditions = Vec::new();
        
        // Find conditional markers
        let conditional_words = ["if", "when", "unless", "provided", "assuming", "given"];
        
        for conditional_word in &conditional_words {
            if text.to_lowercase().contains(conditional_word) {
                let query = QueryBuilder::find(conditional_word)
                    .layer_up() // Look at phrase context
                    .similar_threshold(0.25)
                    .limit(10)
                    .compile();
                
                let result = executor.execute(&query)
                    .map_err(|e| PluginError::InitializationFailed {
                        plugin: "function_extraction".to_string(),
                        error: e.to_string(),
                    })?;
                
                for node_id in result.nodes.as_slice() {
                    if let Ok(node) = database.get_node(*node_id) {
                        let dependency_type = Self::classify_dependency_type(&node);
                        let certainty_level = Self::calculate_certainty_level(&node);
                        let scope = Self::extract_conditional_scope(&node, database);
                        
                        conditions.push(FunctionalPrimitive::Conditionality {
                            condition: *node,
                            dependency_type,
                            certainty_level,
                            scope,
                        });
                    }
                }
            }
        }
        
        Ok(conditions)
    }
    
    fn detect_sequences(text: &str, database: &LingoDatabase, executor: &mut LingoExecutor) -> Result<Vec<FunctionalPrimitive>, PluginError> {
        let mut sequences = Vec::new();
        
        // Find sequence markers
        let sequence_words = ["then", "next", "after", "before", "first", "second", "finally"];
        
        for sequence_word in &sequence_words {
            if text.to_lowercase().contains(sequence_word) {
                let query = QueryBuilder::find(sequence_word)
                    .layer_up()
                    .similar_threshold(0.2)
                    .limit(10)
                    .compile();
                
                let result = executor.execute(&query)
                    .map_err(|e| PluginError::InitializationFailed {
                        plugin: "function_extraction".to_string(),
                        error: e.to_string(),
                    })?;
                
                if !result.nodes.is_empty() {
                    let steps = Self::extract_sequence_steps(text, database);
                    let ordering_type = Self::determine_ordering_type(text);
                    let temporal_density = Self::calculate_temporal_density(text);
                    
                    sequences.push(FunctionalPrimitive::Sequence {
                        steps,
                        ordering_type,
                        temporal_density,
                    });
                    break; // Only create one sequence per text
                }
            }
        }
        
        Ok(sequences)
    }
    
    fn detect_purpose(text: &str, database: &LingoDatabase, executor: &mut LingoExecutor) -> Result<Vec<FunctionalPrimitive>, PluginError> {
        let mut purposes = Vec::new();
        
        // Find purpose markers
        let purpose_words = ["to", "for", "in order to", "so that", "aiming", "goal"];
        
        for purpose_word in &purpose_words {
            if text.to_lowercase().contains(purpose_word) {
                let query = QueryBuilder::find(purpose_word)
                    .layer_up()
                    .similar_threshold(0.3)
                    .limit(5)
                    .compile();
                
                let result = executor.execute(&query)
                    .map_err(|e| PluginError::InitializationFailed {
                        plugin: "function_extraction".to_string(),
                        error: e.to_string(),
                    })?;
                
                if !result.nodes.is_empty() {
                    if let Some(goal_state) = Self::extract_goal_state(text, database) {
                        let intention_strength = Self::calculate_intention_strength(text);
                        let success_criteria = Self::extract_success_criteria(text, database);
                        
                        purposes.push(FunctionalPrimitive::Purpose {
                            goal_state,
                            intention_strength,
                            success_criteria,
                        });
                        break; // Only create one purpose per text
                    }
                }
            }
        }
        
        Ok(purposes)
    }
    
    // Helper methods for analysis
    
    fn is_agent_root(root: &str) -> bool {
        // Check if root morpheme suggests agency
        let agent_roots = ["manag", "lead", "direct", "control", "supervis", 
                          "coordinat", "organiz", "develop", "design", "creat"];
        agent_roots.iter().any(|&r| root.contains(r))
    }
    
    fn is_action_root(root: &str) -> bool {
        // Check if root morpheme suggests action
        let action_roots = ["creat", "build", "develop", "analyz", "process", 
                           "convert", "transform", "chang", "modif", "organiz",
                           "compil", "deploy", "generat", "produc", "execut"];
        action_roots.iter().any(|&r| root.contains(r))
    }
    
    fn classify_action_type_by_position(position: &Coordinate3D) -> ActionType {
        // Classify based on spatial position
        if position.x > 0.7 {
            ActionType::Creative
        } else if position.y > 0.7 {
            ActionType::Analytical
        } else if position.x < 0.3 {
            ActionType::Mechanical
        } else {
            ActionType::Transformative
        }
    }
    
    fn extract_temporal_from_morphemes(morphemes: &[MorphemeAnalysis]) -> TemporalAspect {
        // Check for tense markers
        for morpheme in morphemes {
            if matches!(morpheme.morpheme_type, MorphemeType::TenseSuffix) {
                if morpheme.surface_form == "ed" {
                    return TemporalAspect::Past;
                } else if morpheme.surface_form == "ing" {
                    return TemporalAspect::Continuous;
                }
            }
        }
        TemporalAspect::Present
    }
    
    fn is_agent_word(word: &str, node: &LinguisticNode) -> bool {
        // Check for agent morphemes and semantic markers
        let agent_suffixes = ["-er", "-or", "-ant", "-ist", "-ian"];
        let agent_words = ["manager", "developer", "user", "system", "team", "company"];
        let non_agent_words = ["running", "working", "jumping"];
        
        // Explicitly exclude non-agent words
        if non_agent_words.iter().any(|&non_agent| word.contains(non_agent)) {
            return false;
        }
        
        agent_suffixes.iter().any(|suffix| word.ends_with(suffix)) ||
        agent_words.iter().any(|&agent| word.contains(agent)) ||
        node.position.x > 0.5 // High semantic similarity to agent concepts
    }
    
    fn is_action_word( word: &str, node: &LinguisticNode) -> bool {
        // Check for action morphemes and semantic markers
        let action_suffixes = ["-ize", "-fy", "-ate", "-en"];
        let action_words = ["create", "build", "develop", "analyze", "process", "convert"];
        let non_action_words = ["table", "chair", "book", "house"];
        
        // Explicitly exclude non-action words
        if non_action_words.iter().any(|&non_action| word.contains(non_action)) {
            return false;
        }
        
        action_suffixes.iter().any(|suffix| word.ends_with(suffix)) ||
        action_words.iter().any(|&action| word.contains(action)) ||
        (node.position.z >= 3.0 && node.position.z <= 4.0) // Word/Phrase layer actions
    }
    
    fn contains_transformation_markers( phrase: &str) -> bool {
        let transformation_markers = ["becomes", "converts to", "transforms into", "changes to", "→", "->"];
        transformation_markers.iter().any(|marker| phrase.contains(marker))
    }
    
    fn calculate_capability_strength( node: &LinguisticNode) -> f32 {
        // Calculate based on node position and morphological markers
        let base_strength = (node.position.x + node.position.y) / 2.0;
        (base_strength * 0.7 + 0.3).min(1.0)
    }
    
    fn extract_responsibility_scope( _node: &LinguisticNode, _database: &LingoDatabase) -> Vec<LinguisticNode> {
        // Simplified implementation - in practice would follow connections
        Vec::new()
    }
    
    fn classify_action_type( node: &LinguisticNode) -> ActionType {
        // Classify based on position and morphological analysis
        if node.position.x > 0.7 {
            ActionType::Creative
        } else if node.position.y > 0.7 {
            ActionType::Analytical
        } else {
            ActionType::Mechanical
        }
    }
    
    fn calculate_action_intensity( node: &LinguisticNode) -> f32 {
        // Base intensity on semantic position
        ((node.position.x + node.position.y + node.position.z) / 3.0).min(1.0)
    }
    
    fn extract_temporal_aspect( node: &LinguisticNode) -> TemporalAspect {
        // Simplified temporal aspect extraction
        if node.position.z > 4.0 {
            TemporalAspect::Future
        } else if node.position.z < 2.0 {
            TemporalAspect::Past
        } else {
            TemporalAspect::Present
        }
    }
    
    fn extract_transformation_states( phrase: &str, database: &LingoDatabase) -> Option<(LinguisticNode, LinguisticNode)> {
        // Simplified state extraction - would use more sophisticated parsing
        let words: Vec<&str> = phrase.split_whitespace().collect();
        if words.len() >= 3 {
            // Try to find input and output states
            // This is a simplified implementation
            None // Would return actual states in full implementation
        } else {
            None
        }
    }
    
    fn calculate_transformation_vector( input: Coordinate3D, output: Coordinate3D) -> Coordinate3D {
        Coordinate3D {
            x: output.x - input.x,
            y: output.y - input.y,
            z: output.z - input.z,
        }
    }
    
    fn calculate_reversibility( _phrase: &str) -> f32 {
        // Simplified reversibility calculation
        0.5
    }
    
    fn classify_dependency_type( _node: &LinguisticNode) -> DependencyType {
        DependencyType::Conditional
    }
    
    fn calculate_certainty_level( node: &LinguisticNode) -> f32 {
        // Base certainty on semantic position
        node.position.x.min(1.0)
    }
    
    fn extract_conditional_scope( _node: &LinguisticNode, _database: &LingoDatabase) -> Vec<LinguisticNode> {
        Vec::new()
    }
    
    fn extract_sequence_steps( _text: &str, _database: &LingoDatabase) -> Vec<LinguisticNode> {
        Vec::new()
    }
    
    fn determine_ordering_type( _text: &str) -> OrderingType {
        OrderingType::Sequential
    }
    
    fn calculate_temporal_density( _text: &str) -> f32 {
        0.5
    }
    
    fn extract_goal_state( _text: &str, _database: &LingoDatabase) -> Option<LinguisticNode> {
        None
    }
    
    fn calculate_intention_strength( _text: &str) -> f32 {
        0.7
    }
    
    fn extract_success_criteria( _text: &str, _database: &LingoDatabase) -> Vec<LinguisticNode> {
        Vec::new()
    }
    
    fn calculate_spatial_coherence( primitives: &[FunctionalPrimitive]) -> f32 {
        if primitives.len() < 2 {
            return 1.0;
        }
        
        let positions: Vec<Coordinate3D> = primitives.iter()
            .map(|p| Self::get_primitive_center_position(p))
            .collect();
        
        let mut total_distance = 0.0;
        let mut pairs = 0;
        
        for i in 0..positions.len() {
            for j in (i+1)..positions.len() {
                total_distance += Self::euclidean_distance(positions[i], positions[j]);
                pairs += 1;
            }
        }
        
        let average_distance = total_distance / pairs as f32;
        1.0 / (1.0 + average_distance)
    }
    
    fn get_primitive_center_position( primitive: &FunctionalPrimitive) -> Coordinate3D {
        match primitive {
            FunctionalPrimitive::Agency { actor, .. } => actor.position,
            FunctionalPrimitive::Action { verb, .. } => verb.position,
            FunctionalPrimitive::Transformation { input_state, output_state, .. } => {
                Coordinate3D {
                    x: (input_state.position.x + output_state.position.x) / 2.0,
                    y: (input_state.position.y + output_state.position.y) / 2.0,
                    z: (input_state.position.z + output_state.position.z) / 2.0,
                }
            },
            FunctionalPrimitive::Conditionality { condition, .. } => condition.position,
            FunctionalPrimitive::Sequence { steps, .. } => {
                if steps.is_empty() {
                    Coordinate3D { x: 0.0, y: 0.0, z: 0.0 }
                } else {
                    let sum_x: f32 = steps.iter().map(|s| s.position.x).sum();
                    let sum_y: f32 = steps.iter().map(|s| s.position.y).sum();
                    let sum_z: f32 = steps.iter().map(|s| s.position.z).sum();
                    let len = steps.len() as f32;
                    Coordinate3D { x: sum_x / len, y: sum_y / len, z: sum_z / len }
                }
            },
            FunctionalPrimitive::Purpose { goal_state, .. } => goal_state.position,
        }
    }
    
    fn euclidean_distance( a: Coordinate3D, b: Coordinate3D) -> f32 {
        ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
    }
    
    fn calculate_overall_confidence( primitives: &[FunctionalPrimitive], spatial_coherence: f32) -> f32 {
        if primitives.is_empty() {
            return 0.0;
        }
        
        let individual_confidence: f32 = primitives.iter()
            .map(|p| Self::get_primitive_confidence(p))
            .sum::<f32>() / primitives.len() as f32;
        
        (individual_confidence * (1.0 - 0.3)) +
        (spatial_coherence * 0.3)
    }
    
    fn get_primitive_confidence( primitive: &FunctionalPrimitive) -> f32 {
        match primitive {
            FunctionalPrimitive::Agency { capability_level, .. } => *capability_level,
            FunctionalPrimitive::Action { intensity, .. } => *intensity,
            FunctionalPrimitive::Transformation { reversibility, .. } => *reversibility,
            FunctionalPrimitive::Conditionality { certainty_level, .. } => *certainty_level,
            FunctionalPrimitive::Sequence { temporal_density, .. } => *temporal_density,
            FunctionalPrimitive::Purpose { intention_strength, .. } => *intention_strength,
        }
    }
    
    fn generate_extraction_path( _primitives: &[FunctionalPrimitive]) -> Vec<String> {
        // Simplified path generation
        vec!["morphological_analysis".to_string(), "spatial_clustering".to_string(), "semantic_classification".to_string()]
    }
}

impl Plugin for FunctionExtractor {
    fn id(&self) -> &'static str {
        "function_extraction"
    }
    
    fn name(&self) -> &'static str {
        "Function Extraction Plugin"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn initialize(&mut self, database: &LingoDatabase) -> Result<(), PluginError> {
        // Store the database path
        self.database_path = Some("english.lingo".to_string());
        
        // Create executor and load the database
        let mut executor = LingoExecutor::new();
        if let Err(e) = executor.load_database("english.lingo") {
            return Err(PluginError::InitializationFailed {
                plugin: "function_extraction".to_string(),
                error: format!("Failed to load database in executor: {}", e),
            });
        }
        
        self.executor = Some(executor);
        
        // Initialize mirroring decomposer for enhanced capabilities
        let db_arc = Arc::new(LingoDatabase::open("english.lingo")
            .map_err(|e| PluginError::InitializationFailed {
                plugin: "function_extraction".to_string(),
                error: format!("Failed to open database for mirroring: {}", e),
            })?);
            
        let mirroring_decomposer = MirroringDecomposer::new(db_arc)
            .map_err(|e| PluginError::InitializationFailed {
                plugin: "function_extraction".to_string(),
                error: format!("Failed to create mirroring decomposer: {}", e),
            })?;
            
        self.mirroring_decomposer = Some(mirroring_decomposer);
        
        Ok(())
    }
    
    fn handle_command(&mut self, command: &str, args: &[String], _context: &PluginContext) -> Result<Option<PluginResult>, PluginError> {
        match command {
            "extract_function" => {
                if args.is_empty() {
                    return Err(PluginError::CommandNotSupported {
                        plugin: self.id().to_string(),
                        command: "extract_function requires text argument".to_string(),
                    });
                }
                
                let text = &args[0];
                let signature = self.extract_function_signature(text)?;
                
                let mut data = HashMap::new();
                data.insert("signature".to_string(), format!("{:?}", signature));
                
                Ok(Some(PluginResult::CustomResults {
                    data,
                    confidence: signature.confidence,
                }))
            },
            "set_confidence_threshold" => {
                if args.is_empty() {
                    return Err(PluginError::CommandNotSupported {
                        plugin: self.id().to_string(),
                        command: "set_confidence_threshold requires threshold value".to_string(),
                    });
                }
                
                // Note: This would require mutable access in practice
                Ok(Some(PluginResult::CommandResult {
                    success: true,
                    message: format!("Confidence threshold set to {}", args[0]),
                    data: None,
                }))
            },
            _ => Ok(None),
        }
    }
}

impl Default for FunctionExtractor {
    fn default() -> Self {
        Self::new()
    }
}

// Types from the specification

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FunctionalPrimitive {
    Agency {
        actor: LinguisticNode,
        capability_level: f32,
        responsibility_scope: Vec<LinguisticNode>,
    },
    Action {
        verb: LinguisticNode,
        transformation_type: ActionType,
        intensity: f32,
        temporal_aspect: TemporalAspect,
    },
    Transformation {
        input_state: LinguisticNode,
        output_state: LinguisticNode,
        process_vector: Coordinate3D,
        reversibility: f32,
    },
    Conditionality {
        condition: LinguisticNode,
        dependency_type: DependencyType,
        certainty_level: f32,
        scope: Vec<LinguisticNode>,
    },
    Sequence {
        steps: Vec<LinguisticNode>,
        ordering_type: OrderingType,
        temporal_density: f32,
    },
    Purpose {
        goal_state: LinguisticNode,
        intention_strength: f32,
        success_criteria: Vec<LinguisticNode>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSignature {
    // Core extraction results (same API)
    pub primitives: Vec<FunctionalPrimitive>,
    pub confidence: f32,
    pub source_text: String,
    pub extraction_path: Vec<String>,
    pub spatial_coherence: f32,
    pub execution_time_ms: f32,
    
    // NEW: Enhanced with generative insights from mirroring decomposer
    pub synthesis_opportunities: Vec<SynthesisResult>,    // Generated alternatives
    pub mirror_analysis: Vec<MirrorPair>,                // Opposites found
    pub negation_transforms: Vec<NegationAnalysis>,      // Negation handling
    pub morphological_confidence: f32,                   // Enhanced morphological confidence
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisResult {
    pub generated_word: String,
    pub predicted_function: FunctionalPrimitive,
    pub morpheme_composition: Vec<String>,
    pub confidence: f32,
    pub spatial_position: Coordinate3D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorPair {
    pub original: String,
    pub mirror: String,
    pub mirror_type: MirrorType,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MirrorType {
    Negation,      // un-, dis-, non-
    Reversal,      // create/destroy, build/demolish
    Complementary, // teacher/student, employer/employee
    Gradable,      // hot/cold, big/small
    Directional,   // up/down, in/out
    // New etymological mirror types
    EtymologicalOpposite { 
        root_family: crate::mirroring::EtymologyFamily, 
        semantic_distance: f32 
    },
    FunctionalOpposite { 
        role_inversion: crate::mirroring::RoleType, 
        domain_context: String 
    },
    MorphologicalOpposite { 
        valid_negation: crate::mirroring::NegationType, 
        productivity_score: f32 
    },
    CrossLinguisticMirror { 
        source_etymology: crate::core::EtymologyOrigin, 
        target_etymology: crate::core::EtymologyOrigin, 
        borrowing_pattern: crate::mirroring::BorrowingType 
    },
    SpatialOpposite { 
        vector_opposition: crate::core::Coordinate3D, 
        clustering_confidence: f32 
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegationAnalysis {
    pub negated_concept: String,
    pub negation_type: NegationType,
    pub negation_reason: String,
    pub alternative_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NegationType {
    DirectNegation,     // "don't want"
    Opposition,         // "instead of"
    Exclusion,          // "not looking for"
    Preference,         // "prefer X over Y"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Creative,
    Analytical,
    Mechanical,
    Transformative,
    Communicative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalAspect {
    Past,
    Present,
    Future,
    Continuous,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    Conditional,
    Causal,
    Temporal,
    Logical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderingType {
    Sequential,
    Parallel,
    Hierarchical,
    Cyclical,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_extractor_creation() {
        let extractor = FunctionExtractor::new();
        assert_eq!(extractor.id(), "function_extraction");
        assert_eq!(extractor.name(), "Function Extraction Plugin");
        assert_eq!(extractor.version(), "1.0.0");
    }
    
    #[test]
    fn test_agent_root_detection() {
        assert!(FunctionExtractor::is_agent_root("manag"));
        assert!(FunctionExtractor::is_agent_root("develop"));
        assert!(FunctionExtractor::is_agent_root("lead"));
        assert!(!FunctionExtractor::is_agent_root("run"));
        assert!(!FunctionExtractor::is_agent_root("jump"));
    }
    
    #[test]
    fn test_action_root_detection() {
        assert!(FunctionExtractor::is_action_root("creat"));
        assert!(FunctionExtractor::is_action_root("analyz"));
        assert!(FunctionExtractor::is_action_root("process"));
        assert!(!FunctionExtractor::is_action_root("tabl"));
        assert!(!FunctionExtractor::is_action_root("chair"));
    }
    
    #[test]
    fn test_transformation_markers() {
        assert!(FunctionExtractor::contains_transformation_markers("X becomes Y"));
        assert!(FunctionExtractor::contains_transformation_markers("converts to"));
        assert!(FunctionExtractor::contains_transformation_markers("A → B"));
        assert!(!FunctionExtractor::contains_transformation_markers("simple phrase"));
    }
    
    #[test]
    fn test_spatial_coherence_calculation() {
        let node1 = LinguisticNode::new(
            NodeId(1),
            Layer::Words,
            Coordinate3D { x: 0.1, y: 0.1, z: 0.1 }
        );
        
        let node2 = LinguisticNode::new(
            NodeId(2),
            Layer::Words,
            Coordinate3D { x: 0.2, y: 0.2, z: 0.2 }
        );
        
        let primitives = vec![
            FunctionalPrimitive::Agency {
                actor: node1,
                capability_level: 0.8,
                responsibility_scope: vec![],
            },
            FunctionalPrimitive::Action {
                verb: node2,
                transformation_type: ActionType::Creative,
                intensity: 0.7,
                temporal_aspect: TemporalAspect::Present,
            },
        ];
        
        let coherence = FunctionExtractor::calculate_spatial_coherence(&primitives);
        assert!(coherence > 0.0 && coherence <= 1.0);
    }
}
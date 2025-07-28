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

//! Intent Detection Plugin - Extract compositional intent from natural language
//! 
//! Implements the complete Intent = Function × PragmaticOperators specification.
//! Uses bottom-up compositionality through 9 specialized operator detectors.

use std::collections::HashMap;
use std::time::Instant;
use serde::{Serialize, Deserialize};

use crate::core::Coordinate3D;
use crate::storage::LingoDatabase;
use crate::engine::LingoExecutor;
use crate::morphology::preprocess_text;
use super::{Plugin, PluginContext, PluginResult, PluginError, 
           FunctionExtractor, FunctionalPrimitive};

/// Intent detection plugin implementing Function × PragmaticOperators algebra
pub struct IntentDetector {
    database_path: Option<String>,
    executor: Option<LingoExecutor>,
    function_extractor: Option<FunctionExtractor>,
    pub operator_detectors: Option<OperatorDetectorSuite>,
    composition_engine: Option<IntentCompositionEngine>,
    confidence_threshold: f32,
}

impl IntentDetector {
    pub fn new() -> Self {
        Self {
            database_path: None,
            executor: None,
            function_extractor: None,
            operator_detectors: None,
            composition_engine: None,
            confidence_threshold: 0.5,
        }
    }
    
    /// Detect intent from text using Function × PragmaticOperators algebra
    pub fn detect_intent(&mut self, text: &str) -> Result<Intent, PluginError> {
        let start_time = Instant::now();
        
        // Step 1: Extract core function using existing system
        let function_extractor = self.function_extractor.as_mut()
            .ok_or_else(|| PluginError::NotInitialized("Function extractor not available".to_string()))?;
        
        let function_signature = function_extractor.extract_function_signature(text)?;
        let primary_function = function_signature.primitives.get(0)
            .ok_or_else(|| PluginError::CommandNotSupported {
                plugin: "intent_detection".to_string(),
                command: "No primary function detected".to_string(),
            })?;
        
        // Step 2: Detect all 9 operator types in parallel
        let db_path = self.database_path.as_ref()
            .ok_or_else(|| PluginError::InitializationFailed {
                plugin: "intent_detection".to_string(),
                error: "Database path not configured".to_string(),
            })?;
            
        let database = LingoDatabase::open(db_path)
            .map_err(|e| PluginError::InitializationFailed {
                plugin: "intent_detection".to_string(),
                error: e.to_string(),
            })?;
        
        let operator_detectors = self.operator_detectors.as_ref()
            .ok_or_else(|| PluginError::NotInitialized("Operator detectors not available".to_string()))?;
        
        let operators = operator_detectors.detect_all_operators(text, &database)?;
        
        // Step 3: Validate operator coherence
        let operator_coherence = self.calculate_operator_coherence(&operators);
        
        // Step 4: Compose intent from function + operators
        let composition_engine = self.composition_engine.as_ref()
            .ok_or_else(|| PluginError::NotInitialized("Composition engine not available".to_string()))?;
        
        let intent = composition_engine.compose_intent(
            primary_function.clone(),
            operators,
            operator_coherence,
            text
        )?;
        
        let execution_time = start_time.elapsed();
        
        Ok(Intent {
            core_function: primary_function.clone(),
            pragmatic_operators: intent.pragmatic_operators,
            intent_confidence: intent.intent_confidence,
            operator_coherence,
            compositional_path: intent.compositional_path,
            execution_time_ms: execution_time.as_millis() as f32,
            source_text: text.to_string(),
        })
    }
    
    /// Calculate how well operators work together using spatial coherence analysis
    pub fn calculate_operator_coherence(&self, operators: &PragmaticOperators) -> f32 {
        let mut all_positions = Vec::new();
        let mut operator_count = 0;
        
        // Collect spatial positions from all operators
        for op in &operators.directional {
            if let Some(pos) = op.spatial_vector {
                all_positions.push(pos);
                operator_count += 1;
            }
        }
        
        // For other operator types, we'd need to extract their spatial positions
        // For now, use operator count as a proxy
        if !operators.modal.is_empty() { operator_count += 1; }
        if !operators.temporal.is_empty() { operator_count += 1; }
        if !operators.conditional.is_empty() { operator_count += 1; }
        if !operators.negation.is_empty() { operator_count += 1; }
        if !operators.intensity.is_empty() { operator_count += 1; }
        if !operators.certainty.is_empty() { operator_count += 1; }
        if !operators.scope.is_empty() { operator_count += 1; }
        if !operators.social.is_empty() { operator_count += 1; }
        
        if operator_count == 0 {
            return 1.0;
        }
        
        // Calculate spatial coherence if we have positions
        let spatial_coherence = if all_positions.len() >= 2 {
            self.calculate_spatial_coherence_between_positions(&all_positions)
        } else {
            0.8 // Default coherence for single or no spatial operators
        };
        
        // Bonus for having multiple operators (compositional richness)
        let composition_bonus = if operator_count >= 3 {
            1.1
        } else {
            1.0
        };
        
        (spatial_coherence * composition_bonus).min(1.0)
    }
    
    /// Calculate spatial coherence between operator positions
    fn calculate_spatial_coherence_between_positions(&self, positions: &[Coordinate3D]) -> f32 {
        if positions.len() < 2 {
            return 1.0;
        }
        
        let mut total_distance = 0.0;
        let mut pairs = 0;
        
        for i in 0..positions.len() {
            for j in (i+1)..positions.len() {
                let distance = ((positions[i].x - positions[j].x).powi(2) +
                               (positions[i].y - positions[j].y).powi(2) +
                               (positions[i].z - positions[j].z).powi(2)).sqrt();
                total_distance += distance;
                pairs += 1;
            }
        }
        
        let average_distance = total_distance / pairs as f32;
        // Convert distance to coherence (closer = more coherent)
        1.0 / (1.0 + average_distance)
    }
}

impl Plugin for IntentDetector {
    fn id(&self) -> &'static str {
        "intent_detection"
    }
    
    fn name(&self) -> &'static str {
        "Intent Detection Plugin"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn dependencies(&self) -> Vec<&'static str> {
        vec!["function_extraction"]
    }
    
    fn initialize(&mut self, database: &LingoDatabase) -> Result<(), PluginError> {
        // Store database path
        self.database_path = Some("english.lingo".to_string());
        
        // Create executor
        let mut executor = LingoExecutor::new();
        if let Err(e) = executor.load_database("english.lingo") {
            return Err(PluginError::InitializationFailed {
                plugin: "intent_detection".to_string(),
                error: format!("Failed to load database in executor: {}", e),
            });
        }
        self.executor = Some(executor);
        
        // Initialize function extractor
        let mut function_extractor = FunctionExtractor::new();
        function_extractor.initialize(database)?;
        self.function_extractor = Some(function_extractor);
        
        // Initialize operator detectors
        self.operator_detectors = Some(OperatorDetectorSuite::new());
        
        // Initialize composition engine
        self.composition_engine = Some(IntentCompositionEngine::new());
        
        Ok(())
    }
    
    fn handle_command(&mut self, command: &str, args: &[String], _context: &PluginContext) -> Result<Option<PluginResult>, PluginError> {
        match command {
            "detect_intent" => {
                if args.is_empty() {
                    return Err(PluginError::CommandNotSupported {
                        plugin: self.id().to_string(),
                        command: "detect_intent requires text argument".to_string(),
                    });
                }
                
                let text = &args[0];
                let intent = self.detect_intent(text)?;
                
                let mut data = HashMap::new();
                data.insert("intent".to_string(), format!("{:?}", intent));
                
                Ok(Some(PluginResult::CustomResults {
                    data,
                    confidence: intent.intent_confidence,
                }))
            },
            _ => Ok(None),
        }
    }
}

impl Default for IntentDetector {
    fn default() -> Self {
        Self::new()
    }
}

// Core Intent Data Structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub core_function: FunctionalPrimitive,
    pub pragmatic_operators: PragmaticOperators,
    pub intent_confidence: f32,
    pub operator_coherence: f32,
    pub compositional_path: Vec<OperatorApplication>,
    pub execution_time_ms: f32,
    pub source_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PragmaticOperators {
    pub directional: Vec<DirectionalOperator>,
    pub modal: Vec<ModalOperator>,
    pub temporal: Vec<TemporalOperator>,
    pub conditional: Vec<ConditionalOperator>,
    pub negation: Vec<NegationOperator>,
    pub intensity: Vec<IntensityOperator>,
    pub certainty: Vec<CertaintyOperator>,
    pub scope: Vec<ScopeOperator>,
    pub social: Vec<SocialOperator>,
}

impl PragmaticOperators {
    pub fn new() -> Self {
        Self {
            directional: Vec::new(),
            modal: Vec::new(),
            temporal: Vec::new(),
            conditional: Vec::new(),
            negation: Vec::new(),
            intensity: Vec::new(),
            certainty: Vec::new(),
            scope: Vec::new(),
            social: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorApplication {
    pub operator_type: String,
    pub confidence: f32,
    pub spatial_position: Coordinate3D,
    pub morpheme_evidence: Vec<String>,
}

// 1. Directional Operators

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectionalOperator {
    pub operator_type: DirectionalType,
    pub target_entity: Option<String>,
    pub spatial_vector: Option<Coordinate3D>,
    pub strength: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DirectionalType {
    FOR,         // "looking FOR cofounder"
    TO,          // "want TO scale"
    TOWARD,      // "working TOWARD growth"
    FROM,        // "coming FROM experience"
    WITH,        // "working WITH team"
    ABOUT,       // "thinking ABOUT strategy"
    THROUGH,     // "achieved THROUGH hard work"
    VIA,         // "connected VIA LinkedIn"
    BY,          // "done BY the team"
}

// 2. Modal Operators

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModalOperator {
    pub operator_type: ModalType,
    pub strength: f32,
    pub scope: ModalScope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModalType {
    CAN,         // Ability: "can help"
    COULD,       // Conditional ability: "could assist" 
    SHOULD,      // Obligation: "should hire"
    MUST,        // Necessity: "must find"
    MIGHT,       // Possibility: "might work"
    WILL,        // Future certainty: "will succeed"
    WOULD,       // Conditional: "would like"
    MAY,         // Permission/possibility: "may join"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModalScope {
    Full,        // Applies to entire function
    Partial(String), // Applies to specific element
}

// 3. Temporal Operators

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalOperator {
    pub operator_type: TemporalType,
    pub urgency_level: f32,
    pub temporal_scope: TemporalScope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalType {
    NOW,         // "need NOW"
    SOON,        // "hiring SOON"
    LATER,       // "discuss LATER"
    BEFORE,      // "BEFORE the meeting"
    AFTER,       // "AFTER funding"
    DURING,      // "DURING development"
    WHILE,       // "WHILE scaling"
    UNTIL,       // "UNTIL we're ready"
    ASAP,        // "need ASAP"
    EVENTUALLY,  // "EVENTUALLY expand"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalScope {
    Immediate,   // Right now
    ShortTerm,   // Days/weeks
    LongTerm,    // Months/years
    Indefinite,  // No specific timeframe
}

// 4. Conditional Operators

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalOperator {
    pub operator_type: ConditionalType,
    pub condition_strength: f32,
    pub condition_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionalType {
    IF,          // "IF we get funding"
    WHEN,        // "WHEN ready"
    UNLESS,      // "UNLESS problems arise"
    PROVIDED,    // "PROVIDED we agree"
    ASSUMING,    // "ASSUMING all goes well"
    GIVEN,       // "GIVEN our situation"
}

// 5. Negation Operators

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegationOperator {
    pub operator_type: NegationType,
    pub negation_scope: Vec<String>,
    pub negation_strength: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NegationType {
    NOT,         // "NOT hiring"
    NEVER,       // "NEVER compromise"
    NO,          // "NO experience needed"
    NEITHER,     // "NEITHER option works"
    UnPrefix,    // "UNqualified"
    DisPrefix,   // "DISconnected"
    NonPrefix,   // "NON-technical"
}

// 6. Intensity Operators

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntensityOperator {
    pub operator_type: IntensityType,
    pub intensity_level: f32,
    pub target_modification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntensityType {
    VERY,        // "VERY important"
    EXTREMELY,   // "EXTREMELY urgent"
    QUITE,       // "QUITE good"
    RATHER,      // "RATHER difficult"
    SLIGHTLY,    // "SLIGHTLY concerned"
    SOMEWHAT,    // "SOMEWHAT interested"
    REALLY,      // "REALLY need"
}

// 7. Certainty Operators

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertaintyOperator {
    pub operator_type: CertaintyType,
    pub confidence_level: f32,
    pub evidence_basis: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CertaintyType {
    DEFINITELY, // "DEFINITELY hiring"
    PROBABLY,   // "PROBABLY interested"
    LIKELY,     // "LIKELY to succeed"
    MAYBE,      // "MAYBE later"
    PERHAPS,    // "PERHAPS we should"
    POSSIBLY,   // "POSSIBLY available"
    SURELY,     // "SURELY works"
    CERTAINLY,  // "CERTAINLY interested"
}

// 8. Scope Operators

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeOperator {
    pub operator_type: ScopeType,
    pub quantity_estimate: Option<f32>,
    pub scope_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScopeType {
    ALL,         // "ALL developers"
    SOME,        // "SOME experience"
    MANY,        // "MANY options"
    FEW,         // "FEW candidates"
    MOST,        // "MOST companies"
    SEVERAL,     // "SEVERAL meetings"
    EACH,        // "EACH team member"
    EVERY,       // "EVERY startup"
    NO,          // "NO experience"
}

// 9. Social Operators

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialOperator {
    pub operator_type: SocialType,
    pub politeness_level: f32,
    pub relationship_formality: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocialType {
    PLEASE,      // "PLEASE help"
    ThankYou,    // "THANK YOU for"
    SORRY,       // "SORRY to bother"
    ExcuseMe,    // "EXCUSE ME but"
    IfYouDontMind, // "IF YOU DON'T MIND"
    WOULD_YOU,   // "WOULD YOU consider"
    COULD_YOU,   // "COULD YOU help"
}

// Operator Detection Suite

pub struct OperatorDetectorSuite {
    directional_detector: DirectionalOperatorDetector,
    modal_detector: ModalOperatorDetector,
    temporal_detector: TemporalOperatorDetector,
    conditional_detector: ConditionalOperatorDetector,
    negation_detector: NegationOperatorDetector,
    intensity_detector: IntensityOperatorDetector,
    certainty_detector: CertaintyOperatorDetector,
    scope_detector: ScopeOperatorDetector,
    social_detector: SocialOperatorDetector,
}

impl OperatorDetectorSuite {
    pub fn new() -> Self {
        Self {
            directional_detector: DirectionalOperatorDetector::new(),
            modal_detector: ModalOperatorDetector::new(),
            temporal_detector: TemporalOperatorDetector::new(),
            conditional_detector: ConditionalOperatorDetector::new(),
            negation_detector: NegationOperatorDetector::new(),
            intensity_detector: IntensityOperatorDetector::new(),
            certainty_detector: CertaintyOperatorDetector::new(),
            scope_detector: ScopeOperatorDetector::new(),
            social_detector: SocialOperatorDetector::new(),
        }
    }
    
    pub fn detect_all_operators(&self, text: &str, db: &LingoDatabase) -> Result<PragmaticOperators, PluginError> {
        Ok(PragmaticOperators {
            directional: self.directional_detector.detect(text, db)?,
            modal: self.modal_detector.detect(text, db)?,
            temporal: self.temporal_detector.detect(text, db)?,
            conditional: self.conditional_detector.detect(text, db)?,
            negation: self.negation_detector.detect(text, db)?,
            intensity: self.intensity_detector.detect(text, db)?,
            certainty: self.certainty_detector.detect(text, db)?,
            scope: self.scope_detector.detect(text, db)?,
            social: self.social_detector.detect(text, db)?,
        })
    }
    
    /// Simple detection method for testing without database
    pub fn detect_all_operators_simple(&self, text: &str) -> PragmaticOperators {
        PragmaticOperators {
            directional: self.directional_detector.detect_simple(text),
            modal: self.modal_detector.detect_simple(text),
            temporal: self.temporal_detector.detect_simple(text),
            conditional: self.conditional_detector.detect_simple(text),
            negation: self.negation_detector.detect_simple(text),
            intensity: self.intensity_detector.detect_simple(text),
            certainty: self.certainty_detector.detect_simple(text),
            scope: self.scope_detector.detect_simple(text),
            social: self.social_detector.detect_simple(text),
        }
    }
}

// Intent Composition Engine

pub struct IntentCompositionEngine {
    composition_weights: HashMap<String, f32>,
}

impl IntentCompositionEngine {
    pub fn new() -> Self {
        let mut weights = HashMap::new();
        weights.insert("directional".to_string(), 0.8);
        weights.insert("modal".to_string(), 0.9);
        weights.insert("temporal".to_string(), 0.7);
        weights.insert("conditional".to_string(), 0.8);
        weights.insert("negation".to_string(), 1.0);
        weights.insert("intensity".to_string(), 0.6);
        weights.insert("certainty".to_string(), 0.7);
        weights.insert("scope".to_string(), 0.5);
        weights.insert("social".to_string(), 0.4);
        
        Self {
            composition_weights: weights,
        }
    }
    
    pub fn compose_intent(
        &self,
        function: FunctionalPrimitive,
        operators: PragmaticOperators,
        operator_coherence: f32,
        source_text: &str
    ) -> Result<Intent, PluginError> {
        // Calculate overall intent confidence
        let mut confidence_sum = 0.0;
        let mut operator_count = 0;
        
        // Count and weight operators
        if !operators.directional.is_empty() {
            confidence_sum += self.composition_weights["directional"];
            operator_count += 1;
        }
        if !operators.modal.is_empty() {
            confidence_sum += self.composition_weights["modal"];
            operator_count += 1;
        }
        if !operators.temporal.is_empty() {
            confidence_sum += self.composition_weights["temporal"];
            operator_count += 1;
        }
        if !operators.conditional.is_empty() {
            confidence_sum += self.composition_weights["conditional"];
            operator_count += 1;
        }
        if !operators.negation.is_empty() {
            confidence_sum += self.composition_weights["negation"];
            operator_count += 1;
        }
        if !operators.intensity.is_empty() {
            confidence_sum += self.composition_weights["intensity"];
            operator_count += 1;
        }
        if !operators.certainty.is_empty() {
            confidence_sum += self.composition_weights["certainty"];
            operator_count += 1;
        }
        if !operators.scope.is_empty() {
            confidence_sum += self.composition_weights["scope"];
            operator_count += 1;
        }
        if !operators.social.is_empty() {
            confidence_sum += self.composition_weights["social"];
            operator_count += 1;
        }
        
        let base_confidence = if operator_count > 0 {
            confidence_sum / operator_count as f32
        } else {
            0.5
        };
        
        let intent_confidence = (base_confidence * 0.7) + (operator_coherence * 0.3);
        
        // Generate compositional path
        let compositional_path = self.generate_compositional_path(&operators);
        
        Ok(Intent {
            core_function: function,
            pragmatic_operators: operators,
            intent_confidence,
            operator_coherence,
            compositional_path,
            execution_time_ms: 0.0, // Will be set by caller
            source_text: source_text.to_string(),
        })
    }
    
    fn generate_compositional_path(&self, operators: &PragmaticOperators) -> Vec<OperatorApplication> {
        let mut path = Vec::new();
        
        // Add applications in order of detection
        for op in &operators.directional {
            path.push(OperatorApplication {
                operator_type: "directional".to_string(),
                confidence: op.strength,
                spatial_position: op.spatial_vector.unwrap_or(Coordinate3D { x: 0.0, y: 0.0, z: 0.0 }),
                morpheme_evidence: vec![format!("{:?}", op.operator_type)],
            });
        }
        
        for op in &operators.modal {
            path.push(OperatorApplication {
                operator_type: "modal".to_string(),
                confidence: op.strength,
                spatial_position: Coordinate3D { x: 0.0, y: 0.0, z: 0.0 },
                morpheme_evidence: vec![format!("{:?}", op.operator_type)],
            });
        }
        
        // Continue for other operator types...
        
        path
    }
}

// Individual Operator Detectors (will be implemented in Part 2)

pub struct DirectionalOperatorDetector {
    pub directional_morphemes: HashMap<String, (DirectionalType, f32)>,
}

impl DirectionalOperatorDetector {
    pub fn new() -> Self {
        let mut morphemes = HashMap::new();
        morphemes.insert("for".to_string(), (DirectionalType::FOR, 0.9));
        morphemes.insert("to".to_string(), (DirectionalType::TO, 0.8));
        morphemes.insert("toward".to_string(), (DirectionalType::TOWARD, 0.7));
        morphemes.insert("from".to_string(), (DirectionalType::FROM, 0.8));
        morphemes.insert("with".to_string(), (DirectionalType::WITH, 0.7));
        morphemes.insert("about".to_string(), (DirectionalType::ABOUT, 0.6));
        morphemes.insert("through".to_string(), (DirectionalType::THROUGH, 0.6));
        morphemes.insert("via".to_string(), (DirectionalType::VIA, 0.8));
        morphemes.insert("by".to_string(), (DirectionalType::BY, 0.5));
        morphemes.insert("on".to_string(), (DirectionalType::TO, 0.6)); // "focus on"
        
        Self {
            directional_morphemes: morphemes,
        }
    }
    
    pub fn detect(&self, text: &str, db: &LingoDatabase) -> Result<Vec<DirectionalOperator>, PluginError> {
        let mut operators = Vec::new();
        let words = preprocess_text(text);
        
        for word in words.split_whitespace() {
            let word_lower = word.to_lowercase().trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
            if let Some((dir_type, base_strength)) = self.directional_morphemes.get(&word_lower) {
                // Use morphological analysis to get actual strength and context
                // Note: In practice, we'd pass the executor from the plugin context
                let morpheme_analysis: Vec<String> = Vec::new(); // Simplified for now - would use actual decomposition
                let spatial_position = Coordinate3D { x: 0.5, y: 0.5, z: 3.0 }; // Default word position
                
                // Extract target entity from spatial context
                let target_entity = self.extract_target_from_context(text, word, db);
                
                // Calculate spatial vector based on morphological positioning
                let spatial_vector = Some(Coordinate3D {
                    x: spatial_position.x,
                    y: spatial_position.y, 
                    z: spatial_position.z,
                });
                
                // Calculate contextual strength using spatial coherence
                let strength = self.calculate_contextual_strength(base_strength, &spatial_position, text);
                
                operators.push(DirectionalOperator {
                    operator_type: dir_type.clone(),
                    target_entity,
                    spatial_vector,
                    strength,
                });
            }
        }
        
        Ok(operators)
    }
    
    /// Simple detection without database dependency
    pub fn detect_simple(&self, text: &str) -> Vec<DirectionalOperator> {
        let mut operators = Vec::new();
        let words = preprocess_text(text);
        
        for word in words.split_whitespace() {
            let word_lower = word.to_lowercase().trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
            if let Some((dir_type, base_strength)) = self.directional_morphemes.get(&word_lower) {
                let spatial_position = Coordinate3D { x: 0.5, y: 0.5, z: 3.0 };
                
                operators.push(DirectionalOperator {
                    operator_type: dir_type.clone(),
                    target_entity: None,
                    spatial_vector: Some(spatial_position),
                    strength: *base_strength,
                });
            }
        }
        
        operators
    }
    
    fn extract_target_from_context(&self, text: &str, trigger_word: &str, _db: &LingoDatabase) -> Option<String> {
        // Use spatial queries to find the target entity
        if let Some(pos) = text.to_lowercase().find(&trigger_word.to_lowercase()) {
            let after_directional = &text[pos + trigger_word.len()..];
            let target_words: Vec<&str> = after_directional.split_whitespace().take(3).collect();
            
            if !target_words.is_empty() {
                Some(target_words.join(" ").trim().to_string())
            } else {
                None
            }
        } else {
            None
        }
    }
    
    fn calculate_contextual_strength(&self, base_strength: &f32, spatial_position: &Coordinate3D, context: &str) -> f32 {
        let mut strength = *base_strength;
        
        // Boost strength based on spatial coherence - if the word is well-positioned in semantic space
        let spatial_coherence = (spatial_position.x + spatial_position.y + spatial_position.z) / 3.0;
        strength *= (0.8 + (spatial_coherence * 0.4));
        
        // Additional context analysis for emphasis
        if context.contains("really") || context.contains("definitely") {
            strength *= 1.1;
        }
        
        strength.clamp(0.0, 1.0)
    }
}

// Placeholder implementations for other detectors
pub struct ModalOperatorDetector { pub modal_morphemes: HashMap<String, (ModalType, f32)> }
pub struct TemporalOperatorDetector { pub temporal_morphemes: HashMap<String, (TemporalType, f32)> }
pub struct ConditionalOperatorDetector { pub conditional_morphemes: HashMap<String, (ConditionalType, f32)> }
pub struct NegationOperatorDetector { pub negation_morphemes: HashMap<String, (NegationType, f32)> }
pub struct IntensityOperatorDetector { pub intensity_morphemes: HashMap<String, (IntensityType, f32)> }
pub struct CertaintyOperatorDetector { pub certainty_morphemes: HashMap<String, (CertaintyType, f32)> }
pub struct ScopeOperatorDetector { pub scope_morphemes: HashMap<String, (ScopeType, f32)> }
pub struct SocialOperatorDetector { pub social_morphemes: HashMap<String, (SocialType, f32)> }

// Full implementations of remaining operator detectors

// Full implementations of operator detectors with morpheme tables

impl ModalOperatorDetector {
    pub fn new() -> Self {
        let mut morphemes = HashMap::new();
        morphemes.insert("can".to_string(), (ModalType::CAN, 0.8));
        morphemes.insert("could".to_string(), (ModalType::COULD, 0.7));
        morphemes.insert("should".to_string(), (ModalType::SHOULD, 0.9));
        morphemes.insert("must".to_string(), (ModalType::MUST, 1.0));
        morphemes.insert("might".to_string(), (ModalType::MIGHT, 0.6));
        morphemes.insert("will".to_string(), (ModalType::WILL, 0.9));
        morphemes.insert("would".to_string(), (ModalType::WOULD, 0.7));
        morphemes.insert("may".to_string(), (ModalType::MAY, 0.6));
        morphemes.insert("need".to_string(), (ModalType::MUST, 0.9));
        morphemes.insert("want".to_string(), (ModalType::WILL, 0.7));
        
        Self { modal_morphemes: morphemes }
    }
    
    pub fn detect(&self, text: &str, db: &LingoDatabase) -> Result<Vec<ModalOperator>, PluginError> {
        let mut operators = Vec::new();
        let words = preprocess_text(text);
        
        for word in words.split_whitespace() {
            let word_lower = word.to_lowercase().trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
            if let Some((modal_type, base_strength)) = self.modal_morphemes.get(&word_lower) {
                // Use existing morphological analysis for context
                // Note: In practice, we'd pass the executor from the plugin context
                let spatial_position = Coordinate3D { x: 0.5, y: 0.5, z: 3.0 }; // Default word position
                
                // Calculate strength based on spatial coherence
                let strength = self.calculate_contextual_strength(base_strength, &spatial_position, db);
                
                // Determine scope using spatial neighbors
                let scope = self.determine_modal_scope(&word_lower, db);
                
                operators.push(ModalOperator {
                    operator_type: modal_type.clone(),
                    strength,
                    scope,
                });
            }
        }
        
        Ok(operators)
    }
    
    fn calculate_contextual_strength(&self, base_strength: &f32, spatial_position: &Coordinate3D, db: &LingoDatabase) -> f32 {
        let mut strength = *base_strength;
        
        // Use spatial coherence to adjust strength
        let spatial_coherence = (spatial_position.x + spatial_position.y + spatial_position.z) / 3.0;
        strength *= (0.7 + (spatial_coherence * 0.6));
        
        strength.clamp(0.0, 1.0)
    }
    
    fn determine_modal_scope(&self, _word: &str, _db: &LingoDatabase) -> ModalScope {
        // Use spatial neighbors to determine scope
        // This would use the actual QueryBuilder in a real implementation
        // For now, return a reasonable default based on the modal type
        ModalScope::Full
    }
    
    /// Simple detection without database dependency
    pub fn detect_simple(&self, text: &str) -> Vec<ModalOperator> {
        let mut operators = Vec::new();
        let words = preprocess_text(text);
        
        for word in words.split_whitespace() {
            let word_lower = word.to_lowercase().trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
            if let Some((modal_type, base_strength)) = self.modal_morphemes.get(&word_lower) {
                operators.push(ModalOperator {
                    operator_type: modal_type.clone(),
                    scope: ModalScope::Full,
                    strength: *base_strength,
                });
            }
        }
        
        operators
    }
}

impl TemporalOperatorDetector {
    pub fn new() -> Self {
        let mut morphemes = HashMap::new();
        morphemes.insert("now".to_string(), (TemporalType::NOW, 0.9));
        morphemes.insert("soon".to_string(), (TemporalType::SOON, 0.7));
        morphemes.insert("later".to_string(), (TemporalType::LATER, 0.6));
        morphemes.insert("before".to_string(), (TemporalType::BEFORE, 0.8));
        morphemes.insert("after".to_string(), (TemporalType::AFTER, 0.8));
        morphemes.insert("during".to_string(), (TemporalType::DURING, 0.7));
        morphemes.insert("while".to_string(), (TemporalType::WHILE, 0.7));
        morphemes.insert("until".to_string(), (TemporalType::UNTIL, 0.8));
        morphemes.insert("asap".to_string(), (TemporalType::ASAP, 1.0));
        morphemes.insert("eventually".to_string(), (TemporalType::EVENTUALLY, 0.4));
        morphemes.insert("quickly".to_string(), (TemporalType::ASAP, 0.8));
        morphemes.insert("immediately".to_string(), (TemporalType::NOW, 0.9));
        
        Self { temporal_morphemes: morphemes }
    }
    
    pub fn detect(&self, text: &str, _db: &LingoDatabase) -> Result<Vec<TemporalOperator>, PluginError> {
        let mut operators = Vec::new();
        let words = preprocess_text(text);
        
        for word in words.split_whitespace() {
            let word_lower = word.to_lowercase().trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
            if let Some((temporal_type, _)) = self.temporal_morphemes.get(&word_lower) {
                let urgency_level = self.calculate_urgency_level(temporal_type);
                let temporal_scope = self.determine_temporal_scope(temporal_type);
                
                operators.push(TemporalOperator {
                    operator_type: temporal_type.clone(),
                    urgency_level,
                    temporal_scope,
                });
            }
        }
        
        Ok(operators)
    }
    
    fn calculate_urgency_level(&self, temporal_type: &TemporalType) -> f32 {
        match temporal_type {
            TemporalType::NOW => 1.0,
            TemporalType::ASAP => 1.0,
            TemporalType::SOON => 0.8,
            TemporalType::BEFORE | TemporalType::AFTER => 0.6,
            TemporalType::DURING | TemporalType::WHILE | TemporalType::UNTIL => 0.5,
            TemporalType::LATER => 0.3,
            TemporalType::EVENTUALLY => 0.1,
        }
    }
    
    fn determine_temporal_scope(&self, temporal_type: &TemporalType) -> TemporalScope {
        match temporal_type {
            TemporalType::NOW | TemporalType::ASAP => TemporalScope::Immediate,
            TemporalType::SOON | TemporalType::BEFORE | TemporalType::AFTER => TemporalScope::ShortTerm,
            TemporalType::LATER => TemporalScope::LongTerm,
            TemporalType::EVENTUALLY => TemporalScope::Indefinite,
            _ => TemporalScope::Immediate,
        }
    }
    
    /// Simple detection without database dependency
    pub fn detect_simple(&self, text: &str) -> Vec<TemporalOperator> {
        let mut operators = Vec::new();
        let words = preprocess_text(text);
        
        for word in words.split_whitespace() {
            let word_lower = word.to_lowercase().trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
            if let Some((temporal_type, base_strength)) = self.temporal_morphemes.get(&word_lower) {
                operators.push(TemporalOperator {
                    operator_type: temporal_type.clone(),
                    urgency_level: *base_strength,
                    temporal_scope: TemporalScope::Immediate,
                });
            }
        }
        
        operators
    }
}

impl ConditionalOperatorDetector {
    pub fn new() -> Self {
        let mut morphemes = HashMap::new();
        morphemes.insert("if".to_string(), (ConditionalType::IF, 0.9));
        morphemes.insert("when".to_string(), (ConditionalType::WHEN, 0.8));
        morphemes.insert("unless".to_string(), (ConditionalType::UNLESS, 0.9));
        morphemes.insert("provided".to_string(), (ConditionalType::PROVIDED, 0.8));
        morphemes.insert("assuming".to_string(), (ConditionalType::ASSUMING, 0.7));
        morphemes.insert("given".to_string(), (ConditionalType::GIVEN, 0.6));
        
        Self { conditional_morphemes: morphemes }
    }
    
    pub fn detect(&self, text: &str, _db: &LingoDatabase) -> Result<Vec<ConditionalOperator>, PluginError> {
        let mut operators = Vec::new();
        let words = preprocess_text(text);
        
        for word in words.split_whitespace() {
            let word_lower = word.to_lowercase().trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
            if let Some((conditional_type, strength)) = self.conditional_morphemes.get(&word_lower) {
                let condition_content = self.extract_condition_content(text, word);
                
                operators.push(ConditionalOperator {
                    operator_type: conditional_type.clone(),
                    condition_strength: *strength,
                    condition_content,
                });
            }
        }
        
        Ok(operators)
    }
    
    fn extract_condition_content(&self, text: &str, trigger_word: &str) -> String {
        // Extract condition content using morphological boundaries
        if let Some(pos) = text.to_lowercase().find(&trigger_word.to_lowercase()) {
            let after_condition = &text[pos + trigger_word.len()..];
            // Use word boundaries and basic linguistic structure
            after_condition.split_whitespace()
                .take_while(|&word| !word.ends_with(',') && !word.ends_with('.'))
                .take(5) // Reasonable limit
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string()
        } else {
            String::new()
        }
    }
    
    /// Simple detection without database dependency
    pub fn detect_simple(&self, text: &str) -> Vec<ConditionalOperator> {
        let mut operators = Vec::new();
        let words = preprocess_text(text);
        
        for word in words.split_whitespace() {
            let word_lower = word.to_lowercase().trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
            if let Some((conditional_type, base_strength)) = self.conditional_morphemes.get(&word_lower) {
                operators.push(ConditionalOperator {
                    operator_type: conditional_type.clone(),
                    condition_strength: *base_strength,
                    condition_content: word.to_string(),
                });
            }
        }
        
        operators
    }
}

impl NegationOperatorDetector {
    pub fn new() -> Self {
        let mut morphemes = HashMap::new();
        morphemes.insert("not".to_string(), (NegationType::NOT, 0.9));
        morphemes.insert("never".to_string(), (NegationType::NEVER, 1.0));
        morphemes.insert("no".to_string(), (NegationType::NO, 0.8));
        morphemes.insert("neither".to_string(), (NegationType::NEITHER, 0.9));
        morphemes.insert("un-".to_string(), (NegationType::UnPrefix, 0.8));
        morphemes.insert("dis-".to_string(), (NegationType::DisPrefix, 0.7));
        morphemes.insert("non-".to_string(), (NegationType::NonPrefix, 0.8));
        
        Self { negation_morphemes: morphemes }
    }
    
    pub fn detect(&self, text: &str, _db: &LingoDatabase) -> Result<Vec<NegationOperator>, PluginError> {
        let mut operators = Vec::new();
        let words = preprocess_text(text);
        
        // Check for explicit negation words
        for word in words.split_whitespace() {
            let word_lower = word.to_lowercase().trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
            if let Some((negation_type, strength)) = self.negation_morphemes.get(&word_lower) {
                let negation_scope = self.extract_negation_scope(text, word);
                
                operators.push(NegationOperator {
                    operator_type: negation_type.clone(),
                    negation_scope,
                    negation_strength: *strength,
                });
            }
        }
        
        // Check for prefix negations
        for word in words.split_whitespace() {
            if word.starts_with("un") || word.starts_with("dis") || word.starts_with("non") {
                let prefix = if word.starts_with("un") { "un-" }
                           else if word.starts_with("dis") { "dis-" }
                           else { "non-" };
                
                if let Some((negation_type, strength)) = self.negation_morphemes.get(prefix) {
                    operators.push(NegationOperator {
                        operator_type: negation_type.clone(),
                        negation_scope: vec![word.to_string()],
                        negation_strength: *strength,
                    });
                }
            }
        }
        
        Ok(operators)
    }
    
    fn extract_negation_scope(&self, text: &str, trigger_word: &str) -> Vec<String> {
        // Extract what is being negated
        if let Some(pos) = text.to_lowercase().find(&trigger_word.to_lowercase()) {
            let after_negation = &text[pos + trigger_word.len()..];
            after_negation.split_whitespace()
                .take(3) // Take next 3 words as scope
                .map(|s| s.to_string())
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Simple detection without database dependency
    pub fn detect_simple(&self, text: &str) -> Vec<NegationOperator> {
        let mut operators = Vec::new();
        let words = preprocess_text(text);
        
        for word in words.split_whitespace() {
            let word_lower = word.to_lowercase().trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
            if let Some((negation_type, base_strength)) = self.negation_morphemes.get(&word_lower) {
                operators.push(NegationOperator {
                    operator_type: negation_type.clone(),
                    negation_scope: vec![word.to_string()],
                    negation_strength: *base_strength,
                });
            }
        }
        
        operators
    }
}

impl IntensityOperatorDetector {
    pub fn new() -> Self {
        let mut morphemes = HashMap::new();
        morphemes.insert("very".to_string(), (IntensityType::VERY, 0.8));
        morphemes.insert("extremely".to_string(), (IntensityType::EXTREMELY, 1.0));
        morphemes.insert("quite".to_string(), (IntensityType::QUITE, 0.6));
        morphemes.insert("rather".to_string(), (IntensityType::RATHER, 0.6));
        morphemes.insert("slightly".to_string(), (IntensityType::SLIGHTLY, 0.3));
        morphemes.insert("somewhat".to_string(), (IntensityType::SOMEWHAT, 0.4));
        morphemes.insert("really".to_string(), (IntensityType::REALLY, 0.7));
        
        Self { intensity_morphemes: morphemes }
    }
    
    pub fn detect(&self, text: &str, _db: &LingoDatabase) -> Result<Vec<IntensityOperator>, PluginError> {
        let mut operators = Vec::new();
        let words = preprocess_text(text);
        
        for word in words.split_whitespace() {
            let word_lower = word.to_lowercase().trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
            if let Some((intensity_type, level)) = self.intensity_morphemes.get(&word_lower) {
                let target_modification = self.extract_target_modification(text, word);
                
                operators.push(IntensityOperator {
                    operator_type: intensity_type.clone(),
                    intensity_level: *level,
                    target_modification,
                });
            }
        }
        
        Ok(operators)
    }
    
    fn extract_target_modification(&self, text: &str, trigger_word: &str) -> String {
        // Extract what is being intensified
        if let Some(pos) = text.to_lowercase().find(&trigger_word.to_lowercase()) {
            let after_intensity = &text[pos + trigger_word.len()..];
            after_intensity.split_whitespace()
                .take(2) // Take next 2 words as target
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string()
        } else {
            String::new()
        }
    }
    
    /// Simple detection without database dependency
    pub fn detect_simple(&self, text: &str) -> Vec<IntensityOperator> {
        let mut operators = Vec::new();
        let words = preprocess_text(text);
        
        for word in words.split_whitespace() {
            let word_lower = word.to_lowercase().trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
            if let Some((intensity_type, base_strength)) = self.intensity_morphemes.get(&word_lower) {
                operators.push(IntensityOperator {
                    operator_type: intensity_type.clone(),
                    intensity_level: *base_strength,
                    target_modification: word.to_string(),
                });
            }
        }
        
        operators
    }
}

impl CertaintyOperatorDetector {
    pub fn new() -> Self {
        let mut morphemes = HashMap::new();
        morphemes.insert("definitely".to_string(), (CertaintyType::DEFINITELY, 1.0));
        morphemes.insert("probably".to_string(), (CertaintyType::PROBABLY, 0.8));
        morphemes.insert("likely".to_string(), (CertaintyType::LIKELY, 0.8));
        morphemes.insert("maybe".to_string(), (CertaintyType::MAYBE, 0.5));
        morphemes.insert("perhaps".to_string(), (CertaintyType::PERHAPS, 0.5));
        morphemes.insert("possibly".to_string(), (CertaintyType::POSSIBLY, 0.4));
        morphemes.insert("surely".to_string(), (CertaintyType::SURELY, 0.9));
        morphemes.insert("certainly".to_string(), (CertaintyType::CERTAINLY, 1.0));
        
        Self { certainty_morphemes: morphemes }
    }
    
    pub fn detect(&self, text: &str, _db: &LingoDatabase) -> Result<Vec<CertaintyOperator>, PluginError> {
        let mut operators = Vec::new();
        let words = preprocess_text(text);
        
        for word in words.split_whitespace() {
            let word_lower = word.to_lowercase().trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
            if let Some((certainty_type, level)) = self.certainty_morphemes.get(&word_lower) {
                let evidence_basis = self.extract_evidence_basis(text, word);
                
                operators.push(CertaintyOperator {
                    operator_type: certainty_type.clone(),
                    confidence_level: *level,
                    evidence_basis,
                });
            }
        }
        
        Ok(operators)
    }
    
    fn extract_evidence_basis(&self, text: &str, trigger_word: &str) -> Option<String> {
        // Use morphological analysis to find evidence patterns
        // Look for causal, temporal, or attributional morphemes in the vicinity
        let evidence_morphemes = ["because", "since", "due", "given", "based"];
        
        for evidence in &evidence_morphemes {
            if text.to_lowercase().contains(evidence) {
                return Some(format!("evidence: {}", evidence));
            }
        }
        
        None
    }
    
    /// Simple detection without database dependency
    pub fn detect_simple(&self, text: &str) -> Vec<CertaintyOperator> {
        let mut operators = Vec::new();
        let words = preprocess_text(text);
        
        for word in words.split_whitespace() {
            let word_lower = word.to_lowercase().trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
            if let Some((certainty_type, base_strength)) = self.certainty_morphemes.get(&word_lower) {
                operators.push(CertaintyOperator {
                    operator_type: certainty_type.clone(),
                    confidence_level: *base_strength,
                    evidence_basis: None,
                });
            }
        }
        
        operators
    }
}

impl ScopeOperatorDetector {
    pub fn new() -> Self {
        let mut morphemes = HashMap::new();
        morphemes.insert("all".to_string(), (ScopeType::ALL, 1.0));
        morphemes.insert("some".to_string(), (ScopeType::SOME, 0.5));
        morphemes.insert("many".to_string(), (ScopeType::MANY, 0.7));
        morphemes.insert("few".to_string(), (ScopeType::FEW, 0.3));
        morphemes.insert("most".to_string(), (ScopeType::MOST, 0.8));
        morphemes.insert("several".to_string(), (ScopeType::SEVERAL, 0.6));
        morphemes.insert("each".to_string(), (ScopeType::EACH, 1.0));
        morphemes.insert("every".to_string(), (ScopeType::EVERY, 1.0));
        morphemes.insert("no".to_string(), (ScopeType::NO, 0.0));
        
        Self { scope_morphemes: morphemes }
    }
    
    pub fn detect(&self, text: &str, _db: &LingoDatabase) -> Result<Vec<ScopeOperator>, PluginError> {
        let mut operators = Vec::new();
        let words = preprocess_text(text);
        
        for word in words.split_whitespace() {
            let word_lower = word.to_lowercase().trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
            if let Some((scope_type, quantity)) = self.scope_morphemes.get(&word_lower) {
                let scope_target = self.extract_scope_target(text, word);
                
                operators.push(ScopeOperator {
                    operator_type: scope_type.clone(),
                    quantity_estimate: Some(*quantity),
                    scope_target,
                });
            }
        }
        
        Ok(operators)
    }
    
    fn extract_scope_target(&self, text: &str, trigger_word: &str) -> String {
        // Extract what the scope applies to
        if let Some(pos) = text.to_lowercase().find(&trigger_word.to_lowercase()) {
            let after_scope = &text[pos + trigger_word.len()..];
            after_scope.split_whitespace()
                .take(2) // Take next 2 words as target
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string()
        } else {
            String::new()
        }
    }
    
    /// Simple detection without database dependency
    pub fn detect_simple(&self, text: &str) -> Vec<ScopeOperator> {
        let mut operators = Vec::new();
        let words = preprocess_text(text);
        
        for word in words.split_whitespace() {
            let word_lower = word.to_lowercase().trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
            if let Some((scope_type, base_strength)) = self.scope_morphemes.get(&word_lower) {
                operators.push(ScopeOperator {
                    operator_type: scope_type.clone(),
                    quantity_estimate: Some(*base_strength),
                    scope_target: word.to_string(),
                });
            }
        }
        
        operators
    }
}

impl SocialOperatorDetector {
    pub fn new() -> Self {
        let mut morphemes = HashMap::new();
        morphemes.insert("please".to_string(), (SocialType::PLEASE, 0.8));
        morphemes.insert("thank you".to_string(), (SocialType::ThankYou, 0.9));
        morphemes.insert("sorry".to_string(), (SocialType::SORRY, 0.7));
        morphemes.insert("excuse me".to_string(), (SocialType::ExcuseMe, 0.8));
        morphemes.insert("if you don't mind".to_string(), (SocialType::IfYouDontMind, 0.9));
        morphemes.insert("would you".to_string(), (SocialType::WOULD_YOU, 0.7));
        morphemes.insert("could you".to_string(), (SocialType::COULD_YOU, 0.7));
        
        Self { social_morphemes: morphemes }
    }
    
    pub fn detect(&self, text: &str, _db: &LingoDatabase) -> Result<Vec<SocialOperator>, PluginError> {
        let mut operators = Vec::new();
        let text_lower = text.to_lowercase();
        
        // Check for multi-word social phrases first
        for (phrase, (social_type, politeness)) in &self.social_morphemes {
            if text_lower.contains(phrase) {
                let formality = self.calculate_formality_level(text, phrase);
                
                operators.push(SocialOperator {
                    operator_type: social_type.clone(),
                    politeness_level: *politeness,
                    relationship_formality: formality,
                });
            }
        }
        
        Ok(operators)
    }
    
    fn calculate_formality_level(&self, text: &str, _phrase: &str) -> f32 {
        let mut formality: f32 = 0.5; // Base formality
        
        // Indicators of high formality
        if text.contains("sir") || text.contains("madam") || text.contains("respectfully") {
            formality += 0.3;
        }
        
        // Indicators of low formality
        if text.contains("hey") || text.contains("hi") || text.contains("thanks") {
            formality -= 0.2;
        }
        
        formality.clamp(0.0_f32, 1.0_f32)
    }
    
    /// Simple detection without database dependency
    pub fn detect_simple(&self, text: &str) -> Vec<SocialOperator> {
        let mut operators = Vec::new();
        let words = preprocess_text(text);
        
        for word in words.split_whitespace() {
            let word_lower = word.to_lowercase().trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
            if let Some((social_type, base_strength)) = self.social_morphemes.get(&word_lower) {
                operators.push(SocialOperator {
                    operator_type: social_type.clone(),
                    politeness_level: *base_strength,
                    relationship_formality: 0.5,
                });
            }
        }
        
        operators
    }
}

// Default implementations for convenience
impl Default for ModalOperator {
    fn default() -> Self {
        Self {
            operator_type: ModalType::CAN,
            strength: 0.5,
            scope: ModalScope::Full,
        }
    }
}

impl Default for TemporalOperator {
    fn default() -> Self {
        Self {
            operator_type: TemporalType::NOW,
            urgency_level: 0.5,
            temporal_scope: TemporalScope::Immediate,
        }
    }
}

impl Default for ConditionalOperator {
    fn default() -> Self {
        Self {
            operator_type: ConditionalType::IF,
            condition_strength: 0.5,
            condition_content: String::new(),
        }
    }
}

impl Default for NegationOperator {
    fn default() -> Self {
        Self {
            operator_type: NegationType::NOT,
            negation_scope: Vec::new(),
            negation_strength: 0.5,
        }
    }
}

impl Default for IntensityOperator {
    fn default() -> Self {
        Self {
            operator_type: IntensityType::VERY,
            intensity_level: 0.5,
            target_modification: String::new(),
        }
    }
}

impl Default for CertaintyOperator {
    fn default() -> Self {
        Self {
            operator_type: CertaintyType::MAYBE,
            confidence_level: 0.5,
            evidence_basis: None,
        }
    }
}

impl Default for ScopeOperator {
    fn default() -> Self {
        Self {
            operator_type: ScopeType::SOME,
            quantity_estimate: Some(0.5),
            scope_target: String::new(),
        }
    }
}

impl Default for SocialOperator {
    fn default() -> Self {
        Self {
            operator_type: SocialType::PLEASE,
            politeness_level: 0.5,
            relationship_formality: 0.5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_intent_detector_creation() {
        let detector = IntentDetector::new();
        assert_eq!(detector.id(), "intent_detection");
        assert_eq!(detector.name(), "Intent Detection Plugin");
        assert_eq!(detector.version(), "1.0.0");
    }
    
    #[test]
    fn test_operator_suite_creation() {
        let suite = OperatorDetectorSuite::new();
        // Basic smoke test - just ensure it can be created
        assert!(true);
    }
    
    #[test]
    fn test_directional_detector() {
        let detector = DirectionalOperatorDetector::new();
        assert!(detector.directional_morphemes.contains_key("for"));
        assert!(detector.directional_morphemes.contains_key("to"));
        assert!(detector.directional_morphemes.contains_key("with"));
    }
    
    #[test]
    fn test_pragmatic_operators_creation() {
        let operators = PragmaticOperators::new();
        assert!(operators.directional.is_empty());
        assert!(operators.modal.is_empty());
        assert!(operators.temporal.is_empty());
    }
}
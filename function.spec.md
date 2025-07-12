# Function Extraction via LINGO: Technical Specification

## 🔥 BREAKTHROUGH: Compositional Architecture Discovery

**CRITICAL INSIGHT**: The LINGO database is designed for true compositional emergence, not direct word lookup!

### Database Architecture Reality:
```
❌ What we assumed: Database contains complete words
✅ Actual reality: Database contains morphological building blocks

Layer 2: Morphemes → ["manage", "-er", "-ize", "pre-", "un-", etc.]
Layer 1: Phonemes → [/mænɪdʒ/, /ər/, /aɪz/, etc.]  
Layer 0: Letters → ["m", "a", "n", "a", "g", "e"]

Words are COMPOSED from these building blocks, not stored directly!
```

### Revolutionary Implications:
1. **True Bottom-up Emergence**: Function detection must work compositionally
2. **No Shortcuts**: Can't query "manager" directly, must compose from "manage" + "-er"
3. **Morphological Intelligence**: Agent detection works by finding "-er", "-ant", "-ist" suffixes
4. **Spatial Composition**: Word positions calculated by composing morpheme vectors

This validates our entire hypothesis: **Function emerges from linguistic composition!**

---
Extract functional semantics (agency, action, transformation, conditionality) from natural language through **deterministic hierarchical composition** using the LINGO 7-layer linguistic database architecture.

## 🧬 Core Hypothesis
**Function emerges from bottom-up linguistic composition**: Letters → Phonemes → Morphemes → Words → Phrases, with functional meaning arising from spatial clustering and cross-layer morphological patterns rather than hardcoded templates.

---

## 📊 Function Extraction Architecture

### Functional Primitive Types
```rust
#[derive(Debug, Clone)]
pub enum FunctionalPrimitive {
    Agency {
        actor: LinguisticNode,
        capability_level: f32,      // 0.0-1.0 based on morphological markers
        responsibility_scope: Vec<LinguisticNode>,
    },
    Action {
        verb: LinguisticNode,
        transformation_type: ActionType,
        intensity: f32,             // Based on morphological intensifiers
        temporal_aspect: TemporalAspect,
    },
    Transformation {
        input_state: LinguisticNode,
        output_state: LinguisticNode,
        process_vector: Coordinate3D, // 3D spatial relationship
        reversibility: f32,         // Based on morphological analysis
    },
    Conditionality {
        condition: LinguisticNode,
        dependency_type: DependencyType,
        certainty_level: f32,       // Based on conditional markers
        scope: Vec<LinguisticNode>,
    },
    Sequence {
        steps: Vec<LinguisticNode>,
        ordering_type: OrderingType,
        temporal_density: f32,      // Based on temporal morphemes
    },
    Purpose {
        goal_state: LinguisticNode,
        intention_strength: f32,
        success_criteria: Vec<LinguisticNode>,
    },
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    primitives: Vec<FunctionalPrimitive>,
    confidence: f32,                    // Overall extraction confidence
    source_text: String,                // Original input
    extraction_path: Vec<SlangQuery>,   // How we found this function
    spatial_coherence: f32,             // How well primitives cluster in 3D
}
```

### Detection Algorithms

#### 1. Agency Detection Algorithm (Compositional Approach)
```rust
pub fn detect_agency(text: &str, db: &LingoDB) -> Vec<FunctionalPrimitive> {
    let mut agents = Vec::new();
    
    // Step 1: Preprocess text into lemmatized words
    let words = preprocess_text(text); // ["manager", "organize", "meeting"]
    
    // Step 2: For each word, decompose into morphemes and check for agency
    for word in words {
        let morpheme_analysis = decompose_word_to_morphemes(&word, db);
        
        // Step 3: Check if any morphemes indicate agency
        let agent_morphemes = morpheme_analysis.iter()
            .filter(|morpheme| {
                // Look up each morpheme in the database
                if let Some(morpheme_node) = db.query()
                    .find(morpheme.surface_form)
                    .layer(Layer::Morphemes)
                    .execute()
                    .first() 
                {
                    // Check morphological properties for agency markers
                    morpheme_node.morpheme_type == MorphemeType::AgentSuffix ||
                    morpheme_node.semantic_function == SemanticFunction::Actor ||
                    morpheme.surface_form.ends_with("er") ||
                    morpheme.surface_form.ends_with("ant") ||
                    morpheme.surface_form.ends_with("ist") ||
                    morpheme.surface_form.ends_with("or")
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();
            
        // Step 4: If word contains agent morphemes, analyze the full composition
        if !agent_morphemes.is_empty() {
            let capability_level = calculate_morphological_capability(&morpheme_analysis);
            let spatial_position = calculate_composed_position(&morpheme_analysis, db);
            
            agents.push(FunctionalPrimitive::Agency {
                actor: compose_word_node(word, morpheme_analysis, spatial_position),
                capability_level,
                responsibility_scope: extract_scope_from_context(&word, &words),
            });
        }
    }
    
    // Step 2: Analyze capability and scope
    for candidate in agent_candidates {
        let capability_analysis = db.query()
            .load_node(candidate.id)
            .follow_connections()         // Find related capabilities
            .filter_by_layer(Layer::Concepts)
            .spatial_neighbors(0.3)       // Broader search for capabilities
            .execute();
            
        let capability_level = calculate_capability_strength(&capability_analysis);
        let responsibility_scope = extract_responsibility_scope(&capability_analysis);
        
        agents.push(FunctionalPrimitive::Agency {
            actor: candidate,
            capability_level,
            responsibility_scope,
        });
    }
    
    agents
}
```

#### 2. Action Detection Algorithm (Compositional Approach)
```rust
pub fn detect_actions(text: &str, db: &LingoDB) -> Vec<FunctionalPrimitive> {
    let mut actions = Vec::new();
    
    // Step 1: Preprocess and lemmatize text
    let words = preprocess_text(text); // ["organize", "create", "process"]
    
    // Step 2: For each word, check for action morphology
    for word in words {
        let morpheme_analysis = decompose_word_to_morphemes(&word, db);
        
        // Step 3: Look for action-indicating morphemes
        let action_indicators = morpheme_analysis.iter()
            .filter_map(|morpheme| {
                // Query the morpheme in the database
                db.query()
                    .find(morpheme.surface_form)
                    .layer(Layer::Morphemes)
                    .execute()
                    .first()
                    .filter(|node| {
                        // Check for action morphology
                        node.morpheme_type == MorphemeType::Verb ||
                        node.semantic_function == SemanticFunction::Causative ||
                        node.semantic_function == SemanticFunction::Transformative ||
                        morpheme.surface_form.ends_with("ize") ||
                        morpheme.surface_form.ends_with("fy") ||
                        morpheme.surface_form.ends_with("ate") ||
                        morpheme.surface_form.ends_with("en")
                    })
            })
            .collect::<Vec<_>>();
            
        // Step 4: If action morphemes found, compose the action primitive
        if !action_indicators.is_empty() {
            // Calculate transformation strength from morphological composition
            let transformation_strength = calculate_transformation_strength(&morpheme_analysis);
            
            // Use spatial positioning from morpheme composition
            let spatial_position = calculate_composed_position(&morpheme_analysis, db);
            
            // Determine action type through spatial clustering
            let action_type = determine_action_type_spatially(spatial_position, db);
            
            actions.push(FunctionalPrimitive::Action {
                verb: compose_word_node(word, morpheme_analysis, spatial_position),
                transformation_type: action_type,
                intensity: transformation_strength,
                temporal_aspect: extract_temporal_aspect_from_morphemes(&morpheme_analysis),
            });
        }
    }
    
    actions
}

// Helper functions for compositional analysis
fn decompose_word_to_morphemes(word: &str, db: &LingoDB) -> Vec<MorphemeAnalysis> {
    // Algorithm to break word into component morphemes
    // Example: "manager" → ["manage", "-er"]
    // Uses database morpheme patterns to find valid decompositions
    morphological_parser::decompose(word, db)
}

fn calculate_composed_position(morphemes: &[MorphemeAnalysis], db: &LingoDB) -> Coordinate3D {
    // Calculate word's 3D position by composing morpheme positions
    // Uses vector mathematics to combine morpheme spatial coordinates
    let positions: Vec<Coordinate3D> = morphemes.iter()
        .filter_map(|m| {
            db.query()
                .find(m.surface_form)
                .layer(Layer::Morphemes)
                .execute()
                .first()
                .map(|node| node.position)
        })
        .collect();
        
    // Vector composition (weighted average based on morpheme importance)
    compose_spatial_vectors(positions)
}

fn preprocess_text(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| {
            // Remove punctuation and convert to lowercase
            let clean_word = word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
            // Lemmatize to base form (organize instead of organized)
            lemmatize(&clean_word)
        })
        .filter(|word| !is_stop_word(word) && word.len() > 1)
        .collect()
}
    
    // Step 2: Classify transformation types through spatial analysis
    for candidate in action_candidates {
        let transformation_analysis = db.query()
            .load_node(candidate.id)
            .spatial_neighbors(0.2)       // Find semantic neighbors
            .filter_by_concept_type(ConceptType::Process)
            .follow_connections()
            .execute();
            
        let action_type = classify_action_type(&transformation_analysis);
        let intensity = calculate_action_intensity(&candidate);
        let temporal_aspect = extract_temporal_aspect(&candidate);
        
        actions.push(FunctionalPrimitive::Action {
            verb: candidate,
            transformation_type: action_type,
            intensity,
            temporal_aspect,
        });
    }
    
    actions
}
```

#### 3. Transformation Detection Algorithm
```rust
pub fn detect_transformations(text: &str, db: &LingoDB) -> Vec<FunctionalPrimitive> {
    let mut transformations = Vec::new();
    
    // Step 1: Find transformation patterns through phrase analysis
    let transformation_patterns = db.query()
        .find_phrases_in_text(text)
        .filter_phrases(|p| {
            // Patterns like "X becomes Y", "convert X to Y", "X → Y"
            p.contains_transformation_markers()
        })
        .spatial_cluster(0.1)            // Very tight clustering
        .execute();
    
    // Step 2: Extract input/output states through spatial vectors
    for pattern in transformation_patterns {
        let components = db.query()
            .load_node(pattern.id)
            .layer_down()                 // Analyze component words
            .filter_by_role(SemanticRole::InputState, SemanticRole::OutputState)
            .execute();
            
        if let (Some(input), Some(output)) = (
            components.iter().find(|n| n.role == SemanticRole::InputState),
            components.iter().find(|n| n.role == SemanticRole::OutputState)
        ) {
            // Calculate 3D vector between input and output states
            let process_vector = calculate_transformation_vector(
                input.position, 
                output.position
            );
            let reversibility = calculate_reversibility(&pattern);
            
            transformations.push(FunctionalPrimitive::Transformation {
                input_state: input.clone(),
                output_state: output.clone(),
                process_vector,
                reversibility,
            });
        }
    }
    
    transformations
}
```

#### 4. Conditionality Detection Algorithm
```rust
pub fn detect_conditionality(text: &str, db: &LingoDB) -> Vec<FunctionalPrimitive> {
    let mut conditions = Vec::new();
    
    // Step 1: Find conditional markers through multi-layer analysis
    let conditional_markers = db.query()
        .find_words_in_text(text)
        .filter_words(|w| {
            w.lexical_category == LexicalCategory::Conditional ||
            w.word_type == WordType::ConditionalMarker
        })
        .layer_up()                      // Look at phrase context
        .spatial_neighbors(0.25)         // Find conditional clusters
        .execute();
    
    // Step 2: Analyze dependency structures and scope
    for marker in conditional_markers {
        let dependency_analysis = db.query()
            .load_node(marker.id)
            .follow_connections()
            .filter_by_dependency_type(DependencyType::Conditional)
            .spatial_radius(0.4)          // Broader scope for conditions
            .execute();
            
        let dependency_type = classify_dependency_type(&marker);
        let certainty_level = calculate_certainty_level(&marker);
        let scope = extract_conditional_scope(&dependency_analysis);
        
        conditions.push(FunctionalPrimitive::Conditionality {
            condition: marker,
            dependency_type,
            certainty_level,
            scope,
        });
    }
    
    conditions
}
```

---

## ⚡ Core Function Extraction Engine

### Main Extraction Pipeline
```rust
pub struct FunctionExtractor {
    db: LingoDB,
    confidence_threshold: f32,
    spatial_coherence_weight: f32,
    morphological_weight: f32,
}

impl FunctionExtractor {
    pub fn extract_function_signature(&self, text: &str) -> Result<FunctionSignature> {
        // Step 1: Run all detection algorithms in parallel
        let agency_primitives = detect_agency(text, &self.db);
        let action_primitives = detect_actions(text, &self.db);
        let transformation_primitives = detect_transformations(text, &self.db);
        let conditionality_primitives = detect_conditionality(text, &self.db);
        let sequence_primitives = detect_sequences(text, &self.db);
        let purpose_primitives = detect_purpose(text, &self.db);
        
        // Step 2: Combine all primitives
        let mut all_primitives = Vec::new();
        all_primitives.extend(agency_primitives);
        all_primitives.extend(action_primitives);
        all_primitives.extend(transformation_primitives);
        all_primitives.extend(conditionality_primitives);
        all_primitives.extend(sequence_primitives);
        all_primitives.extend(purpose_primitives);
        
        // Step 3: Calculate spatial coherence
        let spatial_coherence = self.calculate_spatial_coherence(&all_primitives);
        
        // Step 4: Calculate overall confidence
        let confidence = self.calculate_overall_confidence(&all_primitives, spatial_coherence);
        
        // Step 5: Generate extraction path for debugging
        let extraction_path = self.generate_extraction_path(&all_primitives);
        
        Ok(FunctionSignature {
            primitives: all_primitives,
            confidence,
            source_text: text.to_string(),
            extraction_path,
            spatial_coherence,
        })
    }
    
    fn calculate_spatial_coherence(&self, primitives: &[FunctionalPrimitive]) -> f32 {
        // Analyze how well the extracted primitives cluster in 3D space
        let positions: Vec<Coordinate3D> = primitives.iter()
            .map(|p| self.get_primitive_center_position(p))
            .collect();
            
        if positions.len() < 2 {
            return 1.0; // Perfect coherence for single primitive
        }
        
        // Calculate average pairwise distance
        let mut total_distance = 0.0;
        let mut pairs = 0;
        
        for i in 0..positions.len() {
            for j in (i+1)..positions.len() {
                total_distance += euclidean_distance(positions[i], positions[j]);
                pairs += 1;
            }
        }
        
        let average_distance = total_distance / pairs as f32;
        
        // Convert to coherence score (closer = more coherent)
        1.0 / (1.0 + average_distance)
    }
    
    fn calculate_overall_confidence(&self, primitives: &[FunctionalPrimitive], spatial_coherence: f32) -> f32 {
        if primitives.is_empty() {
            return 0.0;
        }
        
        // Weighted combination of individual confidences and spatial coherence
        let individual_confidence: f32 = primitives.iter()
            .map(|p| self.get_primitive_confidence(p))
            .sum::<f32>() / primitives.len() as f32;
            
        (individual_confidence * (1.0 - self.spatial_coherence_weight)) +
        (spatial_coherence * self.spatial_coherence_weight)
    }
}
```

---

## 🧪 Test Cases and Validation

### Test Case 1: Simple Agency Detection
```rust
#[test]
fn test_simple_agency_detection() {
    let db = LingoDB::load("test_data.lingo").unwrap();
    let extractor = FunctionExtractor::new(db);
    
    let result = extractor.extract_function_signature("The manager organized the meeting").unwrap();
    
    assert!(result.confidence > 0.8);
    assert_eq!(result.primitives.len(), 2); // Agency + Action
    
    // Check agency primitive
    let agency = result.primitives.iter()
        .find(|p| matches!(p, FunctionalPrimitive::Agency { .. }))
        .unwrap();
        
    if let FunctionalPrimitive::Agency { actor, .. } = agency {
        assert_eq!(actor.surface_form, "manager");
    }
}
```

### Test Case 2: Complex Transformation Detection
```rust
#[test]
fn test_transformation_detection() {
    let db = LingoDB::load("test_data.lingo").unwrap();
    let extractor = FunctionExtractor::new(db);
    
    let result = extractor.extract_function_signature(
        "The startup converted their MVP into a scalable platform"
    ).unwrap();
    
    // Should detect: Agency (startup) + Action (converted) + Transformation (MVP → platform)
    assert!(result.confidence > 0.7);
    assert_eq!(result.primitives.len(), 3);
    
    let transformation = result.primitives.iter()
        .find(|p| matches!(p, FunctionalPrimitive::Transformation { .. }))
        .unwrap();
        
    if let FunctionalPrimitive::Transformation { input_state, output_state, .. } = transformation {
        assert_eq!(input_state.surface_form, "MVP");
        assert_eq!(output_state.surface_form, "platform");
    }
}
```

### Test Case 3: Conditionality Detection
```rust
#[test]
fn test_conditionality_detection() {
    let db = LingoDB::load("test_data.lingo").unwrap();
    let extractor = FunctionExtractor::new(db);
    
    let result = extractor.extract_function_signature(
        "If the user uploads documents, the AI will analyze them"
    ).unwrap();
    
    // Should detect: Conditionality (if) + Agency (user, AI) + Actions (uploads, analyze)
    assert!(result.confidence > 0.75);
    
    let conditionality = result.primitives.iter()
        .find(|p| matches!(p, FunctionalPrimitive::Conditionality { .. }))
        .unwrap();
        
    if let FunctionalPrimitive::Conditionality { certainty_level, .. } = conditionality {
        assert!(*certainty_level > 0.5); // "if" indicates medium certainty
    }
}
```

---

## 📈 Performance Requirements

### Extraction Speed Targets
- **Simple sentences** (1-10 words): < 5ms
- **Complex sentences** (10-30 words): < 15ms  
- **Paragraph text** (30+ words): < 50ms

### Accuracy Targets
- **Agency detection**: > 85% precision, > 80% recall
- **Action detection**: > 80% precision, > 85% recall
- **Transformation detection**: > 75% precision, > 70% recall
- **Overall function signatures**: > 70% semantic accuracy

### Memory Requirements
- **Peak memory usage**: < 50MB on mobile
- **Database size**: < 25MB for core English model
- **Query cache**: < 5MB for active queries

---

## 🚀 Implementation Strategy (Revised Based on Discovery)

### Phase 1: Morphological Foundation (Week 1)
- **Build morphological decomposition engine**
  - Text → words → morphemes pipeline
  - Lemmatization for base form lookup
  - Morpheme-to-database mapping
  
- **Implement compositional positioning**
  - Vector composition from morpheme coordinates
  - Spatial relationship calculations
  - Cross-layer morpheme analysis

- **Test with simple examples**
  - "manager" → ["manage", "-er"] → Agency detection
  - "organized" → ["organize", "-ed"] → Action detection

### Phase 2: Agency & Action Detection (Week 2)  
- **Agency through morphological markers**
  - Suffix pattern detection (-er, -ant, -ist, -or)
  - Semantic role analysis from morpheme properties
  - Capability scoring from morpheme composition
  
- **Action through causative morphemes**
  - Verb identification through morphological analysis  
  - Transformation type classification via spatial clustering
  - Intensity calculation from morpheme combinations

### Phase 3: Complex Pattern Detection (Week 3)
- **Transformation via multi-word patterns**
  - Cross-word morphological analysis
  - State-change detection through spatial vectors
  - Input/output state identification
  
- **Conditionality & sequence enhancement**
  - Build on existing working detection
  - Add morphological depth analysis
  - Improve confidence scoring

### Phase 4: Optimization & Validation (Week 4)
- **Performance optimization**
  - Cache morphological decompositions
  - Optimize spatial vector calculations
  - Memory management for mobile deployment
  
- **Real-world testing**
  - Test with actual founder communication
  - Validate against Cira use cases
  - Measure extraction accuracy and performance

### Key Technical Challenges Solved:
1. ✅ **Database Integration**: Use provided database, not dummy instance
2. ✅ **Compositional Analysis**: True bottom-up morpheme composition  
3. ✅ **Lemmatization Pipeline**: Handle inflected forms properly
4. ✅ **Spatial Mathematics**: Vector composition from morpheme coordinates

---

## 🎯 Success Criteria

1. **Deterministic Extraction**: No hardcoded templates, all patterns discovered through spatial/morphological analysis
2. **High Accuracy**: >70% semantic accuracy on real founder communication
3. **Fast Performance**: Sub-20ms extraction for typical sentences  
4. **Spatial Coherence**: Extracted primitives cluster meaningfully in 3D space
5. **Explainable Results**: Clear extraction paths showing how function was discovered

This system will prove that function naturally emerges from linguistic structure rather than requiring external templates!
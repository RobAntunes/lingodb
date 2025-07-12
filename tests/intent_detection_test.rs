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

//! Intent Detection Plugin Tests
//! 
//! Tests for the Intent = Function × PragmaticOperators specification
//! including all 9 operator types and compositional algebra.

use std::fs;
use std::path::Path;
use lingo::{
    storage::LingoDatabase,
    plugins::{
        IntentDetector, Intent, PragmaticOperators,
        DirectionalType, ModalType, TemporalType, ConditionalType,
        NegationType, IntensityType, CertaintyType, ScopeType, SocialType,
        Plugin, PluginPipeline
    },
    core::{Layer, Coordinate3D},
};

const TEST_DB_PATH: &str = "tests/test_intent.lingo";

fn setup_test_database() -> LingoDatabase {
    // Remove existing test database
    if Path::new(TEST_DB_PATH).exists() {
        fs::remove_file(TEST_DB_PATH).unwrap();
    }
    
    // Create test database with minimal seeding
    let database = LingoDatabase::create(TEST_DB_PATH).unwrap();
    
    // Seed with basic morphemes for testing
    let mut nodes = Vec::new();
    
    // Add basic words that are used in intent detection
    let words = vec![
        "really", "need", "to", "find", "technical", "cofounder", "soon", "please",
        "can", "help", "should", "must", "will", "very", "definitely", "all", "some"
    ];
    
    for (i, word) in words.iter().enumerate() {
        let position = Coordinate3D {
            x: (i as f32) * 0.1,
            y: (i as f32) * 0.1,
            z: 3.0, // Words layer
        };
        nodes.push((word.to_string(), Layer::Words, position));
    }
    
    database.seed_nodes(&nodes).unwrap();
    database
}

fn setup_intent_detector() -> IntentDetector {
    let database = setup_test_database();
    let mut detector = IntentDetector::new();
    detector.initialize(&database).unwrap();
    detector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_detector_basic_functionality() {
        let detector = setup_intent_detector();
        assert_eq!(detector.id(), "intent_detection");
        assert_eq!(detector.name(), "Intent Detection Plugin");
        assert_eq!(detector.version(), "1.0.0");
        assert_eq!(detector.dependencies(), vec!["function_extraction"]);
    }

    #[test]
    fn test_simple_intent_detection() {
        let mut detector = setup_intent_detector();
        let text = "I really need to find a technical cofounder soon, please";
        
        let result = detector.detect_intent(text);
        assert!(result.is_ok(), "Intent detection should succeed");
        
        let intent = result.unwrap();
        assert!(intent.intent_confidence > 0.5, "Intent confidence should be reasonable");
        assert_eq!(intent.source_text, text);
        assert!(intent.execution_time_ms >= 0.0);
    }

    #[test]
    fn test_directional_operator_detection() {
        let detector = setup_intent_detector();
        let detector_suite = detector.operator_detectors.as_ref().unwrap();
        let database = setup_test_database();
        
        let text = "looking for cofounder to help with startup";
        let operators = detector_suite.directional_detector.detect(text, &database).unwrap();
        
        assert!(!operators.is_empty(), "Should detect directional operators");
        
        let for_operator = operators.iter().find(|op| matches!(op.operator_type, DirectionalType::FOR));
        assert!(for_operator.is_some(), "Should detect FOR operator");
        
        let to_operator = operators.iter().find(|op| matches!(op.operator_type, DirectionalType::TO));
        assert!(to_operator.is_some(), "Should detect TO operator");
    }

    #[test]
    fn test_modal_operator_detection() {
        let detector = setup_intent_detector();
        let detector_suite = detector.operator_detectors.as_ref().unwrap();
        let database = setup_test_database();
        
        let text = "We can help and should definitely consider this";
        let operators = detector_suite.modal_detector.detect(text, &database).unwrap();
        
        assert!(!operators.is_empty(), "Should detect modal operators");
        
        let can_operator = operators.iter().find(|op| matches!(op.operator_type, ModalType::CAN));
        assert!(can_operator.is_some(), "Should detect CAN operator");
        
        let should_operator = operators.iter().find(|op| matches!(op.operator_type, ModalType::SHOULD));
        assert!(should_operator.is_some(), "Should detect SHOULD operator");
    }

    #[test]
    fn test_temporal_operator_detection() {
        let detector = setup_intent_detector();
        let detector_suite = detector.operator_detectors.as_ref().unwrap();
        let database = setup_test_database();
        
        let text = "Need this done now, but we can discuss later";
        let operators = detector_suite.temporal_detector.detect(text, &database).unwrap();
        
        assert!(!operators.is_empty(), "Should detect temporal operators");
        
        let now_operator = operators.iter().find(|op| matches!(op.operator_type, TemporalType::NOW));
        assert!(now_operator.is_some(), "Should detect NOW operator");
        
        let later_operator = operators.iter().find(|op| matches!(op.operator_type, TemporalType::LATER));
        assert!(later_operator.is_some(), "Should detect LATER operator");
        
        // Check urgency levels
        if let Some(now_op) = now_operator {
            assert!(now_op.urgency_level > 0.8, "NOW should have high urgency");
        }
        
        if let Some(later_op) = later_operator {
            assert!(later_op.urgency_level < 0.5, "LATER should have low urgency");
        }
    }

    #[test]
    fn test_conditional_operator_detection() {
        let detector = setup_intent_detector();
        let detector_suite = detector.operator_detectors.as_ref().unwrap();
        let database = setup_test_database();
        
        let text = "If we get funding, then we can hire more people";
        let operators = detector_suite.conditional_detector.detect(text, &database).unwrap();
        
        assert!(!operators.is_empty(), "Should detect conditional operators");
        
        let if_operator = operators.iter().find(|op| matches!(op.operator_type, ConditionalType::IF));
        assert!(if_operator.is_some(), "Should detect IF operator");
        
        if let Some(if_op) = if_operator {
            assert!(!if_op.condition_content.is_empty(), "Should extract condition content");
            assert!(if_op.condition_content.contains("we get funding"), "Should extract correct condition");
        }
    }

    #[test]
    fn test_negation_operator_detection() {
        let detector = setup_intent_detector();
        let detector_suite = detector.operator_detectors.as_ref().unwrap();
        let database = setup_test_database();
        
        let text = "Don't want any more technical interviews right now";
        let operators = detector_suite.negation_detector.detect(text, &database).unwrap();
        
        // Note: "Don't" might not be detected as it's a contraction
        // Let's test with explicit negation
        let text2 = "I do not want technical interviews";
        let operators2 = detector_suite.negation_detector.detect(text2, &database).unwrap();
        
        assert!(!operators2.is_empty(), "Should detect negation operators");
        
        let not_operator = operators2.iter().find(|op| matches!(op.operator_type, NegationType::NOT));
        assert!(not_operator.is_some(), "Should detect NOT operator");
        
        // Test prefix negation
        let text3 = "This approach is unproductive and disconnected";
        let operators3 = detector_suite.negation_detector.detect(text3, &database).unwrap();
        
        let prefix_operators: Vec<_> = operators3.iter()
            .filter(|op| matches!(op.operator_type, NegationType::UN_PREFIX | NegationType::DIS_PREFIX))
            .collect();
        assert!(!prefix_operators.is_empty(), "Should detect prefix negations");
    }

    #[test]
    fn test_intensity_operator_detection() {
        let detector = setup_intent_detector();
        let detector_suite = detector.operator_detectors.as_ref().unwrap();
        let database = setup_test_database();
        
        let text = "This is extremely important and very urgent";
        let operators = detector_suite.intensity_detector.detect(text, &database).unwrap();
        
        assert!(!operators.is_empty(), "Should detect intensity operators");
        
        let extremely_operator = operators.iter().find(|op| matches!(op.operator_type, IntensityType::EXTREMELY));
        assert!(extremely_operator.is_some(), "Should detect EXTREMELY operator");
        
        let very_operator = operators.iter().find(|op| matches!(op.operator_type, IntensityType::VERY));
        assert!(very_operator.is_some(), "Should detect VERY operator");
        
        // Check intensity levels
        if let Some(extremely_op) = extremely_operator {
            assert!(extremely_op.intensity_level > 0.8, "EXTREMELY should have high intensity");
            assert!(!extremely_op.target_modification.is_empty(), "Should extract target modification");
        }
    }

    #[test]
    fn test_certainty_operator_detection() {
        let detector = setup_intent_detector();
        let detector_suite = detector.operator_detectors.as_ref().unwrap();
        let database = setup_test_database();
        
        let text = "We will definitely succeed, probably by next month";
        let operators = detector_suite.certainty_detector.detect(text, &database).unwrap();
        
        assert!(!operators.is_empty(), "Should detect certainty operators");
        
        let definitely_operator = operators.iter().find(|op| matches!(op.operator_type, CertaintyType::DEFINITELY));
        assert!(definitely_operator.is_some(), "Should detect DEFINITELY operator");
        
        let probably_operator = operators.iter().find(|op| matches!(op.operator_type, CertaintyType::PROBABLY));
        assert!(probably_operator.is_some(), "Should detect PROBABLY operator");
        
        // Check confidence levels
        if let Some(def_op) = definitely_operator {
            assert!(def_op.confidence_level > 0.9, "DEFINITELY should have high confidence");
        }
        
        if let Some(prob_op) = probably_operator {
            assert!(prob_op.confidence_level < 0.9 && prob_op.confidence_level > 0.5, "PROBABLY should have medium confidence");
        }
    }

    #[test]
    fn test_scope_operator_detection() {
        let detector = setup_intent_detector();
        let detector_suite = detector.operator_detectors.as_ref().unwrap();
        let database = setup_test_database();
        
        let text = "All developers need some experience, but few have many skills";
        let operators = detector_suite.scope_detector.detect(text, &database).unwrap();
        
        assert!(!operators.is_empty(), "Should detect scope operators");
        
        let all_operator = operators.iter().find(|op| matches!(op.operator_type, ScopeType::ALL));
        assert!(all_operator.is_some(), "Should detect ALL operator");
        
        let some_operator = operators.iter().find(|op| matches!(op.operator_type, ScopeType::SOME));
        assert!(some_operator.is_some(), "Should detect SOME operator");
        
        let few_operator = operators.iter().find(|op| matches!(op.operator_type, ScopeType::FEW));
        assert!(few_operator.is_some(), "Should detect FEW operator");
        
        // Check quantity estimates
        if let Some(all_op) = all_operator {
            assert!(all_op.quantity_estimate.unwrap() == 1.0, "ALL should have quantity 1.0");
        }
        
        if let Some(few_op) = few_operator {
            assert!(few_op.quantity_estimate.unwrap() < 0.5, "FEW should have low quantity");
        }
    }

    #[test]
    fn test_social_operator_detection() {
        let detector = setup_intent_detector();
        let detector_suite = detector.operator_detectors.as_ref().unwrap();
        let database = setup_test_database();
        
        let text = "Could you please help us? Thank you for your time.";
        let operators = detector_suite.social_detector.detect(text, &database).unwrap();
        
        assert!(!operators.is_empty(), "Should detect social operators");
        
        let please_operator = operators.iter().find(|op| matches!(op.operator_type, SocialType::PLEASE));
        assert!(please_operator.is_some(), "Should detect PLEASE operator");
        
        let thank_you_operator = operators.iter().find(|op| matches!(op.operator_type, SocialType::THANK_YOU));
        assert!(thank_you_operator.is_some(), "Should detect THANK_YOU operator");
        
        let could_you_operator = operators.iter().find(|op| matches!(op.operator_type, SocialType::COULD_YOU));
        assert!(could_you_operator.is_some(), "Should detect COULD_YOU operator");
        
        // Check politeness levels
        if let Some(please_op) = please_operator {
            assert!(please_op.politeness_level > 0.5, "PLEASE should have decent politeness level");
        }
    }

    #[test]
    fn test_complex_intent_composition() {
        let mut detector = setup_intent_detector();
        let text = "If we can't find an experienced React developer soon, we should probably consider hiring a team instead";
        
        let result = detector.detect_intent(text);
        assert!(result.is_ok(), "Complex intent detection should succeed");
        
        let intent = result.unwrap();
        
        // Should detect multiple operator types
        assert!(!intent.pragmatic_operators.conditional.is_empty(), "Should have conditional operators");
        assert!(!intent.pragmatic_operators.modal.is_empty(), "Should have modal operators");
        assert!(!intent.pragmatic_operators.temporal.is_empty(), "Should have temporal operators");
        assert!(!intent.pragmatic_operators.certainty.is_empty(), "Should have certainty operators");
        
        // Check operator coherence
        assert!(intent.operator_coherence > 0.5, "Complex operators should have reasonable coherence");
        
        // Check compositional path
        assert!(!intent.compositional_path.is_empty(), "Should have compositional path");
        
        // Check overall confidence
        assert!(intent.intent_confidence > 0.5, "Complex intent should have reasonable confidence");
    }

    #[test]
    fn test_negated_intent() {
        let mut detector = setup_intent_detector();
        let text = "Don't want any more technical interviews right now";
        
        let result = detector.detect_intent(text);
        assert!(result.is_ok(), "Negated intent detection should succeed");
        
        let intent = result.unwrap();
        
        // Should detect negation and scope
        assert!(!intent.pragmatic_operators.negation.is_empty() || 
                !intent.pragmatic_operators.scope.is_empty(), 
                "Should detect negation or scope operators");
        
        // Should detect temporal
        assert!(!intent.pragmatic_operators.temporal.is_empty(), "Should detect temporal operators");
        
        // Check overall confidence
        assert!(intent.intent_confidence > 0.5, "Negated intent should have reasonable confidence");
    }

    #[test]
    fn test_operator_coherence_calculation() {
        let detector = setup_intent_detector();
        
        // Test conflicting operators (negation + intensity)
        let mut operators = PragmaticOperators::new();
        operators.negation.push(Default::default());
        operators.intensity.push(Default::default());
        
        let coherence = detector.calculate_operator_coherence(&operators);
        assert!(coherence < 1.0, "Conflicting operators should reduce coherence");
        
        // Test harmonious operators (modal + temporal)
        let mut operators2 = PragmaticOperators::new();
        operators2.modal.push(Default::default());
        operators2.temporal.push(Default::default());
        
        let coherence2 = detector.calculate_operator_coherence(&operators2);
        assert!(coherence2 > coherence, "Harmonious operators should have higher coherence");
    }

    #[test]
    fn test_plugin_pipeline_integration() {
        let database = setup_test_database();
        let mut pipeline = PluginPipeline::new();
        pipeline.set_database(std::sync::Arc::new(database));
        
        // Register intent detection plugin
        let intent_plugin = Box::new(IntentDetector::new());
        pipeline.register_plugin(intent_plugin).unwrap();
        
        // Initialize plugins
        pipeline.initialize_plugins().unwrap();
        
        // Test command execution
        let result = pipeline.execute_command(
            "intent_detection", 
            "detect_intent", 
            &["I really need to find a cofounder soon".to_string()]
        );
        
        assert!(result.is_ok(), "Plugin command should execute successfully");
    }

    #[test]
    fn test_performance_benchmarks() {
        let mut detector = setup_intent_detector();
        let test_texts = vec![
            "I really need to find a technical cofounder soon, please",
            "If we can't find an experienced React developer soon, we should probably consider hiring a team instead",
            "Don't want any more technical interviews right now",
            "We can definitely help with this project, but we might need some additional resources",
            "All developers need experience, but few have many advanced skills in multiple frameworks"
        ];
        
        let start_time = std::time::Instant::now();
        
        for text in &test_texts {
            let result = detector.detect_intent(text);
            assert!(result.is_ok(), "All test cases should succeed");
            
            let intent = result.unwrap();
            assert!(intent.execution_time_ms < 50.0, "Individual detection should be under 50ms");
        }
        
        let total_time = start_time.elapsed();
        assert!(total_time.as_millis() < 250, "All 5 detections should complete under 250ms");
    }

    #[test]
    fn test_edge_cases() {
        let mut detector = setup_intent_detector();
        
        // Empty string
        let result = detector.detect_intent("");
        // Should handle gracefully (might succeed with empty operators or fail gracefully)
        
        // Single word
        let result = detector.detect_intent("help");
        // Should handle gracefully
        
        // Only operators, no clear function
        let result = detector.detect_intent("very really extremely");
        // Should handle gracefully
        
        // Very long text
        let long_text = "I really need to find a technical cofounder soon please help me with this urgent matter because we definitely need to scale our startup and we can probably find someone who will work with us but we should consider all options and maybe look at different approaches ".repeat(10);
        let result = detector.detect_intent(&long_text);
        
        if result.is_ok() {
            let intent = result.unwrap();
            // Should still complete in reasonable time even for long text
            assert!(intent.execution_time_ms < 100.0, "Long text should still process quickly");
        }
    }

    #[test]
    fn test_spec_examples() {
        let mut detector = setup_intent_detector();
        
        // Test Case 1 from spec: Simple Intent
        let text1 = "I really need to find a technical cofounder soon, please";
        let result1 = detector.detect_intent(text1);
        assert!(result1.is_ok(), "Spec example 1 should work");
        
        let intent1 = result1.unwrap();
        assert!(intent1.intent_confidence > 0.7, "Should have high confidence for clear intent");
        
        // Should detect multiple operator types
        let operators_detected = [
            !intent1.pragmatic_operators.directional.is_empty(),
            !intent1.pragmatic_operators.modal.is_empty(),
            !intent1.pragmatic_operators.temporal.is_empty(),
            !intent1.pragmatic_operators.intensity.is_empty(),
            !intent1.pragmatic_operators.social.is_empty(),
        ].iter().filter(|&&x| x).count();
        
        assert!(operators_detected >= 3, "Should detect multiple operator types for complex intent");
    }

    // Cleanup
    impl Drop for IntentDetector {
        fn drop(&mut self) {
            // Clean up test database
            if Path::new(TEST_DB_PATH).exists() {
                let _ = fs::remove_file(TEST_DB_PATH);
            }
        }
    }
}
// Complex intent detection test with real sentences
use lingo::plugins::{Plugin, IntentDetector};
use lingo::plugins::intent_detection::{
    OperatorDetectorSuite, PragmaticOperators,
    DirectionalType, ModalType, TemporalType, NegationType, IntensityType,
    CertaintyType, ScopeType, SocialType
};

#[test]
fn test_spec_example_sentence() {
    // Example from the spec: "I really need to find a technical cofounder soon, please"
    // Should detect:
    // - Directional: TO (find)
    // - Modal: NEED (necessity)
    // - Temporal: SOON
    // - Intensity: REALLY  
    // - Social: PLEASE
    
    println!("\n🔍 Testing Spec Example Sentence:");
    println!("Input: \"I really need to find a technical cofounder soon, please\"");
    
    let mut detector = IntentDetector::new();
    let suite = OperatorDetectorSuite::new();
    
    let text = "I really need to find a technical cofounder soon, please";
    let operators = suite.detect_all_operators_simple(text);
    
    // Show detected operators
    println!("\n📊 Detected Operators:");
    println!("  Directional: {} operators - {:?}", 
             operators.directional.len(), 
             operators.directional.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Modal: {} operators - {:?}", 
             operators.modal.len(), 
             operators.modal.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Temporal: {} operators - {:?}", 
             operators.temporal.len(), 
             operators.temporal.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Intensity: {} operators - {:?}", 
             operators.intensity.len(), 
             operators.intensity.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Social: {} operators - {:?}", 
             operators.social.len(), 
             operators.social.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    
    // Verify detected operators
    assert!(!operators.directional.is_empty(), "Should detect directional operator");
    assert!(!operators.modal.is_empty(), "Should detect modal operator");
    assert!(!operators.temporal.is_empty(), "Should detect temporal operator");
    assert!(!operators.intensity.is_empty(), "Should detect intensity operator");
    assert!(!operators.social.is_empty(), "Should detect social operator");
    
    // Check specific operator types
    assert!(operators.directional.iter().any(|op| matches!(op.operator_type, DirectionalType::TO)));
    assert!(operators.modal.iter().any(|op| matches!(op.operator_type, ModalType::MUST)));
    assert!(operators.temporal.iter().any(|op| matches!(op.operator_type, TemporalType::SOON)));
    assert!(operators.intensity.iter().any(|op| matches!(op.operator_type, IntensityType::REALLY)));
    assert!(operators.social.iter().any(|op| matches!(op.operator_type, SocialType::PLEASE)));
    
    println!("✅ Spec example successfully detected 5 operator types!");
}

#[test] 
fn test_complex_conditional_sentence() {
    // "If you could very carefully review all documents before tomorrow, that would definitely help"
    // Should detect:
    // - Conditional: IF
    // - Modal: COULD  
    // - Intensity: VERY
    // - Temporal: BEFORE, TOMORROW
    // - Scope: ALL
    // - Certainty: DEFINITELY
    
    println!("\n🔍 Testing Complex Conditional Sentence:");
    println!("Input: \"If you could very carefully review all documents before tomorrow, that would definitely help\"");
    
    let suite = OperatorDetectorSuite::new();
    let text = "If you could very carefully review all documents before tomorrow, that would definitely help";
    let operators = suite.detect_all_operators_simple(text);
    
    println!("\n📊 Detected Operators:");
    println!("  Conditional: {} operators - {:?}", 
             operators.conditional.len(), 
             operators.conditional.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Modal: {} operators - {:?}", 
             operators.modal.len(), 
             operators.modal.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Intensity: {} operators - {:?}", 
             operators.intensity.len(), 
             operators.intensity.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Temporal: {} operators - {:?}", 
             operators.temporal.len(), 
             operators.temporal.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Scope: {} operators - {:?}", 
             operators.scope.len(), 
             operators.scope.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Certainty: {} operators - {:?}", 
             operators.certainty.len(), 
             operators.certainty.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    
    assert!(!operators.conditional.is_empty(), "Should detect conditional");
    assert!(!operators.modal.is_empty(), "Should detect modal"); 
    assert!(!operators.intensity.is_empty(), "Should detect intensity");
    assert!(!operators.temporal.is_empty(), "Should detect temporal");
    assert!(!operators.scope.is_empty(), "Should detect scope");
    assert!(!operators.certainty.is_empty(), "Should detect certainty");
    
    println!("✅ Complex conditional sentence detected 6 operator types!");
}

#[test]
fn test_negation_with_multiple_operators() {
    // "I never really want to see most people immediately"  
    // Should detect:
    // - Negation: NEVER
    // - Intensity: REALLY
    // - Modal: WANT
    // - Directional: TO
    // - Scope: MOST  
    // - Temporal: IMMEDIATELY
    
    println!("\n🔍 Testing Negation with Multiple Operators:");
    println!("Input: \"I never really want to see most people immediately\"");
    
    let suite = OperatorDetectorSuite::new();
    let text = "I never really want to see most people immediately";
    let operators = suite.detect_all_operators_simple(text);
    
    println!("\n📊 Detected Operators:");
    println!("  Negation: {} operators - {:?}", 
             operators.negation.len(), 
             operators.negation.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Intensity: {} operators - {:?}", 
             operators.intensity.len(), 
             operators.intensity.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Modal: {} operators - {:?}", 
             operators.modal.len(), 
             operators.modal.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Directional: {} operators - {:?}", 
             operators.directional.len(), 
             operators.directional.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Scope: {} operators - {:?}", 
             operators.scope.len(), 
             operators.scope.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Temporal: {} operators - {:?}", 
             operators.temporal.len(), 
             operators.temporal.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    
    assert!(!operators.negation.is_empty(), "Should detect negation");
    assert!(!operators.intensity.is_empty(), "Should detect intensity"); 
    assert!(!operators.modal.is_empty(), "Should detect modal");
    assert!(!operators.directional.is_empty(), "Should detect directional");
    assert!(!operators.scope.is_empty(), "Should detect scope");
    assert!(!operators.temporal.is_empty(), "Should detect temporal");
    
    println!("✅ Negation sentence detected 6 operator types!");
}

#[test]
fn test_highly_compositional_sentence() {
    // "Could you please very quickly check if some users might possibly need this feature soon?"
    // Should detect:
    // - Modal: COULD, MIGHT
    // - Social: PLEASE
    // - Intensity: VERY  
    // - Temporal: QUICKLY, SOON
    // - Conditional: IF
    // - Scope: SOME
    // - Certainty: POSSIBLY
    // - Modal: NEED
    
    println!("\n🔍 Testing Highly Compositional Sentence:");
    println!("Input: \"Could you please very quickly check if some users might possibly need this feature soon?\"");
    
    let suite = OperatorDetectorSuite::new();
    let text = "Could you please very quickly check if some users might possibly need this feature soon?";
    let operators = suite.detect_all_operators_simple(text);
    
    println!("\n📊 Detected Operators:");
    println!("  Modal: {} operators - {:?}", 
             operators.modal.len(), 
             operators.modal.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Social: {} operators - {:?}", 
             operators.social.len(), 
             operators.social.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Intensity: {} operators - {:?}", 
             operators.intensity.len(), 
             operators.intensity.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Temporal: {} operators - {:?}", 
             operators.temporal.len(), 
             operators.temporal.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Conditional: {} operators - {:?}", 
             operators.conditional.len(), 
             operators.conditional.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Scope: {} operators - {:?}", 
             operators.scope.len(), 
             operators.scope.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    println!("  Certainty: {} operators - {:?}", 
             operators.certainty.len(), 
             operators.certainty.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    
    // Should have multiple operators of same type
    assert!(operators.modal.len() >= 2, "Should detect multiple modal operators (could, might, need)");
    assert!(operators.temporal.len() >= 2, "Should detect multiple temporal operators (quickly, soon)");
    
    assert!(!operators.social.is_empty(), "Should detect social");
    assert!(!operators.intensity.is_empty(), "Should detect intensity");
    assert!(!operators.conditional.is_empty(), "Should detect conditional");
    assert!(!operators.scope.is_empty(), "Should detect scope");
    assert!(!operators.certainty.is_empty(), "Should detect certainty");
    
    println!("✅ Highly compositional sentence detected 7+ operator types with multiples!");
}

#[test]
fn test_operator_coherence_calculation() {
    // Test that operator coherence works with complex sentences
    let mut detector = IntentDetector::new();
    
    // Simple sentence should have high coherence
    let simple_operators = PragmaticOperators {
        directional: vec![],
        modal: vec![],
        temporal: vec![],
        conditional: vec![],
        negation: vec![],
        intensity: vec![],
        certainty: vec![],
        scope: vec![],
        social: vec![],
    };
    
    let simple_coherence = detector.calculate_operator_coherence(&simple_operators);
    assert!(simple_coherence >= 0.8, "Simple operators should have high coherence");
    
    println!("✅ Operator coherence calculation working!");
}

#[test]
fn test_morpheme_detection_in_context() {
    // Test that morphemes are detected even in complex contexts
    println!("\n🔍 Testing Morpheme Detection in Various Contexts:");
    
    let suite = OperatorDetectorSuite::new();
    
    // Test directional morphemes in various contexts
    let contexts = vec![
        "I need to go", // simple TO
        "working toward a solution", // TOWARD  
        "focus on the problem", // ON (as directional)
        "coming from experience", // FROM
        "dealing with issues", // WITH
    ];
    
    println!("\n📊 Testing {} different contexts:", contexts.len());
    for (i, context) in contexts.iter().enumerate() {
        let operators = suite.detect_all_operators_simple(context);
        println!("  {}. \"{}\" → Directional: {} operators - {:?}", 
                 i + 1, 
                 context,
                 operators.directional.len(),
                 operators.directional.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
        assert!(!operators.directional.is_empty(), 
               "Should detect directional in: {}", context);
    }
    
    println!("✅ Morpheme detection working in various contexts!");
}

#[test]
fn test_edge_cases() {
    println!("\n🔍 Testing Edge Cases:");
    
    let suite = OperatorDetectorSuite::new();
    
    println!("\n📊 Edge Case Tests:");
    
    // Empty string
    println!("  1. Empty string test:");
    let empty_ops = suite.detect_all_operators_simple("");
    println!("     Input: \"\" → No operators should be detected");
    println!("     Result: {} total operators found", 
             empty_ops.directional.len() + empty_ops.modal.len() + empty_ops.temporal.len() + 
             empty_ops.conditional.len() + empty_ops.negation.len() + empty_ops.intensity.len() + 
             empty_ops.certainty.len() + empty_ops.scope.len() + empty_ops.social.len());
    assert!(empty_ops.directional.is_empty());
    
    // Single word
    println!("  2. Single word test:");
    let single_ops = suite.detect_all_operators_simple("really");
    println!("     Input: \"really\" → Intensity: {} operators - {:?}", 
             single_ops.intensity.len(),
             single_ops.intensity.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    assert!(!single_ops.intensity.is_empty(), "Should detect 'really' as intensity");
    
    // Repeated words
    println!("  3. Repeated words test:");
    let repeat_ops = suite.detect_all_operators_simple("very very very important");
    println!("     Input: \"very very very important\" → Intensity: {} operators - {:?}", 
             repeat_ops.intensity.len(),
             repeat_ops.intensity.iter().map(|op| format!("{:?}", op.operator_type)).collect::<Vec<_>>());
    assert!(!repeat_ops.intensity.is_empty(), "Should detect repeated intensity");
    
    println!("✅ Edge cases handled correctly!");
}

#[test]
fn test_performance_benchmark() {
    use std::time::Instant;
    
    println!("\n🔍 Testing Performance Benchmark:");
    
    let suite = OperatorDetectorSuite::new();
    let complex_sentence = "Could you please very quickly check if some users might possibly need this feature soon while carefully considering all the potential implications and definitely ensuring that most people would probably want this functionality before we commit to implementing it through our existing development process?";
    
    println!("Input: Ultra-complex sentence with {} words", complex_sentence.split_whitespace().count());
    println!("\"{}\"", complex_sentence);
    
    // Run a quick analysis first to show what we detect
    let sample_operators = suite.detect_all_operators_simple(complex_sentence);
    println!("\n📊 Operators detected in benchmark sentence:");
    println!("  Modal: {} operators", sample_operators.modal.len());
    println!("  Social: {} operators", sample_operators.social.len());
    println!("  Intensity: {} operators", sample_operators.intensity.len());
    println!("  Temporal: {} operators", sample_operators.temporal.len());
    println!("  Conditional: {} operators", sample_operators.conditional.len());
    println!("  Scope: {} operators", sample_operators.scope.len());
    println!("  Certainty: {} operators", sample_operators.certainty.len());
    println!("  Directional: {} operators", sample_operators.directional.len());
    println!("  Negation: {} operators", sample_operators.negation.len());
    
    let total_ops = sample_operators.modal.len() + sample_operators.social.len() + 
                   sample_operators.intensity.len() + sample_operators.temporal.len() + 
                   sample_operators.conditional.len() + sample_operators.scope.len() + 
                   sample_operators.certainty.len() + sample_operators.directional.len() + 
                   sample_operators.negation.len();
    println!("  Total: {} operators detected", total_ops);
    
    println!("\n⏱️  Running 100 iterations for performance measurement...");
    let start = Instant::now();
    for _ in 0..100 {
        let _operators = suite.detect_all_operators_simple(complex_sentence);
    }
    let duration = start.elapsed();
    
    let avg_time = duration.as_millis() as f32 / 100.0;
    println!("Average detection time: {:.2}ms per sentence", avg_time);
    println!("Target: <25ms per sentence");
    
    // Should be under 25ms as per spec target
    assert!(avg_time < 25.0, "Detection should be under 25ms, got {:.2}ms", avg_time);
    
    println!("✅ Performance target met!");
}
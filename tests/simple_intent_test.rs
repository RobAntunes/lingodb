// Simple intent detection test
use lingo::plugins::{Plugin};
use lingo::plugins::intent_detection::{
    IntentDetector, DirectionalOperatorDetector, ModalOperatorDetector,
    TemporalOperatorDetector, ConditionalOperatorDetector, NegationOperatorDetector,
    IntensityOperatorDetector, CertaintyOperatorDetector, ScopeOperatorDetector,
    SocialOperatorDetector, OperatorDetectorSuite, PragmaticOperators,
    DirectionalType, ModalType, TemporalType, NegationType, IntensityType,
    CertaintyType, ScopeType, SocialType
};

#[test]
fn test_directional_operator_detector() {
    let detector = DirectionalOperatorDetector::new();
    
    // Check morpheme table has expected entries
    assert!(detector.directional_morphemes.contains_key("for"));
    assert!(detector.directional_morphemes.contains_key("to"));
    assert!(detector.directional_morphemes.contains_key("with"));
    
    // Check values
    if let Some((op_type, strength)) = detector.directional_morphemes.get("for") {
        assert!(matches!(op_type, DirectionalType::FOR));
        assert_eq!(*strength, 0.9);
    }
}

#[test]
fn test_modal_operator_detector() {
    let detector = ModalOperatorDetector::new();
    
    assert!(detector.modal_morphemes.contains_key("can"));
    assert!(detector.modal_morphemes.contains_key("must"));
    assert!(detector.modal_morphemes.contains_key("should"));
    
    if let Some((op_type, strength)) = detector.modal_morphemes.get("must") {
        assert!(matches!(op_type, ModalType::MUST));
        assert_eq!(*strength, 1.0);
    }
}

#[test]
fn test_temporal_operator_detector() {
    let detector = TemporalOperatorDetector::new();
    
    assert!(detector.temporal_morphemes.contains_key("now"));
    assert!(detector.temporal_morphemes.contains_key("soon"));
    assert!(detector.temporal_morphemes.contains_key("asap"));
    
    if let Some((op_type, _)) = detector.temporal_morphemes.get("asap") {
        assert!(matches!(op_type, TemporalType::ASAP));
    }
}

#[test]
fn test_negation_operator_detector() {
    let detector = NegationOperatorDetector::new();
    
    assert!(detector.negation_morphemes.contains_key("not"));
    assert!(detector.negation_morphemes.contains_key("never"));
    assert!(detector.negation_morphemes.contains_key("un-"));
    
    if let Some((op_type, strength)) = detector.negation_morphemes.get("never") {
        assert!(matches!(op_type, NegationType::NEVER));
        assert_eq!(*strength, 1.0);
    }
}

#[test]
fn test_intensity_operator_detector() {
    let detector = IntensityOperatorDetector::new();
    
    assert!(detector.intensity_morphemes.contains_key("very"));
    assert!(detector.intensity_morphemes.contains_key("extremely"));
    assert!(detector.intensity_morphemes.contains_key("slightly"));
    
    if let Some((op_type, strength)) = detector.intensity_morphemes.get("extremely") {
        assert!(matches!(op_type, IntensityType::EXTREMELY));
        assert_eq!(*strength, 1.0);
    }
}

#[test]
fn test_certainty_operator_detector() {
    let detector = CertaintyOperatorDetector::new();
    
    assert!(detector.certainty_morphemes.contains_key("definitely"));
    assert!(detector.certainty_morphemes.contains_key("maybe"));
    assert!(detector.certainty_morphemes.contains_key("probably"));
    
    if let Some((op_type, strength)) = detector.certainty_morphemes.get("definitely") {
        assert!(matches!(op_type, CertaintyType::DEFINITELY));
        assert_eq!(*strength, 1.0);
    }
}

#[test]
fn test_scope_operator_detector() {
    let detector = ScopeOperatorDetector::new();
    
    assert!(detector.scope_morphemes.contains_key("all"));
    assert!(detector.scope_morphemes.contains_key("some"));
    assert!(detector.scope_morphemes.contains_key("few"));
    
    if let Some((op_type, quantity)) = detector.scope_morphemes.get("all") {
        assert!(matches!(op_type, ScopeType::ALL));
        assert_eq!(*quantity, 1.0);
    }
}

#[test]
fn test_social_operator_detector() {
    let detector = SocialOperatorDetector::new();
    
    assert!(detector.social_morphemes.contains_key("please"));
    assert!(detector.social_morphemes.contains_key("thank you"));
    assert!(detector.social_morphemes.contains_key("sorry"));
    
    if let Some((op_type, politeness)) = detector.social_morphemes.get("please") {
        assert!(matches!(op_type, SocialType::PLEASE));
        assert_eq!(*politeness, 0.8);
    }
}

#[test]
fn test_operator_detector_suite() {
    let suite = OperatorDetectorSuite::new();
    // Just ensure it can be created without panicking
}

#[test]
fn test_pragmatic_operators_creation() {
    let operators = PragmaticOperators::new();
    
    // All operator vectors should be empty initially
    assert!(operators.directional.is_empty());
    assert!(operators.modal.is_empty());
    assert!(operators.temporal.is_empty());
    assert!(operators.conditional.is_empty());
    assert!(operators.negation.is_empty());
    assert!(operators.intensity.is_empty());
    assert!(operators.certainty.is_empty());
    assert!(operators.scope.is_empty());
    assert!(operators.social.is_empty());
}

#[test]
fn test_intent_detector_basic_properties() {
    let detector = IntentDetector::new();
    
    // Test plugin trait implementation
    assert_eq!(detector.id(), "intent_detection");
    assert_eq!(detector.name(), "Intent Detection Plugin");
    assert_eq!(detector.version(), "1.0.0");
    assert_eq!(detector.dependencies(), vec!["function_extraction"]);
}

#[test]
fn test_morpheme_coverage() {
    // Test that all 9 operator types from the spec are covered
    let directional = DirectionalOperatorDetector::new();
    let modal = ModalOperatorDetector::new();
    let temporal = TemporalOperatorDetector::new();
    let conditional = ConditionalOperatorDetector::new();
    let negation = NegationOperatorDetector::new();
    let intensity = IntensityOperatorDetector::new();
    let certainty = CertaintyOperatorDetector::new();
    let scope = ScopeOperatorDetector::new();
    let social = SocialOperatorDetector::new();
    
    // Test all spec morphemes are present
    
    // Directional
    assert!(directional.directional_morphemes.contains_key("for"));
    assert!(directional.directional_morphemes.contains_key("to"));
    assert!(directional.directional_morphemes.contains_key("toward"));
    assert!(directional.directional_morphemes.contains_key("from"));
    assert!(directional.directional_morphemes.contains_key("with"));
    assert!(directional.directional_morphemes.contains_key("about"));
    assert!(directional.directional_morphemes.contains_key("through"));
    assert!(directional.directional_morphemes.contains_key("via"));
    assert!(directional.directional_morphemes.contains_key("by"));
    
    // Modal
    assert!(modal.modal_morphemes.contains_key("can"));
    assert!(modal.modal_morphemes.contains_key("could"));
    assert!(modal.modal_morphemes.contains_key("should"));
    assert!(modal.modal_morphemes.contains_key("must"));
    assert!(modal.modal_morphemes.contains_key("might"));
    assert!(modal.modal_morphemes.contains_key("will"));
    assert!(modal.modal_morphemes.contains_key("would"));
    assert!(modal.modal_morphemes.contains_key("may"));
    
    // Temporal  
    assert!(temporal.temporal_morphemes.contains_key("now"));
    assert!(temporal.temporal_morphemes.contains_key("soon"));
    assert!(temporal.temporal_morphemes.contains_key("later"));
    assert!(temporal.temporal_morphemes.contains_key("before"));
    assert!(temporal.temporal_morphemes.contains_key("after"));
    assert!(temporal.temporal_morphemes.contains_key("during"));
    assert!(temporal.temporal_morphemes.contains_key("while"));
    assert!(temporal.temporal_morphemes.contains_key("until"));
    assert!(temporal.temporal_morphemes.contains_key("asap"));
    assert!(temporal.temporal_morphemes.contains_key("eventually"));
    
    // Conditional
    assert!(conditional.conditional_morphemes.contains_key("if"));
    assert!(conditional.conditional_morphemes.contains_key("when"));
    assert!(conditional.conditional_morphemes.contains_key("unless"));
    assert!(conditional.conditional_morphemes.contains_key("provided"));
    assert!(conditional.conditional_morphemes.contains_key("assuming"));
    assert!(conditional.conditional_morphemes.contains_key("given"));
    
    // Negation
    assert!(negation.negation_morphemes.contains_key("not"));
    assert!(negation.negation_morphemes.contains_key("never"));
    assert!(negation.negation_morphemes.contains_key("no"));
    assert!(negation.negation_morphemes.contains_key("neither"));
    assert!(negation.negation_morphemes.contains_key("un-"));
    assert!(negation.negation_morphemes.contains_key("dis-"));
    assert!(negation.negation_morphemes.contains_key("non-"));
    
    // Intensity
    assert!(intensity.intensity_morphemes.contains_key("very"));
    assert!(intensity.intensity_morphemes.contains_key("extremely"));
    assert!(intensity.intensity_morphemes.contains_key("quite"));
    assert!(intensity.intensity_morphemes.contains_key("rather"));
    assert!(intensity.intensity_morphemes.contains_key("slightly"));
    assert!(intensity.intensity_morphemes.contains_key("somewhat"));
    assert!(intensity.intensity_morphemes.contains_key("really"));
    
    // Certainty
    assert!(certainty.certainty_morphemes.contains_key("definitely"));
    assert!(certainty.certainty_morphemes.contains_key("probably"));
    assert!(certainty.certainty_morphemes.contains_key("likely"));
    assert!(certainty.certainty_morphemes.contains_key("maybe"));
    assert!(certainty.certainty_morphemes.contains_key("perhaps"));
    assert!(certainty.certainty_morphemes.contains_key("possibly"));
    assert!(certainty.certainty_morphemes.contains_key("surely"));
    assert!(certainty.certainty_morphemes.contains_key("certainly"));
    
    // Scope
    assert!(scope.scope_morphemes.contains_key("all"));
    assert!(scope.scope_morphemes.contains_key("some"));
    assert!(scope.scope_morphemes.contains_key("many"));
    assert!(scope.scope_morphemes.contains_key("few"));
    assert!(scope.scope_morphemes.contains_key("most"));
    assert!(scope.scope_morphemes.contains_key("several"));
    assert!(scope.scope_morphemes.contains_key("each"));
    assert!(scope.scope_morphemes.contains_key("every"));
    assert!(scope.scope_morphemes.contains_key("no"));
    
    // Social
    assert!(social.social_morphemes.contains_key("please"));
    assert!(social.social_morphemes.contains_key("thank you"));
    assert!(social.social_morphemes.contains_key("sorry"));
    assert!(social.social_morphemes.contains_key("excuse me"));
    assert!(social.social_morphemes.contains_key("if you don't mind"));
    assert!(social.social_morphemes.contains_key("would you"));
    assert!(social.social_morphemes.contains_key("could you"));
}

#[test]
fn test_spec_examples_structure() {
    // Test that we can represent the spec examples
    
    // Example: "I really need to find a technical cofounder soon, please"
    // Should detect:
    // - Directional: TO (find)
    // - Modal: NEED (necessity)
    // - Temporal: SOON
    // - Intensity: REALLY  
    // - Social: PLEASE
    
    let directional_detector = DirectionalOperatorDetector::new();
    let intensity_detector = IntensityOperatorDetector::new();
    let social_detector = SocialOperatorDetector::new();
    let temporal_detector = TemporalOperatorDetector::new();
    
    // Verify the morphemes exist for this example
    assert!(directional_detector.directional_morphemes.contains_key("to"));
    assert!(intensity_detector.intensity_morphemes.contains_key("really"));
    assert!(social_detector.social_morphemes.contains_key("please"));
    assert!(temporal_detector.temporal_morphemes.contains_key("soon"));
    
    println!("✅ Intent detection structure successfully validates spec examples!");
}
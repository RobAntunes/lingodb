// Debug intent detection 
use lingo::plugins::intent_detection::OperatorDetectorSuite;

#[test]
fn debug_temporal_detection() {
    let suite = OperatorDetectorSuite::new();
    
    // Test the specific sentence that's failing
    let text = "I really need to find a technical cofounder soon, please";
    let operators = suite.detect_all_operators_simple(text);
    
    println!("Input text: '{}'", text);
    println!("Temporal operators detected: {:?}", operators.temporal);
    println!("Modal operators detected: {:?}", operators.modal);
    println!("Directional operators detected: {:?}", operators.directional);
    println!("Intensity operators detected: {:?}", operators.intensity);
    println!("Social operators detected: {:?}", operators.social);
    
    // Test individual words
    let words = ["soon", "really", "need", "to", "please"];
    for word in words {
        let word_ops = suite.detect_all_operators_simple(word);
        println!("Word '{}': temporal={}, modal={}, directional={}, intensity={}, social={}", 
                 word,
                 word_ops.temporal.len(),
                 word_ops.modal.len(), 
                 word_ops.directional.len(),
                 word_ops.intensity.len(),
                 word_ops.social.len());
    }
}

#[test]
fn debug_word_processing() {
    use lingo::morphology::preprocess_text;
    
    let text = "I really need to find a technical cofounder soon, please";
    let processed = preprocess_text(text);
    println!("Original: '{}'", text);
    println!("Processed: '{}'", processed);
    
    let words: Vec<&str> = processed.split_whitespace().collect();
    println!("Words: {:?}", words);
    
    // Check if 'soon,' is the issue
    for (i, word) in words.iter().enumerate() {
        println!("Word {}: '{}'", i, word);
        if word.contains("soon") {
            println!("  -> Found 'soon' with extra chars: '{}'", word);
        }
    }
}

#[test] 
fn debug_negation_sentence() {
    let suite = OperatorDetectorSuite::new();
    let text = "I never really want to see most people immediately";
    let operators = suite.detect_all_operators_simple(text);
    
    println!("Negation sentence: '{}'", text);
    println!("Modal operators: {:?}", operators.modal);
    println!("Negation operators: {:?}", operators.negation);
    println!("Intensity operators: {:?}", operators.intensity);
    println!("Directional operators: {:?}", operators.directional);
    println!("Scope operators: {:?}", operators.scope);
    println!("Temporal operators: {:?}", operators.temporal);
}
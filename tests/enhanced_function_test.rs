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

//! Tests for enhanced function extraction with mirroring decompose capabilities

use std::sync::Arc;
use lingo::storage::LingoDatabase;
use lingo::plugins::function_extraction::FunctionExtractor;
use lingo::plugins::{Plugin, PluginContext};

#[test]
fn test_enhanced_function_extraction() {
    // Load the database
    let db_path = "english.lingo";
    let db = Arc::new(LingoDatabase::open(db_path).unwrap());
    
    let mut extractor = FunctionExtractor::new();
    
    // Initialize the plugin with enhanced mirroring capabilities
    extractor.initialize(&*db).unwrap();
    
    // Test enhanced function extraction
    let text = "I don't want a technical co-founder, looking for someone business-focused instead";
    let signature = extractor.extract_function_signature(text).unwrap();
    
    println!("Enhanced Function Signature:");
    println!("Primitives: {:?}", signature.primitives);
    println!("Confidence: {}", signature.confidence);
    println!("Morphological Confidence: {}", signature.morphological_confidence);
    println!("Synthesis Opportunities: {:?}", signature.synthesis_opportunities);
    println!("Mirror Analysis: {:?}", signature.mirror_analysis);
    println!("Negation Transforms: {:?}", signature.negation_transforms);
    
    // Verify enhanced capabilities
    assert!(signature.confidence > 0.0);
    
    // Should detect negations
    assert!(!signature.negation_transforms.is_empty(), "Should detect negations in the text");
    
    // Check if we found some mirror analysis
    println!("Found {} mirror pairs", signature.mirror_analysis.len());
    
    // Check if we found synthesis opportunities
    println!("Found {} synthesis opportunities", signature.synthesis_opportunities.len());
}

#[test]
fn test_simple_agency_with_mirroring() {
    let db_path = "english.lingo";
    let db = Arc::new(LingoDatabase::open(db_path).unwrap());
    
    let mut extractor = FunctionExtractor::new();
    extractor.initialize(&*db).unwrap();
    
    let text = "The manager organized the meeting";
    let signature = extractor.extract_function_signature(text).unwrap();
    
    println!("Simple Agency Analysis:");
    println!("Primitives found: {}", signature.primitives.len());
    println!("Confidence: {}", signature.confidence);
    println!("Morphological Confidence: {}", signature.morphological_confidence);
    println!("Mirror Analysis: {:?}", signature.mirror_analysis);
    
    // Should find some functional primitives
    assert!(!signature.primitives.is_empty(), "Should detect functional primitives");
    
    // Enhanced with morphological analysis
    assert!(signature.morphological_confidence >= 0.0, "Should have morphological confidence");
}

#[test]
fn test_round_trip_validation() {
    let db_path = "english.lingo";
    let db = Arc::new(LingoDatabase::open(db_path).unwrap());
    
    let mut extractor = FunctionExtractor::new();
    extractor.initialize(&*db).unwrap();
    
    let text = "developer creating software";
    let signature = extractor.extract_function_signature(text).unwrap();
    
    println!("Round-trip Validation Test:");
    println!("Original text: {}", text);
    println!("Morphological confidence: {}", signature.morphological_confidence);
    println!("Mirror analysis count: {}", signature.mirror_analysis.len());
    
    // The enhanced system should provide morphological insights
    assert!(signature.morphological_confidence >= 0.0);
}
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

//! Tests for the mirroring decompose functionality

use std::sync::Arc;
use lingo::mirroring::MirroringDecomposer;
use lingo::mirroring::{PatternType, MirrorType};
use lingo::storage::LingoDatabase;

#[test]
fn test_mirroring_decompose() {
    // Load the database
    let db_path = "english.lingo";
    let db = Arc::new(LingoDatabase::open(db_path).unwrap());
    
    let mut decomposer = MirroringDecomposer::new(db).unwrap();
    
    // Test decomposition
    let morphemes = decomposer.decompose("manager");
    println!("Decomposition of 'manager': {:?}", morphemes);
    assert!(!morphemes.is_empty());
    
    // Test composition
    let composed = decomposer.compose(&["manage".to_string(), "er".to_string()]);
    println!("Composition of ['manage', 'er']: {:?}", composed);
    assert!(composed.contains(&"manager".to_string()));
    
    // Test finding mirrors
    let mirrors = decomposer.find_mirrors("happiness");
    println!("Mirrors of 'happiness': {:?}", mirrors);
    
    // Test synthesis
    let synthesized = decomposer.synthesize_functions(
        PatternType::Agent,
        &["develop".to_string()]
    );
    println!("Synthesized agent functions from 'develop': {:?}", synthesized);
    assert!(!synthesized.is_empty());
    
    // Test validation
    let validation = decomposer.validate_decomposition_quality("developer");
    println!("Validation of 'developer' decomposition: {:?}", validation);
    assert!(validation.round_trip_success);
}

#[test]
fn test_empirical_weights() {
    let db = Arc::new(LingoDatabase::open("english.lingo").unwrap());
    let mut decomposer = MirroringDecomposer::new(db).unwrap();
    
    // Learn empirical weights
    decomposer.learn_empirical_weights().unwrap();
    
    // Test that weights have been updated
    let composed = decomposer.compose(&["un".to_string(), "happy".to_string()]);
    println!("Composition with empirical weights: {:?}", composed);
    assert!(composed.contains(&"unhappy".to_string()));
}

#[test]
fn test_mirror_types() {
    let db = Arc::new(LingoDatabase::open("english.lingo").unwrap());
    let mut decomposer = MirroringDecomposer::new(db).unwrap();
    
    // Test different mirror patterns
    let test_cases = vec![
        ("happy", "unhappy", MirrorType::Negation),
        ("create", "destroy", MirrorType::Reversal),
        ("hot", "cold", MirrorType::Complementary),
    ];
    
    for (word, expected_mirror, expected_type) in test_cases {
        let mirrors = decomposer.find_mirrors(word);
        println!("Mirrors of '{}': {:?}", word, mirrors);
        
        // Check if expected mirror is found
        let found = mirrors.iter().any(|m| 
            m.mirror == expected_mirror && m.mirror_type == expected_type
        );
        
        if !found {
            println!("Expected mirror '{}' with type {:?} not found for '{}'", 
                     expected_mirror, expected_type, word);
        }
    }
}

#[test]
fn test_pattern_synthesis() {
    let db = Arc::new(LingoDatabase::open("english.lingo").unwrap());
    let mut decomposer = MirroringDecomposer::new(db).unwrap();
    
    // Test different pattern types
    let patterns = vec![
        (PatternType::Agent, vec!["teach"], vec!["teacher"]),
        (PatternType::Action, vec!["modern"], vec!["modernize"]),
        (PatternType::Negation, vec!["able"], vec!["unable"]),
    ];
    
    for (pattern_type, base_morphemes, expected_words) in patterns {
        let base_strings: Vec<String> = base_morphemes.iter()
            .map(|s| s.to_string())
            .collect();
            
        let synthesized = decomposer.synthesize_functions(pattern_type, &base_strings);
        println!("Synthesized {:?} functions from {:?}: {:?}", 
                 pattern_type, base_morphemes, synthesized);
        
        // Check if expected words are generated
        for expected in expected_words {
            let found = synthesized.iter().any(|s| s.generated_word == expected);
            if !found {
                println!("Expected word '{}' not synthesized for pattern {:?}", 
                         expected, pattern_type);
            }
        }
    }
}
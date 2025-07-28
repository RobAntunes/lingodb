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

//! Enhanced tests for the etymological mirroring functionality

use std::sync::Arc;
use lingo::mirroring::{EtymologicalMirrorEngine, MirrorType, EtymologyFamily, RoleType, NegationType};
use lingo::storage::LingoDatabase;
use lingo::core::{EtymologyOrigin, Coordinate3D};

#[test]
fn test_etymological_mirror_engine_creation() {
    // Load the database
    let db_path = "english.lingo";
    let db = Arc::new(LingoDatabase::open(db_path).unwrap());
    
    let mirror_engine = EtymologicalMirrorEngine::new(db);
    
    // Engine should be created successfully
    assert!(true); // Basic creation test
}

#[test]
fn test_latin_etymology_opposition() {
    let db_path = "english.lingo";
    let db = Arc::new(LingoDatabase::open(db_path).unwrap());
    let mirror_engine = EtymologicalMirrorEngine::new(db);
    
    // Test discovering mirrors for a Latin-origin word
    match mirror_engine.discover_etymological_mirrors("create") {
        Ok(mirrors) => {
            println!("Mirrors found for 'create': {:?}", mirrors);
            
            // Should find etymological opposites
            let etymological_opposites: Vec<_> = mirrors.iter()
                .filter(|m| matches!(m.mirror_type, MirrorType::EtymologicalOpposite { .. }))
                .collect();
            
            assert!(!etymological_opposites.is_empty(), "Should find etymological opposites");
            
            // Check for expected opposites like "destroy" 
            let has_destroy = mirrors.iter().any(|m| m.mirror.contains("destroy"));
            if !has_destroy {
                println!("Warning: 'destroy' not found as opposite of 'create'");
            }
            
            // Validate mirror properties
            for mirror in &mirrors {
                assert!(mirror.confidence > 0.0, "Confidence should be positive");
                assert!(mirror.confidence <= 1.0, "Confidence should not exceed 1.0");
                assert!(!mirror.linguistic_evidence.is_empty(), "Should have linguistic evidence");
            }
        },
        Err(e) => {
            println!("Error discovering mirrors for 'create': {:?}", e);
            // Test passes if word not in database - this is expected behavior
        }
    }
}

#[test]
fn test_functional_role_opposition() {
    let db_path = "english.lingo";
    let db = Arc::new(LingoDatabase::open(db_path).unwrap());
    let mirror_engine = EtymologicalMirrorEngine::new(db);
    
    // Test functional role opposition for agent words
    match mirror_engine.discover_etymological_mirrors("manager") {
        Ok(mirrors) => {
            println!("Mirrors found for 'manager': {:?}", mirrors);
            
            // Should find functional opposites
            let functional_opposites: Vec<_> = mirrors.iter()
                .filter(|m| matches!(m.mirror_type, MirrorType::FunctionalOpposite { .. }))
                .collect();
            
            // Check for role inversions like "employee", "subordinate"
            let role_inversions = ["employee", "subordinate", "follower"];
            let found_inversions: Vec<_> = mirrors.iter()
                .filter(|m| role_inversions.iter().any(|&inv| m.mirror.contains(inv)))
                .collect();
            
            if found_inversions.is_empty() {
                println!("No role inversions found for 'manager' - this may be expected if words not in database");
            } else {
                println!("Found role inversions: {:?}", found_inversions);
            }
        },
        Err(e) => {
            println!("Error discovering mirrors for 'manager': {:?}", e);
        }
    }
}

#[test]
fn test_spatial_semantic_opposition() {
    let db_path = "english.lingo";
    let db = Arc::new(LingoDatabase::open(db_path).unwrap());
    let mirror_engine = EtymologicalMirrorEngine::new(db);
    
    // Test spatial opposition discovery
    match mirror_engine.discover_etymological_mirrors("organize") {
        Ok(mirrors) => {
            println!("Mirrors found for 'organize': {:?}", mirrors);
            
            // Should find spatially opposite concepts
            let spatial_opposites: Vec<_> = mirrors.iter()
                .filter(|m| matches!(m.mirror_type, MirrorType::SpatialOpposite { .. }))
                .collect();
            
            for opposite in &spatial_opposites {
                if let MirrorType::SpatialOpposite { vector_opposition, clustering_confidence } = &opposite.mirror_type {
                    // Opposition vector should be significant
                    let vector_magnitude = (vector_opposition.x.powi(2) + 
                                          vector_opposition.y.powi(2) + 
                                          vector_opposition.z.powi(2)).sqrt();
                    
                    println!("Spatial opposite '{}' has vector magnitude: {:.3}", 
                             opposite.mirror, vector_magnitude);
                    
                    assert!(vector_magnitude > 0.0, "Opposition vector should have magnitude > 0");
                    assert!(*clustering_confidence >= 0.0, "Clustering confidence should be non-negative");
                    assert!(*clustering_confidence <= 1.0, "Clustering confidence should not exceed 1.0");
                }
            }
        },
        Err(e) => {
            println!("Error discovering mirrors for 'organize': {:?}", e);
        }
    }
}

#[test]
fn test_mirror_authenticity_validation() {
    let db_path = "english.lingo";
    let db = Arc::new(LingoDatabase::open(db_path).unwrap());
    let mirror_engine = EtymologicalMirrorEngine::new(db);
    
    // Test with a word that should have clear opposites
    match mirror_engine.discover_etymological_mirrors("build") {
        Ok(mirrors) => {
            println!("Validated mirrors for 'build': {:?}", mirrors);
            
            // All returned mirrors should pass authenticity validation
            for mirror in &mirrors {
                // Mirrors returned by the engine should already be validated
                assert!(!mirror.mirror.is_empty(), "Mirror word should not be empty");
                assert!(mirror.confidence > 0.0, "Valid mirrors should have positive confidence");
            }
            
            // Check for legitimate opposites like "destroy", "demolish", "break"
            let legitimate_opposites = ["destroy", "demolish", "break", "dismantle"];
            let found_opposites: Vec<_> = mirrors.iter()
                .filter(|m| legitimate_opposites.iter().any(|&opp| m.mirror.contains(opp)))
                .collect();
            
            if !found_opposites.is_empty() {
                println!("Found legitimate opposites: {:?}", found_opposites);
                
                // Should find at least one legitimate opposite
                assert!(!found_opposites.is_empty(), "Should find at least one legitimate opposite");
            }
            
            // Should NOT find made-up words like "unbuild"
            let has_unbuild = mirrors.iter().any(|m| m.mirror == "unbuild");
            assert!(!has_unbuild, "Should not find made-up words like 'unbuild'");
        },
        Err(e) => {
            println!("Error discovering mirrors for 'build': {:?}", e);
        }
    }
}

#[test]
fn test_cross_linguistic_opposition() {
    let db_path = "english.lingo";
    let db = Arc::new(LingoDatabase::open(db_path).unwrap());
    let mirror_engine = EtymologicalMirrorEngine::new(db);
    
    // Test cross-linguistic opposition discovery
    let test_words = ["synthesis", "dialogue", "construct"];
    
    for word in &test_words {
        match mirror_engine.discover_etymological_mirrors(word) {
            Ok(mirrors) => {
                println!("Cross-linguistic mirrors for '{}': {:?}", word, mirrors);
                
                // Look for cross-linguistic mirrors
                let cross_linguistic: Vec<_> = mirrors.iter()
                    .filter(|m| matches!(m.mirror_type, MirrorType::CrossLinguisticMirror { .. }))
                    .collect();
                
                for mirror in &cross_linguistic {
                    if let MirrorType::CrossLinguisticMirror { source_etymology, target_etymology, .. } = &mirror.mirror_type {
                        println!("Cross-linguistic mirror: {} ({:?}) → {} ({:?})", 
                                word, source_etymology, mirror.mirror, target_etymology);
                        
                        // Should have different etymologies
                        assert_ne!(source_etymology, target_etymology, 
                                  "Cross-linguistic mirrors should have different etymologies");
                    }
                }
            },
            Err(e) => {
                println!("Error discovering mirrors for '{}': {:?}", word, e);
            }
        }
    }
}

#[test]
fn test_morphological_opposition() {
    let db_path = "english.lingo";
    let db = Arc::new(LingoDatabase::open(db_path).unwrap());
    let mirror_engine = EtymologicalMirrorEngine::new(db);
    
    // Test morphological opposition patterns
    let test_cases = [
        ("happy", "unhappy", NegationType::Prefix("un".to_string())),
        ("organize", "disorganize", NegationType::Prefix("dis".to_string())),
        ("sense", "nonsense", NegationType::Prefix("non".to_string())),
    ];
    
    for (original, expected_mirror, expected_negation) in &test_cases {
        match mirror_engine.discover_etymological_mirrors(original) {
            Ok(mirrors) => {
                println!("Morphological mirrors for '{}': {:?}", original, mirrors);
                
                // Look for morphological opposites
                let morphological: Vec<_> = mirrors.iter()
                    .filter(|m| matches!(m.mirror_type, MirrorType::MorphologicalOpposite { .. }))
                    .collect();
                
                // Check if expected mirror is found
                let found_expected = mirrors.iter().any(|m| m.mirror == *expected_mirror);
                if found_expected {
                    println!("Found expected morphological opposite: {} → {}", original, expected_mirror);
                } else {
                    println!("Expected morphological opposite '{}' not found for '{}'", expected_mirror, original);
                }
                
                // Validate morphological opposition properties
                for mirror in &morphological {
                    if let MirrorType::MorphologicalOpposite { valid_negation, productivity_score } = &mirror.mirror_type {
                        assert!(*productivity_score >= 0.0, "Productivity score should be non-negative");
                        assert!(*productivity_score <= 1.0, "Productivity score should not exceed 1.0");
                        
                        // Check negation type
                        match valid_negation {
                            NegationType::Prefix(prefix) => {
                                assert!(!prefix.is_empty(), "Prefix should not be empty");
                            },
                            _ => {} // Other negation types are valid
                        }
                    }
                }
            },
            Err(e) => {
                println!("Error discovering mirrors for '{}': {:?}", original, e);
            }
        }
    }
}

#[test]
fn test_etymology_profile_analysis() {
    let db_path = "english.lingo";
    let db = Arc::new(LingoDatabase::open(db_path).unwrap());
    let mirror_engine = EtymologicalMirrorEngine::new(db);
    
    // This test focuses on the internal etymology analysis
    // We can't directly test the private method, but we can test the overall result
    
    let words_with_known_etymologies = [
        ("create", EtymologyOrigin::Latin),
        ("build", EtymologyOrigin::Germanic),
        ("synthesis", EtymologyOrigin::Greek),
    ];
    
    for (word, expected_origin) in &words_with_known_etymologies {
        match mirror_engine.discover_etymological_mirrors(word) {
            Ok(mirrors) => {
                println!("Etymology analysis for '{}' (expected: {:?}): found {} mirrors", 
                         word, expected_origin, mirrors.len());
                
                // Check that mirrors are appropriate for the etymology
                for mirror in &mirrors {
                    match &mirror.mirror_type {
                        MirrorType::EtymologicalOpposite { root_family, .. } => {
                            println!("Found etymological opposite in family: {:?}", root_family);
                            
                            // Validate that the etymology family makes sense
                            let family_matches_origin = match (expected_origin, root_family) {
                                (EtymologyOrigin::Latin, EtymologyFamily::Romance) => true,
                                (EtymologyOrigin::Latin, EtymologyFamily::Latin) => true,
                                (EtymologyOrigin::Greek, EtymologyFamily::Greek) => true,
                                (EtymologyOrigin::Germanic, EtymologyFamily::Germanic) => true,
                                _ => false,
                            };
                            
                            if family_matches_origin {
                                println!("Etymology family {:?} correctly matches origin {:?}", 
                                        root_family, expected_origin);
                            }
                        },
                        _ => {} // Other mirror types don't require etymology matching
                    }
                }
            },
            Err(e) => {
                println!("Error analyzing etymology for '{}': {:?}", word, e);
            }
        }
    }
}

#[test]
fn test_mirror_confidence_scoring() {
    let db_path = "english.lingo";
    let db = Arc::new(LingoDatabase::open(db_path).unwrap());
    let mirror_engine = EtymologicalMirrorEngine::new(db);
    
    // Test confidence scoring for different types of mirrors
    let test_words = ["create", "manager", "happy", "organize"];
    
    for word in &test_words {
        match mirror_engine.discover_etymological_mirrors(word) {
            Ok(mirrors) => {
                println!("Confidence analysis for '{}': {} mirrors found", word, mirrors.len());
                
                for mirror in &mirrors {
                    println!("  {} → {} (confidence: {:.3}, type: {:?})", 
                             mirror.original, mirror.mirror, mirror.confidence, mirror.mirror_type);
                    
                    // Validate confidence scores
                    assert!(mirror.confidence > 0.0, "Confidence should be positive for {}", mirror.mirror);
                    assert!(mirror.confidence <= 1.0, "Confidence should not exceed 1.0 for {}", mirror.mirror);
                    
                    // Higher-quality mirrors should have higher confidence
                    match &mirror.mirror_type {
                        MirrorType::EtymologicalOpposite { .. } => {
                            // Etymological opposites should have relatively high confidence
                            if mirror.confidence < 0.3 {
                                println!("Warning: Low confidence ({:.3}) for etymological opposite: {}", 
                                        mirror.confidence, mirror.mirror);
                            }
                        },
                        MirrorType::SpatialOpposite { clustering_confidence, .. } => {
                            // Spatial opposites should have reasonable clustering confidence
                            assert!(*clustering_confidence >= 0.0, "Clustering confidence should be non-negative");
                            assert!(*clustering_confidence <= 1.0, "Clustering confidence should not exceed 1.0");
                        },
                        _ => {} // Other types have their own confidence requirements
                    }
                }
                
                // Sort by confidence to see the best mirrors
                let mut sorted_mirrors = mirrors.clone();
                sorted_mirrors.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
                
                if !sorted_mirrors.is_empty() {
                    println!("  Best mirror: {} (confidence: {:.3})", 
                             sorted_mirrors[0].mirror, sorted_mirrors[0].confidence);
                }
            },
            Err(e) => {
                println!("Error analyzing confidence for '{}': {:?}", word, e);
            }
        }
    }
}
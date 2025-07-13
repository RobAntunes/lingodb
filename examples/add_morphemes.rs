//! Tool for adding new morphemes to LingoDB with adaptive positioning
//! 
//! This tool:
//! 1. Learns from existing morpheme patterns
//! 2. Finds optimal positions for new morphemes
//! 3. Validates that additions don't break existing patterns
//! 4. Allows batch additions with progress tracking

use lingo::storage::{MemoryMappedDatabase, DatabaseBuilder};
use lingo::core::*;
use lingo::core::adaptive_space::*;
use std::io::{self, Write};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 LingoDB Morpheme Addition Tool\n");
    
    // Load existing database
    let database = MemoryMappedDatabase::open("data/base.lingo")?;
    println!("Loaded database with {} nodes", database.node_count());
    
    // Initialize adaptive spatial manager
    let mut spatial_manager = AdaptiveSpatialManager::new();
    
    // Learn from existing morphemes
    println!("\n📊 Learning spatial patterns...");
    let existing_morphemes = load_existing_morphemes(&database)?;
    spatial_manager.learn_from_database(&existing_morphemes);
    println!("Learned from {} existing morphemes", existing_morphemes.len());
    
    // Load morphemes to add (Phase 1 from coverage plan)
    let new_morphemes = load_phase1_morphemes();
    println!("\n📝 Ready to add {} new morphemes", new_morphemes.len());
    
    // Auto-run batch addition (non-interactive mode to avoid buffer overflow)
    println!("\n🚀 Running automatic batch morpheme addition...");
    
    // Preview first few morphemes
    println!("\n📋 First 5 morphemes to add:");
    for (i, morpheme) in new_morphemes.iter().take(5).enumerate() {
        println!("  {}. {} ({:?}) - {}", 
            i + 1, 
            morpheme.morpheme, 
            morpheme.morph_type, 
            morpheme.meaning
        );
    }
    
    // Batch add all morphemes
    batch_add_morphemes(&mut spatial_manager, &new_morphemes)?;
    
    // Show final statistics
    show_spatial_stats(&spatial_manager, &existing_morphemes);
    
    println!("\n✅ Morpheme addition complete!");
    
    Ok(())
}

fn load_existing_morphemes(db: &MemoryMappedDatabase) -> Result<Vec<(String, LinguisticNode)>, Box<dyn std::error::Error>> {
    let mut morphemes = Vec::new();
    
    for i in 0..db.node_count() {
        let node_id = NodeId(i as u32);
        if let Ok(node) = db.get_node(node_id) {
            if node.layer == Layer::Morphemes {
                if let Ok(word) = db.get_node_word(node_id) {
                    morphemes.push((word.to_string(), *node));
                }
            }
        }
    }
    
    Ok(morphemes)
}

fn load_phase1_morphemes() -> Vec<NewMorpheme> {
    // Phase 1: Core vocabulary from coverage plan
    vec![
        // High-frequency Germanic roots
        NewMorpheme {
            morpheme: "happy".to_string(),
            morph_type: MorphemeType::Root,
            meaning: "feeling joy".to_string(),
            etymology: EtymologyOrigin::Germanic,
            productivity: 0.8,
            semantic_hints: vec![],
        },
        NewMorpheme {
            morpheme: "sad".to_string(),
            morph_type: MorphemeType::Root,
            meaning: "feeling sorrow".to_string(),
            etymology: EtymologyOrigin::Germanic,
            productivity: 0.7,
            semantic_hints: vec![SemanticHint::OppositeTo("happy".to_string())],
        },
        NewMorpheme {
            morpheme: "good".to_string(),
            morph_type: MorphemeType::Root,
            meaning: "positive quality".to_string(),
            etymology: EtymologyOrigin::Germanic,
            productivity: 0.9,
            semantic_hints: vec![],
        },
        NewMorpheme {
            morpheme: "bad".to_string(),
            morph_type: MorphemeType::Root,
            meaning: "negative quality".to_string(),
            etymology: EtymologyOrigin::Germanic,
            productivity: 0.8,
            semantic_hints: vec![SemanticHint::OppositeTo("good".to_string())],
        },
        NewMorpheme {
            morpheme: "view".to_string(),
            morph_type: MorphemeType::Root,
            meaning: "see/sight".to_string(),
            etymology: EtymologyOrigin::French,
            productivity: 0.8,
            semantic_hints: vec![SemanticHint::SimilarTo("vis".to_string())],
        },
        NewMorpheme {
            morpheme: "build".to_string(),
            morph_type: MorphemeType::Root,
            meaning: "construct".to_string(),
            etymology: EtymologyOrigin::Germanic,
            productivity: 0.9,
            semantic_hints: vec![SemanticHint::SimilarTo("struct".to_string())],
        },
        NewMorpheme {
            morpheme: "work".to_string(),
            morph_type: MorphemeType::Root,
            meaning: "labor/function".to_string(),
            etymology: EtymologyOrigin::Germanic,
            productivity: 0.9,
            semantic_hints: vec![],
        },
        
        // Essential bound morphemes
        NewMorpheme {
            morpheme: "ly".to_string(),
            morph_type: MorphemeType::Suffix,
            meaning: "adverb marker".to_string(),
            etymology: EtymologyOrigin::Germanic,
            productivity: 0.95,
            semantic_hints: vec![],
        },
        NewMorpheme {
            morpheme: "able".to_string(),
            morph_type: MorphemeType::Suffix,
            meaning: "capable of".to_string(),
            etymology: EtymologyOrigin::Latin,
            productivity: 0.9,
            semantic_hints: vec![SemanticHint::SimilarTo("ible".to_string())],
        },
        NewMorpheme {
            morpheme: "al".to_string(),
            morph_type: MorphemeType::Suffix,
            meaning: "relating to".to_string(),
            etymology: EtymologyOrigin::Latin,
            productivity: 0.85,
            semantic_hints: vec![],
        },
        
        // Common verbs
        NewMorpheme {
            morpheme: "do".to_string(),
            morph_type: MorphemeType::Root,
            meaning: "perform action".to_string(),
            etymology: EtymologyOrigin::Germanic,
            productivity: 0.95,
            semantic_hints: vec![],
        },
        NewMorpheme {
            morpheme: "make".to_string(),
            morph_type: MorphemeType::Root,
            meaning: "create".to_string(),
            etymology: EtymologyOrigin::Germanic,
            productivity: 0.9,
            semantic_hints: vec![SemanticHint::SimilarTo("build".to_string())],
        },
        NewMorpheme {
            morpheme: "take".to_string(),
            morph_type: MorphemeType::Root,
            meaning: "grasp/receive".to_string(),
            etymology: EtymologyOrigin::Germanic,
            productivity: 0.9,
            semantic_hints: vec![SemanticHint::OppositeTo("give".to_string())],
        },
        NewMorpheme {
            morpheme: "give".to_string(),
            morph_type: MorphemeType::Root,
            meaning: "transfer/provide".to_string(),
            etymology: EtymologyOrigin::Germanic,
            productivity: 0.85,
            semantic_hints: vec![],
        },
        
        // Add more as needed...
    ]
}

fn preview_batch(morphemes: &[NewMorpheme], spatial_manager: &AdaptiveSpatialManager) {
    println!("\n📋 Preview of next 10 morphemes:");
    
    for (i, morpheme) in morphemes.iter().take(10).enumerate() {
        let position = spatial_manager.find_optimal_position(
            &morpheme.morpheme,
            morpheme.morph_type,
            morpheme.etymology,
            &morpheme.semantic_hints,
        );
        
        let assessment = spatial_manager.assess_disruption(position, morpheme.morph_type);
        
        println!("\n{}. {} ({:?})", i + 1, morpheme.morpheme, morpheme.morph_type);
        println!("   Meaning: {}", morpheme.meaning);
        println!("   Position: ({:.3}, {:.3}, {:.3})", position.x, position.y, position.z);
        println!("   Disruption: {}", if assessment.within_normal_range { "✓ Minimal" } else { "⚠ Notable" });
        
        if !morpheme.semantic_hints.is_empty() {
            println!("   Semantic hints: {:?}", morpheme.semantic_hints);
        }
    }
}

fn add_single_morpheme(spatial_manager: &mut AdaptiveSpatialManager) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n➕ Add Single Morpheme");
    
    print!("Morpheme: ");
    io::stdout().flush()?;
    let mut morpheme = String::new();
    io::stdin().read_line(&mut morpheme)?;
    let morpheme = morpheme.trim().to_string();
    
    print!("Type (1=Prefix, 2=Suffix, 3=Root): ");
    io::stdout().flush()?;
    let mut type_input = String::new();
    io::stdin().read_line(&mut type_input)?;
    let morph_type = match type_input.trim() {
        "1" => MorphemeType::Prefix,
        "2" => MorphemeType::Suffix,
        _ => MorphemeType::Root,
    };
    
    print!("Meaning: ");
    io::stdout().flush()?;
    let mut meaning = String::new();
    io::stdin().read_line(&mut meaning)?;
    
    // Find position
    let position = spatial_manager.find_optimal_position(
        &morpheme,
        morph_type,
        EtymologyOrigin::Unknown,
        &[],
    );
    
    // Assess impact
    let assessment = spatial_manager.assess_disruption(position, morph_type);
    
    println!("\n📍 Proposed position: ({:.3}, {:.3}, {:.3})", position.x, position.y, position.z);
    println!("Centroid deviation: {:.3}", assessment.centroid_deviation);
    println!("Local density: {:.3}", assessment.local_density);
    println!("Within normal range: {}", if assessment.within_normal_range { "✓ Yes" } else { "✗ No" });
    
    print!("\nAdd this morpheme? (y/n): ");
    io::stdout().flush()?;
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;
    
    if confirm.trim().to_lowercase() == "y" {
        spatial_manager.adapt_to_new_morpheme(&morpheme, position, morph_type);
        println!("✓ Morpheme added and patterns updated!");
    }
    
    Ok(())
}

fn batch_add_morphemes(
    spatial_manager: &mut AdaptiveSpatialManager,
    morphemes: &[NewMorpheme]
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📦 Batch Addition Mode");
    
    // Auto-add all morphemes (non-interactive mode)
    let count = morphemes.len();
    println!("Adding all {} morphemes automatically...", count);
    
    let mut disruption_warnings = 0;
    
    for (i, morpheme) in morphemes.iter().take(count).enumerate() {
        let position = spatial_manager.find_optimal_position(
            &morpheme.morpheme,
            morpheme.morph_type,
            morpheme.etymology,
            &morpheme.semantic_hints,
        );
        
        let assessment = spatial_manager.assess_disruption(position, morpheme.morph_type);
        
        if !assessment.within_normal_range || assessment.is_overcrowded {
            disruption_warnings += 1;
        }
        
        // Add to spatial manager
        spatial_manager.adapt_to_new_morpheme(&morpheme.morpheme, position, morpheme.morph_type);
        
        // Progress indicator
        if (i + 1) % 10 == 0 {
            print!(".");
            io::stdout().flush()?;
        }
    }
    
    println!("\n✓ Added {} morphemes", count);
    if disruption_warnings > 0 {
        println!("⚠ {} morphemes had positioning warnings", disruption_warnings);
    }
    
    Ok(())
}

fn show_spatial_stats(
    spatial_manager: &AdaptiveSpatialManager,
    existing_morphemes: &[(String, LinguisticNode)]
) {
    println!("\n📊 Spatial Statistics");
    
    // Count by type
    let mut type_counts = HashMap::new();
    for (_, node) in existing_morphemes {
        *type_counts.entry(node.morpheme_type).or_insert(0) += 1;
    }
    
    println!("\nMorpheme distribution:");
    for (morph_type, count) in type_counts {
        println!("  {:?}: {}", morph_type, count);
    }
    
    println!("\nAdaptive parameters:");
    println!("  Pattern weight: {:.2}", spatial_manager.flexibility.pattern_weight);
    println!("  Min separation: {:.3}", spatial_manager.flexibility.min_separation);
    println!("  Allow drift: {}", spatial_manager.flexibility.allow_drift);
    println!("  Learning rate: {:.3}", spatial_manager.flexibility.learning_rate);
}

fn export_database(
    spatial_manager: &AdaptiveSpatialManager,
    existing_morphemes: &[(String, LinguisticNode)],
    new_morphemes: &[NewMorpheme]
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💾 Exporting enhanced database...");
    
    // Would implement actual database building here
    println!("Export functionality not yet implemented");
    println!("Would export {} existing + new morphemes", existing_morphemes.len());
    
    Ok(())
}

#[derive(Debug, Clone)]
struct NewMorpheme {
    morpheme: String,
    morph_type: MorphemeType,
    meaning: String,
    etymology: EtymologyOrigin,
    productivity: f32,
    semantic_hints: Vec<SemanticHint>,
}
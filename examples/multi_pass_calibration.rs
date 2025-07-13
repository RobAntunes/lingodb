//! Multi-pass spatial calibration - runs multiple calibration rounds
//! This approach runs several shorter calibration passes to gradually improve

use lingo::storage::MemoryMappedDatabase;
use lingo::core::*;
use lingo::core::adaptive_space::*;
use lingo::data::data_integration::*;
use lingo::data::english_base::MorphemeData;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Multi-Pass Spatial Calibration");
    println!("=================================");
    
    // Load the database and set up spatial manager
    let database = MemoryMappedDatabase::open("data/base.lingo")?;
    println!("📚 Loaded database with {} nodes", database.node_count());
    
    let mut spatial_manager = AdaptiveSpatialManager::new();
    let existing_morphemes = load_existing_morphemes(&database)?;
    spatial_manager.learn_from_database(&existing_morphemes);
    
    // Bulk upload morphemes
    println!("\n📦 Bulk uploading morphemes...");
    let total_uploaded = perform_bulk_upload(&mut spatial_manager)?;
    
    let initial_disruption = spatial_manager.calculate_global_disruption_score();
    println!("📊 Initial disruption: {:.2}%", initial_disruption * 100.0);
    
    // Run multiple calibration passes
    let mut current_disruption = initial_disruption;
    let passes = 5;
    let iterations_per_pass = 100;
    
    println!("\n🔧 Running {} calibration passes ({} iterations each)...", passes, iterations_per_pass);
    
    for pass in 1..=passes {
        println!("\n--- Pass {} ---", pass);
        let before = spatial_manager.calculate_global_disruption_score();
        
        let results = spatial_manager.calibrate_spatial_layout(iterations_per_pass);
        let after = spatial_manager.calculate_global_disruption_score();
        
        let improvement = (before - after) / before * 100.0;
        println!("Pass {} results:", pass);
        println!("  Before: {:.2}%, After: {:.2}%", before * 100.0, after * 100.0);
        println!("  Improvement: {:.1}%", improvement);
        println!("  Morphemes repositioned: {}", results.morphemes_repositioned);
        
        current_disruption = after;
        
        // Stop if improvement is minimal
        if improvement < 1.0 {
            println!("🎯 Convergence achieved - minimal improvement in pass {}", pass);
            break;
        }
    }
    
    // Final summary
    let total_improvement = (initial_disruption - current_disruption) / initial_disruption * 100.0;
    println!("\n📊 Final Summary:");
    println!("===============");
    println!("Initial disruption: {:.2}%", initial_disruption * 100.0);
    println!("Final disruption: {:.2}%", current_disruption * 100.0); 
    println!("Total improvement: {:.1}%", total_improvement);
    println!("Morphemes remaining problematic: {:.0}", current_disruption * total_uploaded as f32);
    
    Ok(())
}

// ... (include the same helper functions from spatial_calibration.rs)

fn perform_bulk_upload(spatial_manager: &mut AdaptiveSpatialManager) -> Result<usize, Box<dyn std::error::Error>> {
    let all_prefixes = get_all_prefixes();
    let all_suffixes = get_all_suffixes(); 
    let all_roots = get_all_roots();
    
    let mut total_uploaded = 0;
    total_uploaded += bulk_add_morphemes(spatial_manager, &all_prefixes, MorphemeType::Prefix)?;
    total_uploaded += bulk_add_morphemes(spatial_manager, &all_suffixes, MorphemeType::Suffix)?;
    total_uploaded += bulk_add_morphemes(spatial_manager, &all_roots, MorphemeType::Root)?;
    
    Ok(total_uploaded)
}

fn bulk_add_morphemes(
    spatial_manager: &mut AdaptiveSpatialManager,
    morphemes: &[&MorphemeData],
    morph_type: MorphemeType
) -> Result<usize, Box<dyn std::error::Error>> {
    
    for morpheme_data in morphemes {
        let position = spatial_manager.find_optimal_position(
            morpheme_data.morpheme,
            morph_type,
            morpheme_data.etymology,
            &vec![],
        );
        spatial_manager.adapt_to_new_morpheme(morpheme_data.morpheme, position, morph_type);
    }
    
    Ok(morphemes.len())
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
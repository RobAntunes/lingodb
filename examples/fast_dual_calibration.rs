//! Fast dual approach - optimized for 2-minute timeout
//! Combines multiple short passes with reduced search space

use lingo::storage::MemoryMappedDatabase;
use lingo::core::*;
use lingo::core::adaptive_space::*;
use lingo::data::data_integration::*;
use lingo::data::english_base::MorphemeData;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Fast Dual-Approach Calibration");
    println!("=================================");
    println!("Multiple passes + optimized algorithm for 2-min timeout");
    
    // Load and setup
    let database = MemoryMappedDatabase::open("data/base.lingo")?;
    println!("📚 Loaded database with {} nodes", database.node_count());
    
    let mut spatial_manager = AdaptiveSpatialManager::new();
    let existing_morphemes = load_existing_morphemes(&database)?;
    spatial_manager.learn_from_database(&existing_morphemes);
    
    // Bulk upload
    println!("\n📦 Bulk uploading morphemes...");
    let total_uploaded = perform_bulk_upload(&mut spatial_manager)?;
    
    let initial_disruption = spatial_manager.calculate_global_disruption_score();
    println!("📊 Initial disruption: {:.2}% ({:.0} morphemes)", 
        initial_disruption * 100.0, initial_disruption * total_uploaded as f32);
    
    // Fast multi-pass calibration (designed for 2-min timeout)
    let passes = 8;          // More passes
    let iterations_per_pass = 25;  // Fewer iterations per pass
    
    println!("\n🔧 Running {} fast calibration passes ({} iterations each)...", passes, iterations_per_pass);
    println!("⏱️ Optimized for 2-minute timeout");
    
    let mut current_disruption = initial_disruption;
    let mut total_repositioned = 0;
    let mut converged_passes = 0;
    
    for pass in 1..=passes {
        let before = spatial_manager.calculate_global_disruption_score();
        
        print!("Pass {}: ", pass);
        let results = spatial_manager.calibrate_spatial_layout(iterations_per_pass);
        let after = spatial_manager.calculate_global_disruption_score();
        
        let improvement = (before - after) / before * 100.0;
        total_repositioned += results.morphemes_repositioned;
        
        println!("{:.2}% → {:.2}% ({:.1}% improvement, {} repositioned)", 
            before * 100.0, after * 100.0, improvement, results.morphemes_repositioned);
        
        current_disruption = after;
        
        // Track convergence
        if improvement < 2.0 {
            converged_passes += 1;
            if converged_passes >= 2 {
                println!("🎯 Convergence detected after {} passes", pass);
                break;
            }
        } else {
            converged_passes = 0;  // Reset convergence counter
        }
    }
    
    // Final summary
    let total_improvement = (initial_disruption - current_disruption) / initial_disruption * 100.0;
    println!("\n📊 Final Dual-Approach Results:");
    println!("==============================");
    println!("Initial disruption: {:.2}% ({:.0} morphemes)", 
        initial_disruption * 100.0, initial_disruption * total_uploaded as f32);
    println!("Final disruption: {:.2}% ({:.0} morphemes)", 
        current_disruption * 100.0, current_disruption * total_uploaded as f32);
    println!("Total improvement: {:.1}% reduction", total_improvement);
    println!("Total repositioned: {} morphemes", total_repositioned);
    
    // Quality assessment
    let quality_score = if total_improvement > 50.0 {
        "🌟 Outstanding"
    } else if total_improvement > 40.0 {
        "🎉 Excellent" 
    } else if total_improvement > 25.0 {
        "✅ Good"
    } else {
        "⚠️ Modest"
    };
    
    println!("Quality assessment: {}", quality_score);
    
    // Quick spatial metrics
    println!("\n🔍 Spatial Quality (sampled):");
    let avg_separation = calculate_sampled_separation(&spatial_manager);
    println!("  Average separation: {:.3}", avg_separation);
    
    println!("\n💡 Dual approach complete! This combines:");
    println!("   • Multiple calibration passes for incremental improvement");
    println!("   • Optimized algorithm for faster convergence");
    println!("   • Progress tracking to show results within timeout");
    
    Ok(())
}

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

fn calculate_sampled_separation(spatial_manager: &AdaptiveSpatialManager) -> f32 {
    let positions: Vec<_> = spatial_manager.position_history.iter()
        .map(|(_, pos, _)| *pos)
        .collect();
    
    if positions.len() < 2 { return 0.0; }
    
    let mut total_distance = 0.0;
    let mut count = 0;
    
    // Sample only 500 pairs for speed
    let sample_size = 500.min(positions.len());
    let step = positions.len() / sample_size;
    
    for i in (0..positions.len()).step_by(step.max(1)).take(sample_size / 2) {
        for j in ((i+step)..positions.len()).step_by(step.max(1)).take(sample_size / 2) {
            let dist = ((positions[i].x - positions[j].x).powi(2) + 
                       (positions[i].y - positions[j].y).powi(2) + 
                       (positions[i].z - positions[j].z).powi(2)).sqrt();
            total_distance += dist;
            count += 1;
            if count >= 100 { break; }  // Limit to 100 calculations for speed
        }
        if count >= 100 { break; }
    }
    
    if count > 0 { total_distance / count as f32 } else { 0.0 }
}
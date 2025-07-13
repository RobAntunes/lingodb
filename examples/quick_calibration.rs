//! Quick calibration with optimized algorithm
//! Reduces search space for faster convergence on large datasets

use lingo::storage::MemoryMappedDatabase;
use lingo::core::*;
use lingo::core::adaptive_space::*;
use lingo::data::data_integration::*;
use lingo::data::english_base::MorphemeData;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Quick Spatial Calibration (Optimized)");
    println!("=======================================");
    
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
    println!("📊 Initial disruption: {:.2}% ({:.0} morphemes)", 
        initial_disruption * 100.0, initial_disruption * total_uploaded as f32);
    
    // Quick calibration - fewer iterations but optimized  
    println!("\n⚡ Running optimized calibration (50 iterations)...");
    let results = spatial_manager.calibrate_spatial_layout(50);
    
    let final_disruption = spatial_manager.calculate_global_disruption_score();
    let improvement = (initial_disruption - final_disruption) / initial_disruption * 100.0;
    
    println!("\n📊 Quick Calibration Results:");
    println!("============================");
    println!("Initial disruption: {:.2}% ({:.0} morphemes)", 
        initial_disruption * 100.0, initial_disruption * total_uploaded as f32);
    println!("Final disruption: {:.2}% ({:.0} morphemes)", 
        final_disruption * 100.0, final_disruption * total_uploaded as f32);
    println!("Improvement: {:.1}% reduction", improvement);
    println!("Morphemes repositioned: {}", results.morphemes_repositioned);
    println!("Convergence: {}", if results.convergence_achieved { "✅ Yes" } else { "❌ No" });
    
    // Quality metrics
    println!("\n🔍 Spatial Quality:");
    let avg_separation = calculate_average_separation(&spatial_manager);
    println!("  Average separation: {:.3}", avg_separation);
    
    let clustering_quality = calculate_clustering_quality(&spatial_manager);
    println!("  Type clustering: {:.2}", clustering_quality);
    
    if improvement > 30.0 {
        println!("\n🎉 Excellent improvement! Calibration very effective.");
    } else if improvement > 15.0 {
        println!("\n✅ Good improvement! Consider more iterations for better results.");
    } else {
        println!("\n⚠️ Modest improvement. May need different optimization strategy.");
    }
    
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

fn calculate_average_separation(spatial_manager: &AdaptiveSpatialManager) -> f32 {
    let positions: Vec<_> = spatial_manager.position_history.iter()
        .map(|(_, pos, _)| *pos)
        .collect();
    
    if positions.len() < 2 { return 0.0; }
    
    let mut total_distance = 0.0;
    let mut count = 0;
    
    // Sample subset for performance on large datasets
    let sample_size = positions.len().min(1000);
    let step = positions.len() / sample_size;
    
    for i in (0..positions.len()).step_by(step.max(1)) {
        for j in ((i+step)..positions.len()).step_by(step.max(1)) {
            let dist = ((positions[i].x - positions[j].x).powi(2) + 
                       (positions[i].y - positions[j].y).powi(2) + 
                       (positions[i].z - positions[j].z).powi(2)).sqrt();
            total_distance += dist;
            count += 1;
        }
    }
    
    if count > 0 { total_distance / count as f32 } else { 0.0 }
}

fn calculate_clustering_quality(spatial_manager: &AdaptiveSpatialManager) -> f32 {
    let mut type_groups: std::collections::HashMap<MorphemeType, Vec<Coordinate3D>> = std::collections::HashMap::new();
    
    for (_, pos, morph_type) in &spatial_manager.position_history {
        type_groups.entry(*morph_type).or_insert_with(Vec::new).push(*pos);
    }
    
    let mut quality_sum = 0.0;
    let mut type_count = 0;
    
    for (_, positions) in type_groups {
        if positions.len() > 1 {
            // Sample for performance
            let sample_size = positions.len().min(100);
            let step = positions.len() / sample_size;
            
            let mut total_dist = 0.0;
            let mut pair_count = 0;
            
            for i in (0..positions.len()).step_by(step.max(1)) {
                for j in ((i+step)..positions.len()).step_by(step.max(1)) {
                    let dist = ((positions[i].x - positions[j].x).powi(2) + 
                               (positions[i].y - positions[j].y).powi(2) + 
                               (positions[i].z - positions[j].z).powi(2)).sqrt();
                    total_dist += dist;
                    pair_count += 1;
                }
            }
            
            let avg_dist = if pair_count > 0 { total_dist / pair_count as f32 } else { 1.0 };
            quality_sum += 1.0 - avg_dist.min(1.0);
            type_count += 1;
        }
    }
    
    if type_count > 0 { quality_sum / type_count as f32 } else { 0.0 }
}
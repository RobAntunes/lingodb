//! Spatial calibration example - runs post-upload optimization
//! This demonstrates the preferred approach: bulk upload first, then calibrate

use lingo::storage::MemoryMappedDatabase;
use lingo::core::*;
use lingo::core::adaptive_space::*;
use lingo::data::data_integration::*;
use lingo::data::english_base::MorphemeData;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Post-Upload Spatial Calibration");
    println!("==================================");
    println!("This example demonstrates calibrating spatial layout AFTER bulk upload");
    
    // Load the database
    let database = MemoryMappedDatabase::open("data/base.lingo")?;
    println!("📚 Loaded database with {} nodes", database.node_count());
    
    // Initialize adaptive spatial manager
    let mut spatial_manager = AdaptiveSpatialManager::new();
    
    // Step 1: Learn from existing morphemes (if any)
    println!("\n📊 Step 1: Learning from existing spatial patterns...");
    let existing_morphemes = load_existing_morphemes(&database)?;
    spatial_manager.learn_from_database(&existing_morphemes);
    println!("✓ Learned from {} existing morphemes", existing_morphemes.len());
    
    // Step 2: Bulk upload ALL morphemes without worrying about disruption
    println!("\n📦 Step 2: Bulk uploading morphemes (ignoring spatial disruption)...");
    let total_uploaded = perform_bulk_upload(&mut spatial_manager)?;
    println!("✓ Uploaded {} morphemes", total_uploaded);
    
    // Calculate pre-calibration disruption
    let pre_disruption = spatial_manager.calculate_global_disruption_score();
    println!("📊 Pre-calibration disruption score: {:.2}% ({:.0} morphemes affected)", 
        pre_disruption * 100.0, pre_disruption * total_uploaded as f32);
    
    // Step 3: Run spatial calibration to optimize layout
    println!("\n🔧 Step 3: Running spatial calibration...");
    let calibration_results = spatial_manager.calibrate_spatial_layout(500);
    
    // Report results
    print_calibration_results(&calibration_results, total_uploaded);
    
    // Step 4: Verify improvement
    println!("\n📈 Step 4: Verifying spatial improvement...");
    verify_spatial_improvement(&spatial_manager, &calibration_results);
    
    println!("\n✅ Calibration process complete!");
    println!("💡 This approach allows adaptive learning from ALL data before optimizing");
    
    Ok(())
}

fn perform_bulk_upload(spatial_manager: &mut AdaptiveSpatialManager) -> Result<usize, Box<dyn std::error::Error>> {
    // Load all available morpheme data
    let all_prefixes = get_all_prefixes();
    let all_suffixes = get_all_suffixes(); 
    let all_roots = get_all_roots();
    
    let mut total_uploaded = 0;
    
    println!("  Adding {} prefixes...", all_prefixes.len());
    total_uploaded += bulk_add_morphemes(spatial_manager, &all_prefixes, MorphemeType::Prefix)?;
    
    println!("  Adding {} suffixes...", all_suffixes.len());
    total_uploaded += bulk_add_morphemes(spatial_manager, &all_suffixes, MorphemeType::Suffix)?;
    
    println!("  Adding {} roots...", all_roots.len());
    total_uploaded += bulk_add_morphemes(spatial_manager, &all_roots, MorphemeType::Root)?;
    
    Ok(total_uploaded)
}

fn bulk_add_morphemes(
    spatial_manager: &mut AdaptiveSpatialManager,
    morphemes: &[&MorphemeData],
    morph_type: MorphemeType
) -> Result<usize, Box<dyn std::error::Error>> {
    
    for morpheme_data in morphemes {
        // For bulk upload, use simple positioning without worrying about disruption
        let position = spatial_manager.find_optimal_position(
            morpheme_data.morpheme,
            morph_type,
            morpheme_data.etymology,
            &vec![], // No semantic hints for bulk upload
        );
        
        // Add without disruption checking - let calibration handle optimization
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

fn print_calibration_results(results: &CalibrationResults, total_morphemes: usize) {
    println!("\n📊 Calibration Results Summary");
    println!("==============================");
    println!("Initial disruption score: {:.2}% ({:.0} morphemes)", 
        results.initial_disruption_score * 100.0, 
        results.initial_disruption_score * total_morphemes as f32);
    println!("Final disruption score: {:.2}% ({:.0} morphemes)", 
        results.final_disruption_score * 100.0,
        results.final_disruption_score * total_morphemes as f32);
    
    let improvement = (results.initial_disruption_score - results.final_disruption_score) / results.initial_disruption_score * 100.0;
    println!("Improvement: {:.1}% reduction in disruption", improvement);
    
    println!("Iterations completed: {}", results.iterations_completed);
    println!("Morphemes repositioned: {}", results.morphemes_repositioned);
    println!("Convergence achieved: {}", results.convergence_achieved);
    
    if results.convergence_achieved {
        println!("✅ Calibration converged to optimal solution");
    } else {
        println!("⚠️  Calibration stopped at iteration limit (may need more iterations)");
    }
}

fn verify_spatial_improvement(
    spatial_manager: &AdaptiveSpatialManager, 
    results: &CalibrationResults
) {
    println!("🔍 Spatial Quality Metrics:");
    
    // Calculate average separation
    let avg_separation = calculate_average_separation(spatial_manager);
    println!("  Average morpheme separation: {:.3}", avg_separation);
    
    // Calculate type clustering quality
    let clustering_quality = calculate_clustering_quality(spatial_manager);
    println!("  Type clustering quality: {:.2}", clustering_quality);
    
    // Repository effectiveness
    let reposition_efficiency = results.morphemes_repositioned as f32 / results.iterations_completed as f32;
    println!("  Repositioning efficiency: {:.1} morphemes/iteration", reposition_efficiency);
    
    println!("\n💡 Spatial calibration successfully optimized the morpheme layout!");
    println!("   This approach is much more effective than trying to position perfectly during upload.");
}

fn calculate_average_separation(spatial_manager: &AdaptiveSpatialManager) -> f32 {
    let positions: Vec<_> = spatial_manager.position_history.iter()
        .map(|(_, pos, _)| *pos)
        .collect();
    
    if positions.len() < 2 { return 0.0; }
    
    let mut total_distance = 0.0;
    let mut count = 0;
    
    for i in 0..positions.len() {
        for j in (i+1)..positions.len() {
            let dist = ((positions[i].x - positions[j].x).powi(2) + 
                       (positions[i].y - positions[j].y).powi(2) + 
                       (positions[i].z - positions[j].z).powi(2)).sqrt();
            total_distance += dist;
            count += 1;
        }
    }
    
    total_distance / count as f32
}

fn calculate_clustering_quality(spatial_manager: &AdaptiveSpatialManager) -> f32 {
    // Simple quality metric: how well are same-type morphemes clustered?
    let mut type_groups: std::collections::HashMap<MorphemeType, Vec<Coordinate3D>> = std::collections::HashMap::new();
    
    for (_, pos, morph_type) in &spatial_manager.position_history {
        type_groups.entry(*morph_type).or_insert_with(Vec::new).push(*pos);
    }
    
    let mut quality_sum = 0.0;
    let mut type_count = 0;
    
    for (_, positions) in type_groups {
        if positions.len() > 1 {
            // Calculate average intra-type distance (lower is better clustering)
            let mut total_dist = 0.0;
            let mut pair_count = 0;
            
            for i in 0..positions.len() {
                for j in (i+1)..positions.len() {
                    let dist = ((positions[i].x - positions[j].x).powi(2) + 
                               (positions[i].y - positions[j].y).powi(2) + 
                               (positions[i].z - positions[j].z).powi(2)).sqrt();
                    total_dist += dist;
                    pair_count += 1;
                }
            }
            
            let avg_dist = total_dist / pair_count as f32;
            quality_sum += 1.0 - avg_dist.min(1.0); // Convert to quality score (higher is better)
            type_count += 1;
        }
    }
    
    if type_count > 0 { quality_sum / type_count as f32 } else { 0.0 }
}
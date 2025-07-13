//! Advanced pattern detection - exploring semantic gradients
//! Based on the "in ↔ out" gradient concept

use lingo::storage::MemoryMappedDatabase;
use lingo::core::*;
use lingo::query::QueryBuilder;
use lingo::engine::LingoExecutor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌊 Testing Semantic Gradient Exploration\n");
    
    // Load the base database
    let database = MemoryMappedDatabase::open("data/base.lingo")?;
    println!("Database loaded with {} nodes\n", database.node_count());
    
    // Create executor
    let mut executor = LingoExecutor::new();
    executor.set_database(database);
    
    // Test 1: In-Out gradient
    println!("📍 Test 1: 'in' ↔ 'out' containment gradient\n");
    explore_gradient(&mut executor, "in", "out", "containment")?;
    
    // Test 2: Pre-Post temporal gradient
    println!("\n⏰ Test 2: 'pre' ↔ 'post' temporal gradient\n");
    explore_gradient(&mut executor, "pre", "post", "temporal")?;
    
    // Test 3: Sub-Super hierarchical gradient
    println!("\n📊 Test 3: 'sub' ↔ 'super' hierarchical gradient\n");
    explore_gradient(&mut executor, "sub", "super", "hierarchical")?;
    
    // Test 4: Micro-Macro scale gradient
    println!("\n🔍 Test 4: 'micro' ↔ 'macro' scale gradient\n");
    explore_gradient(&mut executor, "micro", "macro", "scale")?;
    
    // Test 5: Find all oppositional pairs
    println!("\n↔️ Test 5: Discovering oppositional pairs\n");
    find_oppositional_pairs(&mut executor)?;
    
    Ok(())
}

fn explore_gradient(
    executor: &mut LingoExecutor, 
    start_word: &str, 
    end_word: &str,
    gradient_name: &str
) -> Result<(), Box<dyn std::error::Error>> {
    
    // Find the start and end points
    let start_result = executor.execute(&QueryBuilder::find(start_word).compile())?;
    let end_result = executor.execute(&QueryBuilder::find(end_word).compile())?;
    
    if start_result.nodes.is_empty() || end_result.nodes.is_empty() {
        println!("  ❌ Cannot explore gradient: '{}' or '{}' not found", start_word, end_word);
        return Ok(());
    }
    
    if let Some(db) = executor.database.as_ref() {
        let start_id = start_result.nodes.as_slice()[0];
        let end_id = end_result.nodes.as_slice()[0];
        
        if let (Ok(start_node), Ok(end_node)) = (db.get_node(start_id), db.get_node(end_id)) {
            let point_start = start_node.position;
            let point_end = end_node.position;
            
            // Calculate semantic vector
            let semantic_vector = Coordinate3D {
                x: point_end.x - point_start.x,
                y: point_end.y - point_start.y,
                z: point_end.z - point_start.z,
            };
            
            let distance = magnitude(&semantic_vector);
            println!("  📏 Distance: {:.3}, Vector: ({:.3}, {:.3}, {:.3})", 
                     distance, semantic_vector.x, semantic_vector.y, semantic_vector.z);
            
            // Sample 9 intermediate points
            println!("\n  🎯 Gradient exploration (sampling along {} axis):", gradient_name);
            
            for i in 1..=9 {
                let t = i as f32 / 10.0;
                let sample_point = Coordinate3D {
                    x: point_start.x + t * semantic_vector.x,
                    y: point_start.y + t * semantic_vector.y,
                    z: point_start.z + t * semantic_vector.z,
                };
                
                // Find nearby concepts
                let nearby = find_concepts_near_point(db, sample_point, 0.15)?;
                
                print!("    {:>3}% toward '{}': ", (t * 100.0) as i32, end_word);
                
                if nearby.is_empty() {
                    println!("[empty space]");
                } else {
                    // Sort by distance and show closest 3
                    let mut nearby_with_dist: Vec<_> = nearby.into_iter()
                        .map(|(word, pos)| {
                            let dist = distance_between(&pos, &sample_point);
                            (word, dist)
                        })
                        .collect();
                    nearby_with_dist.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                    
                    let closest: Vec<_> = nearby_with_dist.into_iter()
                        .take(3)
                        .map(|(word, dist)| format!("{} ({:.3})", word, dist))
                        .collect();
                    
                    println!("{}", closest.join(", "));
                }
            }
        }
    }
    
    Ok(())
}

fn find_concepts_near_point(
    db: &MemoryMappedDatabase,
    target: Coordinate3D,
    radius: f32
) -> Result<Vec<(String, Coordinate3D)>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    
    // Search all morphemes
    for i in 0..db.node_count() {
        let node_id = NodeId(i as u32);
        if let Ok(node) = db.get_node(node_id) {
            if node.layer == Layer::Morphemes {
                let pos = node.position; // Copy to avoid alignment issues
                let dist = distance_between(&pos, &target);
                if dist <= radius && dist > 0.001 { // Exclude exact matches
                    if let Ok(word) = db.get_node_word(node_id) {
                        results.push((word.to_string(), pos));
                    }
                }
            }
        }
    }
    
    Ok(results)
}

fn find_oppositional_pairs(executor: &mut LingoExecutor) -> Result<(), Box<dyn std::error::Error>> {
    // Common oppositional prefixes
    let opposites = [
        ("pre", "post"),
        ("sub", "super"),
        ("micro", "macro"),
        ("hyper", "hypo"),
        ("in", "out"),
        ("under", "over"),
        ("anti", "pro"),
    ];
    
    println!("  Checking common oppositional pairs:\n");
    
    let mut found_pairs = Vec::new();
    
    for (word1, word2) in &opposites {
        let result1 = executor.execute(&QueryBuilder::find(word1).compile())?;
        let result2 = executor.execute(&QueryBuilder::find(word2).compile())?;
        
        if !result1.nodes.is_empty() && !result2.nodes.is_empty() {
            if let Some(db) = executor.database.as_ref() {
                if let (Ok(node1), Ok(node2)) = (
                    db.get_node(result1.nodes.as_slice()[0]),
                    db.get_node(result2.nodes.as_slice()[0])
                ) {
                    let pos1 = node1.position; // Copy to avoid alignment issues
                    let pos2 = node2.position;
                    let dist = distance_between(&pos1, &pos2);
                    found_pairs.push((word1, word2, dist));
                    println!("    ✓ {} ↔ {}: distance = {:.3}", word1, word2, dist);
                }
            }
        } else {
            println!("    ✗ {} ↔ {}: one or both not found", word1, word2);
        }
    }
    
    // Analyze patterns
    if !found_pairs.is_empty() {
        let avg_distance: f32 = found_pairs.iter().map(|(_, _, d)| d).sum::<f32>() / found_pairs.len() as f32;
        println!("\n  📊 Average oppositional distance: {:.3}", avg_distance);
        
        // Check consistency
        let variance: f32 = found_pairs.iter()
            .map(|(_, _, d)| (d - avg_distance).powi(2))
            .sum::<f32>() / found_pairs.len() as f32;
        let std_dev = variance.sqrt();
        
        println!("  📊 Standard deviation: {:.3}", std_dev);
        
        if std_dev < 0.1 {
            println!("  ✨ High consistency! Oppositions cluster at similar distances.");
        } else {
            println!("  🔍 Variable distances suggest different types of opposition.");
        }
    }
    
    Ok(())
}

// Helper functions
fn magnitude(v: &Coordinate3D) -> f32 {
    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

fn distance_between(a: &Coordinate3D, b: &Coordinate3D) -> f32 {
    let diff = Coordinate3D {
        x: b.x - a.x,
        y: b.y - a.y,
        z: b.z - a.z,
    };
    magnitude(&diff)
}
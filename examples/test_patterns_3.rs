//! Test pattern 3: Comprehensive opposition analysis
//! Find natural clustering of semantic oppositions

use lingo::storage::MemoryMappedDatabase;
use lingo::core::*;
use lingo::query::QueryBuilder;
use lingo::engine::LingoExecutor;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct OppositionPair<'a> {
    word1: &'a str,
    word2: &'a str,
    opposition_type: OppositionType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum OppositionType {
    Spatial,       // in/out, up/down, left/right
    Temporal,      // pre/post, before/after
    Hierarchical,  // sub/super, under/over
    Scale,         // micro/macro, mini/maxi
    Polarity,      // positive/negative, pro/anti
    State,         // on/off, open/close
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Testing Opposition Clustering Patterns\n");
    
    // Load the base database
    let database = MemoryMappedDatabase::open("data/base.lingo")?;
    println!("Database loaded with {} nodes\n", database.node_count());
    
    // Create executor
    let mut executor = LingoExecutor::new();
    executor.set_database(database);
    
    // Define comprehensive opposition pairs
    let all_opposition_pairs = vec![
        // Spatial oppositions
        OppositionPair { word1: "in", word2: "out", opposition_type: OppositionType::Spatial },
        OppositionPair { word1: "up", word2: "down", opposition_type: OppositionType::Spatial },
        OppositionPair { word1: "left", word2: "right", opposition_type: OppositionType::Spatial },
        OppositionPair { word1: "fore", word2: "aft", opposition_type: OppositionType::Spatial },
        OppositionPair { word1: "front", word2: "back", opposition_type: OppositionType::Spatial },
        
        // Temporal oppositions
        OppositionPair { word1: "pre", word2: "post", opposition_type: OppositionType::Temporal },
        OppositionPair { word1: "ante", word2: "post", opposition_type: OppositionType::Temporal },
        OppositionPair { word1: "pro", word2: "retro", opposition_type: OppositionType::Temporal },
        
        // Hierarchical oppositions
        OppositionPair { word1: "sub", word2: "super", opposition_type: OppositionType::Hierarchical },
        OppositionPair { word1: "under", word2: "over", opposition_type: OppositionType::Hierarchical },
        OppositionPair { word1: "hypo", word2: "hyper", opposition_type: OppositionType::Hierarchical },
        OppositionPair { word1: "infra", word2: "supra", opposition_type: OppositionType::Hierarchical },
        
        // Scale oppositions
        OppositionPair { word1: "micro", word2: "macro", opposition_type: OppositionType::Scale },
        OppositionPair { word1: "mini", word2: "maxi", opposition_type: OppositionType::Scale },
        OppositionPair { word1: "nano", word2: "mega", opposition_type: OppositionType::Scale },
        
        // Polarity oppositions
        OppositionPair { word1: "pro", word2: "anti", opposition_type: OppositionType::Polarity },
        OppositionPair { word1: "philo", word2: "phobe", opposition_type: OppositionType::Polarity },
        OppositionPair { word1: "eu", word2: "dys", opposition_type: OppositionType::Polarity },
        
        // State oppositions
        OppositionPair { word1: "en", word2: "dis", opposition_type: OppositionType::State },
        OppositionPair { word1: "con", word2: "dis", opposition_type: OppositionType::State },
        OppositionPair { word1: "syn", word2: "dia", opposition_type: OppositionType::State },
    ];
    
    // Calculate distances for all opposition pairs
    let mut opposition_distances = Vec::new();
    
    for opposition_pair in &all_opposition_pairs {
        if let Some((point_a, point_b)) = get_opposition_coordinates(&mut executor, opposition_pair) {
            let distance = calculate_distance(&point_a, &point_b);
            opposition_distances.push((
                opposition_pair.clone(),
                distance,
                opposition_pair.opposition_type.clone()
            ));
        }
    }
    
    // Sort by distance to see natural clustering
    opposition_distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
    // Display results
    println!("📊 Opposition Distances (sorted):\n");
    for (pair, distance, _) in &opposition_distances {
        println!("  {} ↔ {} : {:.3} ({:?})", 
                 pair.word1, pair.word2, distance, pair.opposition_type);
    }
    
    // Find natural distance clusters
    let clusters = find_distance_clusters(&opposition_distances);
    
    println!("\n🎯 Natural Distance Clusters:\n");
    for (i, cluster) in clusters.iter().enumerate() {
        let avg_distance: f32 = cluster.iter().map(|(_, d, _)| d).sum::<f32>() / cluster.len() as f32;
        println!("Cluster {} (avg distance: {:.3}):", i + 1, avg_distance);
        
        // Group by type within cluster
        let mut by_type: HashMap<OppositionType, Vec<String>> = HashMap::new();
        for (pair, _, opp_type) in cluster {
            by_type.entry(opp_type.clone())
                .or_default()
                .push(format!("{}/{}", pair.word1, pair.word2));
        }
        
        for (opp_type, pairs) in by_type {
            println!("  {:?}: {}", opp_type, pairs.join(", "));
        }
        println!();
    }
    
    // Analyze patterns by opposition type
    analyze_by_type(&opposition_distances);
    
    // Test gradient consistency
    test_gradient_consistency(&mut executor, &opposition_distances)?;
    
    Ok(())
}

fn get_opposition_coordinates(
    executor: &mut LingoExecutor,
    pair: &OppositionPair<'_>
) -> Option<(Coordinate3D, Coordinate3D)> {
    let result1 = executor.execute(&QueryBuilder::find(pair.word1).compile()).ok()?;
    let result2 = executor.execute(&QueryBuilder::find(pair.word2).compile()).ok()?;
    
    if result1.nodes.is_empty() || result2.nodes.is_empty() {
        return None;
    }
    
    if let Some(db) = executor.database.as_ref() {
        let node1 = db.get_node(result1.nodes.as_slice()[0]).ok()?;
        let node2 = db.get_node(result2.nodes.as_slice()[0]).ok()?;
        
        // Copy positions to avoid alignment issues
        let pos1 = node1.position;
        let pos2 = node2.position;
        
        Some((pos1, pos2))
    } else {
        None
    }
}

fn calculate_distance(a: &Coordinate3D, b: &Coordinate3D) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let dz = b.z - a.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn find_distance_clusters<'a>(
    opposition_distances: &'a [(OppositionPair<'a>, f32, OppositionType)]
) -> Vec<Vec<(OppositionPair<'a>, f32, OppositionType)>> {
    let mut clusters = Vec::new();
    let mut current_cluster = Vec::new();
    let cluster_threshold = 0.05; // Distance threshold for clustering
    
    for (i, (pair, distance, opp_type)) in opposition_distances.iter().enumerate() {
        if i == 0 || (distance - opposition_distances[i-1].1).abs() < cluster_threshold {
            current_cluster.push((pair.clone(), *distance, opp_type.clone()));
        } else {
            if !current_cluster.is_empty() {
                clusters.push(current_cluster);
            }
            current_cluster = vec![(pair.clone(), *distance, opp_type.clone())];
        }
    }
    
    if !current_cluster.is_empty() {
        clusters.push(current_cluster);
    }
    
    clusters
}

fn analyze_by_type(opposition_distances: &[(OppositionPair<'_>, f32, OppositionType)]) {
    println!("\n📈 Analysis by Opposition Type:\n");
    
    let mut by_type: HashMap<OppositionType, Vec<f32>> = HashMap::new();
    
    for (_, distance, opp_type) in opposition_distances {
        by_type.entry(opp_type.clone())
            .or_default()
            .push(*distance);
    }
    
    for (opp_type, distances) in by_type {
        if distances.is_empty() { continue; }
        
        let sum: f32 = distances.iter().sum();
        let avg = sum / distances.len() as f32;
        let min = distances.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max = distances.iter().fold(0.0, |a, &b| a.max(b));
        
        println!("  {:?}:", opp_type);
        println!("    Count: {}", distances.len());
        println!("    Average: {:.3}", avg);
        println!("    Range: {:.3} - {:.3}", min, max);
        
        // Calculate standard deviation
        let variance: f32 = distances.iter()
            .map(|d| (d - avg).powi(2))
            .sum::<f32>() / distances.len() as f32;
        let std_dev = variance.sqrt();
        println!("    Std Dev: {:.3}", std_dev);
    }
}

fn test_gradient_consistency(
    executor: &mut LingoExecutor,
    opposition_distances: &[(OppositionPair<'_>, f32, OppositionType)]
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🌈 Gradient Consistency Test:\n");
    
    // Test a few key gradients
    let test_pairs = vec![
        ("in", "out"),
        ("pre", "post"),
        ("micro", "macro"),
    ];
    
    for (word1, word2) in test_pairs {
        if let Some((pos1, pos2)) = get_pair_positions(executor, word1, word2) {
            println!("  Testing {} ↔ {} gradient:", word1, word2);
            
            // Sample 5 points along the gradient
            for i in 1..=5 {
                let t = i as f32 / 6.0;
                let sample = Coordinate3D {
                    x: pos1.x + t * (pos2.x - pos1.x),
                    y: pos1.y + t * (pos2.y - pos1.y),
                    z: pos1.z + t * (pos2.z - pos1.z),
                };
                
                let nearby = find_nearby_morphemes(executor, sample, 0.1);
                println!("    {:.0}%: {:?}", t * 100.0, nearby);
            }
        }
    }
    
    Ok(())
}

fn get_pair_positions(
    executor: &mut LingoExecutor,
    word1: &str,
    word2: &str
) -> Option<(Coordinate3D, Coordinate3D)> {
    let pair = OppositionPair {
        word1,
        word2,
        opposition_type: OppositionType::Spatial, // Dummy type
    };
    get_opposition_coordinates(executor, &pair)
}

fn find_nearby_morphemes(
    executor: &LingoExecutor,
    target: Coordinate3D,
    radius: f32
) -> Vec<String> {
    let mut results = Vec::new();
    
    if let Some(db) = executor.database.as_ref() {
        for i in 0..db.node_count() {
            let node_id = NodeId(i as u32);
            if let Ok(node) = db.get_node(node_id) {
                if node.layer == Layer::Morphemes {
                    let pos = node.position;
                    let dist = calculate_distance(&pos, &target);
                    if dist <= radius && dist > 0.001 {
                        if let Ok(word) = db.get_node_word(node_id) {
                            results.push(word.to_string());
                        }
                    }
                }
            }
        }
    }
    
    // Sort by distance and take top 3
    results.truncate(3);
    results
}
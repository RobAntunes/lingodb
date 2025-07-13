//! Test pattern detection capabilities with the comprehensive LingoDB

use lingo::storage::MemoryMappedDatabase;
use lingo::core::*;
use lingo::query::{QueryBuilder, FilterCriteria};
use lingo::engine::LingoExecutor;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing LingoDB Pattern Detection Capabilities\n");
    
    // Load the comprehensive database
    let database = MemoryMappedDatabase::open("comprehensive.lingo")?;
    println!("Database loaded with {} nodes\n", database.node_count());
    
    // Create executor
    let mut executor = LingoExecutor::new();
    executor.set_database(database);
    
    // Test 1: Morphological patterns
    test_morphological_patterns(&mut executor)?;
    
    // Test 2: Etymology patterns  
    test_etymology_patterns(&mut executor)?;
    
    // Test 3: Semantic oppositions
    test_semantic_oppositions(&mut executor)?;
    
    // Test 4: Productivity patterns
    test_productivity_patterns(&mut executor)?;
    
    // Test 5: Cross-layer connections
    test_cross_layer_patterns(&mut executor)?;
    
    Ok(())
}

fn test_morphological_patterns(executor: &mut LingoExecutor) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Test 1: Morphological Patterns\n");
    
    // Directly iterate through nodes to find morphemes
    let mut morpheme_nodes = Vec::new();
    let mut prefixes = Vec::new();
    let mut suffixes = Vec::new();
    let mut roots = Vec::new();
    
    if let Some(db) = executor.database.as_ref() {
        for i in 0..db.node_count() {
            let node_id = NodeId(i as u32);
            if let Ok(node) = db.get_node(node_id) {
                if node.layer == Layer::Morphemes {
                    morpheme_nodes.push(node_id);
                    if let Ok(word) = db.get_node_word(node_id) {
                        match node.morpheme_type {
                            MorphemeType::Prefix => prefixes.push(word.to_string()),
                            MorphemeType::Suffix => suffixes.push(word.to_string()),
                            MorphemeType::Root => roots.push(word.to_string()),
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    
    println!("Found {} morphemes in total", morpheme_nodes.len());
    
    println!("\nMorpheme distribution (from first 30):");
    println!("  Prefixes: {} (e.g., {:?})", 
             prefixes.len(), 
             prefixes.iter().take(5).collect::<Vec<_>>());
    println!("  Suffixes: {} (e.g., {:?})", 
             suffixes.len(),
             suffixes.iter().take(5).collect::<Vec<_>>());
    println!("  Roots: {} (e.g., {:?})", 
             roots.len(),
             roots.iter().take(5).collect::<Vec<_>>());
    
    // Look for morpheme combinations
    println!("\nTesting morpheme combination patterns:");
    test_morpheme_combination(executor, "un", "happy", "ness")?;
    test_morpheme_combination(executor, "re", "build", "er")?;
    test_morpheme_combination(executor, "pre", "view", "ing")?;
    
    println!();
    Ok(())
}

fn test_morpheme_combination(executor: &mut LingoExecutor, prefix: &str, root: &str, suffix: &str) 
    -> Result<(), Box<dyn std::error::Error>> {
    
    let prefix_query = QueryBuilder::find(prefix).compile();
    let root_query = QueryBuilder::find(root).compile();
    let suffix_query = QueryBuilder::find(suffix).compile();
    
    let prefix_result = executor.execute(&prefix_query)?;
    let root_result = executor.execute(&root_query)?;
    let suffix_result = executor.execute(&suffix_query)?;
    
    if !prefix_result.nodes.is_empty() && !root_result.nodes.is_empty() && !suffix_result.nodes.is_empty() {
        if let Some(db) = executor.database.as_ref() {
            if let (Ok(p_node), Ok(r_node), Ok(s_node)) = (
                db.get_node(prefix_result.nodes.as_slice()[0]),
                db.get_node(root_result.nodes.as_slice()[0]),
                db.get_node(suffix_result.nodes.as_slice()[0])
            ) {
                let p1 = p_node.position;
                let p2 = r_node.position;
                let p3 = s_node.position;
                
                // Calculate semantic trajectory
                let v1 = Coordinate3D { 
                    x: p2.x - p1.x, 
                    y: p2.y - p1.y, 
                    z: p2.z - p1.z 
                };
                let v2 = Coordinate3D { 
                    x: p3.x - p2.x, 
                    y: p3.y - p2.y, 
                    z: p3.z - p2.z 
                };
                
                let mag1 = magnitude(&v1);
                let mag2 = magnitude(&v2);
                
                if mag1 > 0.0 && mag2 > 0.0 {
                    let trajectory_consistency = dot_product(&v1, &v2) / (mag1 * mag2);
                    println!("  {} + {} + {} → '{}{}{}' (trajectory consistency: {:.3})",
                             prefix, root, suffix, prefix, root, suffix, trajectory_consistency);
                } else {
                    println!("  {} + {} + {} → '{}{}{}' (collocated morphemes)",
                             prefix, root, suffix, prefix, root, suffix);
                }
            }
        }
    }
    
    Ok(())
}

fn test_etymology_patterns(executor: &mut LingoExecutor) -> Result<(), Box<dyn std::error::Error>> {
    println!("🌍 Test 2: Etymology Patterns\n");
    
    // For now, we'll skip etymology analysis since nodes don't have etymology field
    // Just count morphemes by type
    
    if let Some(db) = executor.database.as_ref() {
        let mut type_counts = std::collections::HashMap::new();
        
        for i in 0..db.node_count() {
            let node_id = NodeId(i as u32);
            if let Ok(node) = db.get_node(node_id) {
                if node.layer == Layer::Morphemes {
                    *type_counts.entry(node.morpheme_type).or_insert(0) += 1;
                }
            }
        }
        
        println!("Morpheme type distribution:");
        for (morph_type, count) in &type_counts {
            println!("  {:?}: {} morphemes", morph_type, count);
        }
    }
    
    println!();
    Ok(())
}

fn test_semantic_oppositions(executor: &mut LingoExecutor) -> Result<(), Box<dyn std::error::Error>> {
    println!("↔️ Test 3: Semantic Oppositions\n");
    
    // Test antonym connections
    let antonym_pairs = [
        ("happy", "sad"),
        ("up", "down"),
        ("in", "out"),
        ("pro", "anti"),
    ];
    
    for (word1, word2) in &antonym_pairs {
        let query1 = QueryBuilder::find(word1).compile();
        let query2 = QueryBuilder::find(word2).compile();
        
        let result1 = executor.execute(&query1)?;
        let result2 = executor.execute(&query2)?;
        
        if !result1.nodes.is_empty() && !result2.nodes.is_empty() {
            if let Some(db) = executor.database.as_ref() {
                if let (Ok(node1), Ok(node2)) = (
                    db.get_node(result1.nodes.as_slice()[0]),
                    db.get_node(result2.nodes.as_slice()[0])
                ) {
                    let p1 = node1.position;
                    let p2 = node2.position;
                    
                    // Calculate semantic vector
                    let semantic_vector = Coordinate3D {
                        x: p2.x - p1.x,
                        y: p2.y - p1.y,
                        z: p2.z - p1.z,
                    };
                    
                    let distance = magnitude(&semantic_vector);
                    
                    println!("  {} ↔ {}: distance={:.3}, vector=({:.3}, {:.3}, {:.3})",
                             word1, word2, distance, 
                             semantic_vector.x, semantic_vector.y, semantic_vector.z);
                    
                    // Sample intermediate points - disabled due to borrowing issues
                    // Would need to refactor the executor architecture to support this
                    println!("    Gradient: [spatial queries disabled in this test]");
                } else {
                    println!("  {} ↔ {}: nodes found but couldn't load them", word1, word2);
                }
            }
        } else {
            println!("  {} ↔ {}: one or both not found in database", word1, word2);
        }
    }
    
    println!();
    Ok(())
}

fn test_productivity_patterns(executor: &mut LingoExecutor) -> Result<(), Box<dyn std::error::Error>> {
    println!("📈 Test 4: Productivity Patterns\n");
    
    // Find highly productive morphemes by checking flags
    let mut productive_morphemes = Vec::new();
    let mut by_type: HashMap<MorphemeType, Vec<String>> = HashMap::new();
    
    if let Some(db) = executor.database.as_ref() {
        for i in 0..db.node_count() {
            let node_id = NodeId(i as u32);
            if let Ok(node) = db.get_node(node_id) {
                if node.layer == Layer::Morphemes && node.flags.contains(NodeFlags::IS_PRODUCTIVE) {
                    productive_morphemes.push(node_id);
                    if let Ok(word) = db.get_node_word(node_id) {
                        by_type.entry(node.morpheme_type)
                            .or_default()
                            .push(word.to_string());
                    }
                }
            }
        }
    }
    
    println!("Highly productive morphemes: {}", productive_morphemes.len());
    
    for (morph_type, nodes) in &by_type {
        println!("\n  {:?}s ({} total):", morph_type, nodes.len());
        for (i, word) in nodes.iter().take(5).enumerate() {
            println!("    {}. {}", i + 1, word);
        }
    }
    
    // Test productivity gradient
    println!("\nProductivity spatial distribution:");
    
    let mut productive_positions = Vec::new();
    let mut all_positions = Vec::new();
    
    // Collect positions
    if let Some(db) = executor.database.as_ref() {
        for i in 0..db.node_count() {
            let node_id = NodeId(i as u32);
            if let Ok(node) = db.get_node(node_id) {
                if node.layer == Layer::Morphemes {
                    all_positions.push(node.position);
                    if node.flags.contains(NodeFlags::IS_PRODUCTIVE) {
                        productive_positions.push(node.position);
                    }
                }
            }
        }
    }
    
    if !productive_positions.is_empty() && !all_positions.is_empty() {
        let productive_centroid = calculate_centroid(&productive_positions);
        let all_centroid = calculate_centroid(&all_positions);
        
        let shift = Coordinate3D {
            x: productive_centroid.x - all_centroid.x,
            y: productive_centroid.y - all_centroid.y,
            z: productive_centroid.z - all_centroid.z,
        };
        
        println!("  Productive morphemes shift from average: ({:.3}, {:.3}, {:.3})",
                 shift.x, shift.y, shift.z);
        println!("  Shift magnitude: {:.3}", magnitude(&shift));
    }
    
    println!();
    Ok(())
}

fn test_cross_layer_patterns(executor: &mut LingoExecutor) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 Test 5: Cross-Layer Patterns\n");
    
    // Test letter-phoneme connections
    let vowel_letters = ["a", "e", "i", "o", "u"];
    
    println!("Letter-Phoneme mappings:");
    for letter in &vowel_letters {
        let letter_query = QueryBuilder::find(letter)
            .filter(FilterCriteria::Layer(Layer::Letters))
            .compile();
        
        let letter_result = executor.execute(&letter_query)?;
        
        if !letter_result.nodes.is_empty() {
            let letter_id = letter_result.nodes.as_slice()[0];
            
            print!("  {} → ", letter);
            let mut phoneme_count = 0;
            
            if let Some(db) = executor.database.as_ref() {
                // Find connected phonemes
                if let Ok(connections) = db.get_node_connections(letter_id) {
                    for conn in connections {
                        if conn.connection_type == ConnectionType::Hypernymy {
                            if let Ok(target) = db.get_node(conn.target_node) {
                                if target.layer == Layer::Phonemes {
                                    if let Ok(phoneme) = db.get_node_word(conn.target_node) {
                                        if phoneme_count > 0 { print!(", "); }
                                        print!("{}", phoneme);
                                        phoneme_count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            if phoneme_count == 0 {
                print!("(no connections found)");
            }
            println!();
        }
    }
    
    // Test morpheme decomposition patterns
    println!("\nMorpheme spatial patterns:");
    
    let mut prefix_positions = Vec::new();
    let mut suffix_positions = Vec::new();
    let mut root_positions = Vec::new();
    
    // Collect morpheme positions by type
    if let Some(db) = executor.database.as_ref() {
        for i in 0..db.node_count() {
            let node_id = NodeId(i as u32);
            if let Ok(node) = db.get_node(node_id) {
                if node.layer == Layer::Morphemes {
                    match node.morpheme_type {
                        MorphemeType::Prefix => prefix_positions.push(node.position),
                        MorphemeType::Suffix => suffix_positions.push(node.position),
                        MorphemeType::Root => root_positions.push(node.position),
                        _ => {}
                    }
                }
            }
        }
    }
    
    let prefix_centroid = calculate_centroid(&prefix_positions);
    let suffix_centroid = calculate_centroid(&suffix_positions);
    let root_centroid = calculate_centroid(&root_positions);
    
    println!("  Prefix centroid: ({:.3}, {:.3}, {:.3})", 
             prefix_centroid.x, prefix_centroid.y, prefix_centroid.z);
    println!("  Root centroid: ({:.3}, {:.3}, {:.3})", 
             root_centroid.x, root_centroid.y, root_centroid.z);
    println!("  Suffix centroid: ({:.3}, {:.3}, {:.3})", 
             suffix_centroid.x, suffix_centroid.y, suffix_centroid.z);
    
    // Calculate the "word formation vector"
    let prefix_to_root = Coordinate3D {
        x: root_centroid.x - prefix_centroid.x,
        y: root_centroid.y - prefix_centroid.y,
        z: root_centroid.z - prefix_centroid.z,
    };
    
    let root_to_suffix = Coordinate3D {
        x: suffix_centroid.x - root_centroid.x,
        y: suffix_centroid.y - root_centroid.y,
        z: suffix_centroid.z - root_centroid.z,
    };
    
    println!("\n  Word formation trajectory:");
    println!("    Prefix→Root: ({:.3}, {:.3}, {:.3}), magnitude={:.3}", 
             prefix_to_root.x, prefix_to_root.y, prefix_to_root.z,
             magnitude(&prefix_to_root));
    println!("    Root→Suffix: ({:.3}, {:.3}, {:.3}), magnitude={:.3}", 
             root_to_suffix.x, root_to_suffix.y, root_to_suffix.z,
             magnitude(&root_to_suffix));
    
    Ok(())
}

// Helper functions
fn magnitude(v: &Coordinate3D) -> f32 {
    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

fn dot_product(v1: &Coordinate3D, v2: &Coordinate3D) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

fn calculate_centroid(positions: &[Coordinate3D]) -> Coordinate3D {
    if positions.is_empty() {
        return Coordinate3D { x: 0.0, y: 0.0, z: 0.0 };
    }
    
    let sum = positions.iter().fold(
        Coordinate3D { x: 0.0, y: 0.0, z: 0.0 },
        |acc, pos| Coordinate3D {
            x: acc.x + pos.x,
            y: acc.y + pos.y,
            z: acc.z + pos.z,
        }
    );
    
    let n = positions.len() as f32;
    Coordinate3D {
        x: sum.x / n,
        y: sum.y / n,
        z: sum.z / n,
    }
}

fn calculate_average_distance(positions: &[Coordinate3D], centroid: &Coordinate3D) -> f32 {
    if positions.is_empty() {
        return 0.0;
    }
    
    let sum_distances: f32 = positions.iter()
        .map(|pos| {
            let diff = Coordinate3D {
                x: pos.x - centroid.x,
                y: pos.y - centroid.y,
                z: pos.z - centroid.z,
            };
            magnitude(&diff)
        })
        .sum();
    
    sum_distances / positions.len() as f32
}
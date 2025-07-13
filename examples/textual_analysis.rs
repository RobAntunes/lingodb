//! Comprehensive textual analysis of LingoDB patterns
//! Shows spatial organization and relationships in text format

use lingo::storage::MemoryMappedDatabase;
use lingo::core::*;
use lingo::query::QueryBuilder;
use lingo::engine::LingoExecutor;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📚 LingoDB Textual Analysis Report\n");
    println!("{}", "=".repeat(60));
    
    // Load the base database
    let database = MemoryMappedDatabase::open("data/base.lingo")?;
    println!("\n📊 DATABASE OVERVIEW");
    println!("Total nodes: {}", database.node_count());
    
    // Create executor
    let mut executor = LingoExecutor::new();
    executor.set_database(database);
    
    // Analyze morpheme distribution
    analyze_morpheme_distribution(&executor)?;
    
    // Show spatial layout as ASCII art
    show_spatial_layout(&executor)?;
    
    // Analyze etymology patterns
    analyze_etymology_patterns(&mut executor)?;
    
    // Show semantic networks
    show_semantic_networks(&mut executor)?;
    
    // Coverage analysis
    analyze_coverage(&mut executor)?;
    
    Ok(())
}

fn analyze_morpheme_distribution(executor: &LingoExecutor) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📈 MORPHEME DISTRIBUTION");
    println!("{}", "-".repeat(60));
    
    let mut stats = HashMap::new();
    let mut total_morphemes = 0;
    
    if let Some(db) = executor.database.as_ref() {
        for i in 0..db.node_count() {
            let node_id = NodeId(i as u32);
            if let Ok(node) = db.get_node(node_id) {
                match node.layer {
                    Layer::Letters => *stats.entry("Letters").or_insert(0) += 1,
                    Layer::Phonemes => *stats.entry("Phonemes").or_insert(0) += 1,
                    Layer::Morphemes => {
                        total_morphemes += 1;
                        match node.morpheme_type {
                            MorphemeType::Prefix => *stats.entry("  → Prefixes").or_insert(0) += 1,
                            MorphemeType::Suffix => *stats.entry("  → Suffixes").or_insert(0) += 1,
                            MorphemeType::Root => *stats.entry("  → Roots").or_insert(0) += 1,
                            _ => *stats.entry("  → Other").or_insert(0) += 1,
                        }
                    },
                    Layer::Words => *stats.entry("Words").or_insert(0) += 1,
                    _ => *stats.entry("Other").or_insert(0) += 1,
                }
            }
        }
    }
    
    // Display stats
    println!("Letters:    {:>5}", stats.get("Letters").unwrap_or(&0));
    println!("Phonemes:   {:>5}", stats.get("Phonemes").unwrap_or(&0));
    println!("Morphemes:  {:>5} total", total_morphemes);
    println!("  → Prefixes: {:>4}", stats.get("  → Prefixes").unwrap_or(&0));
    println!("  → Suffixes: {:>4}", stats.get("  → Suffixes").unwrap_or(&0));
    println!("  → Roots:    {:>4}", stats.get("  → Roots").unwrap_or(&0));
    
    Ok(())
}

fn show_spatial_layout(executor: &LingoExecutor) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🗺️  SPATIAL LAYOUT (ASCII Visualization)");
    println!("{}", "-".repeat(60));
    println!("\nX-axis distribution (left to right):");
    println!("0.0                    0.5                    1.0");
    println!("|---------------------|---------------------|");
    println!("PREFIXES              ROOTS               SUFFIXES");
    println!("(pre,un,dis)       (tech,bio,log)      (tion,er,ness)");
    
    // Create a simple 2D grid representation
    println!("\nTop-down view (X-Y plane):");
    println!("Y");
    println!("↑");
    
    // 10x20 ASCII grid
    let mut grid: Vec<Vec<char>> = vec![vec![' '; 40]; 10];
    
    if let Some(db) = executor.database.as_ref() {
        for i in 0..db.node_count() {
            let node_id = NodeId(i as u32);
            if let Ok(node) = db.get_node(node_id) {
                if node.layer == Layer::Morphemes {
                    let pos = node.position;
                    let x = (pos.x * 39.0) as usize;
                    let y = 9 - ((pos.y * 9.0) as usize).min(9);
                    
                    let symbol = match node.morpheme_type {
                        MorphemeType::Prefix => 'P',
                        MorphemeType::Suffix => 'S',
                        MorphemeType::Root => 'R',
                        _ => '?',
                    };
                    
                    if x < 40 && y < 10 {
                        grid[y][x] = symbol;
                    }
                }
            }
        }
    }
    
    // Print grid
    for row in grid {
        print!("|");
        for cell in row {
            print!("{}", cell);
        }
        println!("|");
    }
    println!("└" + &"─".repeat(40) + "┘→ X");
    
    Ok(())
}

fn analyze_etymology_patterns(executor: &mut LingoExecutor) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🌍 ETYMOLOGY PATTERNS");
    println!("{}", "-".repeat(60));
    
    // Sample some morphemes and show their etymology
    let test_morphemes = vec!["pre", "post", "tech", "bio", "graph", "un", "re", "tion"];
    
    println!("Sample morpheme etymologies:");
    for morpheme in test_morphemes {
        if let Ok(result) = executor.execute(&QueryBuilder::find(morpheme).compile()) {
            if !result.nodes.is_empty() {
                if let Some(db) = executor.database.as_ref() {
                    if let Ok(node) = db.get_node(result.nodes.as_slice()[0]) {
                        // Infer etymology from position/flags
                        let etymology = if node.flags.contains(NodeFlags::IS_TECHNICAL) {
                            "Technical/Modern"
                        } else {
                            "Germanic/Latin"
                        };
                        println!("  {:>8} → {}", morpheme, etymology);
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn show_semantic_networks(executor: &mut LingoExecutor) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🕸️  SEMANTIC NETWORKS");
    println!("{}", "-".repeat(60));
    
    // Test some semantic relationships
    let test_pairs = vec![
        ("in", "out", "spatial opposition"),
        ("pre", "post", "temporal opposition"),
        ("micro", "macro", "scale opposition"),
        ("sub", "super", "hierarchical opposition"),
    ];
    
    println!("Key semantic relationships:");
    for (word1, word2, relation) in test_pairs {
        if let (Some(pos1), Some(pos2)) = get_positions(executor, word1, word2) {
            let distance = calculate_distance(&pos1, &pos2);
            println!("  {} ↔ {} ({}): distance = {:.3}", word1, word2, relation, distance);
            
            // Show intermediate space
            let mid = Coordinate3D {
                x: (pos1.x + pos2.x) / 2.0,
                y: (pos1.y + pos2.y) / 2.0,
                z: (pos1.z + pos2.z) / 2.0,
            };
            
            let nearby = find_nearby(executor, mid, 0.1);
            if !nearby.is_empty() {
                println!("    Midpoint concepts: {:?}", nearby);
            }
        }
    }
    
    Ok(())
}

fn analyze_coverage(executor: &mut LingoExecutor) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 COVERAGE ANALYSIS");
    println!("{}", "-".repeat(60));
    
    // Test common English words
    let common_words = vec![
        // Can be built from morphemes
        ("unhappy", vec!["un", "happy"]),
        ("rebuild", vec!["re", "build"]),
        ("preview", vec!["pre", "view"]),
        ("technology", vec!["tech", "no", "log", "y"]),
        ("biological", vec!["bio", "log", "ical"]),
        ("microscope", vec!["micro", "scope"]),
        ("telephone", vec!["tele", "phone"]),
        ("submarine", vec!["sub", "marine"]),
        ("supernatural", vec!["super", "natural"]),
        ("antibiotic", vec!["anti", "bio", "tic"]),
    ];
    
    let mut found = 0;
    let mut total = 0;
    
    println!("\nMorphological decomposition test:");
    for (word, morphemes) in common_words {
        print!("  {:15} → ", word);
        let mut all_found = true;
        let mut missing = Vec::new();
        
        for morpheme in &morphemes {
            total += 1;
            let result = executor.execute(&QueryBuilder::find(morpheme).compile())?;
            if result.nodes.is_empty() {
                all_found = false;
                missing.push(morpheme);
            } else {
                found += 1;
            }
        }
        
        if all_found {
            println!("✓ All morphemes found: {}", morphemes.join("-"));
        } else {
            println!("✗ Missing: {:?}", missing);
        }
    }
    
    let coverage = (found as f32 / total as f32) * 100.0;
    println!("\nCoverage: {}/{} morphemes found ({:.1}%)", found, total, coverage);
    
    println!("\n💡 RECOMMENDATIONS:");
    if coverage < 90.0 {
        println!("- Add more common roots (e.g., 'happy', 'view', 'build')");
        println!("- Include bound morphemes (e.g., 'y', 'ical', 'tic')");
        println!("- Add phonological variants (e.g., 'phon' vs 'phone')");
    } else {
        println!("- Coverage is excellent!");
        println!("- Consider adding specialized vocabulary domains");
        println!("- Focus on semantic relationship quality");
    }
    
    Ok(())
}

// Helper functions
fn get_positions(executor: &mut LingoExecutor, word1: &str, word2: &str) -> (Option<Coordinate3D>, Option<Coordinate3D>) {
    let mut pos1 = None;
    let mut pos2 = None;
    
    if let Ok(result1) = executor.execute(&QueryBuilder::find(word1).compile()) {
        if !result1.nodes.is_empty() {
            if let Some(db) = executor.database.as_ref() {
                if let Ok(node) = db.get_node(result1.nodes.as_slice()[0]) {
                    pos1 = Some(node.position);
                }
            }
        }
    }
    
    if let Ok(result2) = executor.execute(&QueryBuilder::find(word2).compile()) {
        if !result2.nodes.is_empty() {
            if let Some(db) = executor.database.as_ref() {
                if let Ok(node) = db.get_node(result2.nodes.as_slice()[0]) {
                    pos2 = Some(node.position);
                }
            }
        }
    }
    
    (pos1, pos2)
}

fn calculate_distance(a: &Coordinate3D, b: &Coordinate3D) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let dz = b.z - a.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn find_nearby(executor: &LingoExecutor, target: Coordinate3D, radius: f32) -> Vec<String> {
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
    
    results.truncate(3);
    results
}
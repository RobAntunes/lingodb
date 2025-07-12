//! Test layer navigation specifically

use lingo::storage::MemoryMappedDatabase;
use lingo::query::QueryBuilder;
use lingo::engine::LingoExecutor;
use lingo::core::error::Result;

fn main() -> Result<()> {
    let db_path = "example.lingo";
    let database = MemoryMappedDatabase::open(db_path)?;
    
    let mut executor = LingoExecutor::new();
    executor.set_database(database);
    
    println!("=== LAYER NAVIGATION TEST ===\n");
    
    // Test 1: Find technical and go up
    println!("Test 1: Find 'technical' and navigate up to concept layer");
    let query = QueryBuilder::find("technical")
        .layer_up()
        .compile();
        
    let result = executor.execute(&query)?;
    println!("Results: {} nodes found", result.nodes.len());
    
    if let Some(db) = &executor.database {
        for (i, node_id) in result.nodes.as_slice().iter().enumerate() {
            if let Ok(word) = db.get_node_word(*node_id) {
                if let Ok(node) = db.get_node(*node_id) {
                    let layer = node.layer;
                    println!("  [{}] {} (layer: {:?})", i + 1, word, layer);
                }
            }
        }
    }
    
    // Test 2: Find techn morpheme and go up
    println!("\nTest 2: Find 'techn' morpheme and navigate up to word layer");
    let query = QueryBuilder::find("techn")
        .layer_up()
        .compile();
        
    let result = executor.execute(&query)?;
    println!("Results: {} nodes found", result.nodes.len());
    
    if let Some(db) = &executor.database {
        for (i, node_id) in result.nodes.as_slice().iter().enumerate() {
            if let Ok(word) = db.get_node_word(*node_id) {
                if let Ok(node) = db.get_node(*node_id) {
                    let layer = node.layer;
                    println!("  [{}] {} (layer: {:?})", i + 1, word, layer);
                }
            }
        }
    }
    
    // Test 3: Chain - find technical, similar words, then up to concepts
    println!("\nTest 3: Find similar to 'technical', then navigate up");
    let query = QueryBuilder::find("technical")
        .similar_threshold(0.9)
        .layer_up()
        .compile();
        
    let result = executor.execute(&query)?;
    println!("Results: {} nodes found", result.nodes.len());
    
    if let Some(db) = &executor.database {
        for (i, node_id) in result.nodes.as_slice().iter().enumerate() {
            if let Ok(word) = db.get_node_word(*node_id) {
                println!("  [{}] {}", i + 1, word);
            }
        }
    }
    
    Ok(())
}
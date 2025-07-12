//! Demo of querying a Lingo database using SLANG bytecode

use lingo::core::error::Result;
use lingo::query::QueryBuilder;
use lingo::engine::LingoExecutor;
use lingo::storage::MemoryMappedDatabase;
use std::time::Instant;

fn main() -> Result<()> {
    println!("=== Lingo Query Demo ===\n");
    
    // Check if database file exists
    let db_path = "example.lingo";
    if !std::path::Path::new(db_path).exists() {
        println!("Database file not found. Please run 'cargo run --example build_db' first.");
        return Ok(());
    }
    
    // Open the database
    println!("Opening database: {}", db_path);
    let start = Instant::now();
    let database = MemoryMappedDatabase::open(db_path)?;
    println!("Database opened in {:?}", start.elapsed());
    
    // Print database info
    println!("\nDatabase Info:");
    println!("  Nodes: {}", database.node_count());
    println!("  Connections: {}", database.connection_count());
    println!("  Language: {}", std::str::from_utf8(&database.header().language_code)
        .unwrap_or("unknown").trim_end_matches('\0'));
    
    // Create executor and load database
    let mut executor = LingoExecutor::new();
    executor.set_database(database);
    
    // Demo 1: Find a specific word
    println!("\n--- Query 1: Find 'technical' ---");
    demo_find_word(&mut executor, "technical")?;
    
    // Demo 2: Find similar words
    println!("\n--- Query 2: Find words similar to 'technical' ---");
    demo_find_similar(&mut executor, "technical")?;
    
    // Demo 3: Navigate layers
    println!("\n--- Query 3: Find concepts above 'viral' ---");
    demo_layer_navigation(&mut executor, "viral")?;
    
    // Demo 4: Follow connections
    println!("\n--- Query 4: Follow connections from 'viral' ---");
    demo_follow_connections(&mut executor, "viral")?;
    
    // Demo 5: Complex query chain
    println!("\n--- Query 5: Complex query chain ---");
    demo_complex_query(&mut executor)?;
    
    Ok(())
}

fn demo_find_word(executor: &mut LingoExecutor, word: &str) -> Result<()> {
    // Build query
    let query = QueryBuilder::find(word)
        .limit(5)
        .compile();
    
    println!("Query: find('{}').limit(5)", word);
    println!("Bytecode: {} instructions", query.bytecode.len());
    
    // Execute
    let _start = Instant::now();
    let result = executor.execute(&query)?;
    
    println!("Results: {} nodes found in {:?}", 
        result.nodes.len(), 
        result.execution_time
    );
    
    // Print node details
    if let Some(db) = executor.database.as_ref() {
        for (i, node_id) in result.nodes.as_slice().iter().enumerate() {
            if let Ok(word) = db.get_node_word(*node_id) {
                println!("  [{}] {}", i + 1, word);
            }
        }
    }
    
    Ok(())
}

fn demo_find_similar(executor: &mut LingoExecutor, word: &str) -> Result<()> {
    // Build query
    let query = QueryBuilder::find(word)
        .similar_threshold(0.8)
        .limit(10)
        .compile();
    
    println!("Query: find('{}').similar(0.8).limit(10)", word);
    
    // Execute
    let result = executor.execute(&query)?;
    
    println!("Results: {} similar nodes found", result.nodes.len());
    
    // Print similar words
    if let Some(db) = executor.database.as_ref() {
        for (i, node_id) in result.nodes.as_slice().iter().enumerate() {
            if let Ok(word) = db.get_node_word(*node_id) {
                if let Ok(node) = db.get_node(*node_id) {
                    // Copy position to avoid unaligned access
                    let pos = node.position;
                    println!("  [{}] {} (pos: {:.2}, {:.2}, {:.2})", 
                        i + 1, word,
                        pos.x, pos.y, pos.z
                    );
                }
            }
        }
    }
    
    Ok(())
}

fn demo_layer_navigation(executor: &mut LingoExecutor, word: &str) -> Result<()> {
    // Build query to go up layers
    let query = QueryBuilder::find(word)
        .layer_up()
        .limit(5)
        .compile();
    
    println!("Query: find('{}').layer_up().limit(5)", word);
    
    // Execute
    let result = executor.execute(&query)?;
    
    println!("Results: {} parent concepts found", result.nodes.len());
    
    // Print parent concepts
    if let Some(db) = executor.database.as_ref() {
        for (i, node_id) in result.nodes.as_slice().iter().enumerate() {
            if let Ok(word) = db.get_node_word(*node_id) {
                if let Ok(node) = db.get_node(*node_id) {
                    println!("  [{}] {} (layer: {:?})", 
                        i + 1, word, node.layer
                    );
                }
            }
        }
    }
    
    Ok(())
}

fn demo_follow_connections(executor: &mut LingoExecutor, word: &str) -> Result<()> {
    // Build query to follow strongest connection
    let query = QueryBuilder::find(word)
        .follow_connection()
        .limit(5)
        .compile();
    
    println!("Query: find('{}').follow_connection().limit(5)", word);
    
    // Execute
    let result = executor.execute(&query)?;
    
    println!("Results: {} connected nodes found", result.nodes.len());
    
    // Print connected nodes
    if let Some(db) = executor.database.as_ref() {
        for (i, node_id) in result.nodes.as_slice().iter().enumerate() {
            if let Ok(word) = db.get_node_word(*node_id) {
                println!("  [{}] {}", i + 1, word);
            }
        }
    }
    
    Ok(())
}

fn demo_complex_query(executor: &mut LingoExecutor) -> Result<()> {
    // Build a complex query chain
    let query = QueryBuilder::find("technical")
        .similar_threshold(0.7)
        .layer_up()
        .follow_connection()
        .limit(10)
        .compile();
    
    println!("Query: find('technical').similar(0.7).layer_up().follow_connection().limit(10)");
    println!("Bytecode: {} instructions", query.bytecode.len());
    println!("Estimated cost: {}", query.estimated_cost);
    
    // Execute
    let _start = Instant::now();
    let result = executor.execute(&query)?;
    
    println!("Results: {} nodes found in {:?}", 
        result.nodes.len(), 
        result.execution_time
    );
    println!("Instructions executed: {}", result.instructions_executed);
    
    // Print results
    if let Some(db) = executor.database.as_ref() {
        for (i, node_id) in result.nodes.as_slice().iter().enumerate() {
            if let Ok(word) = db.get_node_word(*node_id) {
                if let Ok(node) = db.get_node(*node_id) {
                    // Copy fields to avoid unaligned access
                    let layer = node.layer;
                    let connections_count = node.connections_count;
                    println!("  [{}] {} (layer: {:?}, connections: {})", 
                        i + 1, word, layer, connections_count
                    );
                }
            }
        }
    }
    
    Ok(())
}
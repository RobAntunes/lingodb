//! Lingo database example

use lingo::{
    QueryBuilder,
    LingoExecutor,
    logging,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    logging::init_logging()?;
    
    println!("🚀 Lingo database query example");
    
    // Create an executor and load a pre-built database
    let mut executor = LingoExecutor::new();
    
    // Try to load the English database
    match executor.load_database("english.lingo") {
        Ok(_) => println!("✅ Loaded English database"),
        Err(e) => {
            println!("❌ Failed to load database: {}", e);
            println!("Please ensure 'english.lingo' exists in the current directory");
            return Ok(());
        }
    }
    
    // Example queries
    println!("\n📝 Running example queries...\n");
    
    // Query 1: Find similar words to "technical"
    let query = QueryBuilder::find("technical")
        .similar_threshold(0.8)
        .limit(10)
        .compile();
    
    match executor.execute(&query) {
        Ok(result) => {
            println!("Query: Find words similar to 'technical'");
            println!("Found {} node IDs", result.nodes.len());
            println!("Execution time: {:?}", result.execution_time);
        }
        Err(e) => println!("Query failed: {}", e),
    }
    
    // Query 2: Explore morphemes of a word
    println!("\n");
    let query = QueryBuilder::find("technology")
        .layer_down()  // Go to morphemes
        .limit(10)
        .compile();
    
    match executor.execute(&query) {
        Ok(result) => {
            println!("Query: Find morphemes in 'technology'");
            println!("Found {} node IDs", result.nodes.len());
            println!("Execution time: {:?}", result.execution_time);
        }
        Err(e) => println!("Query failed: {}", e),
    }
    
    Ok(())
}
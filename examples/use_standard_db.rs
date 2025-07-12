//! Demo using the standard English database

use lingo::storage::MemoryMappedDatabase;
use lingo::query::QueryBuilder;
use lingo::engine::LingoExecutor;
use lingo::discovery::AutoLinguisticBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Lingo Standard Database Demo\n");
    
    // Load the pre-built standard English database
    let database = MemoryMappedDatabase::open("english.lingo")?;
    let mut executor = LingoExecutor::new();
    executor.set_database(database);
    
    println!("📚 Loaded standard English database!");
    
    // Demo 1: Find morphemes
    println!("\n━━━ Demo 1: Morpheme Search ━━━");
    let query = QueryBuilder::find("bio").compile();
    let result = executor.execute(&query)?;
    
    println!("🔍 Searching for 'bio' morpheme...");
    if let Some(db) = &executor.database {
        for (i, node_id) in result.nodes.as_slice().iter().enumerate() {
            if let Ok(word) = db.get_node_word(*node_id) {
                if let Ok(node) = db.get_node(*node_id) {
                    println!("  [{}] {} - Layer: {:?}", i + 1, word, node.layer);
                }
            }
        }
    }
    
    // Demo 2: Navigate up from morpheme
    println!("\n━━━ Demo 2: Layer Navigation ━━━");
    println!("🔍 Finding words that contain 'tech' morpheme...");
    
    // First find tech morpheme, then navigate up
    let query = QueryBuilder::find("tech")
        .layer_up()
        .compile();
        
    let result = executor.execute(&query)?;
    
    if result.nodes.is_empty() {
        // If no direct navigation, let's add some words that use our morphemes
        println!("\n📝 Let me add some words to demonstrate...");
        
        let mut builder = AutoLinguisticBuilder::new();
        
        // Add words that use our standard morphemes
        let words = vec![
            "technology", "biology", "biography", 
            "technical", "technique", "biotechnology",
            "prehistoric", "postmodern", "antibiotic",
            "telescope", "microscope", "transform",
            "international", "supernatural", "hyperactive"
        ];
        
        for word in &words {
            println!("  Adding: {}", word);
            builder.add_word(word)?;
        }
        
        builder.build("enhanced.lingo")?;
        
        // Reload with enhanced database
        let database = MemoryMappedDatabase::open("enhanced.lingo")?;
        executor.set_database(database);
        
        println!("\n✅ Enhanced database created! Now let's search again...");
        
        // Try the query again
        let query = QueryBuilder::find("tech")
            .layer_up()
            .compile();
        let result = executor.execute(&query)?;
        
        if let Some(db) = &executor.database {
            println!("\nWords containing 'tech':");
            for (i, node_id) in result.nodes.as_slice().iter().enumerate() {
                if let Ok(word) = db.get_node_word(*node_id) {
                    println!("  [{}] {}", i + 1, word);
                }
            }
        }
    }
    
    // Demo 3: Search by etymology
    println!("\n━━━ Demo 3: Etymology Search ━━━");
    println!("🔍 Finding Greek-origin morphemes...");
    
    // This would need etymology-based queries in the future
    // For now, let's show what morphemes we have
    let morphemes = ["bio", "geo", "tech", "log", "graph", "phon"];
    
    for morpheme in &morphemes {
        let query = QueryBuilder::find(morpheme).compile();
        let result = executor.execute(&query)?;
        
        if !result.nodes.is_empty() {
            println!("  ✓ {} (Greek origin)", morpheme);
        }
    }
    
    // Demo 4: Phoneme connections
    println!("\n━━━ Demo 4: Phoneme Layer ━━━");
    println!("🔍 Exploring phonemes...");
    
    let phonemes = ["/t/", "/k/", "/s/", "/θ/"];
    
    for phoneme in &phonemes {
        let query = QueryBuilder::find(phoneme).compile();
        let result = executor.execute(&query)?;
        
        if !result.nodes.is_empty() {
            println!("  ✓ {} found in phoneme layer", phoneme);
        }
    }
    
    // Show database stats
    println!("\n━━━ Database Statistics ━━━");
    if let Some(db) = &executor.database {
        let mut layer_counts = [0usize; 7];
        for i in 1..=db.node_count() {
            if let Ok(node) = db.get_node(lingo::core::NodeId(i as u32)) {
                layer_counts[node.layer as usize] += 1;
            }
        }
        
        println!("📊 Nodes by layer:");
        println!("   Letters:    {} nodes", layer_counts[0]);
        println!("   Phonemes:   {} nodes", layer_counts[1]);
        println!("   Morphemes:  {} nodes", layer_counts[2]);
        println!("   Words:      {} nodes", layer_counts[3]);
        println!("   Phrases:    {} nodes", layer_counts[4]);
        println!("   Concepts:   {} nodes", layer_counts[5]);
        println!("   Domains:    {} nodes", layer_counts[6]);
    }
    
    println!("\n✨ The standard database provides a complete foundation for linguistic analysis!");
    println!("   Users can immediately:");
    println!("   • Search for morphemes and their meanings");
    println!("   • Add new words that automatically connect to existing morphemes");
    println!("   • Navigate the linguistic hierarchy");
    println!("   • Discover etymological patterns");
    
    Ok(())
}
//! Showcase test suite that displays the Lingo database in action!

use lingo::core::*;
use lingo::storage::{DatabaseBuilder, MemoryMappedDatabase};
use lingo::query::QueryBuilder;
use lingo::engine::LingoExecutor;
use tempfile::TempDir;

/// Print a fancy header
fn print_header(title: &str) {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║ {:<64} ║", title);
    println!("╚══════════════════════════════════════════════════════════════════╝");
}

/// Print a section
fn print_section(title: &str) {
    let padding = if title.len() < 60 { 60 - title.len() } else { 0 };
    println!("\n┌─ {} ─{}┐", title, "─".repeat(padding));
}

/// Create a rich linguistic database
fn create_showcase_database(path: &str) -> error::Result<()> {
    let mut builder = DatabaseBuilder::new();
    
    print_header("BUILDING THE LINGO DATABASE");
    println!("\n📚 Creating linguistic hierarchy...\n");
    
    // Layer 0: Letters
    print_section("Layer 0: Letters & Characters");
    let t = builder.add_node("t", Layer::Letters, Coordinate3D { x: 0.1, y: 0.1, z: 0.05 })?;
    let e = builder.add_node("e", Layer::Letters, Coordinate3D { x: 0.15, y: 0.1, z: 0.05 })?;
    let c = builder.add_node("c", Layer::Letters, Coordinate3D { x: 0.2, y: 0.1, z: 0.05 })?;
    let h = builder.add_node("h", Layer::Letters, Coordinate3D { x: 0.25, y: 0.1, z: 0.05 })?;
    println!("  ✓ Added letters: t, e, c, h");
    
    // Layer 1: Phonemes
    print_section("Layer 1: Phonemes");
    let tek_phoneme = builder.add_node("/tek/", Layer::Phonemes, Coordinate3D { x: 0.3, y: 0.15, z: 0.2 })?;
    let _ai_phoneme = builder.add_node("/aɪ/", Layer::Phonemes, Coordinate3D { x: 0.4, y: 0.15, z: 0.2 })?;
    println!("  ✓ Added phonemes: /tek/, /aɪ/");
    
    // Layer 2: Morphemes
    print_section("Layer 2: Morphemes");
    let tech_morph = builder.add_node_full(
        "tech",
        Layer::Morphemes,
        Coordinate3D { x: 0.3, y: 0.2, z: 0.3 },
        EtymologyOrigin::Greek,
        MorphemeType::Prefix,
        NodeFlags::IS_PRODUCTIVE | NodeFlags::IS_TECHNICAL,
    )?;
    let bio_morph = builder.add_node_full(
        "bio",
        Layer::Morphemes,
        Coordinate3D { x: 0.7, y: 0.2, z: 0.3 },
        EtymologyOrigin::Greek,
        MorphemeType::Prefix,
        NodeFlags::IS_PRODUCTIVE,
    )?;
    let logy_morph = builder.add_node_full(
        "logy",
        Layer::Morphemes,
        Coordinate3D { x: 0.35, y: 0.25, z: 0.3 },
        EtymologyOrigin::Greek,
        MorphemeType::Suffix,
        NodeFlags::IS_PRODUCTIVE,
    )?;
    let ology_morph = builder.add_node_full(
        "ology",
        Layer::Morphemes,
        Coordinate3D { x: 0.4, y: 0.25, z: 0.3 },
        EtymologyOrigin::Greek,
        MorphemeType::Suffix,
        NodeFlags::IS_PRODUCTIVE,
    )?;
    println!("  ✓ Added morphemes: tech (Greek prefix), bio (Greek prefix)");
    println!("  ✓ Added morphemes: logy (Greek suffix), ology (Greek suffix)");
    
    // Layer 3: Words
    print_section("Layer 3: Words");
    let technical = builder.add_node_full(
        "technical",
        Layer::Words,
        Coordinate3D { x: 0.5, y: 0.3, z: 0.4 },
        EtymologyOrigin::Greek,
        MorphemeType::Root,
        NodeFlags::IS_TECHNICAL | NodeFlags::IS_FREQUENT,
    )?;
    let technology = builder.add_node_full(
        "technology",
        Layer::Words,
        Coordinate3D { x: 0.52, y: 0.32, z: 0.42 },
        EtymologyOrigin::Greek,
        MorphemeType::Compound,
        NodeFlags::IS_TECHNICAL | NodeFlags::IS_FREQUENT,
    )?;
    let technique = builder.add_node_full(
        "technique",
        Layer::Words,
        Coordinate3D { x: 0.48, y: 0.3, z: 0.4 },
        EtymologyOrigin::Greek,
        MorphemeType::Root,
        NodeFlags::IS_TECHNICAL,
    )?;
    let biology = builder.add_node_full(
        "biology",
        Layer::Words,
        Coordinate3D { x: 0.7, y: 0.35, z: 0.42 },
        EtymologyOrigin::Greek,
        MorphemeType::Compound,
        NodeFlags::IS_TECHNICAL,
    )?;
    let biotechnology = builder.add_node_full(
        "biotechnology",
        Layer::Words,
        Coordinate3D { x: 0.6, y: 0.33, z: 0.43 },
        EtymologyOrigin::Greek,
        MorphemeType::Compound,
        NodeFlags::IS_TECHNICAL,
    )?;
    
    // Cross-domain words
    let viral_med = builder.add_node_full(
        "viral",
        Layer::Words,
        Coordinate3D { x: 0.8, y: 0.8, z: 0.4 },
        EtymologyOrigin::Latin,
        MorphemeType::Root,
        NodeFlags::IS_TECHNICAL,
    )?;
    let viral_marketing = builder.add_node_full(
        "viral",
        Layer::Words,
        Coordinate3D { x: 0.3, y: 0.9, z: 0.4 },
        EtymologyOrigin::Latin,
        MorphemeType::Root,
        NodeFlags::IS_LEARNED,
    )?;
    
    println!("  ✓ Added technical words: technical, technology, technique");
    println!("  ✓ Added biological words: biology, biotechnology");
    println!("  ✓ Added cross-domain word: viral (medical & marketing contexts)");
    
    // Layer 4: Phrases
    print_section("Layer 4: Phrases");
    let cutting_edge = builder.add_node("cutting edge", Layer::Phrases, Coordinate3D { x: 0.5, y: 0.4, z: 0.55 })?;
    let state_of_art = builder.add_node("state of the art", Layer::Phrases, Coordinate3D { x: 0.52, y: 0.42, z: 0.56 })?;
    let viral_marketing_phrase = builder.add_node("go viral", Layer::Phrases, Coordinate3D { x: 0.3, y: 0.85, z: 0.55 })?;
    println!("  ✓ Added phrases: cutting edge, state of the art, go viral");
    
    // Layer 5: Concepts
    print_section("Layer 5: Concepts");
    let tech_innovation = builder.add_node_full(
        "technological innovation",
        Layer::Concepts,
        Coordinate3D { x: 0.5, y: 0.35, z: 0.7 },
        EtymologyOrigin::Greek,
        MorphemeType::Root,
        NodeFlags::IS_TECHNICAL,
    )?;
    let life_sciences = builder.add_node_full(
        "life sciences",
        Layer::Concepts,
        Coordinate3D { x: 0.7, y: 0.4, z: 0.7 },
        EtymologyOrigin::Greek,
        MorphemeType::Root,
        NodeFlags::IS_TECHNICAL,
    )?;
    let communication = builder.add_node_full(
        "communication",
        Layer::Concepts,
        Coordinate3D { x: 0.3, y: 0.8, z: 0.7 },
        EtymologyOrigin::Latin,
        MorphemeType::Root,
        NodeFlags::empty(),
    )?;
    println!("  ✓ Added concepts: technological innovation, life sciences, communication");
    
    // Layer 6: Domains
    print_section("Layer 6: Domains");
    let stem_domain = builder.add_node("STEM", Layer::Domains, Coordinate3D { x: 0.6, y: 0.5, z: 0.95 })?;
    let marketing_domain = builder.add_node("Marketing", Layer::Domains, Coordinate3D { x: 0.3, y: 0.8, z: 0.95 })?;
    println!("  ✓ Added domains: STEM, Marketing");
    
    // Add connections
    print_section("Building Connections");
    
    // Letter to morpheme connections
    builder.add_connection(t, tech_morph, ConnectionType::Hypernymy, 0.9)?;
    builder.add_connection(e, tech_morph, ConnectionType::Hypernymy, 0.8)?;
    builder.add_connection(c, tech_morph, ConnectionType::Hypernymy, 0.8)?;
    builder.add_connection(h, tech_morph, ConnectionType::Hypernymy, 0.8)?;
    println!("  ✓ Connected letters → morphemes");
    
    // Phoneme to morpheme
    builder.add_connection(tek_phoneme, tech_morph, ConnectionType::Hypernymy, 0.95)?;
    println!("  ✓ Connected phonemes → morphemes");
    
    // Morpheme to word connections
    builder.add_connection(tech_morph, technical, ConnectionType::Hypernymy, 0.95)?;
    builder.add_connection(tech_morph, technology, ConnectionType::Hypernymy, 0.95)?;
    builder.add_connection(tech_morph, technique, ConnectionType::Hypernymy, 0.9)?;
    builder.add_connection(logy_morph, technology, ConnectionType::Hypernymy, 0.95)?;
    builder.add_connection(bio_morph, biology, ConnectionType::Hypernymy, 0.95)?;
    builder.add_connection(logy_morph, biology, ConnectionType::Hypernymy, 0.95)?;
    builder.add_connection(bio_morph, biotechnology, ConnectionType::Hypernymy, 0.95)?;
    builder.add_connection(tech_morph, biotechnology, ConnectionType::Hypernymy, 0.95)?;
    builder.add_connection(ology_morph, biotechnology, ConnectionType::Hypernymy, 0.9)?;
    println!("  ✓ Connected morphemes → words");
    
    // Word to phrase connections
    builder.add_connection(technical, cutting_edge, ConnectionType::Hypernymy, 0.8)?;
    builder.add_connection(technology, state_of_art, ConnectionType::Hypernymy, 0.85)?;
    builder.add_connection(viral_marketing, viral_marketing_phrase, ConnectionType::Hypernymy, 0.9)?;
    println!("  ✓ Connected words → phrases");
    
    // Word/phrase to concept connections
    builder.add_connection(technical, tech_innovation, ConnectionType::Hypernymy, 0.9)?;
    builder.add_connection(technology, tech_innovation, ConnectionType::Hypernymy, 0.95)?;
    builder.add_connection(biotechnology, tech_innovation, ConnectionType::Hypernymy, 0.85)?;
    builder.add_connection(cutting_edge, tech_innovation, ConnectionType::Hypernymy, 0.9)?;
    builder.add_connection(state_of_art, tech_innovation, ConnectionType::Hypernymy, 0.9)?;
    builder.add_connection(biology, life_sciences, ConnectionType::Hypernymy, 0.95)?;
    builder.add_connection(biotechnology, life_sciences, ConnectionType::Hypernymy, 0.9)?;
    builder.add_connection(viral_marketing, communication, ConnectionType::Hypernymy, 0.85)?;
    builder.add_connection(viral_marketing_phrase, communication, ConnectionType::Hypernymy, 0.9)?;
    println!("  ✓ Connected words/phrases → concepts");
    
    // Concept to domain connections
    builder.add_connection(tech_innovation, stem_domain, ConnectionType::Hypernymy, 0.95)?;
    builder.add_connection(life_sciences, stem_domain, ConnectionType::Hypernymy, 0.95)?;
    builder.add_connection(communication, marketing_domain, ConnectionType::Hypernymy, 0.9)?;
    println!("  ✓ Connected concepts → domains");
    
    // Cross-domain analogy
    builder.add_connection(viral_med, viral_marketing, ConnectionType::Analogy, 0.85)?;
    builder.add_connection(viral_marketing, viral_med, ConnectionType::Analogy, 0.85)?;
    println!("  ✓ Created cross-domain analogy: viral (medical) ↔ viral (marketing)");
    
    // Meronymy (part-of) relationships
    builder.add_connection(technical, tech_morph, ConnectionType::Meronymy, 0.95)?;
    builder.add_connection(technology, tech_morph, ConnectionType::Meronymy, 0.95)?;
    builder.add_connection(technology, logy_morph, ConnectionType::Meronymy, 0.95)?;
    builder.add_connection(biology, bio_morph, ConnectionType::Meronymy, 0.95)?;
    builder.add_connection(biology, logy_morph, ConnectionType::Meronymy, 0.95)?;
    builder.add_connection(biotechnology, bio_morph, ConnectionType::Meronymy, 0.95)?;
    builder.add_connection(biotechnology, tech_morph, ConnectionType::Meronymy, 0.95)?;
    builder.add_connection(biotechnology, ology_morph, ConnectionType::Meronymy, 0.95)?;
    println!("  ✓ Created part-of relationships for compound words");
    
    println!("\n✅ Database construction complete!");
    builder.build(path)?;
    Ok(())
}

/// Display query results with formatting
fn display_results(executor: &LingoExecutor, result: &lingo::engine::QueryResult, query_desc: &str) {
    println!("\n🔍 Query: {}", query_desc);
    println!("⏱️  Execution time: {:?}", result.execution_time);
    println!("📊 Instructions executed: {}", result.instructions_executed);
    println!("🎯 Results found: {}", result.nodes.len());
    
    if let Some(db) = &executor.database {
        println!("\n   Results:");
        for (i, node_id) in result.nodes.as_slice().iter().enumerate() {
            if let Ok(word) = db.get_node_word(*node_id) {
                if let Ok(node) = db.get_node(*node_id) {
                    let layer = node.layer;
                    let flags = node.flags;
                    let etymology = node.etymology_origin;
                    
                    print!("   [{:2}] {:20} (Layer: {:?}", i + 1, word, layer);
                    
                    // Show special properties
                    let mut props = Vec::new();
                    if flags.contains(NodeFlags::IS_TECHNICAL) {
                        props.push("Technical");
                    }
                    if flags.contains(NodeFlags::IS_PRODUCTIVE) {
                        props.push("Productive");
                    }
                    if flags.contains(NodeFlags::IS_FREQUENT) {
                        props.push("Frequent");
                    }
                    
                    if !props.is_empty() {
                        print!(", {}", props.join(", "));
                    }
                    
                    println!(", Origin: {:?})", etymology);
                }
            }
        }
    }
}

#[test]
fn test_showcase_lingo_database() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("showcase.lingo");
    
    // Create the database
    create_showcase_database(db_path.to_str().unwrap()).unwrap();
    
    // Load and query the database
    let database = MemoryMappedDatabase::open(&db_path).unwrap();
    let mut executor = LingoExecutor::new();
    executor.set_database(database);
    
    print_header("QUERYING THE LINGO DATABASE");
    
    // Test 1: Simple word search
    print_section("Test 1: Exact Word Search");
    let query = QueryBuilder::find("technology").compile();
    let result = executor.execute(&query).unwrap();
    display_results(&executor, &result, "Find 'technology'");
    
    // Test 2: Find similar words
    print_section("Test 2: Spatial Similarity Search");
    let query = QueryBuilder::find("technical")
        .similar_threshold(0.95)
        .compile();
    let result = executor.execute(&query).unwrap();
    display_results(&executor, &result, "Find words similar to 'technical' (95% similarity)");
    
    // Test 3: Navigate up from morpheme to words
    print_section("Test 3: Layer Navigation (Morpheme → Words)");
    let query = QueryBuilder::find("tech")
        .layer_up()
        .compile();
    let result = executor.execute(&query).unwrap();
    display_results(&executor, &result, "Find all words containing 'tech' morpheme");
    
    // Test 4: Navigate up multiple layers
    print_section("Test 4: Multi-Layer Navigation (Words → Concepts → Domains)");
    let query = QueryBuilder::find("biotechnology")
        .layer_up()  // To concepts
        .layer_up()  // To domains
        .compile();
    let result = executor.execute(&query).unwrap();
    display_results(&executor, &result, "Navigate from 'biotechnology' up to domains");
    
    // Test 5: Cross-domain connections
    print_section("Test 5: Cross-Domain Connections");
    let query = QueryBuilder::find("viral")
        .follow_connection()
        .compile();
    let result = executor.execute(&query).unwrap();
    display_results(&executor, &result, "Find cross-domain connections for 'viral'");
    
    // Test 6: Complex query chain
    print_section("Test 6: Complex Query Chain");
    let query = QueryBuilder::find("bio")
        .layer_up()           // Find words with 'bio'
        .similar_threshold(0.9)  // Find similar words
        .layer_up()           // Go to concepts
        .limit(5)
        .compile();
    let result = executor.execute(&query).unwrap();
    display_results(&executor, &result, "From 'bio' morpheme → words → similar → concepts");
    
    // Test 7: Phrase navigation
    print_section("Test 7: Phrase to Concept Navigation");
    let query = QueryBuilder::find("cutting edge")
        .layer_up()
        .compile();
    let result = executor.execute(&query).unwrap();
    display_results(&executor, &result, "Navigate from phrase to concept");
    
    // Show database statistics
    print_header("DATABASE STATISTICS");
    if let Some(db) = &executor.database {
        println!("\n📈 Total nodes: {}", db.node_count());
        println!("🔗 Total connections: {}", db.connection_count());
        println!("💾 Database file size: {} bytes", std::fs::metadata(&db_path).unwrap().len());
        
        // Count nodes by layer
        let mut layer_counts = [0usize; 7];
        for i in 1..=db.node_count() {
            if let Ok(node) = db.get_node(NodeId(i as u32)) {
                layer_counts[node.layer as usize] += 1;
            }
        }
        
        println!("\n📊 Nodes by layer:");
        println!("   Letters:    {} nodes", layer_counts[0]);
        println!("   Phonemes:   {} nodes", layer_counts[1]);
        println!("   Morphemes:  {} nodes", layer_counts[2]);
        println!("   Words:      {} nodes", layer_counts[3]);
        println!("   Phrases:    {} nodes", layer_counts[4]);
        println!("   Concepts:   {} nodes", layer_counts[5]);
        println!("   Domains:    {} nodes", layer_counts[6]);
    }
    
    print_header("LINGO DATABASE SHOWCASE COMPLETE!");
    println!("\n🎉 The revolutionary linguistic database is discovering connections!\n");
}
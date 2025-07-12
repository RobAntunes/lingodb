//! Example: Build a Lingo database

use lingo::core::{
    NodeId, Layer, Coordinate3D, EtymologyOrigin, MorphemeType,
    NodeFlags, ConnectionType,
};
use lingo::storage::DatabaseBuilder;
use lingo::core::error::Result;

fn main() -> Result<()> {
    println!("Building example Lingo database...\n");
    
    // Create a new database builder
    let mut builder = DatabaseBuilder::new();
    
    // Add technical word cluster
    println!("Adding technical word cluster...");
    
    // "technical" - root word
    let tech1 = builder.add_node_full(
        "technical",
        Layer::Words,
        Coordinate3D { x: 0.5, y: 0.3, z: 0.4 },
        EtymologyOrigin::Greek,
        MorphemeType::Root,
        NodeFlags::IS_TECHNICAL | NodeFlags::IS_PRODUCTIVE,
    )?;
    
    // "technology" - derived word
    let tech2 = builder.add_node_full(
        "technology",
        Layer::Words,
        Coordinate3D { x: 0.5, y: 0.3, z: 0.4 },
        EtymologyOrigin::Greek,
        MorphemeType::Compound,
        NodeFlags::IS_TECHNICAL | NodeFlags::IS_PRODUCTIVE,
    )?;
    
    // "technique" - related word
    let tech3 = builder.add_node_full(
        "technique",
        Layer::Words,
        Coordinate3D { x: 0.45, y: 0.3, z: 0.4 },
        EtymologyOrigin::Greek,
        MorphemeType::Root,
        NodeFlags::IS_TECHNICAL,
    )?;
    
    // "techn" - morpheme
    let tech_morph = builder.add_node_full(
        "techn",
        Layer::Morphemes,
        Coordinate3D { x: 0.5, y: 0.0, z: 0.3 },
        EtymologyOrigin::Greek,
        MorphemeType::Prefix,
        NodeFlags::IS_PRODUCTIVE,
    )?;
    
    // Add cross-domain example: "viral"
    println!("Adding cross-domain word: 'viral'...");
    
    // Medical context
    let viral_med = builder.add_node_full(
        "viral",
        Layer::Words,
        Coordinate3D { x: 0.7, y: 0.8, z: 0.4 },
        EtymologyOrigin::Latin,
        MorphemeType::Root,
        NodeFlags::IS_TECHNICAL,
    )?;
    
    // Marketing context (same word, different position)
    let viral_marketing = builder.add_node_full(
        "viral",
        Layer::Words,
        Coordinate3D { x: 0.3, y: 0.9, z: 0.4 },
        EtymologyOrigin::Latin,
        MorphemeType::Root,
        NodeFlags::IS_LEARNED,
    )?;
    
    // Add connections
    println!("Adding connections...");
    
    // Part-of connections (words contain morpheme)
    builder.add_connection(
        tech1,
        tech_morph,
        ConnectionType::Meronymy,
        0.9,
    )?;
    
    builder.add_connection(
        tech2,
        tech_morph,
        ConnectionType::Meronymy,
        0.9,
    )?;
    
    builder.add_connection(
        tech3,
        tech_morph,
        ConnectionType::Meronymy,
        0.85,
    )?;
    
    // Cross-domain analogy
    builder.add_connection(
        viral_med,
        viral_marketing,
        ConnectionType::Analogy,
        0.8,
    )?;
    
    // Reverse connection
    builder.add_connection(
        viral_marketing,
        viral_med,
        ConnectionType::Analogy,
        0.8,
    )?;
    
    // CRUCIAL: Add hypernymy connections for layer navigation
    println!("Adding layer navigation connections...");
    
    // Create a concept node for "technical concepts"
    let tech_concept = builder.add_node_full(
        "technical_concepts",
        Layer::Concepts,  // Higher layer!
        Coordinate3D { x: 0.5, y: 0.3, z: 0.6 }, // Higher Z for abstraction
        EtymologyOrigin::Greek,
        MorphemeType::Root,
        NodeFlags::IS_TECHNICAL,
    )?;
    
    // Connect words UP to concept (Hypernymy = IS-A relationship)
    builder.add_connection(
        tech1,
        tech_concept,
        ConnectionType::Hypernymy,  // "technical" IS-A "technical_concept"
        0.95,
    )?;
    
    builder.add_connection(
        tech2,
        tech_concept,
        ConnectionType::Hypernymy,  // "technology" IS-A "technical_concept"
        0.95,
    )?;
    
    builder.add_connection(
        tech3,
        tech_concept,
        ConnectionType::Hypernymy,  // "technique" IS-A "technical_concept"
        0.9,
    )?;
    
    // Also connect morpheme UP to words (showing layer hierarchy)
    builder.add_connection(
        tech_morph,
        tech1,
        ConnectionType::Hypernymy,  // morpheme "techn" IS-PART-OF word "technical"
        0.9,
    )?;
    
    // Build the database
    println!("\nBuilding database file...");
    let db_file = "example.lingo";
    builder.build(db_file)?;
    
    println!("Database built successfully: {}", db_file);
    
    // Print statistics
    let file_size = std::fs::metadata(db_file)?.len();
    println!("\nDatabase statistics:");
    println!("  File size: {} bytes", file_size);
    println!("  Nodes: 7 (including concept layer)");
    println!("  Connections: 9 (with hypernymy for layer navigation)");
    println!("  Layers: Morphemes → Words → Concepts");
    println!("  Spatial index: Octree");
    
    Ok(())
}
//! Lingo database example

use lingo::{
    core::{Coordinate3D, Layer, ConnectionType, NodeFlags, EtymologyOrigin, MorphemeType},
    storage::DatabaseBuilder,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Building Lingo database example...");
    
    // Create a new database builder
    let mut builder = DatabaseBuilder::new();
    builder
        .set_language("en-US")
        .set_model_version("1.0.0");
    
    // Add some example nodes
    
    // Technical words cluster
    let technical_id = builder.add_node_full(
        "technical",
        Layer::Words,
        Coordinate3D::new(0.5, 0.8, 0.525), // Greek-influenced, technical domain
        EtymologyOrigin::Greek,
        MorphemeType::Root,
        NodeFlags::IS_TECHNICAL | NodeFlags::IS_FREQUENT,
    )?;
    
    let technology_id = builder.add_node_full(
        "technology",
        Layer::Words,
        Coordinate3D::new(0.52, 0.8, 0.525),
        EtymologyOrigin::Greek,
        MorphemeType::Root,
        NodeFlags::IS_TECHNICAL | NodeFlags::IS_FREQUENT,
    )?;
    
    let technique_id = builder.add_node_full(
        "technique",
        Layer::Words,
        Coordinate3D::new(0.48, 0.8, 0.525),
        EtymologyOrigin::Greek,
        MorphemeType::Root,
        NodeFlags::IS_TECHNICAL,
    )?;
    
    // Add morpheme for "techn-"
    let techn_morpheme_id = builder.add_node_full(
        "techn",
        Layer::Morphemes,
        Coordinate3D::new(0.5, 0.8, 0.375), // Same X,Y but lower Z (morpheme layer)
        EtymologyOrigin::Greek,
        MorphemeType::Root,
        NodeFlags::IS_PRODUCTIVE,
    )?;
    
    // Viral concept (cross-domain)
    let viral_medical_id = builder.add_node_full(
        "viral",
        Layer::Words,
        Coordinate3D::new(0.3, 0.4, 0.525), // Latin origin, medical context
        EtymologyOrigin::Latin,
        MorphemeType::Root,
        NodeFlags::IS_TECHNICAL,
    )?;
    
    let viral_marketing_id = builder.add_node_full(
        "viral",
        Layer::Words,
        Coordinate3D::new(0.7, 0.9, 0.525), // Modern usage, business context
        EtymologyOrigin::Modern,
        MorphemeType::Root,
        NodeFlags::IS_FREQUENT,
    )?;
    
    // Add connections
    
    // Derivation connections
    builder.add_connection(
        techn_morpheme_id,
        technical_id,
        ConnectionType::Derivation,
        0.95,
    )?;
    
    builder.add_connection(
        techn_morpheme_id,
        technology_id,
        ConnectionType::Derivation,
        0.95,
    )?;
    
    builder.add_connection(
        techn_morpheme_id,
        technique_id,
        ConnectionType::Derivation,
        0.95,
    )?;
    
    // Semantic connections
    builder.add_connection(
        technical_id,
        technology_id,
        ConnectionType::Synonymy,
        0.8,
    )?;
    
    // Cross-domain analogy
    builder.add_connection(
        viral_medical_id,
        viral_marketing_id,
        ConnectionType::Analogy,
        0.85,
    )?;
    
    // Build and save the database
    let db_path = "example.lingo";
    builder.build(db_path)?;
    
    println!("Database built successfully: {}", db_path);
    println!("File size: {} bytes", std::fs::metadata(db_path)?.len());
    
    Ok(())
}
//! Integration tests for the Lingo database system

use lingo::core::*;
use lingo::storage::{DatabaseBuilder, MemoryMappedDatabase, LingoFileHeader};
use lingo::query::QueryBuilder;
use lingo::engine::LingoExecutor;
use tempfile::TempDir;

/// Create a test database with linguistic hierarchy
fn create_test_database(path: &str) -> error::Result<()> {
    let mut builder = DatabaseBuilder::new();
    
    // Layer 0: Letters/Characters
    let t_letter = builder.add_node_full(
        "t",
        Layer::Letters,
        Coordinate3D { x: 0.1, y: 0.1, z: 0.05 },
        EtymologyOrigin::Greek,
        MorphemeType::Root,
        NodeFlags::empty(),
    )?;
    
    // Layer 2: Morphemes
    let tech_morph = builder.add_node_full(
        "tech",
        Layer::Morphemes,
        Coordinate3D { x: 0.3, y: 0.2, z: 0.3 },
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
    
    // Layer 3: Words
    let technical = builder.add_node_full(
        "technical",
        Layer::Words,
        Coordinate3D { x: 0.5, y: 0.3, z: 0.4 },
        EtymologyOrigin::Greek,
        MorphemeType::Root,
        NodeFlags::IS_TECHNICAL,
    )?;
    
    let technology = builder.add_node_full(
        "technology",
        Layer::Words,
        Coordinate3D { x: 0.5, y: 0.3, z: 0.4 },
        EtymologyOrigin::Greek,
        MorphemeType::Compound,
        NodeFlags::IS_TECHNICAL,
    )?;
    
    let technique = builder.add_node_full(
        "technique",
        Layer::Words,
        Coordinate3D { x: 0.45, y: 0.3, z: 0.4 },
        EtymologyOrigin::Greek,
        MorphemeType::Root,
        NodeFlags::IS_TECHNICAL,
    )?;
    
    // Cross-domain word
    let viral_med = builder.add_node_full(
        "viral",
        Layer::Words,
        Coordinate3D { x: 0.7, y: 0.8, z: 0.4 },
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
    
    // Layer 5: Concepts
    let tech_concept = builder.add_node_full(
        "technical_concepts",
        Layer::Concepts,
        Coordinate3D { x: 0.5, y: 0.3, z: 0.6 },
        EtymologyOrigin::Greek,
        MorphemeType::Root,
        NodeFlags::IS_TECHNICAL,
    )?;
    
    let communication_concept = builder.add_node_full(
        "communication",
        Layer::Concepts,
        Coordinate3D { x: 0.5, y: 0.85, z: 0.6 },
        EtymologyOrigin::Latin,
        MorphemeType::Root,
        NodeFlags::empty(),
    )?;
    
    // Add connections
    
    // Hierarchy: Letter -> Morpheme
    builder.add_connection(t_letter, tech_morph, ConnectionType::Hypernymy, 0.8)?;
    
    // Hierarchy: Morpheme -> Word
    builder.add_connection(tech_morph, technical, ConnectionType::Hypernymy, 0.9)?;
    builder.add_connection(tech_morph, technology, ConnectionType::Hypernymy, 0.9)?;
    builder.add_connection(logy_morph, technology, ConnectionType::Hypernymy, 0.9)?;
    
    // Part-of: Word contains Morpheme
    builder.add_connection(technical, tech_morph, ConnectionType::Meronymy, 0.95)?;
    builder.add_connection(technology, tech_morph, ConnectionType::Meronymy, 0.95)?;
    builder.add_connection(technology, logy_morph, ConnectionType::Meronymy, 0.95)?;
    
    // Hierarchy: Word -> Concept
    builder.add_connection(technical, tech_concept, ConnectionType::Hypernymy, 0.95)?;
    builder.add_connection(technology, tech_concept, ConnectionType::Hypernymy, 0.95)?;
    builder.add_connection(technique, tech_concept, ConnectionType::Hypernymy, 0.9)?;
    builder.add_connection(viral_marketing, communication_concept, ConnectionType::Hypernymy, 0.8)?;
    
    // Cross-domain analogy
    builder.add_connection(viral_med, viral_marketing, ConnectionType::Analogy, 0.85)?;
    builder.add_connection(viral_marketing, viral_med, ConnectionType::Analogy, 0.85)?;
    
    builder.build(path)?;
    Ok(())
}

#[test]
fn test_database_creation_and_loading() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.lingo");
    
    // Create database
    create_test_database(db_path.to_str().unwrap()).unwrap();
    
    // Verify file exists
    assert!(db_path.exists());
    
    // Load database
    let database = MemoryMappedDatabase::open(&db_path).unwrap();
    
    // Check counts
    assert_eq!(database.node_count(), 10);
    assert_eq!(database.connection_count(), 13);
}

#[test]
fn test_exact_word_search() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.lingo");
    create_test_database(db_path.to_str().unwrap()).unwrap();
    
    let database = MemoryMappedDatabase::open(&db_path).unwrap();
    let mut executor = LingoExecutor::new();
    executor.set_database(database);
    
    // Search for "technical"
    let query = QueryBuilder::find("technical").compile();
    let result = executor.execute(&query).unwrap();
    
    assert_eq!(result.nodes.len(), 1);
    
    // Verify it's the right word
    if let Some(db) = &executor.database {
        let word = db.get_node_word(result.nodes.as_slice()[0]).unwrap();
        assert_eq!(word, "technical");
    }
}

#[test]
fn test_spatial_similarity_search() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.lingo");
    create_test_database(db_path.to_str().unwrap()).unwrap();
    
    let database = MemoryMappedDatabase::open(&db_path).unwrap();
    let mut executor = LingoExecutor::new();
    executor.set_database(database);
    
    // Find words similar to "technical"
    let query = QueryBuilder::find("technical")
        .similar_threshold(0.9)
        .compile();
    let result = executor.execute(&query).unwrap();
    
    // Should find technical, technology, and technique (all nearby in 3D space)
    assert!(result.nodes.len() >= 2);
    
    // Check that we found technology
    if let Some(db) = &executor.database {
        let words: Vec<String> = result.nodes.as_slice()
            .iter()
            .filter_map(|id| db.get_node_word(*id).ok())
            .map(|s| s.to_string())
            .collect();
        
        assert!(words.contains(&"technology".to_string()));
    }
}

#[test]
fn test_layer_navigation_up() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.lingo");
    create_test_database(db_path.to_str().unwrap()).unwrap();
    
    let database = MemoryMappedDatabase::open(&db_path).unwrap();
    let mut executor = LingoExecutor::new();
    executor.set_database(database);
    
    // Navigate from morpheme to word layer
    let query = QueryBuilder::find("tech")
        .layer_up()
        .compile();
    let result = executor.execute(&query).unwrap();
    
    // Should find words containing "tech"
    assert!(result.nodes.len() >= 2);
    
    if let Some(db) = &executor.database {
        let words: Vec<String> = result.nodes.as_slice()
            .iter()
            .filter_map(|id| db.get_node_word(*id).ok())
            .map(|s| s.to_string())
            .collect();
        
        assert!(words.contains(&"technical".to_string()));
        assert!(words.contains(&"technology".to_string()));
    }
}

#[test]
fn test_cross_domain_connections() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.lingo");
    create_test_database(db_path.to_str().unwrap()).unwrap();
    
    let database = MemoryMappedDatabase::open(&db_path).unwrap();
    let mut executor = LingoExecutor::new();
    executor.set_database(database);
    
    // Find cross-domain connections for "viral"
    let query = QueryBuilder::find("viral")
        .follow_connection()
        .compile();
    let result = executor.execute(&query).unwrap();
    
    // Should find both viral nodes (medical and marketing)
    assert_eq!(result.nodes.len(), 2);
}

#[test]
fn test_complex_query_chain() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.lingo");
    create_test_database(db_path.to_str().unwrap()).unwrap();
    
    let database = MemoryMappedDatabase::open(&db_path).unwrap();
    let mut executor = LingoExecutor::new();
    executor.set_database(database);
    
    // Complex chain: Find technical, get similar, go up to concepts
    let query = QueryBuilder::find("technical")
        .similar_threshold(0.8)
        .layer_up()
        .compile();
    let result = executor.execute(&query).unwrap();
    
    // Should find technical_concepts
    assert!(result.nodes.len() >= 1);
    
    if let Some(db) = &executor.database {
        let concepts: Vec<String> = result.nodes.as_slice()
            .iter()
            .filter_map(|id| db.get_node_word(*id).ok())
            .map(|s| s.to_string())
            .collect();
        
        assert!(concepts.contains(&"technical_concepts".to_string()));
    }
}

#[test]
fn test_bytecode_compilation() {
    // Test that queries compile to expected bytecode
    let query = QueryBuilder::find("test")
        .similar()
        .layer_up()
        .limit(10)
        .compile();
    
    // Should have LoadNode, FindSimilar, LayerUp, Limit, Halt
    assert_eq!(query.bytecode.len(), 5);
    assert_eq!(query.bytecode[0].opcode, bytecode::SlangOp::LoadNode);
    assert_eq!(query.bytecode[1].opcode, bytecode::SlangOp::FindSimilar);
    assert_eq!(query.bytecode[2].opcode, bytecode::SlangOp::LayerUp);
    assert_eq!(query.bytecode[3].opcode, bytecode::SlangOp::Limit);
    assert_eq!(query.bytecode[4].opcode, bytecode::SlangOp::Halt);
}

#[test]
fn test_node_packed_struct_size() {
    // Verify the node struct is exactly 60 bytes as specified
    assert_eq!(std::mem::size_of::<LinguisticNode>(), 60);
}

#[test]
fn test_connection_packed_struct_size() {
    // Verify the connection struct is exactly 20 bytes as specified
    assert_eq!(std::mem::size_of::<OrthogonalConnection>(), 20);
}

#[test]
fn test_file_header_size() {
    // Verify the header is exactly 512 bytes
    assert_eq!(std::mem::size_of::<LingoFileHeader>(), 512);
}
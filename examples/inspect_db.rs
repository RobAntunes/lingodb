//! Inspect contents of a Lingo database

use lingo::storage::MemoryMappedDatabase;
use lingo::core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "comprehensive.lingo";
    println!("Opening database: {}", db_path);
    
    let database = MemoryMappedDatabase::open(db_path)?;
    println!("\nDatabase Info:");
    println!("  Total nodes: {}", database.node_count());
    println!("  Total connections: {}", database.connection_count());
    
    // Count nodes by layer
    let mut letter_count = 0;
    let mut phoneme_count = 0;
    let mut morpheme_count = 0;
    let mut word_count = 0;
    let mut other_count = 0;
    
    for i in 0..database.node_count() {
        if let Ok(node) = database.get_node(NodeId(i as u32)) {
            match node.layer {
                Layer::Letters => letter_count += 1,
                Layer::Phonemes => phoneme_count += 1,
                Layer::Morphemes => morpheme_count += 1,
                Layer::Words => word_count += 1,
                _ => other_count += 1,
            }
        }
    }
    
    println!("\nNodes by layer:");
    println!("  Letters: {}", letter_count);
    println!("  Phonemes: {}", phoneme_count);
    println!("  Morphemes: {}", morpheme_count);
    println!("  Words: {}", word_count);
    println!("  Other: {}", other_count);
    
    // Show sample morphemes
    println!("\nSample morphemes:");
    let mut shown = 0;
    for i in 0..database.node_count() {
        if shown >= 10 { break; }
        
        let node_id = NodeId(i as u32);
        if let Ok(node) = database.get_node(node_id) {
            if node.layer == Layer::Morphemes {
                if let Ok(word) = database.get_node_word(node_id) {
                    // Get etymology from flags
                    let etymology = if node.flags.contains(NodeFlags::IS_TECHNICAL) {
                        "Technical"
                    } else {
                        "Unknown"
                    };
                    println!("  - {} ({:?}, {})", word, node.morpheme_type, etymology);
                    shown += 1;
                }
            }
        }
    }
    
    Ok(())
}
//! Debug the database to understand string matching

use lingo::storage::MemoryMappedDatabase;
use lingo::core::error::Result;

fn main() -> Result<()> {
    let db_path = "example.lingo";
    let database = MemoryMappedDatabase::open(db_path)?;
    
    println!("Database has {} nodes", database.node_count());
    
    // Check each node's word
    for i in 0..database.node_count() {
        let node_id = lingo::core::NodeId(i as u32 + 1); // Node IDs start from 1
        match database.get_node(node_id) {
            Ok(node) => {
                match database.get_node_word(node_id) {
                    Ok(word) => {
                        let offset = node.word_offset;
                        let length = node.word_length;
                        println!("Node {}: word='{}', offset={}, length={}", 
                            node_id.0, word, offset, length);
                    }
                    Err(e) => println!("Node {}: Error getting word: {}", node_id.0, e),
                }
            }
            Err(e) => println!("Node {}: Error getting node: {}", node_id.0, e),
        }
    }
    
    // Try to find nodes by word
    println!("\nSearching for 'technical':");
    let results = database.find_nodes_by_word("technical");
    println!("Found {} nodes", results.len());
    
    Ok(())
}
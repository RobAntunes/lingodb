//! More detailed debug of the database

use lingo::storage::MemoryMappedDatabase;
use lingo::core::error::Result;

fn main() -> Result<()> {
    let db_path = "example.lingo";
    let database = MemoryMappedDatabase::open(db_path)?;
    
    println!("Database has {} nodes", database.node_count());
    
    // Look at raw node data
    for i in 0..3.min(database.node_count()) {
        let node_id = lingo::core::NodeId(i as u32 + 1);
        if let Ok(node) = database.get_node(node_id) {
            println!("\nNode {}:", node_id.0);
            let word_offset = node.word_offset;
            let word_length = node.word_length;
            println!("  word_offset: {}", word_offset);
            println!("  word_length: {}", word_length);
            
            // Try to read the string
            match database.get_node_word(node_id) {
                Ok(word) => println!("  word: '{}'", word),
                Err(e) => println!("  Error: {}", e),
            }
            
            // Show raw position data
            let pos = node.position;
            println!("  position: ({:.2}, {:.2}, {:.2})", pos.x, pos.y, pos.z);
        }
    }
    
    // Look at string table info
    println!("\nString table info:");
    if let Ok(string_table) = database.string_table() {
        println!("  Size: {} bytes", string_table.size());
    }
    
    Ok(())
}
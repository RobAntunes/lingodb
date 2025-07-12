//! Debug connections in the database

use lingo::storage::MemoryMappedDatabase;
use lingo::core::error::Result;

fn main() -> Result<()> {
    let db_path = "example.lingo";
    let database = MemoryMappedDatabase::open(db_path)?;
    
    println!("=== CONNECTION DEBUG ===\n");
    println!("Total connections: {}", database.connection_count());
    
    // Check connections for each node
    for i in 0..database.node_count() {
        let node_id = lingo::core::NodeId(i as u32 + 1);
        
        if let Ok(node) = database.get_node(node_id) {
            if let Ok(word) = database.get_node_word(node_id) {
                println!("\nNode {}: '{}'", node_id.0, word);
                
                // Get connections for this node
                let connections_offset = node.connections_offset;
                let connections_count = node.connections_count;
                
                match database.get_node_connections(node_id) {
                    Ok(connections) => {
                        println!("  Connections: {} (from offset {}, count {})", 
                            connections.len(), 
                            connections_offset,
                            connections_count
                        );
                        
                        for (j, conn) in connections.iter().enumerate() {
                            // Copy fields to avoid alignment issues
                            let target = conn.target_node;
                            let conn_type = conn.connection_type;
                            let strength = conn.strength;
                            
                            // Get target word
                            let target_word = if let Ok(_target_node) = database.get_node(target) {
                                database.get_node_word(target).unwrap_or("?").to_string()
                            } else {
                                format!("Node {}", target.0)
                            };
                            
                            println!("    [{}] -> {} (type: {:?}, strength: {:.2})", 
                                j, target_word, conn_type, strength);
                        }
                    }
                    Err(e) => println!("  Error getting connections: {}", e),
                }
            }
        }
    }
    
    Ok(())
}
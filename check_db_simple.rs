use lingo::storage::LingoDatabase;
use lingo::core::{Layer, NodeId};

fn main() {
    let db = LingoDatabase::open("english.lingo").expect("Failed to open database");
    
    println!("Total nodes: {}\n", db.node_count());
    
    // Just show first 50 words
    println!("First 50 words found:");
    let mut word_count = 0;
    
    for i in 0..db.node_count() {
        if word_count >= 50 { break; }
        
        let node_id = NodeId(i as u32);
        if let Ok(node) = db.get_node(node_id) {
            if matches!(node.layer, Layer::Words) {
                if let Ok(word) = db.get_node_word(node_id) {
                    println!("  {}: {}", word_count + 1, word);
                    word_count += 1;
                }
            }
        }
    }
}
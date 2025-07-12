//! Debug the header to understand file layout

use lingo::storage::MemoryMappedDatabase;
use lingo::core::error::Result;

fn main() -> Result<()> {
    let db_path = "example.lingo";
    let database = MemoryMappedDatabase::open(db_path)?;
    
    let header = database.header();
    
    println!("File Layout:");
    println!("  File size: {} bytes", header.file_size);
    println!("  Node count: {}", header.node_count);
    println!("  Connection count: {}", header.connection_count);
    
    println!("\nSection Offsets:");
    println!("  String table: offset={}, size={}", 
        header.string_table_offset, header.string_table_size);
    println!("  Node array: offset={}, size={}", 
        header.node_array_offset, header.node_array_size);
    println!("  Connection array: offset={}, size={}", 
        header.connection_array_offset, header.connection_array_size);
    println!("  Octree: offset={}, size={}", 
        header.octree_offset, header.octree_size);
    
    Ok(())
}
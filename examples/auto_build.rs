//! Example of automatic database building

use lingo::discovery::AutoLinguisticBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Lingo Auto-Builder Demo\n");
    
    let mut builder = AutoLinguisticBuilder::new();
    
    // Add some words and watch the magic happen!
    let words = vec![
        // Technical terms
        "technology", "biotechnology", "nanotechnology",
        "preprocessing", "postprocessing",
        "antimicrobial", "antibacterial",
        "interconnected", "interdisciplinary",
        
        // Common words that share morphemes
        "biology", "geology", "mythology",
        "biological", "geological", "mythological",
        
        // Words with clear morpheme boundaries
        "unhappy", "happiness", "happily",
        "transformation", "transformer", "transformative",
        
        // Cross-domain words
        "viral", "virus", "antiviral",
        "network", "networking", "networked",
    ];
    
    println!("Building linguistic database with automatic discovery...\n");
    
    for word in words {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        builder.add_word(word)?;
    }
    
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔗 Discovering connections...");
    builder.discover_connections()?;
    
    println!("\n💾 Building database file...");
    builder.build("auto_lingo.db")?;
    
    println!("\n✅ Auto-built linguistic database saved to 'auto_lingo.db'!");
    println!("\nThe system automatically discovered:");
    println!("  • Letter decomposition");
    println!("  • Phoneme generation");
    println!("  • Morpheme boundaries");
    println!("  • Etymology guessing");
    println!("  • 3D positioning");
    println!("  • Hierarchical connections");
    
    Ok(())
}
//! Build the standard English Lingo database

use lingo::data::DatabaseSeeder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║          LINGO STANDARD ENGLISH DATABASE BUILDER                 ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");
    
    let mut seeder = DatabaseSeeder::new();
    
    // Seed with standard English data
    seeder.seed_english()?;
    
    // Get statistics before building
    let stats = seeder.get_stats();
    println!("\n📊 Database Statistics:");
    println!("   Letters:    {} nodes", stats.letters);
    println!("   Phonemes:   {} nodes", stats.phonemes);
    println!("   Morphemes:  {} nodes", stats.morphemes);
    println!("   ---------------------");
    println!("   Total:      {} nodes", stats.total_nodes);
    
    // Build the database
    println!("\n💾 Building database file...");
    seeder.build("english.lingo")?;
    
    // Check file size
    let metadata = std::fs::metadata("english.lingo")?;
    let size_kb = metadata.len() as f64 / 1024.0;
    
    println!("\n✅ Standard English database created!");
    println!("   File: english.lingo");
    println!("   Size: {:.2} KB", size_kb);
    
    println!("\n🎯 This database includes:");
    println!("   • All 26 English letters");
    println!("   • {} IPA phonemes", lingo::data::english_base::ENGLISH_PHONEMES.len());
    println!("   • {} common prefixes", lingo::data::english_base::ENGLISH_PREFIXES.len());
    println!("   • {} common suffixes", lingo::data::english_base::ENGLISH_SUFFIXES.len());
    println!("   • {} common roots", lingo::data::english_base::ENGLISH_ROOTS.len());
    println!("\n   Ready for immediate use! Users can:");
    println!("   • Add their own words and discover connections");
    println!("   • Query morphological relationships");
    println!("   • Navigate the linguistic hierarchy");
    println!("   • Build domain-specific vocabularies on top");
    
    Ok(())
}
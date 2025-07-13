//! Build comprehensive Lingo database with scraped data

use lingo::data::data_integration;
use lingo::storage::DatabaseBuilder;
use lingo::core::*;
use lingo::data;
use lingo::core::error;
use std::collections::HashMap;

fn main() -> error::Result<()> {
    println!("🚀 Building comprehensive Lingo database with scraped data...\n");
    
    // Create enhanced seeder
    let mut seeder = ComprehensiveSeeder::new();
    
    // Seed with all data
    seeder.seed_comprehensive()?;
    
    // Build the database
    seeder.build("comprehensive.lingo")?;
    
    Ok(())
}

/// Enhanced seeder that includes scraped data
struct ComprehensiveSeeder {
    builder: DatabaseBuilder,
    letter_nodes: HashMap<String, NodeId>,
    phoneme_nodes: HashMap<String, NodeId>,
    morpheme_nodes: HashMap<String, NodeId>,
    word_nodes: HashMap<String, NodeId>,
}

impl ComprehensiveSeeder {
    fn new() -> Self {
        Self {
            builder: DatabaseBuilder::new(),
            letter_nodes: HashMap::new(),
            phoneme_nodes: HashMap::new(),
            morpheme_nodes: HashMap::new(),
            word_nodes: HashMap::new(),
        }
    }
    
    fn seed_comprehensive(&mut self) -> error::Result<()> {
        println!("🌱 Seeding comprehensive database...\n");
        
        // Use base seeder for letters and phonemes
        self.seed_letters()?;
        self.seed_phonemes()?;
        
        // Seed with combined morpheme data
        self.seed_all_morphemes()?;
        
        // Add semantic connections from scraped data
        self.seed_semantic_connections()?;
        
        // Create cross-layer connections
        self.create_comprehensive_connections()?;
        
        println!("\n✅ Comprehensive seeding complete!");
        Ok(())
    }
    
    fn seed_letters(&mut self) -> error::Result<()> {
        println!("📝 Adding letters...");
        
        // Reuse logic from base seeder
        for (i, (letter, letter_type)) in lingo::data::english_base::ENGLISH_LETTERS.iter().enumerate() {
            let x = 0.05 + (i % 7) as f32 * 0.13;
            let y = 0.1 + (i / 7) as f32 * 0.2;
            let z = Layer::Letters.z_center();
            
            let position = Coordinate3D { x, y, z };
            
            let flags = match *letter_type {
                "vowel" => NodeFlags::IS_FREQUENT,
                _ => NodeFlags::empty(),
            };
            
            let id = self.builder.add_node_full(
                letter,
                Layer::Letters,
                position,
                EtymologyOrigin::Germanic,
                MorphemeType::Root,
                flags,
            )?;
            
            self.letter_nodes.insert(letter.to_string(), id);
        }
        
        println!("  ✓ Added {} letters", self.letter_nodes.len());
        Ok(())
    }
    
    fn seed_phonemes(&mut self) -> error::Result<()> {
        println!("🔊 Adding phonemes...");
        
        // Reuse logic from base seeder
        for (phoneme, phoneme_type, _example) in lingo::data::english_base::ENGLISH_PHONEMES {
            let (x, y) = calculate_phoneme_position(phoneme_type);
            let z = Layer::Phonemes.z_center();
            
            let position = Coordinate3D { x, y, z };
            
            let flags = if phoneme_type.contains("vowel") || *phoneme == "/ə/" {
                NodeFlags::IS_FREQUENT
            } else {
                NodeFlags::empty()
            };
            
            let id = self.builder.add_node_full(
                phoneme,
                Layer::Phonemes,
                position,
                EtymologyOrigin::Unknown,
                MorphemeType::Root,
                flags,
            )?;
            
            self.phoneme_nodes.insert(phoneme.to_string(), id);
        }
        
        println!("  ✓ Added {} phonemes", self.phoneme_nodes.len());
        Ok(())
    }
    
    fn seed_all_morphemes(&mut self) -> error::Result<()> {
        println!("🧩 Adding morphemes from all sources...");
        
        let mut prefix_count = 0;
        let mut suffix_count = 0;
        let mut root_count = 0;
        
        // Get combined morpheme data
        let all_prefixes = data_integration::get_all_prefixes();
        let all_suffixes = data_integration::get_all_suffixes();
        let all_roots = data_integration::get_all_roots();
        
        // Add all prefixes
        for (i, prefix_data) in all_prefixes.iter().enumerate() {
            
            let position = calculate_morpheme_position(prefix_data, i, all_prefixes.len());
            
            let flags = if prefix_data.productivity > 0.8 {
                NodeFlags::IS_PRODUCTIVE | NodeFlags::IS_FREQUENT
            } else if prefix_data.productivity > 0.6 {
                NodeFlags::IS_PRODUCTIVE
            } else {
                NodeFlags::empty()
            };
            
            let id = self.builder.add_node_full(
                prefix_data.morpheme,
                Layer::Morphemes,
                position,
                prefix_data.etymology,
                prefix_data.morph_type,
                flags,
            )?;
            
            self.morpheme_nodes.insert(prefix_data.morpheme.to_string(), id);
            prefix_count += 1;
        }
        
        // Add all suffixes
        for (i, suffix_data) in all_suffixes.iter().enumerate() {
            
            let position = calculate_morpheme_position(suffix_data, i, all_suffixes.len());
            
            let flags = if suffix_data.productivity > 0.8 {
                NodeFlags::IS_PRODUCTIVE | NodeFlags::IS_FREQUENT
            } else if suffix_data.productivity > 0.6 {
                NodeFlags::IS_PRODUCTIVE
            } else {
                NodeFlags::empty()
            };
            
            let id = self.builder.add_node_full(
                suffix_data.morpheme,
                Layer::Morphemes,
                position,
                suffix_data.etymology,
                suffix_data.morph_type,
                flags,
            )?;
            
            self.morpheme_nodes.insert(suffix_data.morpheme.to_string(), id);
            suffix_count += 1;
        }
        
        // Add all roots
        for (i, root_data) in all_roots.iter().enumerate() {
            
            let position = calculate_morpheme_position(root_data, i, all_roots.len());
            
            let flags = if root_data.productivity > 0.8 {
                NodeFlags::IS_PRODUCTIVE | NodeFlags::IS_TECHNICAL
            } else if root_data.productivity > 0.6 {
                NodeFlags::IS_PRODUCTIVE
            } else {
                NodeFlags::IS_TECHNICAL
            };
            
            let id = self.builder.add_node_full(
                root_data.morpheme,
                Layer::Morphemes,
                position,
                root_data.etymology,
                root_data.morph_type,
                flags,
            )?;
            
            self.morpheme_nodes.insert(root_data.morpheme.to_string(), id);
            root_count += 1;
        }
        
        println!("  ✓ Added {} morphemes ({} prefixes, {} suffixes, {} roots)", 
                 prefix_count + suffix_count + root_count, prefix_count, suffix_count, root_count);
        Ok(())
    }
    
    fn seed_semantic_connections(&mut self) -> error::Result<()> {
        println!("🔗 Adding semantic connections from scraped data...");
        
        let mut connection_count = 0;
        
        // Add sample semantic connections from scraped data
        for conn in lingo::data::scraped_data::SCRAPED_SEMANTIC_CONNECTIONS.iter().take(1000) {
            // Only add connections if both morphemes exist
            if let (Some(&source_id), Some(&target_id)) = 
                (self.morpheme_nodes.get(conn.source), self.morpheme_nodes.get(conn.target)) {
                
                let conn_type = match conn.connection_type {
                    "hypernym" => ConnectionType::Hypernymy,
                    "hyponym" => ConnectionType::Hyponymy,
                    "antonym" => ConnectionType::Antonymy,
                    "similar" => ConnectionType::Synonymy,
                    "meronym" => ConnectionType::Meronymy,
                    "holonym" => ConnectionType::Meronymy,
                    _ => ConnectionType::Analogy,
                };
                
                self.builder.add_connection(source_id, target_id, conn_type, conn.strength)?;
                connection_count += 1;
            }
        }
        
        println!("  ✓ Added {} semantic connections", connection_count);
        Ok(())
    }
    
    fn create_comprehensive_connections(&mut self) -> error::Result<()> {
        println!("🔗 Creating cross-layer connections...");
        
        // Connect letters to phonemes (sample connections)
        let letter_nodes = self.letter_nodes.clone();
        let phoneme_nodes = self.phoneme_nodes.clone();
        self.connect_if_exists(&letter_nodes, "a", &phoneme_nodes, "/æ/", 0.8)?;
        self.connect_if_exists(&letter_nodes, "e", &phoneme_nodes, "/ɛ/", 0.8)?;
        self.connect_if_exists(&letter_nodes, "i", &phoneme_nodes, "/ɪ/", 0.8)?;
        self.connect_if_exists(&letter_nodes, "o", &phoneme_nodes, "/ɑ/", 0.7)?;
        self.connect_if_exists(&letter_nodes, "u", &phoneme_nodes, "/ʌ/", 0.7)?;
        
        // Connect common morpheme patterns
        self.connect_morpheme_patterns()?;
        
        println!("  ✓ Created comprehensive connections");
        Ok(())
    }
    
    fn connect_if_exists(&mut self, 
                        source_map: &HashMap<String, NodeId>, 
                        source_key: &str,
                        target_map: &HashMap<String, NodeId>,
                        target_key: &str,
                        strength: f32) -> error::Result<()> {
        let source_id = source_map.get(source_key).copied();
        let target_id = target_map.get(target_key).copied();
        
        if let (Some(source_id), Some(target_id)) = (source_id, target_id) {
            self.builder.add_connection(source_id, target_id, ConnectionType::Hypernymy, strength)?;
        }
        Ok(())
    }
    
    fn connect_morpheme_patterns(&mut self) -> error::Result<()> {
        // Clone the morpheme nodes to avoid borrowing issues
        let morpheme_nodes = self.morpheme_nodes.clone();
        
        // Connect common prefix-root patterns
        self.connect_if_exists(&morpheme_nodes, "un", &morpheme_nodes, "do", 0.7)?;
        self.connect_if_exists(&morpheme_nodes, "re", &morpheme_nodes, "do", 0.8)?;
        self.connect_if_exists(&morpheme_nodes, "pre", &morpheme_nodes, "fix", 0.7)?;
        
        // Connect common root-suffix patterns
        self.connect_if_exists(&morpheme_nodes, "teach", &morpheme_nodes, "er", 0.8)?;
        self.connect_if_exists(&morpheme_nodes, "build", &morpheme_nodes, "er", 0.8)?;
        self.connect_if_exists(&morpheme_nodes, "happy", &morpheme_nodes, "ness", 0.8)?;
        
        Ok(())
    }
    
    fn build(mut self, path: &str) -> error::Result<()> {
        self.builder.build(path)
    }
}

// Helper functions
fn calculate_phoneme_position(phoneme_type: &str) -> (f32, f32) {
    let x = match phoneme_type {
        s if s.contains("bilabial") => 0.1,
        s if s.contains("labiodental") => 0.2,
        s if s.contains("dental") => 0.3,
        s if s.contains("alveolar") => 0.4,
        s if s.contains("postalveolar") => 0.5,
        s if s.contains("palatal") => 0.6,
        s if s.contains("velar") => 0.7,
        s if s.contains("glottal") => 0.8,
        s if s.contains("front") => 0.3,
        s if s.contains("central") || s.contains("schwa") => 0.5,
        s if s.contains("back") => 0.7,
        _ => 0.5,
    };
    
    let y = match phoneme_type {
        s if s.contains("plosive") => 0.1,
        s if s.contains("fricative") => 0.3,
        s if s.contains("affricate") => 0.4,
        s if s.contains("nasal") => 0.5,
        s if s.contains("lateral") => 0.6,
        s if s.contains("approximant") => 0.7,
        s if s.contains("close") => 0.2,
        s if s.contains("mid") => 0.5,
        s if s.contains("open") => 0.8,
        _ => 0.5,
    };
    
    (x, y)
}

fn calculate_morpheme_position(morpheme: &data::english_base::MorphemeData, index: usize, total: usize) -> Coordinate3D {
    // X-axis: morpheme type
    let x_base = match morpheme.morph_type {
        MorphemeType::Prefix => 0.2,
        MorphemeType::Root => 0.5,
        MorphemeType::Suffix => 0.8,
        _ => 0.5,
    };
    
    // Add variation based on index
    let x = x_base + ((index as f32 / total as f32) - 0.5) * 0.2;
    
    // Y-axis: etymology with variation
    let y_base = morpheme.etymology.base_y_coordinate();
    let y = y_base + (morpheme.productivity - 0.7) * 0.1;
    
    // Z-axis: layer center with slight variation
    let z = Layer::Morphemes.z_center() + (morpheme.productivity - 0.7) * 0.05;
    
    Coordinate3D { x, y, z }
}
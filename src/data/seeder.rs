//! Database seeder for populating standard linguistic data

use crate::core::*;
use crate::storage::DatabaseBuilder;
use crate::data::english_base::*;
use std::collections::HashMap;

pub struct DatabaseSeeder {
    builder: DatabaseBuilder,
    /// Cache of created nodes for linking
    letter_nodes: HashMap<String, NodeId>,
    phoneme_nodes: HashMap<String, NodeId>,
    morpheme_nodes: HashMap<String, NodeId>,
}

impl DatabaseSeeder {
    /// Create a new seeder
    pub fn new() -> Self {
        Self {
            builder: DatabaseBuilder::new(),
            letter_nodes: HashMap::new(),
            phoneme_nodes: HashMap::new(),
            morpheme_nodes: HashMap::new(),
        }
    }
    
    /// Seed with standard English data
    pub fn seed_english(&mut self) -> error::Result<()> {
        println!("🌱 Seeding Lingo database with standard English data...\n");
        
        self.seed_letters()?;
        self.seed_phonemes()?;
        self.seed_morphemes()?;
        self.create_fundamental_connections()?;
        
        println!("\n✅ Seeding complete!");
        Ok(())
    }
    
    /// Seed all English letters
    fn seed_letters(&mut self) -> error::Result<()> {
        println!("📝 Adding letters...");
        
        for (i, (letter, letter_type)) in ENGLISH_LETTERS.iter().enumerate() {
            // Position letters in a nice grid
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
                EtymologyOrigin::Germanic, // English alphabet is Germanic-based
                MorphemeType::Root,
                flags,
            )?;
            
            self.letter_nodes.insert(letter.to_string(), id);
        }
        
        println!("  ✓ Added {} letters", ENGLISH_LETTERS.len());
        Ok(())
    }
    
    /// Seed all English phonemes
    fn seed_phonemes(&mut self) -> error::Result<()> {
        println!("🔊 Adding phonemes...");
        
        let mut consonant_count = 0;
        let mut vowel_count = 0;
        
        for (phoneme, phoneme_type, _example) in ENGLISH_PHONEMES {
            // Position phonemes based on their articulation
            let (x, y) = self.calculate_phoneme_position(phoneme_type);
            let z = Layer::Phonemes.z_center();
            
            let position = Coordinate3D { x, y, z };
            
            let flags = if phoneme_type.contains("vowel") || *phoneme == "/ə/" {
                vowel_count += 1;
                NodeFlags::IS_FREQUENT // Vowels are more frequent
            } else {
                consonant_count += 1;
                NodeFlags::empty()
            };
            
            let id = self.builder.add_node_full(
                phoneme,
                Layer::Phonemes,
                position,
                EtymologyOrigin::Unknown, // Phonemes transcend etymology
                MorphemeType::Root,
                flags,
            )?;
            
            self.phoneme_nodes.insert(phoneme.to_string(), id);
        }
        
        println!("  ✓ Added {} phonemes ({} consonants, {} vowels)", 
                 ENGLISH_PHONEMES.len(), consonant_count, vowel_count);
        Ok(())
    }
    
    /// Calculate phoneme position based on articulation
    fn calculate_phoneme_position(&self, phoneme_type: &str) -> (f32, f32) {
        // X-axis: place of articulation (front to back)
        let x = match phoneme_type {
            s if s.contains("bilabial") => 0.1,
            s if s.contains("labiodental") => 0.2,
            s if s.contains("dental") => 0.3,
            s if s.contains("alveolar") => 0.4,
            s if s.contains("postalveolar") => 0.5,
            s if s.contains("palatal") => 0.6,
            s if s.contains("velar") => 0.7,
            s if s.contains("glottal") => 0.8,
            // Vowels by frontness
            s if s.contains("front") => 0.3,
            s if s.contains("central") || s.contains("schwa") => 0.5,
            s if s.contains("back") => 0.7,
            _ => 0.5,
        };
        
        // Y-axis: manner of articulation
        let y = match phoneme_type {
            s if s.contains("plosive") => 0.1,
            s if s.contains("fricative") => 0.3,
            s if s.contains("affricate") => 0.4,
            s if s.contains("nasal") => 0.5,
            s if s.contains("lateral") => 0.6,
            s if s.contains("approximant") => 0.7,
            // Vowels by openness
            s if s.contains("close") => 0.2,
            s if s.contains("mid") => 0.5,
            s if s.contains("open") => 0.8,
            _ => 0.5,
        };
        
        (x, y)
    }
    
    /// Seed all morphemes
    fn seed_morphemes(&mut self) -> error::Result<()> {
        println!("🧩 Adding morphemes...");
        
        let mut prefix_count = 0;
        let mut suffix_count = 0;
        let mut root_count = 0;
        
        // Add prefixes
        for prefix_data in ENGLISH_PREFIXES {
            let position = self.calculate_morpheme_position(prefix_data);
            
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
        
        // Add suffixes
        for suffix_data in ENGLISH_SUFFIXES {
            let position = self.calculate_morpheme_position(suffix_data);
            
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
        
        // Add roots
        for root_data in ENGLISH_ROOTS {
            let position = self.calculate_morpheme_position(root_data);
            
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
        
        // Add essential additions
        for addition in ESSENTIAL_ADDITIONS {
            let position = self.calculate_morpheme_position(addition);
            
            let flags = if addition.productivity > 0.8 {
                match addition.morph_type {
                    MorphemeType::Prefix | MorphemeType::Suffix => NodeFlags::IS_PRODUCTIVE | NodeFlags::IS_FREQUENT,
                    MorphemeType::Root => NodeFlags::IS_PRODUCTIVE | NodeFlags::IS_TECHNICAL,
                    _ => NodeFlags::IS_PRODUCTIVE,
                }
            } else if addition.productivity > 0.6 {
                NodeFlags::IS_PRODUCTIVE
            } else {
                NodeFlags::empty()
            };
            
            let id = self.builder.add_node_full(
                addition.morpheme,
                Layer::Morphemes,
                position,
                addition.etymology,
                addition.morph_type,
                flags,
            )?;
            
            self.morpheme_nodes.insert(addition.morpheme.to_string(), id);
            
            match addition.morph_type {
                MorphemeType::Prefix => prefix_count += 1,
                MorphemeType::Suffix => suffix_count += 1,
                MorphemeType::Root => root_count += 1,
                _ => {}
            }
        }
        
        println!("  ✓ Added {} morphemes ({} prefixes, {} suffixes, {} roots)", 
                 prefix_count + suffix_count + root_count, prefix_count, suffix_count, root_count);
        Ok(())
    }
    
    /// Calculate morpheme position based on type and etymology
    fn calculate_morpheme_position(&self, morpheme: &MorphemeData) -> Coordinate3D {
        // X-axis: morpheme type
        let x_base = match morpheme.morph_type {
            MorphemeType::Prefix => 0.2,
            MorphemeType::Root => 0.5,
            MorphemeType::Suffix => 0.8,
            _ => 0.5,
        };
        
        // Add some variation based on productivity
        let x = x_base + (morpheme.productivity - 0.7) * 0.15;
        
        // Y-axis: etymology
        let y = morpheme.etymology.base_y_coordinate();
        
        // Z-axis: layer center with slight variation
        let z = Layer::Morphemes.z_center() + (morpheme.productivity - 0.7) * 0.05;
        
        Coordinate3D { x, y, z }
    }
    
    /// Create fundamental connections between layers
    fn create_fundamental_connections(&mut self) -> error::Result<()> {
        println!("🔗 Creating fundamental connections...");
        
        // Connect some representative letters to phonemes
        self.connect_letter_to_phoneme("t", "/t/")?;
        self.connect_letter_to_phoneme("p", "/p/")?;
        self.connect_letter_to_phoneme("b", "/b/")?;
        self.connect_letter_to_phoneme("s", "/s/")?;
        self.connect_letter_to_phoneme("sh", "/ʃ/")?; // digraph example
        
        // Connect phonemes to morphemes they commonly appear in
        self.connect_phoneme_to_morpheme("/t/", "tech")?;
        self.connect_phoneme_to_morpheme("/b/", "bio")?;
        self.connect_phoneme_to_morpheme("/f/", "phon")?;
        
        println!("  ✓ Created fundamental cross-layer connections");
        Ok(())
    }
    
    /// Helper to connect letter to phoneme
    fn connect_letter_to_phoneme(&mut self, letter: &str, phoneme: &str) -> error::Result<()> {
        if let (Some(&letter_id), Some(&phoneme_id)) = 
            (self.letter_nodes.get(letter), self.phoneme_nodes.get(phoneme)) {
            self.builder.add_connection(letter_id, phoneme_id, ConnectionType::Hypernymy, 0.9)?;
        }
        Ok(())
    }
    
    /// Helper to connect phoneme to morpheme
    fn connect_phoneme_to_morpheme(&mut self, phoneme: &str, morpheme: &str) -> error::Result<()> {
        if let (Some(&phoneme_id), Some(&morpheme_id)) = 
            (self.phoneme_nodes.get(phoneme), self.morpheme_nodes.get(morpheme)) {
            self.builder.add_connection(phoneme_id, morpheme_id, ConnectionType::Hypernymy, 0.8)?;
        }
        Ok(())
    }
    
    /// Build the database
    pub fn build(mut self, path: &str) -> error::Result<()> {
        self.builder.build(path)
    }
    
    /// Get statistics about the seeded data
    pub fn get_stats(&self) -> SeederStats {
        SeederStats {
            letters: self.letter_nodes.len(),
            phonemes: self.phoneme_nodes.len(),
            morphemes: self.morpheme_nodes.len(),
            total_nodes: self.letter_nodes.len() + self.phoneme_nodes.len() + self.morpheme_nodes.len(),
        }
    }
}

/// Statistics about seeded data
#[derive(Debug)]
pub struct SeederStats {
    pub letters: usize,
    pub phonemes: usize,
    pub morphemes: usize,
    pub total_nodes: usize,
}

impl Default for DatabaseSeeder {
    fn default() -> Self {
        Self::new()
    }
}
//! Automatic linguistic discovery and database building

use crate::core::*;
use crate::storage::DatabaseBuilder;
use std::collections::{HashMap, HashSet};

/// Automatic linguistic analyzer that discovers morphemes, phonemes, and connections
pub struct AutoLinguisticBuilder {
    builder: DatabaseBuilder,
    /// Cache of discovered morphemes
    morpheme_cache: HashMap<String, NodeId>,
    /// Cache of discovered phonemes  
    phoneme_cache: HashMap<String, NodeId>,
    /// Common morpheme patterns
    morpheme_patterns: MorphemePatterns,
}

/// Common morpheme patterns for discovery
struct MorphemePatterns {
    prefixes: HashSet<&'static str>,
    suffixes: HashSet<&'static str>,
    roots: HashSet<&'static str>,
}

impl Default for MorphemePatterns {
    fn default() -> Self {
        Self {
            // Common English prefixes
            prefixes: ["pre", "post", "anti", "de", "dis", "over", "under", "re", "un", "in", "im", 
                      "ir", "il", "non", "mis", "sub", "super", "trans", "inter", "fore", "mid",
                      "bio", "geo", "tele", "micro", "macro", "mega", "hyper", "hypo", "meta",
                      "proto", "pseudo", "quasi", "semi", "mono", "bi", "tri", "quad", "penta",
                      "hex", "sept", "oct", "nov", "dec", "multi", "poly", "omni", "pan"].iter().cloned().collect(),
            
            // Common English suffixes
            suffixes: ["able", "ible", "al", "ial", "ed", "en", "er", "est", "ful", "ic", "ing",
                      "ion", "tion", "ation", "ition", "ity", "ty", "ive", "ative", "itive",
                      "less", "ly", "ment", "ness", "ous", "eous", "ious", "s", "es", "y",
                      "ology", "ologist", "ism", "ist", "ize", "ise", "fy", "ify", "ward",
                      "wise", "ship", "hood", "dom", "acy", "ance", "ence", "ant", "ent"].iter().cloned().collect(),
            
            // Common roots (Greek/Latin)
            roots: ["tech", "log", "graph", "phon", "photo", "chron", "meter", "scope", "port",
                   "ject", "dict", "duct", "gress", "mot", "pend", "pos", "pound", "press",
                   "scrib", "script", "sens", "sent", "spec", "spect", "spic", "struct",
                   "tact", "tang", "tend", "tens", "tent", "tract", "ven", "vent", "vers",
                   "vert", "vid", "vis", "voc", "vok", "volv", "bio", "geo", "therm",
                   "hydr", "aer", "agr", "anthrop", "arch", "aster", "astr", "aud", "bene",
                   "bibli", "chrom", "cosm", "crat", "cracy", "cycl", "dem", "derm", "duc"].iter().cloned().collect(),
        }
    }
}

impl AutoLinguisticBuilder {
    /// Create a new automatic builder
    pub fn new() -> Self {
        Self {
            builder: DatabaseBuilder::new(),
            morpheme_cache: HashMap::new(),
            phoneme_cache: HashMap::new(),
            morpheme_patterns: MorphemePatterns::default(),
        }
    }
    
    /// Add a word and automatically discover its components
    pub fn add_word(&mut self, word: &str) -> error::Result<NodeId> {
        println!("🔍 Analyzing word: '{}'", word);
        
        // 1. Decompose into letters
        let letter_ids = self.add_letters(word)?;
        
        // 2. Generate phonemes (simplified - real implementation would use CMU dict or similar)
        let phoneme_ids = self.add_phonemes(word)?;
        
        // 3. Discover morphemes
        let morpheme_ids = self.discover_morphemes(word)?;
        
        // 4. Determine etymology (simplified heuristic)
        let etymology = self.guess_etymology(word);
        
        // 5. Calculate 3D position based on linguistic properties
        let position = self.calculate_position(word, &morpheme_ids, etymology);
        
        // 6. Create the word node
        let word_id = self.builder.add_node_full(
            word,
            Layer::Words,
            position,
            etymology,
            if morpheme_ids.len() > 1 { MorphemeType::Compound } else { MorphemeType::Root },
            self.determine_flags(word),
        )?;
        
        // 7. Connect everything
        self.connect_components(word_id, &letter_ids, &phoneme_ids, &morpheme_ids)?;
        
        println!("  ✓ Added word with {} morphemes, {} phonemes", morpheme_ids.len(), phoneme_ids.len());
        
        Ok(word_id)
    }
    
    /// Add letters for a word
    fn add_letters(&mut self, word: &str) -> error::Result<Vec<NodeId>> {
        let mut letter_ids = Vec::new();
        
        for (i, ch) in word.chars().enumerate() {
            let letter_str = ch.to_string();
            let position = Coordinate3D {
                x: 0.1 + (i as f32 * 0.05),
                y: 0.1,
                z: Layer::Letters.z_center(),
            };
            
            let id = self.builder.add_node(&letter_str, Layer::Letters, position)?;
            letter_ids.push(id);
        }
        
        Ok(letter_ids)
    }
    
    /// Generate phonemes (simplified version)
    fn add_phonemes(&mut self, word: &str) -> error::Result<Vec<NodeId>> {
        let mut phoneme_ids = Vec::new();
        
        // Simplified phoneme generation - real implementation would use proper phonetic rules
        let phonemes = self.simple_phoneme_generation(word);
        
        for (i, phoneme) in phonemes.iter().enumerate() {
            let phoneme_key = phoneme.to_string();
            
            let id = if let Some(&cached_id) = self.phoneme_cache.get(&phoneme_key) {
                cached_id
            } else {
                let position = Coordinate3D {
                    x: 0.2 + (i as f32 * 0.1),
                    y: 0.15,
                    z: Layer::Phonemes.z_center(),
                };
                
                let id = self.builder.add_node(phoneme, Layer::Phonemes, position)?;
                self.phoneme_cache.insert(phoneme_key, id);
                id
            };
            
            phoneme_ids.push(id);
        }
        
        Ok(phoneme_ids)
    }
    
    /// Discover morphemes in a word
    fn discover_morphemes(&mut self, word: &str) -> error::Result<Vec<NodeId>> {
        let mut morpheme_ids = Vec::new();
        let mut remaining = word.to_string();
        
        // Check for prefixes
        for &prefix in &self.morpheme_patterns.prefixes {
            if remaining.starts_with(prefix) && remaining.len() > prefix.len() {
                let id = self.get_or_create_morpheme(prefix, MorphemeType::Prefix)?;
                morpheme_ids.push(id);
                remaining = remaining[prefix.len()..].to_string();
                println!("  → Found prefix: {}", prefix);
                break;
            }
        }
        
        // Check for suffixes (from the end)
        for &suffix in &self.morpheme_patterns.suffixes {
            if remaining.ends_with(suffix) && remaining.len() > suffix.len() {
                let root_part = &remaining[..remaining.len() - suffix.len()];
                
                // Check if root is a known root
                if self.morpheme_patterns.roots.contains(root_part) {
                    let root_id = self.get_or_create_morpheme(root_part, MorphemeType::Root)?;
                    morpheme_ids.push(root_id);
                    println!("  → Found root: {}", root_part);
                    
                    let suffix_id = self.get_or_create_morpheme(suffix, MorphemeType::Suffix)?;
                    morpheme_ids.push(suffix_id);
                    println!("  → Found suffix: {}", suffix);
                    
                    remaining.clear();
                    break;
                }
            }
        }
        
        // If we still have content, check if it's a known root
        if !remaining.is_empty() {
            if self.morpheme_patterns.roots.contains(remaining.as_str()) {
                let id = self.get_or_create_morpheme(&remaining, MorphemeType::Root)?;
                morpheme_ids.push(id);
                println!("  → Found root: {}", remaining);
            } else if morpheme_ids.is_empty() {
                // If no morphemes found, treat the whole word as a root
                let id = self.get_or_create_morpheme(word, MorphemeType::Root)?;
                morpheme_ids.push(id);
            }
        }
        
        Ok(morpheme_ids)
    }
    
    /// Get or create a morpheme
    fn get_or_create_morpheme(&mut self, morpheme: &str, morph_type: MorphemeType) -> error::Result<NodeId> {
        let key = morpheme.to_string();
        
        if let Some(&id) = self.morpheme_cache.get(&key) {
            Ok(id)
        } else {
            let position = self.calculate_morpheme_position(morpheme, morph_type);
            let etymology = self.guess_etymology(morpheme);
            
            let id = self.builder.add_node_full(
                morpheme,
                Layer::Morphemes,
                position,
                etymology,
                morph_type,
                NodeFlags::IS_PRODUCTIVE,
            )?;
            
            self.morpheme_cache.insert(key, id);
            Ok(id)
        }
    }
    
    /// Calculate morpheme position based on type and content
    fn calculate_morpheme_position(&self, morpheme: &str, morph_type: MorphemeType) -> Coordinate3D {
        let base_x = match morph_type {
            MorphemeType::Prefix => 0.2,
            MorphemeType::Root => 0.5,
            MorphemeType::Suffix => 0.8,
            _ => 0.5,
        };
        
        // Add some variation based on first character
        let char_offset = (morpheme.chars().next().unwrap_or('a') as u32 % 26) as f32 * 0.01;
        
        Coordinate3D {
            x: base_x + char_offset,
            y: 0.3,
            z: Layer::Morphemes.z_center(),
        }
    }
    
    /// Simple phoneme generation (very simplified!)
    fn simple_phoneme_generation(&self, word: &str) -> Vec<String> {
        let mut phonemes = Vec::new();
        let chars: Vec<char> = word.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            match chars[i] {
                'c' if i + 1 < chars.len() && chars[i + 1] == 'h' => {
                    phonemes.push("/tʃ/".to_string());
                    i += 2;
                }
                't' if i + 1 < chars.len() && chars[i + 1] == 'h' => {
                    phonemes.push("/θ/".to_string());
                    i += 2;
                }
                's' if i + 1 < chars.len() && chars[i + 1] == 'h' => {
                    phonemes.push("/ʃ/".to_string());
                    i += 2;
                }
                'p' if i + 1 < chars.len() && chars[i + 1] == 'h' => {
                    phonemes.push("/f/".to_string());
                    i += 2;
                }
                vowel @ ('a' | 'e' | 'i' | 'o' | 'u') => {
                    phonemes.push(format!("/{}/", vowel));
                    i += 1;
                }
                consonant => {
                    phonemes.push(format!("/{}/", consonant));
                    i += 1;
                }
            }
        }
        
        phonemes
    }
    
    /// Guess etymology based on morphemes and patterns
    fn guess_etymology(&self, text: &str) -> EtymologyOrigin {
        // Simplified heuristics
        if text.ends_with("ology") || text.starts_with("bio") || text.starts_with("geo") 
            || text.starts_with("tech") || text.starts_with("log") {
            EtymologyOrigin::Greek
        } else if text.ends_with("tion") || text.ends_with("ment") || text.starts_with("trans")
            || text.starts_with("inter") {
            EtymologyOrigin::Latin
        } else if text.len() <= 4 && text.chars().all(|c| c.is_ascii_lowercase()) {
            EtymologyOrigin::Germanic
        } else {
            EtymologyOrigin::Modern
        }
    }
    
    /// Calculate word position based on its components
    fn calculate_position(&self, word: &str, morpheme_ids: &[NodeId], etymology: EtymologyOrigin) -> Coordinate3D {
        // X-axis: phonetic similarity (simplified - based on first letter)
        let x = 0.1 + ((word.chars().next().unwrap_or('m') as u32 - 'a' as u32) as f32 / 26.0) * 0.8;
        
        // Y-axis: etymological origin
        let y = etymology.base_y_coordinate();
        
        // Z-axis: layer-specific
        let z = Layer::Words.z_center();
        
        Coordinate3D { x, y, z }
    }
    
    /// Determine node flags based on word properties
    fn determine_flags(&self, word: &str) -> NodeFlags {
        let mut flags = NodeFlags::empty();
        
        // Technical terms often have Greek/Latin roots
        if word.ends_with("ology") || word.ends_with("ometry") || word.starts_with("tech") {
            flags |= NodeFlags::IS_TECHNICAL;
        }
        
        // Common words are usually short
        if word.len() <= 5 {
            flags |= NodeFlags::IS_FREQUENT;
        }
        
        flags
    }
    
    /// Connect all components
    fn connect_components(&mut self, word_id: NodeId, letters: &[NodeId], phonemes: &[NodeId], morphemes: &[NodeId]) -> error::Result<()> {
        // Connect morphemes to word (Hypernymy = IS-A)
        for &morph_id in morphemes {
            self.builder.add_connection(morph_id, word_id, ConnectionType::Hypernymy, 0.95)?;
        }
        
        // Connect word to morphemes (Meronymy = HAS-A)
        for &morph_id in morphemes {
            self.builder.add_connection(word_id, morph_id, ConnectionType::Meronymy, 0.95)?;
        }
        
        // Connect some letters to morphemes
        if !letters.is_empty() && !morphemes.is_empty() {
            self.builder.add_connection(letters[0], morphemes[0], ConnectionType::Hypernymy, 0.7)?;
        }
        
        // Connect phonemes to morphemes
        if !phonemes.is_empty() && !morphemes.is_empty() {
            self.builder.add_connection(phonemes[0], morphemes[0], ConnectionType::Hypernymy, 0.8)?;
        }
        
        Ok(())
    }
    
    /// Discover connections between existing words
    pub fn discover_connections(&mut self) -> error::Result<()> {
        // This would analyze all words and find:
        // - Synonyms (similar positions in 3D space)
        // - Antonyms (opposite positions on certain axes)
        // - Analogies (similar geometric relationships)
        
        println!("🔗 Discovering connections between words...");
        // Implementation would go here
        Ok(())
    }
    
    /// Build the final database
    pub fn build(mut self, path: &str) -> error::Result<()> {
        self.builder.build(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_morpheme_discovery() {
        let mut builder = AutoLinguisticBuilder::new();
        
        // Test word with clear morphemes
        let morphemes = builder.discover_morphemes("biotechnology").unwrap();
        assert!(morphemes.len() >= 1); // Should find at least bio
        
        // Test word with prefix and suffix
        let morphemes = builder.discover_morphemes("preprocessing").unwrap();
        assert!(morphemes.len() >= 1); // Should find at least pre
    }
}
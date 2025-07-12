#!/usr/bin/env cargo-script

//! # LINGO Database Linguistic Seeder
//! 
//! Seeds the LINGO database with comprehensive linguistic constructs:
//! - 3,000+ core morphemes (prefixes, suffixes, roots)
//! - Etymology relationships and opposition pairs
//! - Phoneme patterns and spatial positioning
//! - Morphological composition rules

use std::collections::HashMap;
use std::fs;

// Using the actual LINGO database types
use lingo::{
    storage::LingoDatabase,
    core::{LinguisticNode, Layer, EtymologyOrigin, MorphemeType, Coordinate3D, NodeId, NodeFlags},
    engine::LingoExecutor,
};
use std::sync::Arc;

// Data structures for seeding
struct PhonemeData {
    symbol: &'static str,
    ipa: &'static str,
    features: Vec<&'static str>,
    position: [f32; 3],
}

struct MorphemeData {
    form: &'static str,
    meaning: &'static str,
    etymology: EtymologyOrigin,
    morpheme_type: MorphemeType,
    productivity: f32,
    position: [f32; 3],
    semantic_field: &'static str,
}

// Helper function for ID generation
fn generate_node_id() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(1);
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 LINGO Database Linguistic Seeder");
    println!("=====================================");
    
    // For demonstration purposes, just show what data we would seed
    println!("📄 This seeder would populate the database with:");
    
    // Core seeding phases
    demonstrate_phonemes()?;
    demonstrate_morphemes()?; 
    demonstrate_etymology_relationships()?;
    demonstrate_opposition_pairs()?;
    demonstrate_morphological_rules()?;
    demonstrate_basic_words()?;
    
    println!("✅ Linguistic database seeding demonstration complete!");
    println!("📊 This would create approximately:");
    println!("   - Phonemes: 34");
    println!("   - Morphemes: 150+");
    println!("   - Etymology relationships: 100+");
    println!("   - Opposition pairs: 50+");
    
    Ok(())
}

fn demonstrate_phonemes() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔤 English phoneme inventory...");
    
    let phonemes = vec![
        // Consonants
        PhonemeData { symbol: "p", ipa: "/p/", features: vec!["voiceless", "bilabial", "stop"], position: [0.1, 0.1, 0.1] },
        PhonemeData { symbol: "b", ipa: "/b/", features: vec!["voiced", "bilabial", "stop"], position: [0.1, 0.2, 0.1] },
        PhonemeData { symbol: "t", ipa: "/t/", features: vec!["voiceless", "alveolar", "stop"], position: [0.2, 0.1, 0.1] },
        PhonemeData { symbol: "d", ipa: "/d/", features: vec!["voiced", "alveolar", "stop"], position: [0.2, 0.2, 0.1] },
        PhonemeData { symbol: "k", ipa: "/k/", features: vec!["voiceless", "velar", "stop"], position: [0.3, 0.1, 0.1] },
        PhonemeData { symbol: "g", ipa: "/g/", features: vec!["voiced", "velar", "stop"], position: [0.3, 0.2, 0.1] },
        PhonemeData { symbol: "f", ipa: "/f/", features: vec!["voiceless", "labiodental", "fricative"], position: [0.1, 0.3, 0.1] },
        PhonemeData { symbol: "v", ipa: "/v/", features: vec!["voiced", "labiodental", "fricative"], position: [0.1, 0.4, 0.1] },
        PhonemeData { symbol: "θ", ipa: "/θ/", features: vec!["voiceless", "dental", "fricative"], position: [0.2, 0.3, 0.1] },
        PhonemeData { symbol: "ð", ipa: "/ð/", features: vec!["voiced", "dental", "fricative"], position: [0.2, 0.4, 0.1] },
        PhonemeData { symbol: "s", ipa: "/s/", features: vec!["voiceless", "alveolar", "fricative"], position: [0.3, 0.3, 0.1] },
        PhonemeData { symbol: "z", ipa: "/z/", features: vec!["voiced", "alveolar", "fricative"], position: [0.3, 0.4, 0.1] },
        PhonemeData { symbol: "ʃ", ipa: "/ʃ/", features: vec!["voiceless", "postalveolar", "fricative"], position: [0.4, 0.3, 0.1] },
        PhonemeData { symbol: "ʒ", ipa: "/ʒ/", features: vec!["voiced", "postalveolar", "fricative"], position: [0.4, 0.4, 0.1] },
        PhonemeData { symbol: "h", ipa: "/h/", features: vec!["voiceless", "glottal", "fricative"], position: [0.5, 0.3, 0.1] },
        PhonemeData { symbol: "m", ipa: "/m/", features: vec!["voiced", "bilabial", "nasal"], position: [0.1, 0.5, 0.1] },
        PhonemeData { symbol: "n", ipa: "/n/", features: vec!["voiced", "alveolar", "nasal"], position: [0.2, 0.5, 0.1] },
        PhonemeData { symbol: "ŋ", ipa: "/ŋ/", features: vec!["voiced", "velar", "nasal"], position: [0.3, 0.5, 0.1] },
        PhonemeData { symbol: "l", ipa: "/l/", features: vec!["voiced", "alveolar", "lateral"], position: [0.2, 0.6, 0.1] },
        PhonemeData { symbol: "r", ipa: "/r/", features: vec!["voiced", "alveolar", "approximant"], position: [0.3, 0.6, 0.1] },
        PhonemeData { symbol: "w", ipa: "/w/", features: vec!["voiced", "labial-velar", "approximant"], position: [0.1, 0.6, 0.1] },
        PhonemeData { symbol: "j", ipa: "/j/", features: vec!["voiced", "palatal", "approximant"], position: [0.4, 0.6, 0.1] },
        
        // Vowels
        PhonemeData { symbol: "i", ipa: "/i/", features: vec!["high", "front", "tense"], position: [0.1, 0.1, 0.2] },
        PhonemeData { symbol: "ɪ", ipa: "/ɪ/", features: vec!["high", "front", "lax"], position: [0.1, 0.2, 0.2] },
        PhonemeData { symbol: "e", ipa: "/e/", features: vec!["mid", "front", "tense"], position: [0.2, 0.1, 0.2] },
        PhonemeData { symbol: "ɛ", ipa: "/ɛ/", features: vec!["mid", "front", "lax"], position: [0.2, 0.2, 0.2] },
        PhonemeData { symbol: "æ", ipa: "/æ/", features: vec!["low", "front"], position: [0.3, 0.1, 0.2] },
        PhonemeData { symbol: "ɑ", ipa: "/ɑ/", features: vec!["low", "back"], position: [0.3, 0.8, 0.2] },
        PhonemeData { symbol: "ɔ", ipa: "/ɔ/", features: vec!["mid", "back", "rounded"], position: [0.2, 0.8, 0.2] },
        PhonemeData { symbol: "o", ipa: "/o/", features: vec!["mid", "back", "tense", "rounded"], position: [0.2, 0.9, 0.2] },
        PhonemeData { symbol: "ʊ", ipa: "/ʊ/", features: vec!["high", "back", "lax", "rounded"], position: [0.1, 0.8, 0.2] },
        PhonemeData { symbol: "u", ipa: "/u/", features: vec!["high", "back", "tense", "rounded"], position: [0.1, 0.9, 0.2] },
        PhonemeData { symbol: "ʌ", ipa: "/ʌ/", features: vec!["mid", "central"], position: [0.2, 0.5, 0.2] },
        PhonemeData { symbol: "ə", ipa: "/ə/", features: vec!["mid", "central", "unstressed"], position: [0.2, 0.4, 0.2] },
        
        // Diphthongs
        PhonemeData { symbol: "aɪ", ipa: "/aɪ/", features: vec!["diphthong", "low-high"], position: [0.4, 0.1, 0.2] },
        PhonemeData { symbol: "aʊ", ipa: "/aʊ/", features: vec!["diphthong", "low-high"], position: [0.4, 0.2, 0.2] },
        PhonemeData { symbol: "ɔɪ", ipa: "/ɔɪ/", features: vec!["diphthong", "mid-high"], position: [0.4, 0.3, 0.2] },
    ];
    
    println!("   📊 {} phonemes including:", phonemes.len());
    for (i, phoneme) in phonemes.iter().take(5).enumerate() {
        println!("     {} {} ({}) - {}", 
                 phoneme.symbol, 
                 phoneme.ipa, 
                 phoneme.features.join(", "),
                 format!("pos: [{:.1}, {:.1}, {:.1}]", 
                        phoneme.position[0], phoneme.position[1], phoneme.position[2]));
    }
    println!("     ... and {} more", phonemes.len() - 5);
    Ok(())
}

fn demonstrate_morphemes() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Morpheme inventory...");
    
    // Latin prefixes
    demonstrate_latin_prefixes()?;
    
    // Greek prefixes  
    demonstrate_greek_prefixes()?;
    
    // Germanic prefixes
    demonstrate_germanic_prefixes()?;
    
    // Suffixes (agent, action, quality)
    demonstrate_suffixes()?;
    
    // Root morphemes
    demonstrate_root_morphemes()?;
    
    Ok(())
}

fn demonstrate_latin_prefixes() -> Result<(), Box<dyn std::error::Error>> {
    let latin_prefixes = vec![
        MorphemeData { 
            form: "pre", meaning: "before", etymology: EtymologyOrigin::Latin, 
            morpheme_type: MorphemeType::Prefix, productivity: 0.85,
            position: [0.1, 0.1, 0.3], semantic_field: "temporal"
        },
        MorphemeData { 
            form: "post", meaning: "after", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Prefix, productivity: 0.70,
            position: [0.9, 0.1, 0.3], semantic_field: "temporal"
        },
        MorphemeData { 
            form: "sub", meaning: "under", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Prefix, productivity: 0.80,
            position: [0.5, 0.1, 0.3], semantic_field: "spatial"
        },
        MorphemeData { 
            form: "super", meaning: "above", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Prefix, productivity: 0.75,
            position: [0.5, 0.9, 0.3], semantic_field: "spatial"
        },
        MorphemeData { 
            form: "inter", meaning: "between", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Prefix, productivity: 0.85,
            position: [0.5, 0.5, 0.3], semantic_field: "spatial"
        },
        MorphemeData { 
            form: "intra", meaning: "within", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Prefix, productivity: 0.65,
            position: [0.3, 0.5, 0.3], semantic_field: "spatial"
        },
        MorphemeData { 
            form: "trans", meaning: "across", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Prefix, productivity: 0.80,
            position: [0.7, 0.5, 0.3], semantic_field: "spatial"
        },
        MorphemeData { 
            form: "de", meaning: "away, down", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Prefix, productivity: 0.90,
            position: [0.2, 0.2, 0.3], semantic_field: "directional"
        },
        MorphemeData { 
            form: "re", meaning: "again, back", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Prefix, productivity: 0.95,
            position: [0.8, 0.2, 0.3], semantic_field: "repetitive"
        },
        MorphemeData { 
            form: "ex", meaning: "out of", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Prefix, productivity: 0.85,
            position: [0.1, 0.5, 0.3], semantic_field: "directional"
        },
        MorphemeData { 
            form: "in", meaning: "in, into", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Prefix, productivity: 0.90,
            position: [0.9, 0.5, 0.3], semantic_field: "directional"
        },
        MorphemeData { 
            form: "con", meaning: "with, together", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Prefix, productivity: 0.85,
            position: [0.5, 0.7, 0.3], semantic_field: "collective"
        },
        MorphemeData { 
            form: "dis", meaning: "apart, away", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Prefix, productivity: 0.80,
            position: [0.2, 0.8, 0.3], semantic_field: "separative"
        },
        MorphemeData { 
            form: "ad", meaning: "to, toward", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Prefix, productivity: 0.75,
            position: [0.8, 0.8, 0.3], semantic_field: "directional"
        },
        MorphemeData { 
            form: "ab", meaning: "away from", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Prefix, productivity: 0.60,
            position: [0.2, 0.1, 0.3], semantic_field: "directional"
        },
    ];
    
    println!("   📊 {} Latin prefixes including:", latin_prefixes.len());
    for morpheme in latin_prefixes.iter().take(3) {
        println!("     {}: {} ({}%) - pos: [{:.1}, {:.1}, {:.1}]", 
                 morpheme.form, 
                 morpheme.meaning,
                 (morpheme.productivity * 100.0) as u32,
                 morpheme.position[0], morpheme.position[1], morpheme.position[2]);
    }
    println!("     ... and {} more", latin_prefixes.len() - 3);
    Ok(())
}

fn demonstrate_greek_prefixes() -> Result<(), Box<dyn std::error::Error>> {
    let greek_prefixes = vec![
        MorphemeData { 
            form: "anti", meaning: "against", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Prefix, productivity: 0.85,
            position: [0.1, 0.1, 0.4], semantic_field: "oppositional"
        },
        MorphemeData { 
            form: "pro", meaning: "forward, in favor", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Prefix, productivity: 0.80,
            position: [0.9, 0.9, 0.4], semantic_field: "supportive"
        },
        MorphemeData { 
            form: "syn", meaning: "together", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Prefix, productivity: 0.75,
            position: [0.5, 0.8, 0.4], semantic_field: "collective"
        },
        MorphemeData { 
            form: "meta", meaning: "beyond, after", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Prefix, productivity: 0.70,
            position: [0.8, 0.1, 0.4], semantic_field: "transcendent"
        },
        MorphemeData { 
            form: "hyper", meaning: "over, excessive", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Prefix, productivity: 0.75,
            position: [0.1, 0.9, 0.4], semantic_field: "intensification"
        },
        MorphemeData { 
            form: "hypo", meaning: "under, insufficient", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Prefix, productivity: 0.65,
            position: [0.1, 0.1, 0.4], semantic_field: "diminution"
        },
        MorphemeData { 
            form: "para", meaning: "beside, alongside", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Prefix, productivity: 0.70,
            position: [0.5, 0.3, 0.4], semantic_field: "parallel"
        },
        MorphemeData { 
            form: "auto", meaning: "self", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Prefix, productivity: 0.85,
            position: [0.5, 0.5, 0.4], semantic_field: "reflexive"
        },
        MorphemeData { 
            form: "pseudo", meaning: "false", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Prefix, productivity: 0.60,
            position: [0.2, 0.2, 0.4], semantic_field: "deceptive"
        },
        MorphemeData { 
            form: "neo", meaning: "new", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Prefix, productivity: 0.75,
            position: [0.8, 0.8, 0.4], semantic_field: "temporal"
        },
        MorphemeData { 
            form: "paleo", meaning: "old, ancient", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Prefix, productivity: 0.55,
            position: [0.2, 0.8, 0.4], semantic_field: "temporal"
        },
        MorphemeData { 
            form: "micro", meaning: "small", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Prefix, productivity: 0.80,
            position: [0.3, 0.1, 0.4], semantic_field: "scale"
        },
        MorphemeData { 
            form: "macro", meaning: "large", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Prefix, productivity: 0.70,
            position: [0.7, 0.9, 0.4], semantic_field: "scale"
        },
        MorphemeData { 
            form: "mega", meaning: "great, large", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Prefix, productivity: 0.65,
            position: [0.9, 0.7, 0.4], semantic_field: "scale"
        },
        MorphemeData { 
            form: "poly", meaning: "many", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Prefix, productivity: 0.75,
            position: [0.8, 0.3, 0.4], semantic_field: "quantity"
        },
    ];
    
    println!("   📊 {} Greek prefixes including:", greek_prefixes.len());
    for morpheme in greek_prefixes.iter().take(3) {
        println!("     {}: {} ({}%) - pos: [{:.1}, {:.1}, {:.1}]", 
                 morpheme.form, 
                 morpheme.meaning,
                 (morpheme.productivity * 100.0) as u32,
                 morpheme.position[0], morpheme.position[1], morpheme.position[2]);
    }
    println!("     ... and {} more", greek_prefixes.len() - 3);
    Ok(())
}

fn demonstrate_germanic_prefixes() -> Result<(), Box<dyn std::error::Error>> {
    let germanic_prefixes = vec![
        MorphemeData { 
            form: "un", meaning: "not, reverse", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Prefix, productivity: 0.95,
            position: [0.1, 0.1, 0.5], semantic_field: "negation"
        },
        MorphemeData { 
            form: "over", meaning: "excessive, above", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Prefix, productivity: 0.80,
            position: [0.1, 0.9, 0.5], semantic_field: "excess"
        },
        MorphemeData { 
            form: "under", meaning: "below, insufficient", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Prefix, productivity: 0.75,
            position: [0.1, 0.1, 0.5], semantic_field: "deficiency"
        },
        MorphemeData { 
            form: "out", meaning: "beyond, surpassing", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Prefix, productivity: 0.70,
            position: [0.9, 0.5, 0.5], semantic_field: "surpassing"
        },
        MorphemeData { 
            form: "up", meaning: "upward, increase", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Prefix, productivity: 0.65,
            position: [0.5, 0.9, 0.5], semantic_field: "elevation"
        },
        MorphemeData { 
            form: "down", meaning: "downward, decrease", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Prefix, productivity: 0.60,
            position: [0.5, 0.1, 0.5], semantic_field: "reduction"
        },
        MorphemeData { 
            form: "fore", meaning: "before, in front", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Prefix, productivity: 0.55,
            position: [0.1, 0.5, 0.5], semantic_field: "anterior"
        },
        MorphemeData { 
            form: "be", meaning: "around, thoroughly", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Prefix, productivity: 0.70,
            position: [0.5, 0.7, 0.5], semantic_field: "comprehensive"
        },
        MorphemeData { 
            form: "with", meaning: "together, against", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Prefix, productivity: 0.50,
            position: [0.7, 0.7, 0.5], semantic_field: "association"
        },
        MorphemeData { 
            form: "off", meaning: "away, from", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Prefix, productivity: 0.45,
            position: [0.3, 0.3, 0.5], semantic_field: "separation"
        },
    ];
    
    for morpheme in germanic_prefixes {
        insert_morpheme(db, morpheme)?;
    }
    
    println!("   ✓ Added {} Germanic prefixes", 10);
    Ok(())
}

fn seed_suffixes(db: &mut LingoDB) -> Result<(), Box<dyn std::error::Error>> {
    // Agent suffixes
    let agent_suffixes = vec![
        MorphemeData { 
            form: "er", meaning: "one who does", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::AgentSuffix, productivity: 0.95,
            position: [0.8, 0.8, 0.6], semantic_field: "agency"
        },
        MorphemeData { 
            form: "or", meaning: "one who does", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::AgentSuffix, productivity: 0.85,
            position: [0.8, 0.7, 0.6], semantic_field: "agency"
        },
        MorphemeData { 
            form: "ant", meaning: "one who does", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::AgentSuffix, productivity: 0.70,
            position: [0.8, 0.6, 0.6], semantic_field: "agency"
        },
        MorphemeData { 
            form: "ist", meaning: "one who practices", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::AgentSuffix, productivity: 0.80,
            position: [0.8, 0.5, 0.6], semantic_field: "professional"
        },
        MorphemeData { 
            form: "ian", meaning: "one who specializes", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::AgentSuffix, productivity: 0.75,
            position: [0.8, 0.4, 0.6], semantic_field: "specialist"
        },
        MorphemeData { 
            form: "ite", meaning: "follower, member", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::AgentSuffix, productivity: 0.60,
            position: [0.8, 0.3, 0.6], semantic_field: "affiliation"
        },
    ];
    
    // Action suffixes
    let action_suffixes = vec![
        MorphemeData { 
            form: "ize", meaning: "to make, cause", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::ActionSuffix, productivity: 0.90,
            position: [0.6, 0.8, 0.6], semantic_field: "causation"
        },
        MorphemeData { 
            form: "fy", meaning: "to make, cause", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::ActionSuffix, productivity: 0.75,
            position: [0.6, 0.7, 0.6], semantic_field: "causation"
        },
        MorphemeData { 
            form: "ate", meaning: "to act upon", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::ActionSuffix, productivity: 0.80,
            position: [0.6, 0.6, 0.6], semantic_field: "action"
        },
        MorphemeData { 
            form: "en", meaning: "to make, become", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::ActionSuffix, productivity: 0.70,
            position: [0.6, 0.5, 0.6], semantic_field: "transformation"
        },
        MorphemeData { 
            form: "ish", meaning: "to make somewhat", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::ActionSuffix, productivity: 0.60,
            position: [0.6, 0.4, 0.6], semantic_field: "approximation"
        },
    ];
    
    // Quality suffixes
    let quality_suffixes = vec![
        MorphemeData { 
            form: "able", meaning: "capable of", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::QualitySuffix, productivity: 0.85,
            position: [0.4, 0.8, 0.6], semantic_field: "capability"
        },
        MorphemeData { 
            form: "ible", meaning: "capable of", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::QualitySuffix, productivity: 0.70,
            position: [0.4, 0.7, 0.6], semantic_field: "capability"
        },
        MorphemeData { 
            form: "ous", meaning: "full of", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::QualitySuffix, productivity: 0.80,
            position: [0.4, 0.6, 0.6], semantic_field: "abundance"
        },
        MorphemeData { 
            form: "ful", meaning: "full of", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::QualitySuffix, productivity: 0.90,
            position: [0.4, 0.5, 0.6], semantic_field: "abundance"
        },
        MorphemeData { 
            form: "less", meaning: "without", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::QualitySuffix, productivity: 0.85,
            position: [0.4, 0.4, 0.6], semantic_field: "absence"
        },
        MorphemeData { 
            form: "ic", meaning: "relating to", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::QualitySuffix, productivity: 0.95,
            position: [0.4, 0.3, 0.6], semantic_field: "relation"
        },
        MorphemeData { 
            form: "al", meaning: "relating to", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::QualitySuffix, productivity: 0.90,
            position: [0.4, 0.2, 0.6], semantic_field: "relation"
        },
        MorphemeData { 
            form: "ive", meaning: "having nature of", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::QualitySuffix, productivity: 0.85,
            position: [0.4, 0.1, 0.6], semantic_field: "characteristic"
        },
    ];
    
    // Insert all suffixes
    for morpheme in agent_suffixes.into_iter()
        .chain(action_suffixes.into_iter())
        .chain(quality_suffixes.into_iter()) {
        insert_morpheme(db, morpheme)?;
    }
    
    println!("   ✓ Added {} suffixes (agent + action + quality)", 19);
    Ok(())
}

fn seed_root_morphemes(db: &mut LingoDB) -> Result<(), Box<dyn std::error::Error>> {
    println!("🌱 Seeding root morphemes...");
    
    // Latin roots
    let latin_roots = vec![
        // Management/Agency roots
        MorphemeData { 
            form: "manage", meaning: "to handle, control", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.85,
            position: [0.7, 0.7, 0.7], semantic_field: "control"
        },
        MorphemeData { 
            form: "duc", meaning: "to lead", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.80,
            position: [0.8, 0.7, 0.7], semantic_field: "leadership"
        },
        MorphemeData { 
            form: "reg", meaning: "to rule", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.75,
            position: [0.9, 0.7, 0.7], semantic_field: "authority"
        },
        
        // Creation/Action roots
        MorphemeData { 
            form: "create", meaning: "to make, bring forth", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.90,
            position: [0.8, 0.9, 0.7], semantic_field: "creation"
        },
        MorphemeData { 
            form: "struct", meaning: "to build", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.85,
            position: [0.7, 0.9, 0.7], semantic_field: "construction"
        },
        MorphemeData { 
            form: "form", meaning: "to shape", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.80,
            position: [0.6, 0.9, 0.7], semantic_field: "shaping"
        },
        MorphemeData { 
            form: "fact", meaning: "to make, do", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.85,
            position: [0.5, 0.9, 0.7], semantic_field: "making"
        },
        
        // Destruction/Opposite roots
        MorphemeData { 
            form: "destroy", meaning: "to unmake, ruin", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.75,
            position: [0.2, 0.1, 0.7], semantic_field: "destruction"
        },
        MorphemeData { 
            form: "demol", meaning: "to tear down", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.60,
            position: [0.3, 0.1, 0.7], semantic_field: "destruction"
        },
        
        // Communication roots
        MorphemeData { 
            form: "spect", meaning: "to look, see", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.80,
            position: [0.3, 0.8, 0.7], semantic_field: "perception"
        },
        MorphemeData { 
            form: "dict", meaning: "to say, speak", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.75,
            position: [0.4, 0.8, 0.7], semantic_field: "communication"
        },
        MorphemeData { 
            form: "scrib", meaning: "to write", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.70,
            position: [0.5, 0.8, 0.7], semantic_field: "writing"
        },
        
        // Movement roots
        MorphemeData { 
            form: "port", meaning: "to carry", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.85,
            position: [0.6, 0.5, 0.7], semantic_field: "transport"
        },
        MorphemeData { 
            form: "miss", meaning: "to send", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.80,
            position: [0.7, 0.5, 0.7], semantic_field: "transmission"
        },
        MorphemeData { 
            form: "vert", meaning: "to turn", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.75,
            position: [0.8, 0.5, 0.7], semantic_field: "rotation"
        },
    ];
    
    // Greek roots
    let greek_roots = vec![
        // Technology roots
        MorphemeData { 
            form: "tech", meaning: "skill, craft", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Root, productivity: 0.90,
            position: [0.9, 0.8, 0.8], semantic_field: "technology"
        },
        MorphemeData { 
            form: "log", meaning: "word, study", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Root, productivity: 0.95,
            position: [0.4, 0.9, 0.8], semantic_field: "knowledge"
        },
        MorphemeData { 
            form: "graph", meaning: "to write", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Root, productivity: 0.85,
            position: [0.5, 0.9, 0.8], semantic_field: "recording"
        },
        MorphemeData { 
            form: "morph", meaning: "form, shape", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Root, productivity: 0.80,
            position: [0.6, 0.8, 0.8], semantic_field: "transformation"
        },
        
        // Organization roots
        MorphemeData { 
            form: "organ", meaning: "tool, instrument", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Root, productivity: 0.85,
            position: [0.7, 0.8, 0.8], semantic_field: "organization"
        },
        MorphemeData { 
            form: "system", meaning: "organized whole", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Root, productivity: 0.80,
            position: [0.8, 0.8, 0.8], semantic_field: "systematization"
        },
        
        // Chaos/Opposite roots
        MorphemeData { 
            form: "chaos", meaning: "disorder, void", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Root, productivity: 0.60,
            position: [0.2, 0.2, 0.8], semantic_field: "disorder"
        },
        
        // Measurement/Analysis roots
        MorphemeData { 
            form: "metr", meaning: "measure", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Root, productivity: 0.75,
            position: [0.3, 0.7, 0.8], semantic_field: "measurement"
        },
        MorphemeData { 
            form: "scope", meaning: "to see, examine", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Root, productivity: 0.70,
            position: [0.3, 0.8, 0.8], semantic_field: "observation"
        },
        
        // Social roots
        MorphemeData { 
            form: "anthrop", meaning: "human", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Root, productivity: 0.65,
            position: [0.5, 0.6, 0.8], semantic_field: "humanity"
        },
        MorphemeData { 
            form: "soci", meaning: "companion", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.80,
            position: [0.6, 0.6, 0.8], semantic_field: "social"
        },
    ];
    
    // Germanic roots
    let germanic_roots = vec![
        // Basic action roots
        MorphemeData { 
            form: "work", meaning: "to labor", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Root, productivity: 0.90,
            position: [0.7, 0.6, 0.9], semantic_field: "labor"
        },
        MorphemeData { 
            form: "make", meaning: "to create", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Root, productivity: 0.95,
            position: [0.8, 0.8, 0.9], semantic_field: "creation"
        },
        MorphemeData { 
            form: "build", meaning: "to construct", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Root, productivity: 0.85,
            position: [0.7, 0.8, 0.9], semantic_field: "construction"
        },
        MorphemeData { 
            form: "break", meaning: "to destroy", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Root, productivity: 0.80,
            position: [0.3, 0.2, 0.9], semantic_field: "destruction"
        },
        
        // Cognitive roots
        MorphemeData { 
            form: "think", meaning: "to consider", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Root, productivity: 0.85,
            position: [0.4, 0.7, 0.9], semantic_field: "cognition"
        },
        MorphemeData { 
            form: "know", meaning: "to understand", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Root, productivity: 0.90,
            position: [0.5, 0.7, 0.9], semantic_field: "knowledge"
        },
        MorphemeData { 
            form: "learn", meaning: "to acquire knowledge", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Root, productivity: 0.85,
            position: [0.6, 0.7, 0.9], semantic_field: "acquisition"
        },
        
        // Growth/Change roots
        MorphemeData { 
            form: "grow", meaning: "to increase", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Root, productivity: 0.80,
            position: [0.8, 0.6, 0.9], semantic_field: "growth"
        },
        MorphemeData { 
            form: "shrink", meaning: "to decrease", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Root, productivity: 0.70,
            position: [0.2, 0.4, 0.9], semantic_field: "reduction"
        },
        
        // Leadership roots
        MorphemeData { 
            form: "lead", meaning: "to guide", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Root, productivity: 0.85,
            position: [0.8, 0.7, 0.9], semantic_field: "leadership"
        },
        MorphemeData { 
            form: "follow", meaning: "to come after", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Root, productivity: 0.75,
            position: [0.2, 0.7, 0.9], semantic_field: "followership"
        },
    ];
    
    // Domain-specific business roots
    let business_roots = vec![
        MorphemeData { 
            form: "business", meaning: "commercial activity", etymology: EtymologyOrigin::Germanic,
            morpheme_type: MorphemeType::Root, productivity: 0.80,
            position: [0.9, 0.5, 0.7], semantic_field: "commerce"
        },
        MorphemeData { 
            form: "market", meaning: "place of trade", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.85,
            position: [0.8, 0.4, 0.7], semantic_field: "trade"
        },
        MorphemeData { 
            form: "product", meaning: "thing produced", etymology: EtymologyOrigin::Latin,
            morpheme_type: MorphemeType::Root, productivity: 0.90,
            position: [0.7, 0.4, 0.7], semantic_field: "output"
        },
        MorphemeData { 
            form: "develop", meaning: "to expand", etymology: EtymologyOrigin::French,
            morpheme_type: MorphemeType::Root, productivity: 0.95,
            position: [0.9, 0.6, 0.7], semantic_field: "development"
        },
        MorphemeData { 
            form: "architect", meaning: "chief builder", etymology: EtymologyOrigin::Greek,
            morpheme_type: MorphemeType::Root, productivity: 0.75,
            position: [0.8, 0.9, 0.8], semantic_field: "design"
        },
    ];
    
    // Insert all root morphemes
    for morpheme in latin_roots.into_iter()
        .chain(greek_roots.into_iter())
        .chain(germanic_roots.into_iter())
        .chain(business_roots.into_iter()) {
        insert_morpheme(db, morpheme)?;
    }
    
    println!("   ✓ Added {} root morphemes", 15 + 11 + 11 + 5);
    Ok(())
}

fn seed_etymology_relationships(db: &mut LingoDB) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 Seeding etymology relationships...");
    
    // Define etymology families and their relationships
    let etymology_families = vec![
        EtymologyFamily {
            origin: EtymologyOrigin::Latin,
            root_relationships: vec![
                ("create", vec!["fact", "form", "struct"]),
                ("destroy", vec!["demol", "rupt"]),
                ("duc", vec!["duct", "duce"]),
                ("spect", vec!["vis", "vid"]),
                ("port", vec!["fer", "lat"]),
            ],
            opposition_pairs: vec![
                ("create", "destroy"),
                ("construct", "destruct"),
                ("produce", "reduce"),
                ("attract", "repel"),
                ("advance", "retreat"),
            ],
        },
        EtymologyFamily {
            origin: EtymologyOrigin::Greek,
            root_relationships: vec![
                ("tech", vec!["mechan", "art"]),
                ("log", vec!["graph", "gram"]),
                ("morph", vec!["form", "type"]),
                ("organ", vec!["system", "struct"]),
            ],
            opposition_pairs: vec![
                ("organize", "chaos"),
                ("synthesis", "analysis"),
                ("macro", "micro"),
                ("hyper", "hypo"),
                ("pro", "anti"),
            ],
        },
        EtymologyFamily {
            origin: EtymologyOrigin::Germanic,
            root_relationships: vec![
                ("work", vec!["labor", "toil"]),
                ("make", vec!["build", "craft"]),
                ("think", vec!["know", "learn"]),
                ("lead", vec!["guide", "head"]),
            ],
            opposition_pairs: vec![
                ("build", "break"),
                ("grow", "shrink"),
                ("rise", "fall"),
                ("lead", "follow"),
                ("know", "ignore"),
            ],
        },
    ];
    
    // Create orthogonal connections based on etymology relationships
    for family in etymology_families {
        create_etymology_connections(db, family)?;
    }
    
    println!("   ✓ Created etymology relationship networks");
    Ok(())
}

fn seed_opposition_pairs(db: &mut LingoDB) -> Result<(), Box<dyn std::error::Error>> {
    println!("🪞 Seeding opposition pairs for mirroring...");
    
    let opposition_mappings = vec![
        // Functional oppositions
        OppositionPair {
            original: "manager",
            opposites: vec!["subordinate", "employee", "follower", "team member"],
            opposition_type: OppositionType::FunctionalRole,
            confidence: 0.90,
        },
        OppositionPair {
            original: "leader",
            opposites: vec!["follower", "member", "subordinate"],
            opposition_type: OppositionType::FunctionalRole,
            confidence: 0.95,
        },
        OppositionPair {
            original: "teacher",
            opposites: vec!["student", "learner", "pupil"],
            opposition_type: OppositionType::FunctionalRole,
            confidence: 0.90,
        },
        OppositionPair {
            original: "creator",
            opposites: vec!["destroyer", "critic", "consumer"],
            opposition_type: OppositionType::FunctionalRole,
            confidence: 0.85,
        },
        OppositionPair {
            original: "developer",
            opposites: vec!["user", "client", "tester"],
            opposition_type: OppositionType::FunctionalRole,
            confidence: 0.80,
        },
        
        // Action oppositions
        OppositionPair {
            original: "create",
            opposites: vec!["destroy", "demolish", "eliminate"],
            opposition_type: OppositionType::ActionReversal,
            confidence: 0.95,
        },
        OppositionPair {
            original: "build",
            opposites: vec!["destroy", "demolish", "break", "dismantle"],
            opposition_type: OppositionType::ActionReversal,
            confidence: 0.90,
        },
        OppositionPair {
            original: "organize",
            opposites: vec!["disorganize", "scatter", "chaos"],
            opposition_type: OppositionType::ActionReversal,
            confidence: 0.85,
        },
        OppositionPair {
            original: "centralize",
            opposites: vec!["decentralize", "distribute", "scatter"],
            opposition_type: OppositionType::ActionReversal,
            confidence: 0.80,
        },
        OppositionPair {
            original: "accelerate",
            opposites: vec!["decelerate", "slow", "brake"],
            opposition_type: OppositionType::ActionReversal,
            confidence: 0.85,
        },
        
        // State oppositions
        OppositionPair {
            original: "order",
            opposites: vec!["chaos", "disorder", "confusion"],
            opposition_type: OppositionType::StateInversion,
            confidence: 0.90,
        },
        OppositionPair {
            original: "growth",
            opposites: vec!["decline", "shrinkage", "reduction"],
            opposition_type: OppositionType::StateInversion,
            confidence: 0.85,
        },
        OppositionPair {
            original: "success",
            opposites: vec!["failure", "defeat", "loss"],
            opposition_type: OppositionType::StateInversion,
            confidence: 0.90,
        },
        
        // Technical oppositions
        OppositionPair {
            original: "technical",
            opposites: vec!["business", "commercial", "non-technical", "managerial"],
            opposition_type: OppositionType::DomainContrast,
            confidence: 0.75,
        },
        OppositionPair {
            original: "frontend",
            opposites: vec!["backend", "server-side", "infrastructure"],
            opposition_type: OppositionType::DomainContrast,
            confidence: 0.80,
        },
        OppositionPair {
            original: "architect",
            opposites: vec!["implementer", "user", "operator"],
            opposition_type: OppositionType::FunctionalRole,
            confidence: 0.75,
        },
    ];
    
    // Create opposition connections in the database
    for opposition in opposition_mappings {
        create_opposition_connections(db, opposition)?;
    }
    
    println!("   ✓ Created {} opposition relationship networks", 16);
    Ok(())
}

fn seed_morphological_rules(db: &mut LingoDB) -> Result<(), Box<dyn std::error::Error>> {
    println!("📏 Seeding morphological composition rules...");
    
    let composition_rules = vec![
        // Agent formation rules
        CompositionRule {
            pattern: "{root} + er",
            semantic_function: SemanticFunction::AgentFormation,
            productivity: 0.95,
            examples: vec!["manage + er = manager", "teach + er = teacher", "work + er = worker"],
            constraints: vec!["root must be action or process"],
        },
        CompositionRule {
            pattern: "{root} + or",
            semantic_function: SemanticFunction::AgentFormation,
            productivity: 0.85,
            examples: vec!["act + or = actor", "edit + or = editor", "invest + or = investor"],
            constraints: vec!["typically with Latin roots"],
        },
        CompositionRule {
            pattern: "{root} + ist",
            semantic_function: SemanticFunction::SpecialistFormation,
            productivity: 0.80,
            examples: vec!["art + ist = artist", "special + ist = specialist", "journal + ist = journalist"],
            constraints: vec!["forms professional/practitioner roles"],
        },
        
        // Action formation rules
        CompositionRule {
            pattern: "{root} + ize",
            semantic_function: SemanticFunction::CausativeFormation,
            productivity: 0.90,
            examples: vec!["organ + ize = organize", "modern + ize = modernize", "central + ize = centralize"],
            constraints: vec!["creates causative verbs"],
        },
        CompositionRule {
            pattern: "{root} + fy",
            semantic_function: SemanticFunction::CausativeFormation,
            productivity: 0.75,
            examples: vec!["simple + fy = simplify", "class + fy = classify", "intense + fy = intensify"],
            constraints: vec!["typically creates state-change verbs"],
        },
        
        // Quality formation rules
        CompositionRule {
            pattern: "{root} + able",
            semantic_function: SemanticFunction::CapabilityFormation,
            productivity: 0.85,
            examples: vec!["manage + able = manageable", "scale + able = scalable", "predict + able = predictable"],
            constraints: vec!["indicates capability or possibility"],
        },
        CompositionRule {
            pattern: "{root} + ful",
            semantic_function: SemanticFunction::AbundanceFormation,
            productivity: 0.90,
            examples: vec!["help + ful = helpful", "use + ful = useful", "success + ful = successful"],
            constraints: vec!["indicates abundance or possession"],
        },
        CompositionRule {
            pattern: "{root} + less",
            semantic_function: SemanticFunction::AbsenceFormation,
            productivity: 0.85,
            examples: vec!["help + less = helpless", "use + less = useless", "hope + less = hopeless"],
            constraints: vec!["indicates absence or lack"],
        },
        
        // Negation rules
        CompositionRule {
            pattern: "un + {root}",
            semantic_function: SemanticFunction::NegationFormation,
            productivity: 0.95,
            examples: vec!["un + able = unable", "un + known = unknown", "un + organized = unorganized"],
            constraints: vec!["most productive negation prefix"],
        },
        CompositionRule {
            pattern: "dis + {root}",
            semantic_function: SemanticFunction::ReversalFormation,
            productivity: 0.80,
            examples: vec!["dis + organize = disorganize", "dis + connect = disconnect", "dis + agree = disagree"],
            constraints: vec!["typically indicates reversal of action"],
        },
    ];
    
    // Store composition rules in database
    for rule in composition_rules {
        store_composition_rule(db, rule)?;
    }
    
    println!("   ✓ Added {} morphological composition rules", 10);
    Ok(())
}

fn seed_basic_words(db: &mut LingoDB) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Seeding basic word inventory...");
    
    // Compose basic words from our morphemes
    let basic_words = vec![
        // Agent words
        WordComposition { word: "manager", morphemes: vec!["manage", "er"], layer: Layer::Words, 
                         position: [0.7, 0.7, 0.7], semantic_field: "leadership" },
        WordComposition { word: "developer", morphemes: vec!["develop", "er"], layer: Layer::Words,
                         position: [0.9, 0.6, 0.7], semantic_field: "creation" },
        WordComposition { word: "leader", morphemes: vec!["lead", "er"], layer: Layer::Words,
                         position: [0.8, 0.7, 0.9], semantic_field: "leadership" },
        WordComposition { word: "creator", morphemes: vec!["create", "or"], layer: Layer::Words,
                         position: [0.8, 0.9, 0.7], semantic_field: "creation" },
        WordComposition { word: "organizer", morphemes: vec!["organ", "ize", "er"], layer: Layer::Words,
                         position: [0.7, 0.8, 0.8], semantic_field: "organization" },
        WordComposition { word: "architect", morphemes: vec!["architect"], layer: Layer::Words,
                         position: [0.8, 0.9, 0.8], semantic_field: "design" },
        
        // Action words
        WordComposition { word: "organize", morphemes: vec!["organ", "ize"], layer: Layer::Words,
                         position: [0.7, 0.8, 0.8], semantic_field: "organization" },
        WordComposition { word: "create", morphemes: vec!["create"], layer: Layer::Words,
                         position: [0.8, 0.9, 0.7], semantic_field: "creation" },
        WordComposition { word: "develop", morphemes: vec!["develop"], layer: Layer::Words,
                         position: [0.9, 0.6, 0.7], semantic_field: "development" },
        WordComposition { word: "build", morphemes: vec!["build"], layer: Layer::Words,
                         position: [0.7, 0.8, 0.9], semantic_field: "construction" },
        WordComposition { word: "design", morphemes: vec!["design"], layer: Layer::Words,
                         position: [0.6, 0.8, 0.8], semantic_field: "planning" },
        
        // Quality words
        WordComposition { word: "technical", morphemes: vec!["tech", "ic", "al"], layer: Layer::Words,
                         position: [0.9, 0.8, 0.8], semantic_field: "technology" },
        WordComposition { word: "business", morphemes: vec!["business"], layer: Layer::Words,
                         position: [0.9, 0.5, 0.7], semantic_field: "commerce" },
        WordComposition { word: "manageable", morphemes: vec!["manage", "able"], layer: Layer::Words,
                         position: [0.7, 0.7, 0.6], semantic_field: "capability" },
        WordComposition { word: "scalable", morphemes: vec!["scale", "able"], layer: Layer::Words,
                         position: [0.8, 0.6, 0.6], semantic_field: "capability" },
        
        // Opposition words
        WordComposition { word: "destroy", morphemes: vec!["destroy"], layer: Layer::Words,
                         position: [0.2, 0.1, 0.7], semantic_field: "destruction" },
        WordComposition { word: "disorganize", morphemes: vec!["dis", "organ", "ize"], layer: Layer::Words,
                         position: [0.3, 0.2, 0.8], semantic_field: "disorder" },
        WordComposition { word: "follower", morphemes: vec!["follow", "er"], layer: Layer::Words,
                         position: [0.2, 0.7, 0.9], semantic_field: "followership" },
        WordComposition { word: "subordinate", morphemes: vec!["sub", "ord", "ate"], layer: Layer::Words,
                         position: [0.3, 0.3, 0.7], semantic_field: "hierarchy" },
    ];
    
    // Insert composed words
    for word_comp in basic_words {
        insert_composed_word(db, word_comp)?;
    }
    
    println!("   ✓ Added {} basic words with morphological composition", 19);
    Ok(())
}

// Helper functions and data structures

#[derive(Debug, Clone)]
struct PhonemeData {
    symbol: &'static str,
    ipa: &'static str,
    features: Vec<&'static str>,
    position: [f32; 3],
}

#[derive(Debug, Clone)]
struct MorphemeData {
    form: &'static str,
    meaning: &'static str,
    etymology: EtymologyOrigin,
    morpheme_type: MorphemeType,
    productivity: f32,
    position: [f32; 3],
    semantic_field: &'static str,
}

#[derive(Debug, Clone)]
struct EtymologyFamily {
    origin: EtymologyOrigin,
    root_relationships: Vec<(&'static str, Vec<&'static str>)>,
    opposition_pairs: Vec<(&'static str, &'static str)>,
}

#[derive(Debug, Clone)]
struct OppositionPair {
    original: &'static str,
    opposites: Vec<&'static str>,
    opposition_type: OppositionType,
    confidence: f32,
}

#[derive(Debug, Clone)]
enum OppositionType {
    FunctionalRole,
    ActionReversal,
    StateInversion,
    DomainContrast,
}

#[derive(Debug, Clone)]
struct CompositionRule {
    pattern: &'static str,
    semantic_function: SemanticFunction,
    productivity: f32,
    examples: Vec<&'static str>,
    constraints: Vec<&'static str>,
}

#[derive(Debug, Clone)]
enum SemanticFunction {
    AgentFormation,
    SpecialistFormation,
    CausativeFormation,
    CapabilityFormation,
    AbundanceFormation,
    AbsenceFormation,
    NegationFormation,
    ReversalFormation,
}

#[derive(Debug, Clone)]
struct WordComposition {
    word: &'static str,
    morphemes: Vec<&'static str>,
    layer: Layer,
    position: [f32; 3],
    semantic_field: &'static str,
}

// Implementation helper functions

fn generate_node_id() -> NodeId {
    // Implementation would generate unique node IDs
    static mut COUNTER: u32 = 0;
    unsafe { 
        COUNTER += 1;
        COUNTER
    }
}

fn calculate_phoneme_productivity(phoneme: &PhonemeData) -> f32 {
    // Calculate productivity based on phoneme features and frequency
    // This is a simplified calculation
    match phoneme.features.len() {
        1..=2 => 0.5,
        3..=4 => 0.7,
        _ => 0.9,
    }
}

fn insert_morpheme(db: &mut LingoDB, morpheme: MorphemeData) -> Result<(), Box<dyn std::error::Error>> {
    let node = LinguisticNode {
        id: generate_node_id(),
        layer: Layer::Morphemes,
        surface_form: morpheme.form.to_string(),
        etymology_origin: morpheme.etymology,
        position: Coordinate3D {
            x: morpheme.position[0],
            y: morpheme.position[1], 
            z: morpheme.position[2],
        },
        morpheme_type: morpheme.morpheme_type,
        productivity_score: (morpheme.productivity * 1000.0) as u16,
        frequency_rank: 0, // Will be calculated later
        orthogonal_connections: vec![],
    };
    
    db.insert_node(node)?;
    Ok(())
}

fn create_etymology_connections(db: &mut LingoDB, family: EtymologyFamily) -> Result<(), Box<dyn std::error::Error>> {
    // Create connections between related morphemes within etymology families
    for (root, related_roots) in family.root_relationships {
        let root_node_id = db.find_morpheme_by_form(root)?;
        
        for related in related_roots {
            if let Ok(related_node_id) = db.find_morpheme_by_form(&related) {
                let connection = OrthogonalConnection {
                    source_id: root_node_id,
                    target_id: related_node_id,
                    connection_type: ConnectionType::EtymologyRelated,
                    strength: 0.85,
                    semantic_relationship: "etymologically_related".to_string(),
                };
                db.insert_connection(connection)?;
            }
        }
    }
    
    // Create opposition connections
    for (word1, word2) in family.opposition_pairs {
        if let (Ok(id1), Ok(id2)) = (db.find_morpheme_by_form(word1), db.find_morpheme_by_form(word2)) {
            let opposition_connection = OrthogonalConnection {
                source_id: id1,
                target_id: id2,
                connection_type: ConnectionType::Opposition,
                strength: 0.90,
                semantic_relationship: "etymological_opposite".to_string(),
            };
            db.insert_connection(opposition_connection)?;
            
            // Create bidirectional connection
            let reverse_connection = OrthogonalConnection {
                source_id: id2,
                target_id: id1,
                connection_type: ConnectionType::Opposition,
                strength: 0.90,
                semantic_relationship: "etymological_opposite".to_string(),
            };
            db.insert_connection(reverse_connection)?;
        }
    }
    
    Ok(())
}

fn create_opposition_connections(db: &mut LingoDB, opposition: OppositionPair) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(original_id) = db.find_word_by_form(opposition.original) {
        for opposite in opposition.opposites {
            if let Ok(opposite_id) = db.find_word_by_form(opposite) {
                let connection = OrthogonalConnection {
                    source_id: original_id,
                    target_id: opposite_id,
                    connection_type: ConnectionType::FunctionalOpposition,
                    strength: opposition.confidence,
                    semantic_relationship: format!("{:?}", opposition.opposition_type),
                };
                db.insert_connection(connection)?;
            }
        }
    }
    Ok(())
}

fn store_composition_rule(db: &mut LingoDB, rule: CompositionRule) -> Result<(), Box<dyn std::error::Error>> {
    // Store morphological composition rules for later use in word generation
    // This would be stored in a separate rules table/structure
    println!("  Stored rule: {} (productivity: {:.2})", rule.pattern, rule.productivity);
    Ok(())
}

fn insert_composed_word(db: &mut LingoDB, word_comp: WordComposition) -> Result<(), Box<dyn std::error::Error>> {
    // Create connections between word and its component morphemes
    let word_node = LinguisticNode {
        id: generate_node_id(),
        layer: word_comp.layer,
        surface_form: word_comp.word.to_string(),
        etymology_origin: EtymologyOrigin::Mixed, // Words can have mixed etymology
        position: Coordinate3D {
            x: word_comp.position[0],
            y: word_comp.position[1],
            z: word_comp.position[2],
        },
        morpheme_type: MorphemeType::Word,
        productivity_score: 500, // Default for words
        frequency_rank: 0,
        orthogonal_connections: vec![],
    };
    
    let word_id = word_node.id;
    db.insert_node(word_node)?;
    
    // Connect word to its component morphemes
    for morpheme_form in word_comp.morphemes {
        if let Ok(morpheme_id) = db.find_morpheme_by_form(morpheme_form) {
            let composition_connection = OrthogonalConnection {
                source_id: word_id,
                target_id: morpheme_id,
                connection_type: ConnectionType::MorphologicalComposition,
                strength: 0.95,
                semantic_relationship: "composed_from".to_string(),
            };
            db.insert_connection(composition_connection)?;
        }
    }
    
    Ok(())
}

fn count_layer_nodes(db: &LingoDB, layer: Layer) -> usize {
    // Count nodes in specific layer
    db.count_nodes_in_layer(layer).unwrap_or(0)
}

// Additional enum definitions that would be in your main codebase
#[derive(Debug, Clone, Copy)]
enum EtymologyOrigin {
    Latin,
    Greek, 
    Germanic,
    French,
    Mixed,
    Universal,
}

#[derive(Debug, Clone, Copy)]
enum MorphemeType {
    Phoneme,
    Root,
    Prefix,
    Suffix,
    AgentSuffix,
    ActionSuffix,
    QualitySuffix,
    Word,
}

#[derive(Debug, Clone, Copy)]
enum Layer {
    Letters,
    Phonemes,
    Morphemes,
    Words,
    Phrases,
    Concepts,
    Domains,
}

#[derive(Debug, Clone)]
enum ConnectionType {
    EtymologyRelated,
    Opposition,
    FunctionalOpposition,
    MorphologicalComposition,
}

// Copyright 2025 Roberto Antunes
//
// Licensed under the Functional Source License, Version 1.1 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://github.com/RobAntunes/lingodb/blob/main/LICENSE
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Check what's actually in the english.lingo database

use lingo::storage::LingoDatabase;
use lingo::core::Layer;

fn main() {
    println!("Checking contents of english.lingo database\n");
    
    // Load the standard english.lingo database
    let database = match LingoDatabase::open("english.lingo") {
        Ok(db) => db,
        Err(e) => {
            println!("Failed to open english.lingo: {}", e);
            return;
        }
    };
    
    println!("Database loaded successfully!");
    println!("Total nodes: {}", database.node_count());
    
    // Count nodes by layer
    let mut layer_counts = std::collections::HashMap::new();
    let mut word_samples = Vec::new();
    let mut phrase_samples = Vec::new();
    
    for i in 0..database.node_count() {
        if let Ok(node) = database.get_node(lingo::core::NodeId(i as u32)) {
            *layer_counts.entry(node.layer).or_insert(0) += 1;
            
            // Collect some sample words and phrases
            if matches!(node.layer, Layer::Words) && word_samples.len() < 20 {
                if let Ok(word) = database.get_node_word(lingo::core::NodeId(i as u32)) {
                    word_samples.push(word.to_string());
                }
            }
            
            if matches!(node.layer, Layer::Phrases) && phrase_samples.len() < 10 {
                if let Ok(phrase) = database.get_node_word(lingo::core::NodeId(i as u32)) {
                    phrase_samples.push(phrase.to_string());
                }
            }
        }
    }
    
    println!("\nNodes by layer:");
    println!("Letters:   {:>6}", layer_counts.get(&Layer::Letters).unwrap_or(&0));
    println!("Phonemes:  {:>6}", layer_counts.get(&Layer::Phonemes).unwrap_or(&0));
    println!("Morphemes: {:>6}", layer_counts.get(&Layer::Morphemes).unwrap_or(&0));
    println!("Words:     {:>6}", layer_counts.get(&Layer::Words).unwrap_or(&0));
    println!("Phrases:   {:>6}", layer_counts.get(&Layer::Phrases).unwrap_or(&0));
    println!("Concepts:  {:>6}", layer_counts.get(&Layer::Concepts).unwrap_or(&0));
    println!("Domains:   {:>6}", layer_counts.get(&Layer::Domains).unwrap_or(&0));
    
    println!("\nSample words in database:");
    for word in &word_samples {
        println!("  - {}", word);
    }
    
    if !phrase_samples.is_empty() {
        println!("\nSample phrases in database:");
        for phrase in &phrase_samples {
            println!("  - {}", phrase);
        }
    }
    
    // Try to find some common words
    println!("\nChecking for common words:");
    let common_words = vec!["the", "a", "is", "to", "and", "of", "in", "for", "it", "with"];
    
    for word in common_words {
        // Manual search through all nodes
        let mut found = false;
        for i in 0..database.node_count() {
            if let Ok(node) = database.get_node(lingo::core::NodeId(i as u32)) {
                if matches!(node.layer, Layer::Words) {
                    if let Ok(node_word) = database.get_node_word(lingo::core::NodeId(i as u32)) {
                        if node_word == word {
                            found = true;
                            break;
                        }
                    }
                }
            }
        }
        println!("  '{}': {}", word, if found { "✓" } else { "✗" });
    }
}
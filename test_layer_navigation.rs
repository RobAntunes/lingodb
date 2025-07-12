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

//! Test layer navigation in function extraction

use lingo::storage::LingoDatabase;
use lingo::engine::LingoExecutor;
use lingo::query::QueryBuilder;
use std::sync::Arc;

fn main() {
    println!("Testing Layer Navigation in Lingo Database\n");
    
    // Load the standard english.lingo database
    println!("Loading english.lingo database...");
    let database = match LingoDatabase::open("english.lingo") {
        Ok(db) => {
            println!("Successfully loaded english.lingo");
            Arc::new(db)
        },
        Err(e) => {
            println!("Failed to open english.lingo: {}", e);
            return;
        }
    };
    
    let mut executor = LingoExecutor::new();
    if let Err(e) = executor.load_database("english.lingo") {
        println!("Failed to load database in executor: {}", e);
        return;
    }
    
    // Test 1: Find a simple word
    println!("\n=== Test 1: Find a word ===");
    let query = QueryBuilder::find("manager")
        .compile();
    
    match executor.execute(&query) {
        Ok(result) => {
            println!("Found {} nodes for 'manager'", result.nodes.len());
            for node_id in result.nodes.as_slice() {
                if let Ok(word) = database.get_node_word(*node_id) {
                    println!("  - Word: {}", word);
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    
    // Test 2: Find a word and go down to morphemes
    println!("\n=== Test 2: Find word → layer down to morphemes ===");
    let query = QueryBuilder::find("manager")
        .layer_down()
        .compile();
    
    match executor.execute(&query) {
        Ok(result) => {
            println!("Found {} morphemes for 'manager'", result.nodes.len());
            for node_id in result.nodes.as_slice() {
                if let Ok(node) = database.get_node(*node_id) {
                    if let Ok(morpheme) = database.get_node_word(*node_id) {
                        println!("  - Morpheme: {} (Layer: {:?})", morpheme, node.layer);
                    }
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    
    // Test 3: Try to find a phrase
    println!("\n=== Test 3: Find a phrase ===");
    let test_phrases = vec![
        "in order to",
        "the cat",
        "good morning",
        "thank you",
    ];
    
    for phrase in test_phrases {
        let query = QueryBuilder::find(phrase)
            .compile();
        
        match executor.execute(&query) {
            Ok(result) => {
                if result.nodes.len() > 0 {
                    println!("✓ Found phrase '{}' ({} nodes)", phrase, result.nodes.len());
                } else {
                    println!("✗ Phrase '{}' not found", phrase);
                }
            }
            Err(e) => println!("Error searching for '{}': {}", phrase, e),
        }
    }
    
    // Test 4: Find a phrase and get its words
    println!("\n=== Test 4: Find phrase → layer down to words ===");
    let query = QueryBuilder::find("in order to")
        .layer_down()
        .compile();
    
    match executor.execute(&query) {
        Ok(result) => {
            println!("Found {} words in phrase 'in order to'", result.nodes.len());
            for node_id in result.nodes.as_slice() {
                if let Ok(word) = database.get_node_word(*node_id) {
                    println!("  - Word: {}", word);
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    
    // Test 5: Try finding a sentence that probably doesn't exist
    println!("\n=== Test 5: Find a sentence (probably won't exist) ===");
    let sentence = "The manager organized the meeting";
    let query = QueryBuilder::find(sentence)
        .compile();
    
    match executor.execute(&query) {
        Ok(result) => {
            if result.nodes.len() > 0 {
                println!("✓ Found sentence '{}' ({} nodes)", sentence, result.nodes.len());
                // Try layer down to get words
                let words_query = QueryBuilder::find(sentence)
                    .layer_down()
                    .compile();
                
                if let Ok(words_result) = executor.execute(&words_query) {
                    println!("  Found {} words:", words_result.nodes.len());
                    for node_id in words_result.nodes.as_slice() {
                        if let Ok(word) = database.get_node_word(*node_id) {
                            println!("    - {}", word);
                        }
                    }
                }
            } else {
                println!("✗ Sentence '{}' not found in database", sentence);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    
    // Test 6: Find individual words from our test sentences
    println!("\n=== Test 6: Find individual words from test sentences ===");
    let test_words = vec![
        "manager", "organized", "meeting",
        "developer", "created", "feature",
        "startup", "converted", "platform",
        "user", "uploads", "documents",
        "compile", "code", "tests", "deploy",
        "system", "processes", "data", "insights",
        "temperature", "rises", "ice", "melts",
        "team", "collaborated", "achieve", "goals"
    ];
    
    let mut found_count = 0;
    for word in &test_words {
        let query = QueryBuilder::find(word)
            .compile();
        
        if let Ok(result) = executor.execute(&query) {
            if result.nodes.len() > 0 {
                println!("✓ Found: {}", word);
                found_count += 1;
            } else {
                println!("✗ Not found: {}", word);
            }
        }
    }
    
    println!("\nFound {}/{} test words in database", found_count, test_words.len());
}
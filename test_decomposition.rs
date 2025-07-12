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

//! Test morphological decomposition functionality

use lingo::{LingoDatabase, LingoExecutor, QueryBuilder};
use lingo::plugins::{PluginPipeline, FunctionExtractor};

fn main() {
    println!("Testing Morphological Decomposition\n");
    
    // Load database and create executor
    let database = match LingoDatabase::open("english.lingo") {
        Ok(db) => db,
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
    
    // Test 1: Direct decomposition using query language
    println!("=== Test 1: Query-based Decomposition ===");
    let test_words = vec!["manager", "developer", "organizing", "created"];
    
    for word in &test_words {
        println!("\nDecomposing '{}':", word);
        
        // Find the word and decompose it
        let query = QueryBuilder::find(word)
            .decompose()
            .compile();
        
        match executor.execute(&query) {
            Ok(result) => {
                if result.nodes.is_empty() {
                    println!("  No morphemes found");
                } else {
                    println!("  Found {} morphemes:", result.nodes.len());
                    for node_id in result.nodes.as_slice() {
                        if let Ok(morpheme) = database.get_node_word(*node_id) {
                            if let Ok(node) = database.get_node(*node_id) {
                                println!("    - '{}' (Layer: {:?}, Pos: ({:.2}, {:.2}, {:.2}))", 
                                    morpheme, 
                                    node.layer,
                                    node.position.x,
                                    node.position.y,
                                    node.position.z
                                );
                            }
                        }
                    }
                }
            }
            Err(e) => println!("  Error: {}", e),
        }
    }
    
    // Test 2: Plugin-based function extraction with compositional approach
    println!("\n\n=== Test 2: Function Extraction with Compositional Analysis ===");
    
    let mut pipeline = PluginPipeline::new();
    let mut function_extractor = FunctionExtractor::new();
    
    // Initialize and register plugin
    if let Err(e) = function_extractor.initialize(&database) {
        println!("Failed to initialize function extractor: {:?}", e);
        return;
    }
    
    if let Err(e) = pipeline.register(Box::new(function_extractor)) {
        println!("Failed to register plugin: {:?}", e);
        return;
    }
    
    // Test sentences
    let test_sentences = vec![
        "The manager organized the meeting",
        "Developers created innovative features",
        "The system processes data efficiently",
    ];
    
    for sentence in &test_sentences {
        println!("\nAnalyzing: \"{}\"", sentence);
        
        match pipeline.execute_command("extract_function", &[sentence.to_string()]) {
            Ok(Some(result)) => {
                println!("  Function extraction successful!");
                match result {
                    lingo::plugins::PluginResult::CustomResults { data, confidence } => {
                        println!("  Confidence: {:.2}", confidence);
                        if let Some(signature) = data.get("signature") {
                            // Parse the debug output to show key findings
                            if signature.contains("Agency") {
                                println!("  ✓ Agency detected");
                            }
                            if signature.contains("Action") {
                                println!("  ✓ Action detected");
                            }
                            if signature.contains("Transformation") {
                                println!("  ✓ Transformation detected");
                            }
                        }
                    }
                    _ => println!("  Unexpected result type"),
                }
            }
            Ok(None) => println!("  No function patterns detected"),
            Err(e) => println!("  Error: {:?}", e),
        }
    }
    
    // Test 3: Manual morphological decomposition
    println!("\n\n=== Test 3: Manual Morphological Analysis ===");
    
    // Test morpheme lookup
    let morphemes = vec!["manage", "er", "develop", "ment", "organize", "ed"];
    
    println!("Looking for morphemes in database:");
    for morpheme in &morphemes {
        let query = QueryBuilder::find(morpheme)
            .layer(lingo::core::Layer::Morphemes)
            .compile();
        
        match executor.execute(&query) {
            Ok(result) => {
                if !result.nodes.is_empty() {
                    println!("  ✓ Found morpheme: '{}'", morpheme);
                } else {
                    println!("  ✗ Not found: '{}'", morpheme);
                }
            }
            Err(e) => println!("  Error searching for '{}': {}", morpheme, e),
        }
    }
}
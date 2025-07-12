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

//! Test function extraction pattern detection

use std::sync::Arc;
use lingo::plugins::{PluginPipeline, FunctionExtractor, PluginResult};
use lingo::storage::LingoDatabase;

fn main() {
    println!("Testing Function Extraction Plugin\n");
    
    // Load the standard english.lingo database
    println!("Loading english.lingo database...");
    let database = match LingoDatabase::open("english.lingo") {
        Ok(db) => {
            println!("Successfully loaded english.lingo");
            db
        },
        Err(e) => {
            println!("Failed to open english.lingo: {}", e);
            println!("Make sure english.lingo exists in the current directory");
            return;
        }
    };
    
    // Set up plugin pipeline
    let mut pipeline = PluginPipeline::new();
    pipeline.set_database(Arc::new(database));
    
    let plugin = Box::new(FunctionExtractor::new());
    if let Err(e) = pipeline.register_plugin(plugin) {
        println!("Failed to register plugin: {}", e);
        return;
    }
    
    if let Err(e) = pipeline.initialize_plugins() {
        println!("Failed to initialize plugins: {}", e);
        return;
    }
    
    // Test sentences with different functional patterns
    let test_sentences = vec![
        ("The manager organized the meeting", "Agency + Action"),
        ("The developer created a new feature", "Agency + Action + Creation"),
        ("The startup converted their MVP into a scalable platform", "Agency + Transformation"),
        ("If the user uploads documents, the AI will analyze them", "Conditionality + Agency + Action"),
        ("First compile the code, then run the tests, finally deploy", "Sequence + Multiple Actions"),
        ("The system processes data to generate insights", "Agency + Action + Purpose"),
        ("When the temperature rises, the ice melts", "Conditionality + Transformation"),
        ("The team collaborated to achieve their goals", "Agency + Action + Purpose"),
    ];
    
    println!("\nTesting Function Extraction Patterns:\n");
    
    for (sentence, expected_pattern) in test_sentences {
        println!("Sentence: \"{}\"", sentence);
        println!("Expected: {}", expected_pattern);
        
        match pipeline.execute_command(
            "function_extraction",
            "extract_function",
            &[sentence.to_string()]
        ) {
            Ok(PluginResult::CustomResults { data, confidence }) => {
                println!("Confidence: {:.2}", confidence);
                if let Some(signature) = data.get("signature") {
                    // Parse the debug output to extract patterns
                    if signature.contains("Agency") {
                        println!("✓ Found Agency pattern");
                    }
                    if signature.contains("Action") {
                        println!("✓ Found Action pattern");
                    }
                    if signature.contains("Transformation") {
                        println!("✓ Found Transformation pattern");
                    }
                    if signature.contains("Conditionality") {
                        println!("✓ Found Conditionality pattern");
                    }
                    if signature.contains("Sequence") {
                        println!("✓ Found Sequence pattern");
                    }
                    if signature.contains("Purpose") {
                        println!("✓ Found Purpose pattern");
                    }
                    
                    println!("Raw output: {}", signature);
                }
            },
            Ok(_) => println!("Unexpected result type"),
            Err(e) => println!("Error: {}", e),
        }
        
        println!("---\n");
    }
}
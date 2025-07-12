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

//! WebAssembly bindings for Lingo Database
//! 
//! This module provides high-level JavaScript/TypeScript bindings using wasm-bindgen.
//! These bindings are memory-safe, fast, and provide excellent TypeScript support.
//! 
//! ## Usage (TypeScript)
//! 
//! ```typescript
//! import { LingoDatabase, QueryBuilder } from '@lingo/core';
//! 
//! // Create and load database
//! const db = new LingoDatabase();
//! await db.loadStandardEnglish();
//! 
//! // Build and execute query
//! const query = QueryBuilder.find("tech")
//!     .layerUp()
//!     .similar(0.8)
//!     .limit(10);
//! 
//! const results = await db.execute(query);
//! console.log(`Found ${results.nodes.length} related terms`);
//! ```

use wasm_bindgen::prelude::*;
use js_sys::{Array, Object, Reflect};
use serde::{Serialize, Deserialize};

use crate::engine::LingoExecutor;
use crate::query::{QueryBuilder as CoreQueryBuilder, CompiledQuery};
use crate::data::DatabaseSeeder;

// Set up panic hook and allocator for WASM
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        // web_sys::console::log_1(&"🚀 Lingo Database WASM initialized".into());
    }
}

/// JavaScript-compatible result type
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct WasmResult {
    success: bool,
    error: Option<String>,
}

#[wasm_bindgen]
impl WasmResult {
    #[wasm_bindgen(getter)]
    pub fn success(&self) -> bool {
        self.success
    }

    #[wasm_bindgen(getter)]
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
}

/// A linguistic node result for JavaScript
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct WasmNode {
    id: u32,
    word: String,
    layer: String,
    x: f32,
    y: f32,
    z: f32,
    etymology: String,
    flags: Vec<String>,
}

#[wasm_bindgen]
impl WasmNode {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 { self.id }

    #[wasm_bindgen(getter)]
    pub fn word(&self) -> String { self.word.clone() }

    #[wasm_bindgen(getter)]
    pub fn layer(&self) -> String { self.layer.clone() }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f32 { self.x }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f32 { self.y }

    #[wasm_bindgen(getter)]
    pub fn z(&self) -> f32 { self.z }

    #[wasm_bindgen(getter)]
    pub fn etymology(&self) -> String { self.etymology.clone() }

    #[wasm_bindgen(getter)]
    pub fn flags(&self) -> Array {
        let arr = Array::new();
        for flag in &self.flags {
            arr.push(&JsValue::from_str(flag));
        }
        arr
    }
}

/// Query results for JavaScript
#[wasm_bindgen]
pub struct WasmQueryResult {
    nodes: Vec<WasmNode>,
    execution_time_ms: f64,
    total_nodes_searched: u32,
}

#[wasm_bindgen]
impl WasmQueryResult {
    #[wasm_bindgen(getter)]
    pub fn nodes(&self) -> Array {
        let arr = Array::new();
        for node in &self.nodes {
            match serde_wasm_bindgen::to_value(node) {
                Ok(val) => { arr.push(&val); },
                Err(_) => { arr.push(&JsValue::NULL); },
            }
        }
        arr
    }

    #[wasm_bindgen(getter = executionTimeMs)]
    pub fn execution_time_ms(&self) -> f64 {
        self.execution_time_ms
    }

    #[wasm_bindgen(getter = totalNodesSearched)]
    pub fn total_nodes_searched(&self) -> u32 {
        self.total_nodes_searched
    }

    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.nodes.len()
    }
}

/// Query builder for JavaScript
#[wasm_bindgen]
pub struct WasmQueryBuilder {
    builder: CoreQueryBuilder,
}

#[wasm_bindgen]
impl WasmQueryBuilder {
    /// Create a new query to find a term
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmQueryBuilder {
        // We'll create an empty builder - the actual search term will be set by find()
        WasmQueryBuilder {
            builder: CoreQueryBuilder::find(""), // Temporary empty string
        }
    }

    /// Find nodes containing the given term
    #[wasm_bindgen]
    pub fn find(term: &str) -> WasmQueryBuilder {
        WasmQueryBuilder {
            builder: CoreQueryBuilder::find(term),
        }
    }

    /// Navigate up to parent layer (e.g., morphemes → words)
    #[wasm_bindgen(js_name = layerUp)]
    pub fn layer_up(mut self) -> WasmQueryBuilder {
        self.builder = self.builder.layer_up();
        self
    }

    /// Navigate down to child layer (e.g., words → morphemes)
    #[wasm_bindgen(js_name = layerDown)]
    pub fn layer_down(mut self) -> WasmQueryBuilder {
        self.builder = self.builder.layer_down();
        self
    }

    /// Find similar nodes based on semantic distance
    #[wasm_bindgen]
    pub fn similar(mut self) -> WasmQueryBuilder {
        self.builder = self.builder.similar();
        self
    }

    /// Find similar nodes with custom threshold (0.0 to 1.0)
    #[wasm_bindgen(js_name = similarThreshold)]
    pub fn similar_threshold(mut self, threshold: f32) -> WasmQueryBuilder {
        self.builder = self.builder.similar_threshold(threshold);
        self
    }

    /// Follow orthogonal connections to related concepts
    #[wasm_bindgen(js_name = followConnection)]
    pub fn follow_connection(mut self) -> WasmQueryBuilder {
        self.builder = self.builder.follow_connection();
        self
    }

    /// Limit the number of results
    #[wasm_bindgen]
    pub fn limit(mut self, limit: u32) -> WasmQueryBuilder {
        self.builder = self.builder.limit(limit as usize);
        self
    }

    /// Compile the query for execution
    #[wasm_bindgen]
    pub fn compile(self) -> WasmCompiledQuery {
        let compiled = self.builder.compile();
        WasmCompiledQuery { query: compiled }
    }
}

/// Compiled query ready for execution
#[wasm_bindgen]
pub struct WasmCompiledQuery {
    query: CompiledQuery,
}

/// The main Lingo Database interface for JavaScript
#[wasm_bindgen]
pub struct LingoDatabase {
    executor: Option<LingoExecutor>,
}

#[wasm_bindgen]
impl LingoDatabase {
    /// Create a new Lingo database instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> LingoDatabase {
        LingoDatabase {
            executor: None,
        }
    }

    /// Load a database from bytes (for web usage)
    #[wasm_bindgen(js_name = loadFromBytes)]
    pub fn load_from_bytes(&mut self, _bytes: &[u8]) -> Result<WasmResult, JsValue> {
        // In a real implementation, we'd write bytes to a temporary file
        // For now, let's return an error indicating this needs implementation
        Err(JsValue::from_str("loadFromBytes not yet implemented - use loadStandardEnglish() instead"))
    }

    /// Load the standard English database (pre-built)
    #[wasm_bindgen(js_name = loadStandardEnglish)]
    pub fn load_standard_english(&mut self) -> Result<WasmResult, JsValue> {
        // Create a temporary database with standard English data
        match self.create_standard_english_database() {
            Ok(_) => {
                let mut executor = LingoExecutor::new();
                match executor.load_database("temp_english.lingo") {
                    Ok(_) => {
                        self.executor = Some(executor);
                        Ok(WasmResult {
                            success: true,
                            error: None,
                        })
                    }
                    Err(e) => Ok(WasmResult {
                        success: false,
                        error: Some(format!("Failed to load database: {}", e)),
                    })
                }
            }
            Err(e) => Ok(WasmResult {
                success: false,
                error: Some(format!("Failed to create database: {}", e)),
            })
        }
    }

    /// Execute a compiled query
    #[wasm_bindgen]
    pub fn execute(&mut self, query: &WasmCompiledQuery) -> Result<WasmQueryResult, JsValue> {
        let executor = self.executor.as_mut()
            .ok_or_else(|| JsValue::from_str("Database not loaded. Call loadStandardEnglish() first."))?;

        let start_time = js_sys::Date::now();
        
        match executor.execute(&query.query) {
            Ok(result) => {
                let execution_time = js_sys::Date::now() - start_time;
                
                // Convert NodeSet to actual nodes by getting them from the database
                let wasm_nodes: Vec<WasmNode> = result.nodes.as_slice().iter().filter_map(|node_id| {
                    // Try to get the actual node from the database
                    if let Some(ref db) = executor.database {
                        if let Ok(node) = db.get_node(*node_id) {
                            let word = db.get_node_word(*node_id).unwrap_or("unknown");
                            return Some(WasmNode {
                                id: node_id.0,
                                word: word.to_string(),
                                layer: format!("{:?}", node.layer),
                                x: node.position.x,
                                y: node.position.y,
                                z: node.position.z,
                                etymology: format!("{:?}", node.etymology_origin),
                                flags: format!("{:?}", node.flags).split('|').map(|s| s.trim().to_string()).collect(),
                            });
                        }
                    }
                    
                    // Fallback if we can't get the node
                    Some(WasmNode {
                        id: node_id.0,
                        word: format!("node_{}", node_id.0),
                        layer: "Unknown".to_string(),
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        etymology: "Unknown".to_string(),
                        flags: vec![],
                    })
                }).collect();

                Ok(WasmQueryResult {
                    nodes: wasm_nodes,
                    execution_time_ms: execution_time,
                    total_nodes_searched: result.instructions_executed as u32, // Use instructions as proxy
                })
            }
            Err(e) => Err(JsValue::from_str(&format!("Query execution failed: {}", e))),
        }
    }

    /// Get database statistics
    #[wasm_bindgen(js_name = getStats)]
    pub fn get_stats(&self) -> Result<JsValue, JsValue> {
        let _executor = self.executor.as_ref()
            .ok_or_else(|| JsValue::from_str("Database not loaded"))?;

        // Return basic stats as a JavaScript object
        let stats = Object::new();
        Reflect::set(&stats, &"loaded".into(), &true.into())?;
        Reflect::set(&stats, &"version".into(), &"1.0.0".into())?;
        
        Ok(stats.into())
    }

    // Helper method to create standard English database
    fn create_standard_english_database(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut seeder = DatabaseSeeder::new();
        seeder.seed_english()?;
        seeder.build("temp_english.lingo")?;
        Ok(())
    }
}

/// Static methods for the QueryBuilder
#[wasm_bindgen]
pub struct QueryBuilder;

#[wasm_bindgen]
impl QueryBuilder {
    /// Create a new query to find a term
    #[wasm_bindgen]
    pub fn find(term: &str) -> WasmQueryBuilder {
        WasmQueryBuilder {
            builder: CoreQueryBuilder::find(term),
        }
    }
}

// Export types for TypeScript
#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_TYPES: &'static str = r#"
export interface NodeResult {
  id: number;
  word: string;
  layer: string;
  x: number;
  y: number;
  z: number;
  etymology: string;
  flags: string[];
}

export interface QueryResult {
  nodes: NodeResult[];
  executionTimeMs: number;
  totalNodesSearched: number;
  length: number;
}

export interface DatabaseStats {
  loaded: boolean;
  version: string;
}

export interface Result {
  success: boolean;
  error?: string;
}
"#;
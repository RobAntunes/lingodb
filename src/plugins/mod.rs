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

//! Plugin Architecture for Lingo Database
//! 
//! This module provides a clean plugin architecture with complete separation of concerns.
//! Plugins can extend the core database functionality without polluting the base implementation.
//! 
//! ## Design Principles
//! 
//! - **Complete SoC**: Plugins are completely isolated from core
//! - **No Pollution**: Core database remains clean and focused
//! - **Composable**: Multiple plugins can be stacked and combined
//! - **Type Safety**: All plugin interactions are statically verified
//! - **Performance**: Zero-cost abstractions where possible

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::engine::LingoExecutor;
use crate::core::{LinguisticNode, NodeId};
use crate::storage::LingoDatabase;

pub mod function_extraction;
pub mod intent_detection;
pub mod registry;

// Re-export public types
pub use registry::{PluginRegistry, PluginError};
pub use function_extraction::{FunctionExtractor, FunctionSignature, FunctionalPrimitive};
pub use intent_detection::{IntentDetector, Intent, PragmaticOperators};

/// Core plugin trait that all plugins must implement
pub trait Plugin: Send + Sync + Any {
    /// Unique identifier for this plugin
    fn id(&self) -> &'static str;
    
    /// Human-readable name
    fn name(&self) -> &'static str;
    
    /// Plugin version
    fn version(&self) -> &'static str;
    
    /// Dependencies on other plugins (by ID)
    fn dependencies(&self) -> Vec<&'static str> { Vec::new() }
    
    /// Initialize the plugin with the database
    fn initialize(&mut self, database: &LingoDatabase) -> Result<(), PluginError>;
    
    /// Process a query before it hits the core engine
    fn pre_process(&self, _input: &str, _context: &PluginContext) -> Result<Option<String>, PluginError> {
        Ok(None) // Default: no preprocessing
    }
    
    /// Process results after core engine execution
    fn post_process(&self, _results: &[LinguisticNode], _context: &PluginContext) -> Result<Option<PluginResult>, PluginError> {
        Ok(None) // Default: no postprocessing
    }
    
    /// Handle custom commands specific to this plugin
    fn handle_command(&mut self, _command: &str, _args: &[String], _context: &PluginContext) -> Result<Option<PluginResult>, PluginError> {
        Ok(None) // Default: no custom commands
    }
    
    /// Cleanup when plugin is unloaded
    fn cleanup(&mut self) -> Result<(), PluginError> {
        Ok(()) // Default: no cleanup needed
    }
}

/// Context provided to plugins during execution
#[derive(Debug, Clone)]
pub struct PluginContext {
    /// Original query text
    pub query_text: String,
    /// Execution metadata
    pub metadata: HashMap<String, String>,
    /// Plugin-specific data storage
    pub plugin_data: Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>>,
}

impl PluginContext {
    pub fn new(query_text: String) -> Self {
        Self {
            query_text,
            metadata: HashMap::new(),
            plugin_data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Store plugin-specific data
    pub fn set_data<T: Any + Send + Sync>(&self, key: String, value: T) -> Result<(), PluginError> {
        let mut data = self.plugin_data.write()
            .map_err(|_| PluginError::LockError("Failed to acquire write lock".to_string()))?;
        data.insert(key, Box::new(value));
        Ok(())
    }
    
    /// Check if plugin-specific data exists
    pub fn has_data(&self, key: &str) -> bool {
        if let Ok(data) = self.plugin_data.read() {
            data.contains_key(key)
        } else {
            false
        }
    }
}

/// Results that plugins can return
#[derive(Debug, Clone)]
pub enum PluginResult {
    /// Modified query text (for pre-processing)
    ModifiedQuery(String),
    /// Enhanced results with additional data
    EnhancedResults {
        original_nodes: Vec<LinguisticNode>,
        enhancements: HashMap<NodeId, PluginEnhancement>,
    },
    /// Completely custom results
    CustomResults {
        data: HashMap<String, String>,
        confidence: f32,
    },
    /// Command execution result
    CommandResult {
        success: bool,
        message: String,
        data: Option<String>,
    },
}

/// Enhancement data that plugins can attach to nodes
#[derive(Debug, Clone)]
pub struct PluginEnhancement {
    pub plugin_id: String,
    pub enhancement_type: String,
    pub confidence: f32,
    pub data: HashMap<String, String>,
}

/// Plugin execution pipeline
pub struct PluginPipeline {
    registry: PluginRegistry,
    database: Option<Arc<LingoDatabase>>,
}

impl PluginPipeline {
    pub fn new() -> Self {
        Self {
            registry: PluginRegistry::new(),
            database: None,
        }
    }
    
    /// Set the database for plugin initialization
    pub fn set_database(&mut self, database: Arc<LingoDatabase>) {
        self.database = Some(database);
    }
    
    /// Register a plugin
    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<(), PluginError> {
        self.registry.register(plugin)
    }
    
    /// Initialize all registered plugins
    pub fn initialize_plugins(&mut self) -> Result<(), PluginError> {
        let database = self.database.as_ref()
            .ok_or_else(|| PluginError::NotInitialized("Database not set".to_string()))?;
        
        self.registry.initialize_all(database.as_ref())
    }
    
    /// Execute the full plugin pipeline
    pub fn execute_pipeline(&self, query: &str, nodes: Vec<LinguisticNode>) -> Result<PluginPipelineResult, PluginError> {
        let context = PluginContext::new(query.to_string());
        let mut current_query = query.to_string();
        let mut current_nodes = nodes;
        let mut enhancements = HashMap::new();
        
        // Pre-processing phase
        for plugin in self.registry.get_active_plugins() {
            if let Some(modified_query) = plugin.pre_process(&current_query, &context)? {
                current_query = modified_query;
            }
        }
        
        // Post-processing phase
        for plugin in self.registry.get_active_plugins() {
            if let Some(result) = plugin.post_process(&current_nodes, &context)? {
                match result {
                    PluginResult::EnhancedResults { original_nodes, enhancements: plugin_enhancements } => {
                        current_nodes = original_nodes;
                        enhancements.extend(plugin_enhancements);
                    },
                    PluginResult::CustomResults { data, confidence } => {
                        // Store custom results in context for other plugins
                        context.set_data("custom_results".to_string(), format!("confidence: {}, data: {:?}", confidence, data))?;
                    },
                    _ => {}, // Ignore other result types in post-processing
                }
            }
        }
        
        Ok(PluginPipelineResult {
            query: current_query,
            nodes: current_nodes,
            enhancements,
            context,
        })
    }
    
    /// Execute a plugin-specific command
    pub fn execute_command(&mut self, plugin_id: &str, command: &str, args: &[String]) -> Result<PluginResult, PluginError> {
        let context = PluginContext::new(String::new());
        self.registry.execute_command(plugin_id, command, args, &context)
    }
    
    /// Get list of registered plugins
    pub fn list_plugins(&self) -> Vec<PluginInfo> {
        self.registry.list_plugins()
    }
}

/// Result of executing the plugin pipeline
#[derive(Debug)]
pub struct PluginPipelineResult {
    pub query: String,
    pub nodes: Vec<LinguisticNode>,
    pub enhancements: HashMap<NodeId, PluginEnhancement>,
    pub context: PluginContext,
}

/// Plugin information for listing
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub active: bool,
    pub dependencies: Vec<String>,
}

impl Default for PluginPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock plugin for testing
    struct MockPlugin {
        id: &'static str,
        initialized: bool,
    }
    
    impl MockPlugin {
        fn new(id: &'static str) -> Self {
            Self { id, initialized: false }
        }
    }
    
    impl Plugin for MockPlugin {
        fn id(&self) -> &'static str { self.id }
        fn name(&self) -> &'static str { "Mock Plugin" }
        fn version(&self) -> &'static str { "1.0.0" }
        
        fn initialize(&mut self, _database: &LingoDatabase) -> Result<(), PluginError> {
            self.initialized = true;
            Ok(())
        }
        
        fn pre_process(&self, input: &str, _context: &PluginContext) -> Result<Option<String>, PluginError> {
            if input.contains("mock") {
                Ok(Some(input.replace("mock", "enhanced")))
            } else {
                Ok(None)
            }
        }
    }
    
    #[test]
    fn test_plugin_registration() {
        let mut pipeline = PluginPipeline::new();
        let plugin = Box::new(MockPlugin::new("test"));
        
        assert!(pipeline.register_plugin(plugin).is_ok());
        
        let plugins = pipeline.list_plugins();
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].id, "test");
    }
    
    #[test]
    fn test_context_data_storage() {
        let context = PluginContext::new("test query".to_string());
        
        // Store data
        assert!(context.set_data("test_key".to_string(), 42i32).is_ok());
        
        // Check data exists
        assert!(context.has_data("test_key"));
        assert!(!context.has_data("nonexistent_key"));
    }
}
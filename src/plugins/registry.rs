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

//! Plugin Registry - Manages plugin lifecycle and dependencies

use std::collections::HashMap;
use std::fmt;

use crate::storage::LingoDatabase;
use super::{Plugin, PluginContext, PluginResult, PluginInfo};

/// Plugin registry manages all registered plugins
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
    active_plugins: Vec<String>,
    dependency_graph: HashMap<String, Vec<String>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            active_plugins: Vec::new(),
            dependency_graph: HashMap::new(),
        }
    }
    
    /// Register a new plugin
    pub fn register(&mut self, mut plugin: Box<dyn Plugin>) -> Result<(), PluginError> {
        let plugin_id = plugin.id().to_string();
        
        // Check for duplicate IDs
        if self.plugins.contains_key(&plugin_id) {
            return Err(PluginError::DuplicatePlugin(plugin_id));
        }
        
        // Build dependency graph
        let dependencies = plugin.dependencies().iter().map(|s| s.to_string()).collect();
        self.dependency_graph.insert(plugin_id.clone(), dependencies);
        
        // Validate dependencies exist
        for dep in plugin.dependencies() {
            if !self.plugins.contains_key(dep) && dep != &plugin_id {
                return Err(PluginError::MissingDependency {
                    plugin: plugin_id,
                    dependency: dep.to_string(),
                });
            }
        }
        
        // Check for circular dependencies
        if self.has_circular_dependency(&plugin_id) {
            return Err(PluginError::CircularDependency(plugin_id));
        }
        
        self.plugins.insert(plugin_id, plugin);
        Ok(())
    }
    
    /// Initialize all plugins in dependency order
    pub fn initialize_all(&mut self, database: &LingoDatabase) -> Result<(), PluginError> {
        let initialization_order = self.resolve_dependency_order()?;
        
        for plugin_id in initialization_order {
            if let Some(plugin) = self.plugins.get_mut(&plugin_id) {
                plugin.initialize(database)?;
                self.active_plugins.push(plugin_id);
            }
        }
        
        Ok(())
    }
    
    /// Get all active plugins in dependency order
    pub fn get_active_plugins(&self) -> Vec<&dyn Plugin> {
        self.active_plugins.iter()
            .filter_map(|id| self.plugins.get(id).map(|p| p.as_ref()))
            .collect()
    }
    
    /// Execute a command on a specific plugin
    pub fn execute_command(
        &mut self,
        plugin_id: &str,
        command: &str,
        args: &[String],
        context: &PluginContext,
    ) -> Result<PluginResult, PluginError> {
        let plugin = self.plugins.get_mut(plugin_id)
            .ok_or_else(|| PluginError::PluginNotFound(plugin_id.to_string()))?;
        
        if !self.active_plugins.contains(&plugin_id.to_string()) {
            return Err(PluginError::PluginNotActive(plugin_id.to_string()));
        }
        
        plugin.handle_command(command, args, context)?
            .ok_or_else(|| PluginError::CommandNotSupported {
                plugin: plugin_id.to_string(),
                command: command.to_string(),
            })
    }
    
    /// List all registered plugins
    pub fn list_plugins(&self) -> Vec<PluginInfo> {
        self.plugins.iter()
            .map(|(id, plugin)| PluginInfo {
                id: id.clone(),
                name: plugin.name().to_string(),
                version: plugin.version().to_string(),
                active: self.active_plugins.contains(id),
                dependencies: plugin.dependencies().iter().map(|s| s.to_string()).collect(),
            })
            .collect()
    }
    
    /// Unload a plugin and all dependents
    pub fn unload_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        // Find all plugins that depend on this one
        let dependents = self.find_dependents(plugin_id);
        
        // Unload dependents first
        for dependent in dependents {
            self.unload_single_plugin(&dependent)?;
        }
        
        // Unload the target plugin
        self.unload_single_plugin(plugin_id)
    }
    
    /// Reload a plugin (unload then load)
    pub fn reload_plugin(&mut self, plugin_id: &str, database: &LingoDatabase) -> Result<(), PluginError> {
        // Store plugin data for reload
        let plugin = self.plugins.remove(plugin_id)
            .ok_or_else(|| PluginError::PluginNotFound(plugin_id.to_string()))?;
        
        // Remove from active list
        self.active_plugins.retain(|id| id != plugin_id);
        
        // Re-register and initialize
        self.register(plugin)?;
        
        if let Some(plugin) = self.plugins.get_mut(plugin_id) {
            plugin.initialize(database)?;
            self.active_plugins.push(plugin_id.to_string());
        }
        
        Ok(())
    }
    
    // Private helper methods
    
    fn has_circular_dependency(&self, plugin_id: &str) -> bool {
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        
        self.has_cycle_util(plugin_id, &mut visited, &mut rec_stack)
    }
    
    fn has_cycle_util(
        &self,
        plugin_id: &str,
        visited: &mut std::collections::HashSet<String>,
        rec_stack: &mut std::collections::HashSet<String>,
    ) -> bool {
        visited.insert(plugin_id.to_string());
        rec_stack.insert(plugin_id.to_string());
        
        if let Some(dependencies) = self.dependency_graph.get(plugin_id) {
            for dep in dependencies {
                if !visited.contains(dep) {
                    if self.has_cycle_util(dep, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(dep) {
                    return true;
                }
            }
        }
        
        rec_stack.remove(plugin_id);
        false
    }
    
    fn resolve_dependency_order(&self) -> Result<Vec<String>, PluginError> {
        let mut result = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut temp_visited = std::collections::HashSet::new();
        
        // Sort plugins by ID to ensure deterministic order
        let mut plugin_ids: Vec<_> = self.plugins.keys().collect();
        plugin_ids.sort();
        
        for plugin_id in plugin_ids {
            if !visited.contains(plugin_id) {
                self.topological_sort(plugin_id, &mut visited, &mut temp_visited, &mut result)?;
            }
        }
        
        Ok(result)
    }
    
    fn topological_sort(
        &self,
        plugin_id: &str,
        visited: &mut std::collections::HashSet<String>,
        temp_visited: &mut std::collections::HashSet<String>,
        result: &mut Vec<String>,
    ) -> Result<(), PluginError> {
        if temp_visited.contains(plugin_id) {
            return Err(PluginError::CircularDependency(plugin_id.to_string()));
        }
        
        if visited.contains(plugin_id) {
            return Ok(());
        }
        
        temp_visited.insert(plugin_id.to_string());
        
        if let Some(dependencies) = self.dependency_graph.get(plugin_id) {
            for dep in dependencies {
                self.topological_sort(dep, visited, temp_visited, result)?;
            }
        }
        
        temp_visited.remove(plugin_id);
        visited.insert(plugin_id.to_string());
        result.push(plugin_id.to_string());
        
        Ok(())
    }
    
    fn find_dependents(&self, plugin_id: &str) -> Vec<String> {
        self.dependency_graph.iter()
            .filter(|(_, deps)| deps.contains(&plugin_id.to_string()))
            .map(|(id, _)| id.clone())
            .collect()
    }
    
    fn unload_single_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        if let Some(mut plugin) = self.plugins.remove(plugin_id) {
            plugin.cleanup()?;
        }
        
        self.active_plugins.retain(|id| id != plugin_id);
        self.dependency_graph.remove(plugin_id);
        
        Ok(())
    }
}

/// Plugin-related errors
#[derive(Debug, Clone)]
pub enum PluginError {
    /// Plugin with this ID already exists
    DuplicatePlugin(String),
    /// Required dependency is missing
    MissingDependency { plugin: String, dependency: String },
    /// Circular dependency detected
    CircularDependency(String),
    /// Plugin not found
    PluginNotFound(String),
    /// Plugin is not active
    PluginNotActive(String),
    /// Command not supported by plugin
    CommandNotSupported { plugin: String, command: String },
    /// Plugin not initialized
    NotInitialized(String),
    /// Lock acquisition failed
    LockError(String),
    /// Type mismatch in data storage
    TypeMismatch(String),
    /// Plugin initialization failed
    InitializationFailed { plugin: String, error: String },
    /// Plugin cleanup failed
    CleanupFailed { plugin: String, error: String },
}

impl fmt::Display for PluginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginError::DuplicatePlugin(id) => {
                write!(f, "Plugin with ID '{}' already exists", id)
            },
            PluginError::MissingDependency { plugin, dependency } => {
                write!(f, "Plugin '{}' requires missing dependency '{}'", plugin, dependency)
            },
            PluginError::CircularDependency(id) => {
                write!(f, "Circular dependency detected involving plugin '{}'", id)
            },
            PluginError::PluginNotFound(id) => {
                write!(f, "Plugin '{}' not found", id)
            },
            PluginError::PluginNotActive(id) => {
                write!(f, "Plugin '{}' is not active", id)
            },
            PluginError::CommandNotSupported { plugin, command } => {
                write!(f, "Plugin '{}' does not support command '{}'", plugin, command)
            },
            PluginError::NotInitialized(msg) => {
                write!(f, "Plugin system not initialized: {}", msg)
            },
            PluginError::LockError(msg) => {
                write!(f, "Lock error: {}", msg)
            },
            PluginError::TypeMismatch(msg) => {
                write!(f, "Type mismatch: {}", msg)
            },
            PluginError::InitializationFailed { plugin, error } => {
                write!(f, "Plugin '{}' initialization failed: {}", plugin, error)
            },
            PluginError::CleanupFailed { plugin, error } => {
                write!(f, "Plugin '{}' cleanup failed: {}", plugin, error)
            },
        }
    }
}

impl std::error::Error for PluginError {}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugins::Plugin;
    use crate::storage::LingoDatabase;
    
    // Mock plugins for testing
    struct MockPluginA;
    struct MockPluginB;
    struct MockPluginC;
    
    impl Plugin for MockPluginA {
        fn id(&self) -> &'static str { "plugin_a" }
        fn name(&self) -> &'static str { "Plugin A" }
        fn version(&self) -> &'static str { "1.0.0" }
        fn initialize(&mut self, _database: &LingoDatabase) -> Result<(), PluginError> { Ok(()) }
    }
    
    impl Plugin for MockPluginB {
        fn id(&self) -> &'static str { "plugin_b" }
        fn name(&self) -> &'static str { "Plugin B" }
        fn version(&self) -> &'static str { "1.0.0" }
        fn dependencies(&self) -> Vec<&'static str> { vec!["plugin_a"] }
        fn initialize(&mut self, _database: &LingoDatabase) -> Result<(), PluginError> { Ok(()) }
    }
    
    impl Plugin for MockPluginC {
        fn id(&self) -> &'static str { "plugin_c" }
        fn name(&self) -> &'static str { "Plugin C" }
        fn version(&self) -> &'static str { "1.0.0" }
        fn dependencies(&self) -> Vec<&'static str> { vec!["plugin_b"] }
        fn initialize(&mut self, _database: &LingoDatabase) -> Result<(), PluginError> { Ok(()) }
    }
    
    #[test]
    fn test_plugin_registration() {
        let mut registry = PluginRegistry::new();
        
        assert!(registry.register(Box::new(MockPluginA)).is_ok());
        assert!(registry.register(Box::new(MockPluginB)).is_ok());
        
        let plugins = registry.list_plugins();
        assert_eq!(plugins.len(), 2);
    }
    
    #[test]
    fn test_duplicate_plugin_error() {
        let mut registry = PluginRegistry::new();
        
        assert!(registry.register(Box::new(MockPluginA)).is_ok());
        
        let result = registry.register(Box::new(MockPluginA));
        assert!(matches!(result, Err(PluginError::DuplicatePlugin(_))));
    }
    
    #[test]
    fn test_dependency_resolution() {
        let mut registry = PluginRegistry::new();
        
        // Register in correct dependency order
        assert!(registry.register(Box::new(MockPluginA)).is_ok());
        assert!(registry.register(Box::new(MockPluginB)).is_ok());
        assert!(registry.register(Box::new(MockPluginC)).is_ok());
        
        let order = registry.resolve_dependency_order().unwrap();
        
        // Should be ordered by dependencies: A, B, C
        assert_eq!(order, vec!["plugin_a", "plugin_b", "plugin_c"]);
    }
    
    #[test]
    fn test_missing_dependency_error() {
        let mut registry = PluginRegistry::new();
        
        // Try to register B without A
        let result = registry.register(Box::new(MockPluginB));
        assert!(matches!(result, Err(PluginError::MissingDependency { .. })));
    }
}
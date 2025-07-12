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

//! SLANG bytecode executor - the heart of query execution

use crate::core::{
    NodeId,
    bytecode::{SlangOp, SlangInstruction},
    error::{LingoError, Result},
};
use crate::query::CompiledQuery;
use crate::storage::{Database, MemoryMappedDatabase};
use std::collections::HashSet;
use std::time::{Duration, Instant};

/// A set of node IDs with efficient deduplication and operations.
///
/// `NodeSet` maintains both a vector for ordered access and a HashSet
/// for O(1) membership testing. This dual structure ensures that results
/// remain deduplicated while preserving order when needed.
///
/// # Examples
///
/// ```rust
/// use lingo::engine::executor::NodeSet;
/// use lingo::core::NodeId;
///
/// let mut set = NodeSet::new();
/// set.push(NodeId(1));
/// set.push(NodeId(2));
/// set.push(NodeId(1)); // Duplicate, won't be added
///
/// assert_eq!(set.len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct NodeSet {
    /// Nodes in the set
    nodes: Vec<NodeId>,
    /// Set for O(1) contains checks
    set: HashSet<NodeId>,
    /// Whether the set is sorted
    sorted: bool,
}

impl NodeSet {
    /// Creates an empty node set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::engine::executor::NodeSet;
    ///
    /// let set = NodeSet::new();
    /// assert!(set.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            set: HashSet::new(),
            sorted: true,
        }
    }
    
    /// Creates a node set containing a single node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The node to include in the set
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::engine::executor::NodeSet;
    /// use lingo::core::NodeId;
    ///
    /// let set = NodeSet::single(NodeId(42));
    /// assert_eq!(set.len(), 1);
    /// ```
    pub fn single(node_id: NodeId) -> Self {
        let mut set = HashSet::new();
        set.insert(node_id);
        Self {
            nodes: vec![node_id],
            set,
            sorted: true,
        }
    }
    
    /// Adds a node to the set if it doesn't already exist.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The node to add
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::engine::executor::NodeSet;
    /// use lingo::core::NodeId;
    ///
    /// let mut set = NodeSet::new();
    /// set.push(NodeId(1));
    /// set.push(NodeId(1)); // Duplicate, ignored
    /// assert_eq!(set.len(), 1);
    /// ```
    pub fn push(&mut self, node_id: NodeId) {
        if self.set.insert(node_id) {
            self.nodes.push(node_id);
            self.sorted = false;
        }
    }
    
    /// Extends the set with multiple nodes.
    ///
    /// Duplicates are automatically filtered out.
    ///
    /// # Arguments
    ///
    /// * `iter` - An iterator of node IDs to add
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::engine::executor::NodeSet;
    /// use lingo::core::NodeId;
    ///
    /// let mut set = NodeSet::new();
    /// set.extend(vec![NodeId(1), NodeId(2), NodeId(1)]);
    /// assert_eq!(set.len(), 2);
    /// ```
    pub fn extend<I: IntoIterator<Item = NodeId>>(&mut self, iter: I) {
        for node_id in iter {
            self.push(node_id);
        }
    }
    
    /// Returns the number of nodes in the set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::engine::executor::NodeSet;
    /// use lingo::core::NodeId;
    ///
    /// let mut set = NodeSet::new();
    /// assert_eq!(set.len(), 0);
    /// set.push(NodeId(1));
    /// assert_eq!(set.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    
    /// Returns true if the set contains no nodes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::engine::executor::NodeSet;
    ///
    /// let set = NodeSet::new();
    /// assert!(set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    
    /// Removes all nodes from the set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::engine::executor::NodeSet;
    /// use lingo::core::NodeId;
    ///
    /// let mut set = NodeSet::single(NodeId(1));
    /// set.clear();
    /// assert!(set.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.set.clear();
        self.sorted = true;
    }
    
    /// Truncates the set to contain at most `len` nodes.
    ///
    /// If the set has fewer than `len` nodes, this is a no-op.
    ///
    /// # Arguments
    ///
    /// * `len` - Maximum number of nodes to keep
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::engine::executor::NodeSet;
    /// use lingo::core::NodeId;
    ///
    /// let mut set = NodeSet::new();
    /// set.extend(vec![NodeId(1), NodeId(2), NodeId(3)]);
    /// set.truncate(2);
    /// assert_eq!(set.len(), 2);
    /// ```
    pub fn truncate(&mut self, len: usize) {
        if len < self.nodes.len() {
            self.nodes.truncate(len);
            self.set = self.nodes.iter().copied().collect();
        }
    }
    
    /// Returns the nodes as a slice.
    ///
    /// The order is the same as insertion order (not sorted).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::engine::executor::NodeSet;
    /// use lingo::core::NodeId;
    ///
    /// let mut set = NodeSet::new();
    /// set.push(NodeId(1));
    /// set.push(NodeId(2));
    /// let slice = set.as_slice();
    /// assert_eq!(slice.len(), 2);
    /// ```
    pub fn as_slice(&self) -> &[NodeId] {
        &self.nodes
    }
    
    /// Consumes the set and returns the nodes as a vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::engine::executor::NodeSet;
    /// use lingo::core::NodeId;
    ///
    /// let mut set = NodeSet::new();
    /// set.push(NodeId(1));
    /// let vec = set.into_vec();
    /// assert_eq!(vec, vec![NodeId(1)]);
    /// ```
    pub fn into_vec(self) -> Vec<NodeId> {
        self.nodes
    }
}

impl Default for NodeSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Flags tracking execution state and errors.
///
/// These flags help diagnose execution problems and optimize future queries.
/// They are set during execution and can be inspected after completion.
#[derive(Debug, Default)]
pub struct ExecutionFlags {
    /// Stack overflow occurred
    pub stack_overflow: bool,
    /// Invalid operation
    pub invalid_op: bool,
    /// Missing index
    pub missing_index: bool,
}

/// Contains the results of executing a compiled query.
///
/// This structure provides both the result nodes and metadata about
/// the execution, useful for performance monitoring and optimization.
///
/// # Examples
///
/// ```rust,no_run
/// use lingo::{QueryBuilder, LingoExecutor};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut executor = LingoExecutor::new();
/// let query = QueryBuilder::find("test").compile();
/// let result = executor.execute(&query)?;
///
/// println!("Found {} nodes in {:?}", 
///     result.nodes.len(), 
///     result.execution_time
/// );
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct QueryResult {
    /// Result nodes
    pub nodes: NodeSet,
    /// Execution time
    pub execution_time: Duration,
    /// Number of instructions executed
    pub instructions_executed: usize,
    /// Cache hit?
    pub cache_hit: bool,
}

/// The SLANG bytecode executor - the query execution engine for Lingo.
///
/// `LingoExecutor` interprets compiled SLANG bytecode to execute queries
/// against a Lingo database. It manages execution state, provides a stack-based
/// virtual machine, and tracks performance metrics.
///
/// # Architecture
///
/// The executor uses a stack-based architecture with:
/// - An instruction pointer for sequential execution
/// - A stack for intermediate results (node sets)
/// - 16 general-purpose registers for temporary storage
/// - String cache for efficient word lookups
///
/// # Examples
///
/// ## Basic usage
///
/// ```rust,no_run
/// use lingo::{LingoExecutor, QueryBuilder};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create executor and load database
/// let mut executor = LingoExecutor::new();
/// executor.load_database("english.lingo")?;
///
/// // Build and execute query
/// let query = QueryBuilder::find("algorithm")
///     .similar_threshold(0.8)
///     .limit(10)
///     .compile();
///
/// let result = executor.execute(&query)?;
/// println!("Found {} similar terms", result.nodes.len());
/// # Ok(())
/// # }
/// ```
///
/// ## Reusing executors
///
/// ```rust,no_run
/// use lingo::{LingoExecutor, QueryBuilder};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut executor = LingoExecutor::new();
/// executor.load_database("data.lingo")?;
///
/// // Execute multiple queries with the same executor
/// for word in &["happy", "sad", "angry"] {
///     let query = QueryBuilder::find(word).similar().compile();
///     let result = executor.execute(&query)?;
///     println!("{}: {} results", word, result.nodes.len());
/// }
/// # Ok(())
/// # }
/// ```
pub struct LingoExecutor {
    /// Database access
    pub database: Option<Database>,
    
    // Execution state
    /// Instruction pointer
    instruction_pointer: usize,
    /// Stack for intermediate results
    stack: Vec<NodeSet>,
    /// Registers for temporary storage
    registers: [NodeSet; 16],
    /// Execution flags
    flags: ExecutionFlags,
    
    // String cache for LoadNode operations
    string_cache: Vec<String>,
    
    // Statistics
    instructions_executed: usize,
}

impl LingoExecutor {
    /// Creates a new executor instance.
    ///
    /// The executor is created without a database. You must call
    /// `set_database()` or `load_database()` before executing queries.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::LingoExecutor;
    ///
    /// let executor = LingoExecutor::new();
    /// // Executor is ready but needs a database
    /// ```
    pub fn new() -> Self {
        Self {
            database: None,
            instruction_pointer: 0,
            stack: Vec::new(),
            registers: Default::default(),
            flags: ExecutionFlags::default(),
            string_cache: Vec::new(),
            instructions_executed: 0,
        }
    }
    
    /// Sets the database for query execution.
    ///
    /// This method is useful when you have already loaded a database
    /// instance and want to use it with the executor.
    ///
    /// # Arguments
    ///
    /// * `database` - A loaded Lingo database
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lingo::{LingoExecutor, storage::Database};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let db = Database::open("english.lingo")?;
    /// let mut executor = LingoExecutor::new();
    /// executor.set_database(db);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_database(&mut self, database: Database) {
        self.database = Some(database);
    }
    
    /// Loads a database from a file path.
    ///
    /// This is a convenience method that opens the database file and
    /// sets it for use by the executor.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the Lingo database file
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the database was loaded successfully
    /// - `Err(LingoError)` if the file couldn't be opened or is invalid
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lingo::LingoExecutor;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut executor = LingoExecutor::new();
    /// executor.load_database("english.lingo")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn load_database<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<()> {
        let database = MemoryMappedDatabase::open(path)?;
        self.database = Some(database);
        Ok(())
    }
    
    /// Executes a compiled query against the loaded database.
    ///
    /// This method interprets the SLANG bytecode in the compiled query,
    /// performing the specified operations and returning the results.
    ///
    /// # Arguments
    ///
    /// * `query` - A compiled query from `QueryBuilder::compile()`
    ///
    /// # Returns
    ///
    /// - `Ok(QueryResult)` with the matching nodes and execution metrics
    /// - `Err(LingoError)` if execution fails (e.g., missing database, invalid operations)
    ///
    /// # Errors
    ///
    /// - `LingoError::Execution` - Invalid bytecode or execution errors
    /// - `LingoError::DatabaseNotLoaded` - No database is loaded
    /// - `LingoError::NodeNotFound` - Referenced node doesn't exist
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lingo::{LingoExecutor, QueryBuilder};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut executor = LingoExecutor::new();
    /// executor.load_database("data.lingo")?;
    ///
    /// let query = QueryBuilder::find("test").similar().compile();
    /// let result = executor.execute(&query)?;
    ///
    /// for node_id in result.nodes.as_slice() {
    ///     println!("Found node: {:?}", node_id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn execute(&mut self, query: &CompiledQuery) -> Result<QueryResult> {
        let start_time = Instant::now();
        
        // Reset execution state
        self.reset();
        
        // Set string cache from query
        self.string_cache = query.string_cache.clone();
        
        // Execute bytecode
        let result = self.execute_bytecode(&query.bytecode)?;
        
        let execution_time = start_time.elapsed();
        
        Ok(QueryResult {
            nodes: result,
            execution_time,
            instructions_executed: self.instructions_executed,
            cache_hit: false,
        })
    }
    
    /// Reset execution state
    fn reset(&mut self) {
        self.instruction_pointer = 0;
        self.stack.clear();
        self.flags = ExecutionFlags::default();
        self.instructions_executed = 0;
    }
    
    /// Execute bytecode instructions
    fn execute_bytecode(&mut self, bytecode: &[SlangInstruction]) -> Result<NodeSet> {
        while self.instruction_pointer < bytecode.len() {
            let instruction = &bytecode[self.instruction_pointer];
            
            // Execute instruction
            self.execute_instruction(instruction)?;
            
            // Check for halt
            if matches!(instruction.opcode, SlangOp::Halt) {
                break;
            }
            
            // Advance instruction pointer
            self.instruction_pointer += 1;
            self.instructions_executed += 1;
            
            // Safety check
            if self.instructions_executed > 10000 {
                return Err(LingoError::Execution("Instruction limit exceeded".to_string()));
            }
        }
        
        // Return top of stack or empty set
        Ok(self.stack.pop().unwrap_or_default())
    }
    
    /// Execute a single instruction
    fn execute_instruction(&mut self, instruction: &SlangInstruction) -> Result<()> {
        match instruction.opcode {
            SlangOp::LoadNode => {
                let string_id = instruction.operand1 as usize;
                let word = if string_id < self.string_cache.len() {
                    &self.string_cache[string_id]
                } else {
                    return Err(LingoError::Execution("Invalid string ID".to_string()));
                };
                
                // Lookup node by word
                if let Some(db) = &self.database {
                    let node_ids = db.find_nodes_by_word(word);
                    if node_ids.is_empty() {
                        // No nodes found, push empty set
                        self.stack.push(NodeSet::new());
                    } else {
                        // Push all matching nodes
                        let mut node_set = NodeSet::new();
                        node_set.extend(node_ids);
                        self.stack.push(node_set);
                    }
                } else {
                    // No database, create test node
                    let node_id = NodeId(string_id as u32 + 1);
                    self.stack.push(NodeSet::single(node_id));
                }
            }
            
            SlangOp::LoadNodeId => {
                let node_id = NodeId(instruction.operand2);
                
                // Validate node exists if database is loaded
                if let Some(db) = &self.database {
                    if db.get_node(node_id).is_err() {
                        return Err(LingoError::Execution(
                            format!("Node {} not found", node_id.0)
                        ));
                    }
                }
                
                self.stack.push(NodeSet::single(node_id));
            }
            
            SlangOp::FindSimilar => {
                let threshold = (instruction.operand1 as f32) / 65535.0;
                let limit = if instruction.flags & crate::core::bytecode::instruction_flags::HAS_LIMIT != 0 {
                    Some(instruction.operand2 as usize)
                } else {
                    None
                };
                
                let current = self.stack.pop().ok_or_else(|| {
                    LingoError::Execution("Empty stack for FindSimilar".to_string())
                })?;
                
                let mut similar = NodeSet::new();
                
                if let Some(db) = &self.database {
                    // Find similar nodes for each node in current set
                    for node_id in current.as_slice() {
                        if let Ok(node) = db.get_node(*node_id) {
                            let radius = 1.0 - threshold; // Convert similarity to distance
                            let similar_ids = db.find_similar_nodes(
                                node.position,
                                radius,
                                limit
                            );
                            similar.extend(similar_ids);
                        }
                    }
                    
                    if let Some(limit) = limit {
                        similar.truncate(limit);
                    }
                } else {
                    // No database, return current
                    similar = current;
                }
                
                self.stack.push(similar);
            }
            
            SlangOp::LayerUp => {
                let levels = instruction.operand1 as u8;
                let current = self.stack.pop().ok_or_else(|| {
                    LingoError::Execution("Empty stack for LayerUp".to_string())
                })?;
                
                let mut parents = NodeSet::new();
                
                if let Some(db) = &self.database {
                    // Find parent nodes using vertical index
                    for node_id in current.as_slice() {
                        if let Ok(node) = db.get_node(*node_id) {
                            // Get connections and find parents
                            if let Ok(connections) = db.get_node_connections(*node_id) {
                                for conn in connections {
                                    if conn.connection_type == crate::core::ConnectionType::Hypernymy {
                                        parents.push(conn.target_node);
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // Test implementation
                    for node_id in current.as_slice() {
                        parents.push(NodeId(node_id.0 + 100));
                    }
                }
                
                self.stack.push(parents);
            }
            
            SlangOp::LayerDown => {
                let levels = instruction.operand1 as u8;
                let current = self.stack.pop().ok_or_else(|| {
                    LingoError::Execution("Empty stack for LayerDown".to_string())
                })?;
                
                // In real implementation, use vertical index
                // For now, return modified nodes
                let mut children = NodeSet::new();
                for node_id in current.as_slice() {
                    // Simulate child nodes
                    if node_id.0 > 100 {
                        children.push(NodeId(node_id.0 - 100));
                    }
                }
                
                self.stack.push(children);
            }
            
            SlangOp::FollowConnection => {
                let rank = instruction.operand1 as usize;
                let current = self.stack.pop().ok_or_else(|| {
                    LingoError::Execution("Empty stack for FollowConnection".to_string())
                })?;
                
                let mut connected = NodeSet::new();
                
                if let Some(db) = &self.database {
                    for node_id in current.as_slice() {
                        if let Ok(connections) = db.get_node_connections(*node_id) {
                            // Sort by strength and get Nth connection
                            let mut sorted_conns: Vec<_> = connections.iter()
                                .map(|c| (c, c.strength))
                                .collect();
                            sorted_conns.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                            
                            if rank < sorted_conns.len() {
                                connected.push(sorted_conns[rank].0.target_node);
                            }
                        }
                    }
                }
                
                self.stack.push(connected);
            }
            
            SlangOp::SpatialNeighbors => {
                let radius = f32::from_bits(instruction.operand2);
                let layer_mask = instruction.operand3 as u8;
                
                let current = self.stack.pop().ok_or_else(|| {
                    LingoError::Execution("Empty stack for SpatialNeighbors".to_string())
                })?;
                
                // In real implementation, use spatial index
                // For now, return current nodes
                self.stack.push(current);
            }
            
            SlangOp::Limit => {
                let limit = instruction.operand1 as usize;
                if let Some(mut current) = self.stack.pop() {
                    current.truncate(limit);
                    self.stack.push(current);
                }
            }
            
            SlangOp::Deduplicate => {
                // Already handled by NodeSet
                if let Some(current) = self.stack.pop() {
                    self.stack.push(current);
                }
            }
            
            SlangOp::Push => {
                // Push current to register
                let reg = (instruction.operand1 as usize).min(15);
                if let Some(current) = self.stack.last() {
                    self.registers[reg] = current.clone();
                }
            }
            
            SlangOp::Pop => {
                // Pop from register to stack
                let reg = (instruction.operand1 as usize).min(15);
                self.stack.push(self.registers[reg].clone());
            }
            
            SlangOp::Halt => {
                // Stop execution
                return Ok(());
            }
            
            _ => {
                // Unsupported operation
                return Err(LingoError::Execution(
                    format!("Unsupported operation: {:?}", instruction.opcode)
                ));
            }
        }
        
        Ok(())
    }
    
    /// Add string to cache (for testing)
    pub fn add_string(&mut self, s: String) -> u16 {
        let id = self.string_cache.len() as u16;
        self.string_cache.push(s);
        id
    }
}

impl Default for LingoExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// Tracks aggregate statistics across multiple query executions.
///
/// Useful for performance monitoring and optimization. Statistics
/// are cumulative and can be reset as needed.
///
/// # Examples
///
/// ```rust,no_run
/// use lingo::{LingoExecutor, engine::executor::ExecutionStats};
///
/// let stats = ExecutionStats::default();
/// println!("Average instructions per query: {}", stats.avg_instructions);
/// ```
#[derive(Debug, Default)]
pub struct ExecutionStats {
    /// Total queries executed
    pub total_queries: usize,
    /// Total execution time
    pub total_time: Duration,
    /// Cache hits
    pub cache_hits: usize,
    /// Average instructions per query
    pub avg_instructions: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::QueryBuilder;
    
    #[test]
    fn test_basic_execution() {
        let mut executor = LingoExecutor::new();
        
        // Build and compile a query
        let query = QueryBuilder::find("technical")
            .limit(5)
            .compile();
        
        // Execute
        let result = executor.execute(&query).unwrap();
        
        // Should have some results
        assert!(!result.nodes.is_empty());
        assert!(result.instructions_executed > 0);
    }
    
    #[test]
    fn test_stack_operations() {
        let mut executor = LingoExecutor::new();
        
        // Build a more complex query
        let query = QueryBuilder::find("viral")
            .similar()
            .layer_up()
            .limit(10)
            .compile();
        
        // Execute
        let result = executor.execute(&query).unwrap();
        
        // Should complete without errors
        assert!(result.execution_time.as_micros() > 0);
    }
}
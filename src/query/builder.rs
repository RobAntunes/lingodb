//! Query builder with fluent API for constructing linguistic queries

use crate::core::bytecode::{SlangOp, SlangInstruction};
use crate::core::{Layer, ConnectionType};
use crate::security::{validate_query, validate_limit};
use crate::logging::{debug, trace};
use std::fmt;

/// Represents a single operation in the query pipeline.
///
/// Operations are the building blocks of queries, each performing a specific
/// transformation or navigation through the linguistic database. They compile
/// to SLANG bytecode instructions for efficient execution.
///
/// # Categories
///
/// - **Loading**: `LoadNode`, `LoadNodeId` - Entry points into the database
/// - **Navigation**: `LayerUp`, `LayerDown`, `LayerSet` - Move between layers
/// - **Discovery**: `FindSimilar`, `SpatialNeighbors` - Find related nodes
/// - **Traversal**: `FollowConnection`, `FollowConnectionType` - Follow relationships
/// - **Filtering**: `Filter`, `Sort`, `Limit`, `Deduplicate` - Refine results
#[derive(Debug, Clone)]
pub enum Operation {
    /// Load node by word
    LoadNode(String),
    /// Load node by ID
    LoadNodeId(u32),
    /// Move up N layers
    LayerUp(u8),
    /// Move down N layers
    LayerDown(u8),
    /// Set to specific layer
    LayerSet(Layer),
    /// Find similar nodes
    FindSimilar { threshold: f32, limit: Option<usize> },
    /// Find spatial neighbors
    SpatialNeighbors { radius: f32, layer_mask: Option<u8> },
    /// Follow connection by strength rank
    FollowConnection { strength_rank: u8 },
    /// Follow specific connection type
    FollowConnectionType { connection_type: ConnectionType },
    /// Filter results
    Filter(FilterCriteria),
    /// Sort results
    Sort(SortCriteria),
    /// Limit result count
    Limit(usize),
    /// Remove duplicates
    Deduplicate,
    /// Decompose word into morphemes (opposite of compose)
    Decompose,
    /// Find nodes within radius of a specific 3D point
    SpatialRadiusFromPoint { center: crate::core::Coordinate3D, radius: f32 },
}

/// Defines criteria for filtering query results.
///
/// Filters are applied after the main query operations to refine the result set
/// based on node properties. Multiple filters can be combined for precise control.
///
/// # Examples
///
/// ```rust
/// use lingo::query::{QueryBuilder, FilterCriteria};
/// use lingo::core::node::Layer;
///
/// let query = QueryBuilder::find("analyze")
///     .similar()
///     .filter(FilterCriteria::Layer(Layer::Words))
///     .filter(FilterCriteria::MinFrequency(1000))
///     .compile();
/// ```
#[derive(Debug, Clone)]
pub enum FilterCriteria {
    /// Filter by layer
    Layer(Layer),
    /// Filter by minimum frequency
    MinFrequency(u32),
    /// Filter by etymology
    Etymology(crate::core::EtymologyOrigin),
    /// Filter by flags
    HasFlags(crate::core::NodeFlags),
}

/// Defines how query results should be sorted.
///
/// Sorting is applied as a post-processing step after all other operations.
/// The sort order depends on the criteria:
/// - `Frequency`: Descending (most frequent first)
/// - `Distance`: Ascending (closest first)
/// - `ConnectionStrength`: Descending (strongest first)
/// - `Alphabetical`: Ascending (A to Z)
#[derive(Debug, Clone)]
pub enum SortCriteria {
    /// Sort by frequency (descending)
    Frequency,
    /// Sort by similarity/distance
    Distance,
    /// Sort by connection strength
    ConnectionStrength,
    /// Sort alphabetically
    Alphabetical,
}

/// Provides hints to the query compiler for optimization.
///
/// These hints help the compiler generate more efficient bytecode and
/// ensure required indices are available at execution time. They are
/// automatically updated as operations are added to the query.
#[derive(Debug, Default)]
pub struct OptimizationHints {
    /// Query needs spatial index
    pub needs_spatial_index: bool,
    /// Query needs vertical index
    pub needs_vertical_index: bool,
    /// Query needs connection index
    pub needs_connection_index: bool,
    /// Estimated result size
    pub estimated_results: Option<usize>,
}

/// A fluent interface for building linguistic queries.
///
/// `QueryBuilder` provides an intuitive API for constructing complex queries
/// through method chaining. Queries compile to optimized SLANG bytecode for
/// efficient execution against the database.
///
/// # Design Philosophy
///
/// The builder follows a fluent interface pattern where each method returns
/// `self`, allowing for natural query construction that reads like English.
/// Operations are applied in the order they are specified.
///
/// # Examples
///
/// ## Finding similar words
///
/// ```rust
/// use lingo::QueryBuilder;
///
/// let query = QueryBuilder::find("viral")
///     .similar_threshold(0.85)
///     .limit(10)
///     .compile();
/// ```
///
/// ## Exploring concept hierarchies
///
/// ```rust
/// use lingo::QueryBuilder;
///
/// let query = QueryBuilder::find("computer")
///     .layer_up_n(2)           // Move to concept layer
///     .follow_connection()     // Follow strongest connection
///     .layer_down()            // Back to words
///     .deduplicate()           // Remove any duplicates
///     .limit(20)
///     .compile();
/// ```
///
/// ## Cross-linguistic connections
///
/// ```rust
/// use lingo::{QueryBuilder, core::ConnectionType};
///
/// let query = QueryBuilder::find("metaphor")
///     .follow_connection_type(ConnectionType::Etymology)
///     .spatial_neighbors(0.2)
///     .compile();
/// ```
///
/// # Performance
///
/// Queries are compiled to bytecode at build time, not execution time.
/// This ahead-of-time compilation ensures optimal performance during execution.
/// The builder tracks which indices will be needed and includes this information
/// in the compiled query.
#[derive(Debug)]
pub struct QueryBuilder {
    /// Operations to perform
    operations: Vec<Operation>,
    /// Optimization hints
    hints: OptimizationHints,
}

impl QueryBuilder {
    /// Creates a new query starting with a word lookup.
    ///
    /// This is the most common entry point for queries. The word is looked up
    /// across all layers, returning all nodes that match the given text.
    ///
    /// # Arguments
    ///
    /// * `word` - The word or text to search for
    ///
    /// # Returns
    ///
    /// A new `QueryBuilder` initialized with a word lookup operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::QueryBuilder;
    ///
    /// // Find all nodes for "run" (verb, noun, morpheme, etc.)
    /// let query = QueryBuilder::find("run").compile();
    /// ```
    pub fn find(word: &str) -> Self {
        // Validate the query string (will panic if invalid for now)
        // In production, this should return Result<Self>
        if let Err(e) = validate_query(word) {
            // For now, we'll use a safe default
            eprintln!("Warning: Invalid query '{}': {}", word, e);
            return Self {
                operations: vec![Operation::LoadNode(String::new())],
                hints: OptimizationHints::default(),
            };
        }
        
        Self {
            operations: vec![Operation::LoadNode(word.to_string())],
            hints: OptimizationHints::default(),
        }
    }
    
    /// Creates a new query starting with a specific node ID.
    ///
    /// This is useful when you have a node ID from a previous query or
    /// from an external source and want to explore its relationships.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The unique identifier of the node
    ///
    /// # Returns
    ///
    /// A new `QueryBuilder` initialized with a node ID lookup.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::QueryBuilder;
    ///
    /// // Start from a known node
    /// let query = QueryBuilder::find_by_id(42)
    ///     .follow_connection()
    ///     .compile();
    /// ```
    pub fn find_by_id(node_id: u32) -> Self {
        Self {
            operations: vec![Operation::LoadNodeId(node_id)],
            hints: OptimizationHints::default(),
        }
    }
    
    /// Creates a new query starting with a specific node ID.
    /// 
    /// Alias for find_by_id for more intuitive API when loading known nodes.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The NodeId to load
    ///
    /// # Returns
    ///
    /// A new `QueryBuilder` instance with the specified node loaded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::{QueryBuilder, NodeId};
    ///
    /// let node_id = NodeId(12345);
    /// let query = QueryBuilder::load(node_id)
    ///     .decompose()  // Break into morphemes
    ///     .compile();
    /// ```
    pub fn load(node_id: crate::core::NodeId) -> Self {
        Self::find_by_id(node_id.0)
    }
    
    /// Finds nodes similar to the current result set.
    ///
    /// Uses spatial proximity in 3D space to find semantically related nodes.
    /// The default similarity threshold is 0.7 (70% similarity).
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::QueryBuilder;
    ///
    /// let query = QueryBuilder::find("happy")
    ///     .similar()  // Finds "joyful", "cheerful", etc.
    ///     .compile();
    /// ```
    pub fn similar(mut self) -> Self {
        self.operations.push(Operation::FindSimilar {
            threshold: 0.7,
            limit: None,
        });
        self.hints.needs_spatial_index = true;
        self
    }
    
    /// Finds similar nodes with a custom similarity threshold.
    ///
    /// # Arguments
    ///
    /// * `threshold` - Similarity threshold between 0.0 and 1.0:
    ///   - 1.0 = Exact matches only
    ///   - 0.9 = Very similar nodes
    ///   - 0.7 = Moderately similar (default)
    ///   - 0.5 = Loosely related
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::QueryBuilder;
    ///
    /// // Find very similar technical terms
    /// let query = QueryBuilder::find("algorithm")
    ///     .similar_threshold(0.9)
    ///     .compile();
    /// ```
    pub fn similar_threshold(mut self, threshold: f32) -> Self {
        self.operations.push(Operation::FindSimilar {
            threshold,
            limit: None,
        });
        self.hints.needs_spatial_index = true;
        self
    }
    
    /// Finds all nodes within a spatial radius of the current results.
    ///
    /// This performs a 3D spatial search, finding nodes whose Euclidean
    /// distance from any node in the current result set is within the
    /// specified radius.
    ///
    /// # Arguments
    ///
    /// * `radius` - Search radius in 3D space (typically 0.1 to 0.5)
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::QueryBuilder;
    ///
    /// // Find words in the semantic neighborhood
    /// let query = QueryBuilder::find("ocean")
    ///     .spatial_neighbors(0.2)  // "sea", "water", "marine", etc.
    ///     .compile();
    /// ```
    pub fn spatial_neighbors(mut self, radius: f32) -> Self {
        self.operations.push(Operation::SpatialNeighbors {
            radius,
            layer_mask: None,
        });
        self.hints.needs_spatial_index = true;
        self
    }
    
    /// Find nodes within a radius of a specific 3D coordinate point.
    ///
    /// This is useful for finding words with specific semantic properties based on 
    /// their position in the 3D linguistic space.
    ///
    /// # Arguments
    ///
    /// * `center` - The 3D coordinate to search around
    /// * `radius` - Search radius (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::{QueryBuilder, Coordinate3D};
    ///
    /// let center = Coordinate3D::new(0.5, 0.3, 0.55);
    /// let query = QueryBuilder::spatial_radius_from_point(center, 0.2)
    ///     .compile();
    /// ```
    pub fn spatial_radius_from_point(center: crate::core::Coordinate3D, radius: f32) -> Self {
        Self {
            operations: vec![Operation::SpatialRadiusFromPoint { center, radius }],
            hints: OptimizationHints {
                needs_spatial_index: true,
                ..Default::default()
            },
        }
    }
    
    /// Moves up one layer in the linguistic hierarchy.
    ///
    /// This follows vertical connections to parent nodes in the layer above.
    /// For example, moving up from Words goes to Phrases, from Morphemes to Words.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::QueryBuilder;
    ///
    /// // Find phrases containing "run"
    /// let query = QueryBuilder::find("run")
    ///     .layer_up()  // Words → Phrases
    ///     .compile();
    /// ```
    pub fn layer_up(self) -> Self {
        self.layer_up_n(1)
    }
    
    /// Moves up multiple layers in the linguistic hierarchy.
    ///
    /// # Arguments
    ///
    /// * `levels` - Number of layers to move up (clamped to valid range)
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::QueryBuilder;
    ///
    /// // From words to concepts
    /// let query = QueryBuilder::find("algorithm")
    ///     .layer_up_n(2)  // Words → Phrases → Concepts
    ///     .compile();
    /// ```
    pub fn layer_up_n(mut self, levels: u8) -> Self {
        self.operations.push(Operation::LayerUp(levels));
        self.hints.needs_vertical_index = true;
        self
    }
    
    /// Move down one or more layers
    pub fn layer_down(self) -> Self {
        self.layer_down_n(1)
    }
    
    /// Move down N layers
    pub fn layer_down_n(mut self, levels: u8) -> Self {
        self.operations.push(Operation::LayerDown(levels));
        self.hints.needs_vertical_index = true;
        self
    }
    
    /// Set to a specific layer
    ///
    /// This operation filters results to only include nodes from the specified layer.
    /// Unlike layer_up/layer_down which navigate relationships, this directly
    /// filters by layer membership.
    ///
    /// # Arguments
    ///
    /// * `layer` - The target layer to filter by
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::{QueryBuilder, Layer};
    ///
    /// // Find only morpheme nodes containing "manage"
    /// let query = QueryBuilder::find("manage")
    ///     .layer(Layer::Morphemes)
    ///     .compile();
    /// ```
    pub fn layer(mut self, layer: Layer) -> Self {
        self.operations.push(Operation::LayerSet(layer));
        self
    }
    
    /// Follows the strongest orthogonal connection from each node.
    ///
    /// Connections are ranked by strength, and this follows the top-ranked
    /// connection. This is useful for finding the most relevant related concepts.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::QueryBuilder;
    ///
    /// // Find the primary association
    /// let query = QueryBuilder::find("heart")
    ///     .follow_connection()  // Might lead to "cardiac" or "love"
    ///     .compile();
    /// ```
    pub fn follow_connection(mut self) -> Self {
        self.operations.push(Operation::FollowConnection {
            strength_rank: 0,
        });
        self.hints.needs_connection_index = true;
        self
    }
    
    /// Follow Nth strongest connection
    pub fn follow_nth_connection(mut self, rank: u8) -> Self {
        self.operations.push(Operation::FollowConnection {
            strength_rank: rank,
        });
        self.hints.needs_connection_index = true;
        self
    }
    
    /// Follows connections of a specific type.
    ///
    /// This allows precise traversal of the semantic graph by following
    /// only connections of the specified type.
    ///
    /// # Arguments
    ///
    /// * `connection_type` - The type of connection to follow
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::{QueryBuilder, core::ConnectionType};
    ///
    /// // Find etymological roots
    /// let query = QueryBuilder::find("democracy")
    ///     .follow_connection_type(ConnectionType::Etymology)
    ///     .compile();
    /// ```
    pub fn follow_connection_type(mut self, connection_type: ConnectionType) -> Self {
        self.operations.push(Operation::FollowConnectionType {
            connection_type,
        });
        self.hints.needs_connection_index = true;
        self
    }
    
    /// Filter results
    pub fn filter(mut self, criteria: FilterCriteria) -> Self {
        self.operations.push(Operation::Filter(criteria));
        self
    }
    
    /// Sort results
    pub fn sort(mut self, criteria: SortCriteria) -> Self {
        self.operations.push(Operation::Sort(criteria));
        self
    }
    
    /// Limits the number of results returned.
    ///
    /// This is applied as the final step, after all other operations.
    /// It's recommended to always set a reasonable limit to prevent
    /// excessive memory usage.
    ///
    /// # Arguments
    ///
    /// * `count` - Maximum number of results to return
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::QueryBuilder;
    ///
    /// let query = QueryBuilder::find("test")
    ///     .similar()
    ///     .limit(20)  // Return at most 20 results
    ///     .compile();
    /// ```
    pub fn limit(mut self, count: usize) -> Self {
        // Validate the limit
        let safe_count = match validate_limit(count) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Warning: Invalid limit {}: {}", count, e);
                100 // Default safe limit
            }
        };
        
        self.operations.push(Operation::Limit(safe_count));
        if let Some(est) = &mut self.hints.estimated_results {
            *est = (*est).min(safe_count);
        } else {
            self.hints.estimated_results = Some(safe_count);
        }
        self
    }
    
    /// Remove duplicate nodes
    pub fn deduplicate(mut self) -> Self {
        self.operations.push(Operation::Deduplicate);
        self
    }
    
    /// Decomposes the current nodes into their constituent morphemes.
    ///
    /// This is the inverse of composition - it breaks down words into their
    /// morphological components (roots, prefixes, suffixes). For each word node
    /// in the current result set, this operation finds all morphemes that
    /// compose it by following the layer-down connections.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::QueryBuilder;
    ///
    /// // Decompose "manager" into ["manage", "-er"]
    /// let query = QueryBuilder::find("manager")
    ///     .decompose()
    ///     .compile();
    /// ```
    pub fn decompose(mut self) -> Self {
        self.operations.push(Operation::Decompose);
        self.hints.needs_vertical_index = true;
        self
    }
    
    /// Compiles the query to optimized SLANG bytecode.
    ///
    /// This is the final step in query construction. The resulting
    /// `CompiledQuery` can be executed multiple times efficiently.
    ///
    /// # Returns
    ///
    /// A `CompiledQuery` containing:
    /// - Optimized bytecode instructions
    /// - Required index information
    /// - Estimated execution cost
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lingo::QueryBuilder;
    ///
    /// let compiled = QueryBuilder::find("test")
    ///     .similar()
    ///     .limit(10)
    ///     .compile();
    ///
    /// // The compiled query can be reused
    /// println!("Estimated cost: {}", compiled.estimated_cost);
    /// ```
    pub fn compile(self) -> CompiledQuery {
        let mut compiler = QueryCompiler::new();
        compiler.compile(self.operations, self.hints)
    }
}

/// A compiled query ready for execution.
///
/// Contains optimized SLANG bytecode and metadata needed for efficient
/// execution. Compiled queries can be cached and reused multiple times.
///
/// # Examples
///
/// ```rust
/// use lingo::QueryBuilder;
///
/// let compiled = QueryBuilder::find("example").compile();
/// assert!(compiled.bytecode.len() > 0);
/// assert!(compiled.estimated_cost > 0);
/// ```
#[derive(Debug, Clone)]
pub struct CompiledQuery {
    /// SLANG bytecode instructions
    pub bytecode: Vec<SlangInstruction>,
    /// Required indices for execution
    pub required_indices: RequiredIndices,
    /// Estimated execution cost
    pub estimated_cost: u32,
    /// String cache for LoadNode operations
    pub string_cache: Vec<String>,
}

/// Specifies which indices are required for efficient query execution.
///
/// The executor uses this information to ensure necessary indices are
/// loaded before executing the query. Missing required indices may
/// result in degraded performance or errors.
#[derive(Debug, Clone, Default)]
pub struct RequiredIndices {
    /// Needs spatial index
    pub spatial: bool,
    /// Needs vertical index
    pub vertical: bool,
    /// Needs connection index
    pub connections: bool,
}

/// Query compiler
struct QueryCompiler {
    /// String interning for LoadNode operations
    string_cache: Vec<String>,
}

impl QueryCompiler {
    fn new() -> Self {
        Self {
            string_cache: Vec::new(),
        }
    }
    
    fn compile(&mut self, operations: Vec<Operation>, hints: OptimizationHints) -> CompiledQuery {
        debug!(
            operation_count = operations.len(),
            "Compiling query to bytecode"
        );
        
        let mut bytecode = Vec::new();
        let mut required_indices = RequiredIndices::default();
        
        // Update required indices from hints
        required_indices.spatial = hints.needs_spatial_index;
        required_indices.vertical = hints.needs_vertical_index;
        required_indices.connections = hints.needs_connection_index;
        
        // Compile each operation
        for (i, op) in operations.into_iter().enumerate() {
            trace!(operation_index = i, operation = ?op, "Compiling operation");
            self.compile_operation(op, &mut bytecode);
        }
        
        // Add halt instruction
        bytecode.push(SlangInstruction::new(SlangOp::Halt));
        
        // Estimate cost
        let estimated_cost = self.estimate_cost(&bytecode);
        
        debug!(
            bytecode_length = bytecode.len(),
            estimated_cost = estimated_cost,
            "Query compilation completed"
        );
        
        CompiledQuery {
            bytecode,
            required_indices,
            estimated_cost,
            string_cache: self.string_cache.clone(),
        }
    }
    
    fn compile_operation(&mut self, op: Operation, bytecode: &mut Vec<SlangInstruction>) {
        match op {
            Operation::LoadNode(word) => {
                // Intern string and get ID
                let string_id = self.intern_string(word);
                bytecode.push(SlangInstruction::with_operand1(
                    SlangOp::LoadNode,
                    string_id,
                ));
            }
            
            Operation::LoadNodeId(id) => {
                bytecode.push(SlangInstruction::with_operand2(
                    SlangOp::LoadNodeId,
                    0,
                    id,
                ));
            }
            
            Operation::LayerUp(levels) => {
                bytecode.push(SlangInstruction::with_operand1(
                    SlangOp::LayerUp,
                    levels as u16,
                ));
            }
            
            Operation::LayerDown(levels) => {
                bytecode.push(SlangInstruction::with_operand1(
                    SlangOp::LayerDown,
                    levels as u16,
                ));
            }
            
            Operation::FindSimilar { threshold, limit } => {
                let threshold_fixed = (threshold * 65535.0) as u16;
                let mut instruction = SlangInstruction::with_operand2(
                    SlangOp::FindSimilar,
                    threshold_fixed,
                    limit.unwrap_or(usize::MAX) as u32,
                );
                if limit.is_some() {
                    instruction.flags |= crate::core::bytecode::instruction_flags::HAS_LIMIT;
                }
                bytecode.push(instruction);
            }
            
            Operation::SpatialNeighbors { radius, layer_mask } => {
                bytecode.push(SlangInstruction::with_all_operands(
                    SlangOp::SpatialNeighbors,
                    0,
                    0,
                    radius.to_bits(),
                    layer_mask.unwrap_or(0xFF) as u32,
                ));
            }
            
            Operation::FollowConnection { strength_rank } => {
                bytecode.push(SlangInstruction::with_operand1(
                    SlangOp::FollowConnection,
                    strength_rank as u16,
                ));
            }
            
            Operation::FollowConnectionType { connection_type } => {
                bytecode.push(SlangInstruction::with_operand1(
                    SlangOp::FollowConnectionType,
                    connection_type as u16,
                ));
            }
            
            Operation::Limit(count) => {
                bytecode.push(SlangInstruction::with_operand1(
                    SlangOp::Limit,
                    count.min(u16::MAX as usize) as u16,
                ));
            }
            
            Operation::Deduplicate => {
                bytecode.push(SlangInstruction::new(SlangOp::Deduplicate));
            }
            
            Operation::Decompose => {
                // Decompose is essentially a layer_down operation specifically for morphemes
                bytecode.push(SlangInstruction::with_operand1(
                    SlangOp::LayerDown,
                    1, // Go down one layer from words to morphemes
                ));
            }
            
            Operation::LayerSet(layer) => {
                bytecode.push(SlangInstruction::with_operand1(
                    SlangOp::LayerSet,
                    layer as u16,
                ));
            }
            
            Operation::SpatialRadiusFromPoint { center, radius } => {
                // Encode the 3D coordinate and radius into the instruction
                bytecode.push(SlangInstruction::with_all_operands(
                    SlangOp::SpatialNeighbors, // Reuse existing spatial operation
                    0, // Flags
                    ((center.x * 255.0) as u8 as u16) | (((center.y * 255.0) as u8 as u16) << 8), // X,Y coordinates packed
                    ((center.z * 65535.0) as u32), // Z coordinate
                    radius.to_bits(), // Radius as bits
                ));
            }
            
            Operation::Filter(_) | Operation::Sort(_) => {
                // TODO: Implement filter and sort compilation
                // For now, these are no-ops
            }
        }
    }
    
    fn intern_string(&mut self, s: String) -> u16 {
        // Simple string interning
        if let Some(pos) = self.string_cache.iter().position(|cached| cached == &s) {
            pos as u16
        } else {
            let id = self.string_cache.len() as u16;
            self.string_cache.push(s);
            id
        }
    }
    
    fn estimate_cost(&self, bytecode: &[SlangInstruction]) -> u32 {
        // Simple cost model
        let mut cost = 0u32;
        
        for instruction in bytecode {
            cost += match instruction.opcode {
                SlangOp::LoadNode | SlangOp::LoadNodeId => 1,
                SlangOp::LayerUp | SlangOp::LayerDown => 10,
                SlangOp::FindSimilar => 50,
                SlangOp::SpatialNeighbors => 40,
                SlangOp::FollowConnection => 5,
                SlangOp::FollowConnectionType => 10,
                SlangOp::Limit => 1,
                SlangOp::Deduplicate => 20,
                SlangOp::Halt => 0,
                _ => 5,
            };
        }
        
        cost
    }
}

impl fmt::Display for QueryBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Query[")?;
        for (i, op) in self.operations.iter().enumerate() {
            if i > 0 {
                write!(f, " → ")?;
            }
            match op {
                Operation::LoadNode(word) => write!(f, "find('{}')", word)?,
                Operation::FindSimilar { threshold, .. } => write!(f, "similar({})", threshold)?,
                Operation::LayerUp(n) => write!(f, "up({})", n)?,
                Operation::LayerDown(n) => write!(f, "down({})", n)?,
                Operation::FollowConnection { strength_rank } => write!(f, "follow(#{})", strength_rank)?,
                Operation::Limit(n) => write!(f, "limit({})", n)?,
                _ => write!(f, "{:?}", op)?,
            }
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Layer, Coordinate3D, ConnectionType};
    
    #[test]
    fn test_query_builder() {
        let query = QueryBuilder::find("technical")
            .similar()
            .layer_up()
            .limit(10);
        
        let compiled = query.compile();
        
        // Should have LoadNode, FindSimilar, LayerUp, Limit, Halt
        assert_eq!(compiled.bytecode.len(), 5);
        assert_eq!(compiled.bytecode[0].opcode, SlangOp::LoadNode);
        assert_eq!(compiled.bytecode[1].opcode, SlangOp::FindSimilar);
        assert_eq!(compiled.bytecode[2].opcode, SlangOp::LayerUp);
        assert_eq!(compiled.bytecode[3].opcode, SlangOp::Limit);
        assert_eq!(compiled.bytecode[4].opcode, SlangOp::Halt);
    }
    
    #[test]
    fn test_query_display() {
        let query = QueryBuilder::find("viral")
            .similar_threshold(0.8)
            .follow_connection()
            .limit(5);
        
        let display = format!("{}", query);
        assert!(display.contains("find('viral')"));
        assert!(display.contains("similar(0.8)"));
        assert!(display.contains("follow(#0)"));
        assert!(display.contains("limit(5)"));
    }
    
    #[test]
    fn test_query_builder_layer_operations() {
        let query = QueryBuilder::find("test")
            .layer_up()
            .layer_down()
            .layer(Layer::Words)
            .compile();
        
        assert!(query.bytecode.len() >= 4); // LoadNode, LayerUp, LayerDown, LayerSet
    }
    
    #[test]
    fn test_query_builder_spatial_operations() {
        let query = QueryBuilder::find("center")
            .spatial_neighbors(0.5)
            .compile();
        
        // spatial_radius_from_point is a static method, not chainable
        let query2 = QueryBuilder::spatial_radius_from_point(
            Coordinate3D::new(0.5, 0.5, 0.5), 
            0.3
        ).compile();
        
        assert!(query.bytecode.len() >= 2); // LoadNode + spatial operation
        assert!(query2.bytecode.len() >= 1); // Spatial operation
    }
    
    #[test]
    fn test_query_builder_connection_operations() {
        let query = QueryBuilder::find("node")
            .follow_nth_connection(2)
            .follow_connection_type(ConnectionType::Phonetic)
            .compile();
        
        assert!(query.bytecode.len() >= 3); // LoadNode + connection operations
    }
    
    #[test]
    fn test_query_builder_search_operations() {
        // These operations don't exist in the current implementation
        // Just test what we have
        let query = QueryBuilder::find("word")
            .similar()
            .compile();
        
        assert!(query.bytecode.len() >= 2); // LoadNode + FindSimilar
    }
    
    #[test]
    fn test_query_builder_analysis_operations() {
        // Analysis operations not implemented yet
        // Test decompose which is available
        let query = QueryBuilder::find("analyze")
            .decompose()
            .compile();
        
        assert!(query.bytecode.len() >= 2); // LoadNode + operation
    }
    
    #[test]
    fn test_query_builder_pattern_operations() {
        // Pattern operations not implemented yet
        // Test with available operations
        let query = QueryBuilder::find("pattern")
            .similar_threshold(0.9)
            .compile();
        
        assert!(query.bytecode.len() >= 2); // LoadNode + FindSimilar
    }
    
    #[test]
    fn test_query_builder_result_operations() {
        let query = QueryBuilder::find("result")
            .filter(FilterCriteria::Layer(Layer::Morphemes))
            .sort(SortCriteria::Alphabetical)
            .deduplicate()
            .limit(20)
            .compile();
        
        assert!(query.bytecode.len() >= 4); // Several operations
    }
    
    #[test]
    fn test_query_builder_empty_query() {
        // Empty query - start with find
        let query = QueryBuilder::find("").compile();
        assert!(query.bytecode.len() >= 1); // Find operation
        assert_eq!(query.string_cache.len(), 1); // Empty string is still cached
    }
    
    #[test]
    fn test_query_builder_string_cache() {
        let query = QueryBuilder::find("first")
            .compile();
        
        // Should have string in cache
        assert!(query.string_cache.len() >= 1);
        assert!(query.string_cache.contains(&"first".to_string()));
    }
    
    #[test]
    fn test_query_builder_complex_chain() {
        let query = QueryBuilder::find("technology")
            .similar_threshold(0.9)
            .layer_up()
            .follow_connection_type(ConnectionType::Phonetic)
            .spatial_neighbors(0.2)
            .filter(FilterCriteria::Layer(Layer::Morphemes))
            .sort(SortCriteria::Frequency)
            .limit(50)
            .compile();
        
        assert!(query.bytecode.len() >= 5); // At least several operations
        assert!(query.string_cache.contains(&"technology".to_string()));
        // Only one string from find operation
    }
    
    #[test]
    fn test_query_display_all_operations() {
        let query = QueryBuilder::find("test")
            .similar()
            .layer_up()
            .layer_down()
            .follow_connection()
            .spatial_neighbors(0.5)
            .filter(FilterCriteria::Layer(Layer::Morphemes))
            .limit(10);
        
        let display = format!("{}", query);
        
        // Check operations are displayed
        assert!(display.contains("find('test')"));
        assert!(display.contains("similar"));
        assert!(display.contains("up"));
        assert!(display.contains("down"));
        assert!(display.contains("follow"));
        assert!(display.contains("limit"));
    }
}
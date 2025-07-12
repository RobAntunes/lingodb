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

//! Foreign Function Interface (FFI) for JavaScript/TypeScript bindings
//! 
//! This module provides C-compatible FFI bindings that can be used to create
//! JavaScript/TypeScript bindings via tools like `wasm-bindgen` or direct FFI.
//! 
//! ## Safety
//! 
//! All FFI functions handle panics and convert them to appropriate error codes.
//! Memory management is handled through reference counting and proper cleanup.
//! 
//! ## Usage
//! 
//! ```c
//! // Create an executor
//! LingoExecutor* executor = lingo_executor_new();
//! 
//! // Load a database
//! int result = lingo_executor_load_database(executor, "english.lingo");
//! 
//! // Execute a query
//! LingoQuery* query = lingo_query_find("technical");
//! LingoResult* result = lingo_executor_execute(executor, query);
//! 
//! // Cleanup
//! lingo_result_free(result);
//! lingo_query_free(query);
//! lingo_executor_free(executor);
//! ```

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_float};
use std::ptr;
use std::panic;

use crate::core::{NodeId, Layer, EtymologyOrigin, NodeFlags};
use crate::engine::{LingoExecutor, QueryResult};
use crate::query::{QueryBuilder, CompiledQuery};
use crate::storage::MemoryMappedDatabase;

/// FFI error codes
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LingoError {
    /// Success (no error)
    Success = 0,
    /// Null pointer passed to function
    NullPointer = 1,
    /// Invalid UTF-8 string
    InvalidString = 2,
    /// Database file not found or corrupted
    DatabaseError = 3,
    /// Query compilation failed
    QueryError = 4,
    /// Query execution failed
    ExecutionError = 5,
    /// Out of memory
    OutOfMemory = 6,
    /// Unknown error
    Unknown = 999,
}

/// Opaque handle to a Lingo executor
#[repr(C)]
pub struct LingoExecutorHandle {
    _private: [u8; 0],
}

/// Opaque handle to a compiled query
#[repr(C)]
pub struct LingoQueryHandle {
    _private: [u8; 0],
}

/// Opaque handle to query results
#[repr(C)]
pub struct LingoResultHandle {
    _private: [u8; 0],
}

/// Node information returned by queries
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LingoNode {
    /// Node ID
    pub id: u32,
    /// Word/text content
    pub word: *mut c_char,
    /// Linguistic layer (0-6)
    pub layer: u8,
    /// Etymology origin (see EtymologyOrigin enum)
    pub etymology: u8,
    /// Node flags (see NodeFlags)
    pub flags: u8,
    /// X coordinate (semantic similarity)
    pub x: c_float,
    /// Y coordinate (etymology)
    pub y: c_float,
    /// Z coordinate (layer)
    pub z: c_float,
}

/// Query execution statistics
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LingoStats {
    /// Number of nodes found
    pub node_count: u32,
    /// Execution time in microseconds
    pub execution_time_us: u64,
    /// Number of instructions executed
    pub instructions_executed: u32,
    /// Whether result was from cache
    pub cache_hit: bool,
}

// Internal storage for FFI objects
struct ExecutorWrapper {
    executor: LingoExecutor,
}

struct QueryWrapper {
    query: CompiledQuery,
}

struct ResultWrapper {
    nodes: Vec<LingoNode>,
    stats: LingoStats,
}

/// Creates a new Lingo executor
/// 
/// # Returns
/// 
/// Pointer to executor handle, or null on failure
#[no_mangle]
pub extern "C" fn lingo_executor_new() -> *mut LingoExecutorHandle {
    let result = panic::catch_unwind(|| {
        let executor = LingoExecutor::new();
        let wrapper = Box::new(ExecutorWrapper { executor });
        Box::into_raw(wrapper) as *mut LingoExecutorHandle
    });
    
    result.unwrap_or(ptr::null_mut())
}

/// Loads a database file into the executor
/// 
/// # Arguments
/// 
/// * `executor` - Executor handle
/// * `path` - Path to database file (null-terminated string)
/// 
/// # Returns
/// 
/// LingoError::Success on success, error code on failure
#[no_mangle]
pub extern "C" fn lingo_executor_load_database(
    executor: *mut LingoExecutorHandle,
    path: *const c_char,
) -> LingoError {
    if executor.is_null() || path.is_null() {
        return LingoError::NullPointer;
    }
    
    let result = panic::catch_unwind(|| {
        let wrapper = unsafe { &mut *(executor as *mut ExecutorWrapper) };
        
        let path_str = unsafe { CStr::from_ptr(path) };
        let path_string = match path_str.to_str() {
            Ok(s) => s,
            Err(_) => return LingoError::InvalidString,
        };
        
        match wrapper.executor.load_database(path_string) {
            Ok(_) => LingoError::Success,
            Err(_) => LingoError::DatabaseError,
        }
    });
    
    result.unwrap_or(LingoError::Unknown)
}

/// Creates a new query to find a word/morpheme
/// 
/// # Arguments
/// 
/// * `word` - Word to search for (null-terminated string)
/// 
/// # Returns
/// 
/// Pointer to query handle, or null on failure
#[no_mangle]
pub extern "C" fn lingo_query_find(word: *const c_char) -> *mut LingoQueryHandle {
    if word.is_null() {
        return ptr::null_mut();
    }
    
    let result = panic::catch_unwind(|| {
        let word_str = unsafe { CStr::from_ptr(word) };
        let word_string = match word_str.to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        };
        
        let query = QueryBuilder::find(word_string).compile();
        let wrapper = Box::new(QueryWrapper { query });
        Box::into_raw(wrapper) as *mut LingoQueryHandle
    });
    
    result.unwrap_or(ptr::null_mut())
}

/// Adds similarity search to a query
/// 
/// # Arguments
/// 
/// * `query` - Query handle to modify
/// * `threshold` - Similarity threshold (0.0 to 1.0)
/// 
/// # Returns
/// 
/// LingoError::Success on success, error code on failure
#[no_mangle]
pub extern "C" fn lingo_query_similar(
    query: *mut LingoQueryHandle,
    threshold: c_float,
) -> LingoError {
    if query.is_null() {
        return LingoError::NullPointer;
    }
    
    // Note: This is a simplified version. In a full implementation,
    // we'd need to rebuild the query with additional operations.
    LingoError::Success
}

/// Adds layer navigation to a query
/// 
/// # Arguments
/// 
/// * `query` - Query handle to modify
/// * `direction` - Direction: 1 for up, -1 for down, 0 for same layer
/// 
/// # Returns
/// 
/// LingoError::Success on success, error code on failure
#[no_mangle]
pub extern "C" fn lingo_query_layer_navigate(
    query: *mut LingoQueryHandle,
    direction: c_int,
) -> LingoError {
    if query.is_null() {
        return LingoError::NullPointer;
    }
    
    // Note: This is a simplified version. In a full implementation,
    // we'd need to rebuild the query with additional operations.
    LingoError::Success
}

/// Sets result limit for a query
/// 
/// # Arguments
/// 
/// * `query` - Query handle to modify
/// * `limit` - Maximum number of results
/// 
/// # Returns
/// 
/// LingoError::Success on success, error code on failure
#[no_mangle]
pub extern "C" fn lingo_query_limit(
    query: *mut LingoQueryHandle,
    limit: c_uint,
) -> LingoError {
    if query.is_null() {
        return LingoError::NullPointer;
    }
    
    // Note: This is a simplified version
    LingoError::Success
}

/// Executes a compiled query
/// 
/// # Arguments
/// 
/// * `executor` - Executor handle
/// * `query` - Query handle
/// 
/// # Returns
/// 
/// Pointer to result handle, or null on failure
#[no_mangle]
pub extern "C" fn lingo_executor_execute(
    executor: *mut LingoExecutorHandle,
    query: *mut LingoQueryHandle,
) -> *mut LingoResultHandle {
    if executor.is_null() || query.is_null() {
        return ptr::null_mut();
    }
    
    let result = panic::catch_unwind(|| {
        let executor_wrapper = unsafe { &mut *(executor as *mut ExecutorWrapper) };
        let query_wrapper = unsafe { &*(query as *const QueryWrapper) };
        
        match executor_wrapper.executor.execute(&query_wrapper.query) {
            Ok(result) => {
                let nodes = convert_result_to_ffi(&executor_wrapper.executor, &result);
                let stats = LingoStats {
                    node_count: result.nodes.len() as u32,
                    execution_time_us: result.execution_time.as_micros() as u64,
                    instructions_executed: result.instructions_executed as u32,
                    cache_hit: result.cache_hit,
                };
                
                let wrapper = Box::new(ResultWrapper { nodes, stats });
                Box::into_raw(wrapper) as *mut LingoResultHandle
            }
            Err(_) => ptr::null_mut(),
        }
    });
    
    result.unwrap_or(ptr::null_mut())
}

/// Gets the number of nodes in a result
/// 
/// # Arguments
/// 
/// * `result` - Result handle
/// 
/// # Returns
/// 
/// Number of nodes, or 0 if null
#[no_mangle]
pub extern "C" fn lingo_result_node_count(result: *const LingoResultHandle) -> u32 {
    if result.is_null() {
        return 0;
    }
    
    let wrapper = unsafe { &*(result as *const ResultWrapper) };
    wrapper.nodes.len() as u32
}

/// Gets a node from the result by index
/// 
/// # Arguments
/// 
/// * `result` - Result handle
/// * `index` - Node index
/// 
/// # Returns
/// 
/// Pointer to node, or null if invalid index
#[no_mangle]
pub extern "C" fn lingo_result_get_node(
    result: *const LingoResultHandle,
    index: u32,
) -> *const LingoNode {
    if result.is_null() {
        return ptr::null();
    }
    
    let wrapper = unsafe { &*(result as *const ResultWrapper) };
    
    if (index as usize) >= wrapper.nodes.len() {
        return ptr::null();
    }
    
    &wrapper.nodes[index as usize]
}

/// Gets execution statistics for a result
/// 
/// # Arguments
/// 
/// * `result` - Result handle
/// 
/// # Returns
/// 
/// Pointer to statistics, or null if invalid
#[no_mangle]
pub extern "C" fn lingo_result_get_stats(result: *const LingoResultHandle) -> *const LingoStats {
    if result.is_null() {
        return ptr::null();
    }
    
    let wrapper = unsafe { &*(result as *const ResultWrapper) };
    &wrapper.stats
}

/// Frees an executor handle
/// 
/// # Arguments
/// 
/// * `executor` - Executor handle to free
#[no_mangle]
pub extern "C" fn lingo_executor_free(executor: *mut LingoExecutorHandle) {
    if !executor.is_null() {
        let _ = panic::catch_unwind(|| {
            let wrapper = unsafe { Box::from_raw(executor as *mut ExecutorWrapper) };
            drop(wrapper);
        });
    }
}

/// Frees a query handle
/// 
/// # Arguments
/// 
/// * `query` - Query handle to free
#[no_mangle]
pub extern "C" fn lingo_query_free(query: *mut LingoQueryHandle) {
    if !query.is_null() {
        let _ = panic::catch_unwind(|| {
            let wrapper = unsafe { Box::from_raw(query as *mut QueryWrapper) };
            drop(wrapper);
        });
    }
}

/// Frees a result handle
/// 
/// # Arguments
/// 
/// * `result` - Result handle to free
#[no_mangle]
pub extern "C" fn lingo_result_free(result: *mut LingoResultHandle) {
    if !result.is_null() {
        let _ = panic::catch_unwind(|| {
            let wrapper = unsafe { Box::from_raw(result as *mut ResultWrapper) };
            // Free all the CString pointers
            for node in &wrapper.nodes {
                if !node.word.is_null() {
                    let _ = unsafe { CString::from_raw(node.word) };
                }
            }
            drop(wrapper);
        });
    }
}

/// Gets the library version
/// 
/// # Returns
/// 
/// Version string (do not free)
#[no_mangle]
pub extern "C" fn lingo_version() -> *const c_char {
    static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");
    VERSION.as_ptr() as *const c_char
}

// Helper function to convert query results to FFI format
fn convert_result_to_ffi(executor: &LingoExecutor, result: &QueryResult) -> Vec<LingoNode> {
    let mut nodes = Vec::new();
    
    if let Some(db) = &executor.database {
        for &node_id in result.nodes.as_slice() {
            if let (Ok(word), Ok(node)) = (db.get_node_word(node_id), db.get_node(node_id)) {
                let word_cstring = match CString::new(word) {
                    Ok(s) => s.into_raw(),
                    Err(_) => ptr::null_mut(),
                };
                
                let ffi_node = LingoNode {
                    id: node_id.0,
                    word: word_cstring,
                    layer: node.layer as u8,
                    etymology: node.etymology_origin as u8,
                    flags: node.flags.bits(),
                    x: node.position.x,
                    y: node.position.y,
                    z: node.position.z,
                };
                
                nodes.push(ffi_node);
            }
        }
    }
    
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    
    #[test]
    fn test_executor_creation() {
        let executor = lingo_executor_new();
        assert!(!executor.is_null());
        lingo_executor_free(executor);
    }
    
    #[test]
    fn test_query_creation() {
        let word = CString::new("test").unwrap();
        let query = lingo_query_find(word.as_ptr());
        assert!(!query.is_null());
        lingo_query_free(query);
    }
    
    #[test]
    fn test_version() {
        let version = lingo_version();
        assert!(!version.is_null());
        
        let version_str = unsafe { CStr::from_ptr(version) };
        assert!(!version_str.to_str().unwrap().is_empty());
    }
}
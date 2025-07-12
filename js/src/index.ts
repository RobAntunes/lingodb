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

/**
 * @fileoverview Lingo Database - JavaScript/TypeScript API
 * 
 * Revolutionary 3D spatial linguistic database with orthogonal connections.
 * This package provides high-level JavaScript bindings for the Lingo Database,
 * enabling powerful linguistic analysis and semantic search capabilities.
 * 
 * @example
 * ```typescript
 * import { LingoDatabase, QueryBuilder } from '@lingo/core';
 * 
 * // Create and load database
 * const db = new LingoDatabase();
 * await db.loadStandardEnglish();
 * 
 * // Build and execute query
 * const query = QueryBuilder.find("tech")
 *     .layerUp()
 *     .similar(0.8)
 *     .limit(10);
 * 
 * const results = await db.execute(query);
 * console.log(`Found ${results.nodes.length} related terms`);
 * ```
 */

// Import WASM bindings based on environment
let wasmModule: any;

if (typeof globalThis !== 'undefined' && (globalThis as any).window !== 'undefined') {
  // Browser environment
  throw new Error('Browser environment not yet supported. Use Node.js for now.');
} else {
  // Node.js environment
  try {
    wasmModule = require('../pkg/lingo');
  } catch (error) {
    throw new Error(
      'WASM module not found. Run "npm run build" to compile the WASM bindings first.'
    );
  }
}

/**
 * Represents a linguistic node in the database
 */
export interface LinguisticNode {
  /** Unique node identifier */
  id: number;
  /** The word or linguistic element */
  word: string;
  /** Linguistic layer (Letters, Phonemes, Morphemes, Words, Phrases, Concepts, Domains) */
  layer: string;
  /** X coordinate (semantic similarity) */
  x: number;
  /** Y coordinate (etymology) */
  y: number;
  /** Z coordinate (layer position) */
  z: number;
  /** Etymology origin */
  etymology: string;
  /** Node flags (technical, frequent, etc.) */
  flags: string[];
}

/**
 * Results from a query execution
 */
export interface QueryResult {
  /** Array of matching nodes */
  nodes: LinguisticNode[];
  /** Query execution time in milliseconds */
  executionTimeMs: number;
  /** Total nodes examined during search */
  totalNodesSearched: number;
  /** Number of results returned */
  length: number;
}

/**
 * Database operation result
 */
export interface DatabaseResult {
  /** Whether the operation succeeded */
  success: boolean;
  /** Error message if operation failed */
  error?: string;
}

/**
 * Database statistics
 */
export interface DatabaseStats {
  /** Whether database is loaded */
  loaded: boolean;
  /** Database version */
  version: string;
}

/**
 * Fluent query builder for constructing database queries
 * 
 * @example
 * ```typescript
 * const query = QueryBuilder.find("bio")
 *   .layerUp()           // Navigate to words containing "bio"
 *   .similar(0.8)        // Find semantically similar terms
 *   .limit(20);          // Limit to 20 results
 * ```
 */
export class QueryBuilder {
  private wasmBuilder: any;

  /**
   * Create a new query to find the specified term
   * @param term - The term to search for
   * @returns New QueryBuilder instance
   */
  static find(term: string): QueryBuilder {
    const builder = new QueryBuilder();
    builder.wasmBuilder = wasmModule.QueryBuilder.find(term);
    return builder;
  }

  private constructor() {}

  /**
   * Navigate up to parent layer (e.g., morphemes → words)
   * @returns QueryBuilder for method chaining
   */
  layerUp(): QueryBuilder {
    this.wasmBuilder = this.wasmBuilder.layerUp();
    return this;
  }

  /**
   * Navigate down to child layer (e.g., words → morphemes)
   * @returns QueryBuilder for method chaining
   */
  layerDown(): QueryBuilder {
    this.wasmBuilder = this.wasmBuilder.layerDown();
    return this;
  }

  /**
   * Find semantically similar nodes with optional threshold
   * @param threshold - Similarity threshold (0.0 to 1.0, optional)
   * @returns QueryBuilder for method chaining
   */
  similar(threshold?: number): QueryBuilder {
    if (threshold !== undefined) {
      this.wasmBuilder = this.wasmBuilder.similarThreshold(threshold);
    } else {
      this.wasmBuilder = this.wasmBuilder.similar();
    }
    return this;
  }

  /**
   * Follow orthogonal connections to related concepts
   * @returns QueryBuilder for method chaining
   */
  followConnection(): QueryBuilder {
    this.wasmBuilder = this.wasmBuilder.followConnection();
    return this;
  }

  /**
   * Limit the number of results returned
   * @param limit - Maximum number of results
   * @returns QueryBuilder for method chaining
   */
  limit(limit: number): QueryBuilder {
    this.wasmBuilder = this.wasmBuilder.limit(limit);
    return this;
  }

  /**
   * Compile the query for execution
   * @returns Compiled query ready for execution
   * @throws Error if compilation fails
   */
  compile(): CompiledQuery {
    try {
      const wasmQuery = this.wasmBuilder.compile();
      return new CompiledQuery(wasmQuery);
    } catch (error) {
      throw new Error(`Query compilation failed: ${error}`);
    }
  }
}

/**
 * A compiled query ready for execution
 */
export class CompiledQuery {
  constructor(private wasmQuery: any) {}

  /**
   * Get the internal WASM query object (for advanced usage)
   * @internal
   */
  getWasmQuery(): any {
    return this.wasmQuery;
  }
}

/**
 * Main Lingo Database interface
 * 
 * Provides high-level access to the Lingo linguistic database with
 * methods for loading data and executing queries.
 * 
 * @example
 * ```typescript
 * const db = new LingoDatabase();
 * await db.loadStandardEnglish();
 * 
 * const query = QueryBuilder.find("technical").similar(0.8);
 * const results = await db.execute(query);
 * ```
 */
export class LingoDatabase {
  private wasmDb: any;

  /**
   * Create a new Lingo database instance
   */
  constructor() {
    this.wasmDb = new wasmModule.LingoDatabase();
  }

  /**
   * Load the standard English linguistic database
   * 
   * This loads a pre-built database containing:
   * - 26 letters with phonetic classifications
   * - 42 IPA phonemes with articulation details  
   * - 100 morphemes covering 90% of academic vocabulary
   * - Etymology mappings for Greek, Latin, Germanic origins
   * 
   * @returns Promise resolving to operation result
   */
  async loadStandardEnglish(): Promise<DatabaseResult> {
    try {
      const result = this.wasmDb.loadStandardEnglish();
      return {
        success: result.success,
        error: result.error || undefined
      };
    } catch (error) {
      return {
        success: false,
        error: `Failed to load standard English database: ${error}`
      };
    }
  }

  /**
   * Load database from raw bytes (for custom databases)
   * @param bytes - Database file contents as Uint8Array
   * @returns Promise resolving to operation result
   */
  async loadFromBytes(bytes: Uint8Array): Promise<DatabaseResult> {
    try {
      const result = this.wasmDb.loadFromBytes(bytes);
      return {
        success: result.success,
        error: result.error || undefined
      };
    } catch (error) {
      return {
        success: false,
        error: `Failed to load database from bytes: ${error}`
      };
    }
  }

  /**
   * Execute a compiled query against the database
   * @param query - Compiled query to execute
   * @returns Promise resolving to query results
   * @throws Error if database not loaded or query execution fails
   */
  async execute(query: CompiledQuery): Promise<QueryResult> {
    try {
      const wasmResult = this.wasmDb.execute(query.getWasmQuery());
      
      // Convert WASM result to TypeScript interface
      return {
        nodes: wasmResult.nodes.map((node: any) => ({
          id: node.id,
          word: node.word,
          layer: node.layer,
          x: node.x,
          y: node.y,
          z: node.z,
          etymology: node.etymology,
          flags: Array.from(node.flags)
        })),
        executionTimeMs: wasmResult.executionTimeMs,
        totalNodesSearched: wasmResult.totalNodesSearched,
        length: wasmResult.length
      };
    } catch (error) {
      throw new Error(`Query execution failed: ${error}`);
    }
  }

  /**
   * Get database statistics and status
   * @returns Promise resolving to database statistics
   */
  async getStats(): Promise<DatabaseStats> {
    try {
      const stats = this.wasmDb.getStats();
      return {
        loaded: stats.loaded,
        version: stats.version
      };
    } catch (error) {
      return {
        loaded: false,
        version: 'unknown'
      };
    }
  }
}

/**
 * Initialize the Lingo Database WASM module
 * Call this once before using any other functions
 * @returns Promise that resolves when initialization is complete
 */
export async function init(): Promise<void> {
  // WASM module is already imported, so this is a no-op
  // In browser environments, this would load the WASM file
  return Promise.resolve();
}

// Export everything for convenient importing
export default {
  LingoDatabase,
  QueryBuilder,
  init
};

/**
 * Version information
 */
export const VERSION = '0.1.0';

/**
 * Check if the current environment supports Lingo Database
 * @returns True if supported, false otherwise
 */
export function isSupported(): boolean {
  try {
    return typeof wasmModule !== 'undefined' && wasmModule.LingoDatabase;
  } catch {
    return false;
  }
}
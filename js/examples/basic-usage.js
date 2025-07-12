// Copyright 2025 Roberto Antunes
//
// Licensed under the Functional Source License, Version 1.1 (the "License");
// you may not use this file except in compliance with the License.

/**
 * Basic usage example for Lingo Database
 * 
 * This example demonstrates:
 * - Loading the standard English database
 * - Building queries with the fluent API
 * - Executing queries and processing results
 * - Error handling and best practices
 */

const { LingoDatabase, QueryBuilder } = require('@lingo/core');

async function basicExample() {
  console.log('🚀 Lingo Database - Basic Usage Example\n');

  try {
    // Create and initialize database
    console.log('1. Creating database instance...');
    const db = new LingoDatabase();

    console.log('2. Loading standard English linguistic data...');
    const loadResult = await db.loadStandardEnglish();
    
    if (!loadResult.success) {
      throw new Error(`Failed to load database: ${loadResult.error}`);
    }
    console.log('   ✅ Database loaded successfully!\n');

    // Get database stats
    const stats = await db.getStats();
    console.log('📊 Database Statistics:');
    console.log(`   - Loaded: ${stats.loaded}`);
    console.log(`   - Version: ${stats.version}\n`);

    // Example 1: Find morphemes containing "tech"
    console.log('🔍 Example 1: Finding "tech" morphemes and related words');
    const techQuery = QueryBuilder.find("tech")
      .layerUp()           // Navigate from morphemes to words
      .similar(0.7)        // Find semantically similar terms
      .limit(10)           // Limit to 10 results
      .compile();

    const techResults = await db.execute(techQuery);
    console.log(`   Found ${techResults.length} results in ${techResults.executionTimeMs.toFixed(2)}ms:`);
    
    techResults.nodes.slice(0, 5).forEach((node, i) => {
      console.log(`   ${i + 1}. "${node.word}" (${node.layer}) - Etymology: ${node.etymology}`);
    });
    console.log('');

    // Example 2: Explore biological morphemes
    console.log('🧬 Example 2: Exploring biological morphemes');
    const bioQuery = QueryBuilder.find("bio")
      .layerUp()
      .followConnection()  // Follow cross-domain connections
      .limit(15)
      .compile();

    const bioResults = await db.execute(bioQuery);
    console.log(`   Found ${bioResults.length} results in ${bioResults.executionTimeMs.toFixed(2)}ms:`);
    
    bioResults.nodes.slice(0, 5).forEach((node, i) => {
      console.log(`   ${i + 1}. "${node.word}" (Layer: ${node.layer})`);
      console.log(`      Position: (${node.x.toFixed(2)}, ${node.y.toFixed(2)}, ${node.z.toFixed(2)})`);
    });
    console.log('');

    // Example 3: Etymology exploration
    console.log('🏛️ Example 3: Finding Greek-origin terms');
    const greekQuery = QueryBuilder.find("log")  // Greek root meaning "word/study"
      .similar(0.8)
      .layerUp()
      .limit(8)
      .compile();

    const greekResults = await db.execute(greekQuery);
    console.log(`   Found ${greekResults.length} results in ${greekResults.executionTimeMs.toFixed(2)}ms:`);
    
    greekResults.nodes.forEach((node, i) => {
      const flags = node.flags.length > 0 ? ` [${node.flags.join(', ')}]` : '';
      console.log(`   ${i + 1}. "${node.word}"${flags} - ${node.etymology}`);
    });
    console.log('');

    // Example 4: Layer navigation
    console.log('📚 Example 4: Layer navigation (Letters → Morphemes → Words)');
    const layerQuery = QueryBuilder.find("t")   // Start with letter "t"
      .layerUp()                                // Go to morphemes containing "t"
      .layerUp()                                // Go to words containing those morphemes
      .limit(6)
      .compile();

    const layerResults = await db.execute(layerQuery);
    console.log(`   Navigation path: Letters → Morphemes → Words`);
    console.log(`   Found ${layerResults.length} words in ${layerResults.executionTimeMs.toFixed(2)}ms:`);
    
    layerResults.nodes.forEach((node, i) => {
      console.log(`   ${i + 1}. "${node.word}" (${node.layer})`);
    });

    console.log('\n🎉 All examples completed successfully!');
    console.log('\n💡 Try building your own queries using the fluent API:');
    console.log('   QueryBuilder.find("your-term").similar(0.8).layerUp().limit(10)');

  } catch (error) {
    console.error('❌ Error:', error.message);
    process.exit(1);
  }
}

// Performance benchmark example
async function performanceExample() {
  console.log('\n⚡ Performance Benchmark\n');

  const db = new LingoDatabase();
  await db.loadStandardEnglish();

  const terms = ['tech', 'bio', 'log', 'graph', 'phon'];
  const results = [];

  console.log('Running queries for performance measurement...');
  
  for (const term of terms) {
    const query = QueryBuilder.find(term)
      .similar(0.7)
      .layerUp()
      .limit(20)
      .compile();

    const result = await db.execute(query);
    results.push({
      term,
      count: result.length,
      timeMs: result.executionTimeMs,
      nodesSearched: result.totalNodesSearched
    });
  }

  console.log('\n📈 Performance Results:');
  console.log('Term'.padEnd(10) + 'Results'.padEnd(10) + 'Time (ms)'.padEnd(12) + 'Nodes Searched');
  console.log('-'.repeat(50));
  
  results.forEach(r => {
    console.log(
      r.term.padEnd(10) + 
      r.count.toString().padEnd(10) + 
      r.timeMs.toFixed(2).padEnd(12) + 
      r.nodesSearched.toString()
    );
  });

  const avgTime = results.reduce((sum, r) => sum + r.timeMs, 0) / results.length;
  console.log(`\nAverage query time: ${avgTime.toFixed(2)}ms`);
}

// Run examples
if (require.main === module) {
  basicExample()
    .then(() => performanceExample())
    .catch(console.error);
}

module.exports = { basicExample, performanceExample };
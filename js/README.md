# 🚀 Lingo Database - JavaScript/TypeScript

Revolutionary 3D spatial linguistic database with WebAssembly performance.

[![License: FSL-1.1-ALv2](https://img.shields.io/badge/License-FSL--1.1--ALv2-lightblue.svg)](../LICENSE)
[![npm version](https://badge.fury.io/js/%40lingo%2Fcore.svg)](https://badge.fury.io/js/%40lingo%2Fcore)

## ⚡ Quick Start

```bash
npm install @lingo/core
```

```typescript
import { LingoDatabase, QueryBuilder } from '@lingo/core';

// Create and load database
const db = new LingoDatabase();
await db.loadStandardEnglish();

// Build and execute query
const query = QueryBuilder.find("tech")
    .layerUp()           // Navigate to words containing "tech"
    .similar(0.8)        // Find semantically similar terms
    .limit(10);

const results = await db.execute(query);
console.log(`Found ${results.nodes.length} related terms`);
```

## 🎯 Features

- ⚡ **WebAssembly Performance** - Near-native speed with memory safety
- 🧠 **Intelligent Morphology** - Automatic word decomposition and analysis
- 📊 **3D Spatial Search** - Semantic similarity through geometric relationships
- 🌐 **Etymology Tracking** - Trace word origins through linguistic history
- 🔗 **Cross-Domain Discovery** - Find analogies between different fields
- 📝 **TypeScript Support** - Full type safety and IntelliSense support

## 📖 API Reference

### LingoDatabase

Main database interface for loading data and executing queries.

```typescript
const db = new LingoDatabase();

// Load standard English database (168 pre-loaded nodes)
await db.loadStandardEnglish();

// Execute queries
const results = await db.execute(compiledQuery);

// Get database statistics
const stats = await db.getStats();
```

### QueryBuilder

Fluent API for building complex linguistic queries.

```typescript
QueryBuilder
  .find("bio")              // Find term
  .layerUp()                // Navigate layers
  .similar(0.8)             // Semantic similarity
  .followConnection()       // Cross-domain connections
  .limit(20)                // Result limit
  .compile();               // Compile for execution
```

### Navigation Methods

- **`.layerUp()`** - Move from morphemes → words → concepts
- **`.layerDown()`** - Move from concepts → words → morphemes  
- **`.similar(threshold)`** - Find semantically related terms
- **`.followConnection()`** - Discover cross-domain analogies

## 🧪 Examples

### 1. Morpheme Discovery

```typescript
// Find all words containing "bio"
const bioQuery = QueryBuilder.find("bio")
  .layerUp()
  .limit(15)
  .compile();

const results = await db.execute(bioQuery);
// Results: biology, biotechnology, biography, antibiotic...
```

### 2. Etymology Search

```typescript
// Find Greek-origin technical terms
const greekQuery = QueryBuilder.find("tech")
  .similar(0.7)
  .layerUp()
  .compile();

// Results: technical, technology, technique, polytechnic...
```

### 3. Cross-Domain Analysis

```typescript
// Discover how "network" is used across domains
const networkQuery = QueryBuilder.find("network")
  .followConnection()
  .compile();

// Results: computer networks, social networks, neural networks...
```

### 4. Layer Navigation

```typescript
// Journey from letter to concepts
const journeyQuery = QueryBuilder.find("t")
  .layerUp()    // → morphemes with "t"
  .layerUp()    // → words
  .layerUp()    // → concepts
  .compile();

// Traces: "t" → "tech" → "technology" → "innovation"
```

## 🏗️ Building from Source

```bash
# Install dependencies
npm install

# Build WASM bindings
npm run build

# Build for different targets
npm run build:web      # Browser
npm run build:bundler  # Webpack/Rollup
npm run build:all      # All targets

# Run tests
npm test
```

## 📊 Performance

| Operation          | Time   | Memory      |
| ------------------ | ------ | ----------- |
| Database Load      | ~5ms   | 15KB        |
| Simple Query       | ~2ms   | Zero-copy   |
| Complex Navigation | ~8ms   | Stack-based |
| Morpheme Discovery | ~12ms  | Cached      |

**Environment**: Node.js 20, Apple M2

## 🛠️ Requirements

- **Node.js**: ≥16.0.0
- **WebAssembly**: Supported in all modern environments
- **Memory**: ~15KB for standard English database
- **Browser**: Chrome 57+, Firefox 52+, Safari 11+

## 🎯 Use Cases

### 📚 Education Technology
```typescript
// Build vocabulary learning apps
const vocab = QueryBuilder.find("photo")
  .similar()
  .layerUp()
  .compile();
// Finds: photograph, photography, photon, telephoto...
```

### 🔍 Search Enhancement
```typescript
// Expand search queries semantically
const search = QueryBuilder.find("quick")
  .similar(0.8)
  .compile();
// Expands: fast, rapid, swift, speedy, expeditious...
```

### 🧠 NLP Pipelines
```typescript
// Morphological preprocessing
for (const word of ["preprocessing", "unbelievable"]) {
  const morphemes = QueryBuilder.find(word)
    .layerDown()
    .compile();
  // Powers stemming, lemmatization, feature extraction
}
```

## 📜 License

Licensed under the **Functional Source License 1.1 (FSL-1.1-ALv2)**. 

- ✅ Use for internal purposes, research, and education
- ❌ Commercial competing products (until Apache 2.0 conversion in 2027)
- 🔄 Automatically converts to Apache 2.0 after 2 years

See [LICENSE](../LICENSE) for full details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and add tests
4. Run the build: `npm run build:all`
5. Submit a PR with benchmarks for performance changes

## 📊 Built With

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[WebAssembly](https://webassembly.org/)** - High-performance web execution
- **[wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)** - Rust/JS interop
- **[TypeScript](https://www.typescriptlang.org/)** - Type-safe JavaScript

---

**[⭐ Star us on GitHub](https://github.com/RobAntunes/lingodb)** • **[📖 Documentation](https://github.com/RobAntunes/lingodb/tree/main/js)** • **[💬 Issues](https://github.com/RobAntunes/lingodb/issues)**

_Made with ❤️ by Roberto Antunes_
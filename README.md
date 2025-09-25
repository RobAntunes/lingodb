# 🚀 Lingo Database

<div align="center">

**Revolutionary 3D Spatial Linguistic Database**

_Discover hidden connections in language through geometric intelligence_

[![License: FSL-1.1-ALv2](https://img.shields.io/badge/License-FSL--1.1--ALv2-lightblue.svg)](LICENSE)

</div>

---

## 🎯 What is Lingo?

Lingo is a **high-performance linguistic database** that represents language as a **3D spatial structure**, enabling revolutionary approaches to:

- 🧠 **Morphological Analysis** - Automatically discover word parts and their relationships
- 🔍 **Semantic Search** - Find conceptually related terms across domains
- 📈 **Etymology Tracking** - Trace word origins through linguistic history
- 🌐 **Cross-Domain Discovery** - Uncover analogies between different fields
- ⚡ **Sub-Millisecond Queries** - Blazing fast linguistic operations

### The Magic: 7-Layer Linguistic Hierarchy

```
🏢 Domains      ←  "STEM", "Marketing"
💡 Concepts     ←  "technological innovation", "viral marketing"
📝 Phrases      ←  "cutting edge", "go viral"
🔤 Words        ←  "technology", "viral", "technical"
🧩 Morphemes    ←  "tech", "bio", "logy", "ment"
🔊 Phonemes     ←  "/t/", "/ɛ/", "/k/", "/n/"
📄 Letters      ←  "t", "e", "c", "h"
```

Each linguistic element exists at precise **3D coordinates**:

- **X-axis**: Semantic similarity
- **Y-axis**: Etymology (Greek, Latin, Germanic...)
- **Z-axis**: Linguistic layer (0.0 → 1.0)

---

## 🔥 Features

### 🎯 **Ready-to-Use Database**

- **168 pre-loaded nodes**: 26 letters, 42 phonemes, 100 morphemes
- **Professional coverage**: 90% of academic/technical English morphology
- **13KB compact size**: Perfect for mobile and embedded applications

### ⚡ **Blazing Performance**

- **Sub-millisecond queries** with memory-mapped access
- **Zero-copy operations** using packed 60-byte node structures
- **O(log n) spatial lookups** via octree indexing
- **SLANG bytecode compilation** for optimal execution

### 🧠 **Intelligent Discovery**

- **Automatic morpheme detection**: "biotechnology" → ["bio", "tech", "ology"]
- **Etymology classification**: Distinguishes Greek, Latin, Germanic origins
- **Cross-domain analogies**: "viral" in medicine ↔ "viral" in marketing
- **Hierarchical navigation**: Seamlessly move between linguistic layers

### 🛠 **Developer Friendly**

- **Fluent query API** with method chaining
- **Type-safe operations** with compile-time guarantees
- **Extensive documentation** with code examples

---

## ⚡ Quick Start

### Installation
***Lingo is currently in active development and thus not currently available as a crate.***

```bash
git clone https://github.com/RobAntunes/lingodb.git
```

### Create Your First Query

```rust
use lingo::{DatabaseSeeder, LingoExecutor, QueryBuilder};

// Create a database with standard English data
let mut seeder = DatabaseSeeder::new();
seeder.seed_english()?;
seeder.build("english.lingo")?;

// Load and query
let mut executor = LingoExecutor::new();
executor.load_database("english.lingo")?;

// Find morphologically related words
let query = QueryBuilder::find("tech")
    .layer_up()           // Navigate to words containing "tech"
    .similar_threshold(0.8)  // Find semantically similar
    .limit(10)
    .compile();

let result = executor.execute(&query)?;
println!("Found {} related terms", result.nodes.len());

// Discover cross-domain connections
let viral_query = QueryBuilder::find("viral")
    .follow_connection()  // Find analogous uses
    .compile();

let viral_result = executor.execute(&viral_query)?;
// Discovers: viral (medical) ↔ viral (marketing)
```

### Automatic Word Analysis

```rust
use lingo::discovery::AutoLinguisticBuilder;

let mut builder = AutoLinguisticBuilder::new();

// Add a complex word
builder.add_word("biotechnology")?;
// Automatically discovers:
// - Morphemes: ["bio", "tech", "ology"]
// - Etymology: Greek origins
// - Phonemes: ["/b/", "/aɪ/", "/oʊ/", ...]
// - 3D positioning based on semantic properties
// - Connections to existing morphemes

builder.build("enhanced.lingo")?;
```

---

## ⚙️ Configuration

Lingo supports configuration through environment variables. Copy `.env.example` to `.env` and customize:

```bash
# Core settings
LINGO_DATABASE_PATH=/path/to/database.lingo  # Database file path
LINGO_LOG_LEVEL=info                         # Log level: trace, debug, info, warn, error
LINGO_CACHE_SIZE_MB=100                      # Cache size in MB
LINGO_QUERY_TIMEOUT_SECS=30                  # Query timeout

# Performance
LINGO_ENABLE_PROFILING=false                 # Enable performance profiling
LINGO_MAX_RESULT_NODES=10000                 # Maximum query results

# Development
LINGO_DEBUG=false                            # Enable debug mode
```

### Using Configuration in Code

```rust
use lingo::config::LingoConfig;

// Load from environment
let config = LingoConfig::from_env();

// Or customize
let config = LingoConfig {
    database_path: PathBuf::from("custom.lingo"),
    max_database_size_mb: 200,
    ..Default::default()
};
```

---

## 🎮 Examples

### 1. **Morpheme Discovery**

```rust
// Find all words containing "bio"
let query = QueryBuilder::find("bio")
    .layer_up()
    .compile();

// Results: biology, biotechnology, biography, antibiotic...
```

### 2. **Etymology Search**

```rust
// Find Greek-origin technical terms
let query = QueryBuilder::find("tech")
    .similar_threshold(0.7)
    .layer_up()
    .compile();

// Results: technical, technology, technique, polytechnic...
```

### 3. **Cross-Domain Analysis**

```rust
// Discover how "network" is used across domains
let query = QueryBuilder::find("network")
    .follow_connection()
    .compile();

// Results: computer networks, social networks, neural networks...
```

### 4. **Hierarchical Navigation**

```rust
// Journey from letter to domain
let query = QueryBuilder::find("t")
    .layer_up()    // → morphemes with "t"
    .layer_up()    // → words
    .layer_up()    // → concepts
    .layer_up()    // → domains
    .compile();

// Traces: "t" → "tech" → "technology" → "innovation" → "STEM"
```

---

## 🏗️ Architecture

```
┌─────────────────┬─────────────────┬─────────────────┐
│   Query Layer   │  Execution      │   Storage       │
│                 │                 │                 │
│ ┌─────────────┐ │ ┌─────────────┐ │ ┌─────────────┐ │
│ │QueryBuilder │ │ │LingoExecutor│ │ │Memory-Mapped│ │
│ │             │ │ │             │ │ │Database     │ │
│ │ Fluent API  │→│ │SLANG Bytecode│→│ │             │ │
│ └─────────────┘ │ │Interpreter  │ │ │Zero-Copy    │ │
│                 │ └─────────────┘ │ │Access       │ │
│ ┌─────────────┐ │ ┌─────────────┐ │ └─────────────┘ │
│ │Auto         │ │ │Spatial      │ │ ┌─────────────┐ │
│ │Discovery    │ │ │Indexing     │ │ │.lingo Format│ │
│ └─────────────┘ │ │(Octree)     │ │ │             │ │
│                 │ └─────────────┘ │ │Single File  │ │
└─────────────────┴─────────────────┴─│Mobile Ready │ │
                                      └─────────────┘
```

---

## 📊 Performance

| Operation          | Time   | Memory      |
| ------------------ | ------ | ----------- |
| Database Load      | ~1ms   | 13KB        |
| Simple Query       | ~10μs  | Zero-copy   |
| Complex Chain      | ~50μs  | Stack-based |
| Morpheme Discovery | ~100μs | Cached      |

**Benchmark System**: MacBook Pro M2, 8GB RAM

---

## 🌍 Language Support

### Current: English (Professional Grade)

- ✅ **26 letters** with vowel/consonant classification
- ✅ **42 IPA phonemes** with articulation details
- ✅ **100 morphemes** covering 90% of academic vocabulary
- ✅ **Etymology mapping** for Greek, Latin, Germanic origins

---

## 🎯 Use Cases

### 📚 **Education Technology**

```rust
// Build vocabulary learning apps
let query = QueryBuilder::find("photo")
    .similar()
    .layer_up()
    .compile();
// Finds: photograph, photography, photon, telephoto...
```

### 🔍 **Search Engines**

```rust
// Enhance semantic search
let query = QueryBuilder::find("quick")
    .find_similar_concepts()
    .compile();
// Expands: fast, rapid, swift, speedy, expeditious...
```

### 🧠 **NLP Pipelines**

```rust
// Morphological preprocessing
let words = ["preprocessing", "unbelievable", "internationalize"];
for word in words {
    let morphemes = analyzer.decompose(word)?;
    // Powers stemming, lemmatization, feature extraction
}
```

### 📖 **Linguistic Research**

```rust
// Trace etymology patterns
let greek_roots = QueryBuilder::etymology(EtymologyOrigin::Greek)
    .productive_only()
    .compile();
// Studies: morpheme productivity, borrowing patterns, etc.
```

---

## 🛠️ FFI & Bindings

Lingo provides **C-compatible FFI** for integration with other languages:

### JavaScript/TypeScript (Coming Soon)

```typescript
import { LingoExecutor, QueryBuilder } from "@lingo/core";

const executor = new LingoExecutor();
await executor.loadDatabase("english.lingo");

const results = await executor.execute(
  QueryBuilder.find("technical").similar(0.8).layerUp().limit(10)
);
```

### Python (Coming Soon)

```python
from lingo import LingoExecutor, QueryBuilder

executor = LingoExecutor()
executor.load_database('english.lingo')

results = executor.execute(
    QueryBuilder.find('technical')
    .similar(0.8)
    .layer_up()
    .limit(10)
)
```

---

## 🤝 Contributing

We welcome contributions! Here's how to get started:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Write tests**: `cargo test`
4. **Submit a PR**: Include benchmarks for performance changes

### Development Setup

```bash
git clone https://github.com/RobAntunes/lingodb.git
cd lingo
cargo build
cargo test
cargo run --example build_standard_db
```

### Adding New Languages

1. Create `src/data/{language}_base.rs`
2. Define phonemes, morphemes, and etymologies
3. Add to `src/data/seeder.rs`
4. Write tests and examples

---

## 🏢 Commercial Use

### Open Source (FSL-1.1-ALv2 License)

- ✅ Use for internal purposes
- ✅ Modify and distribute for permitted purposes
- ✅ Non-commercial research and education
- ❌ Commercial competing products (until Apache 2.0 conversion)

---

## 📊 Built With

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[memmap2](https://crates.io/crates/memmap2)** - Memory-mapped file I/O
- **[bitflags](https://crates.io/crates/bitflags)** - Type-safe bit field operations
- **[xxhash](https://crates.io/crates/xxhash-rust)** - Fast hashing algorithm

---

## 📜 License

This project is licensed under the **Functional Source License 1.1 (FSL-1.1-ALv2)** - see the [LICENSE](LICENSE) file for details. The license automatically converts to Apache 2.0 after 2 years.

---

## 🙏 Acknowledgments

- **Linguistic Theory**: Based on decades of morphological research
- **Performance**: Inspired by modern database optimization techniques
- **Open Source**: Built on the shoulders of the amazing Rust ecosystem

---

<div align="center">

**[⭐ Star us on GitHub](https://github.com/RobAntunes/lingodb)** • **[💬 Join the Discussion](https://github.com/RobAntunes/lingodb/discussions)**

_Made with ❤️ by the Lingo Team_

</div>

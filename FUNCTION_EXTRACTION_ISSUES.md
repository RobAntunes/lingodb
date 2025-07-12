# Function Extraction Plugin - Current Issues and Improvements Needed

## Overview
The plugin architecture and base Lingo Database work perfectly. The function extraction plugin successfully integrates with the system and can detect some patterns (Conditionality and Sequence). However, the detection algorithms need refinement to fully extract all functional patterns.

## What's Working ✅

### Successfully Detected Patterns:
1. **Conditionality** - Detects "if", "when", "unless" clauses
   - Example: "If the user uploads documents..." → Correctly identified
   - Confidence: 0.86

2. **Sequence** - Detects temporal markers like "first", "then", "finally"
   - Example: "First compile... then run... finally deploy" → Correctly identified
   - Confidence: 0.65

### System Integration:
- Plugin registers and initializes correctly
- Commands execute without errors
- Results return in expected format
- No pollution of base module

## What's Not Working ❌

### Failed Pattern Detection:

1. **Agency Detection**
   - Not finding agent words: "manager", "developer", "system", "team"
   - Issue: Query searches for full sentence instead of individual words
   - Current: `QueryBuilder::find(text)` where text = "The manager organized the meeting"
   - Needed: Parse text into words first, then query each word

2. **Action Detection**
   - Not finding action verbs: "organized", "created", "processes", "analyze"
   - Issue: Same as above - needs word-level analysis
   - Additional issue: May need to look at base forms (lemmas) not inflected forms

3. **Transformation Detection**
   - Not finding patterns like "converted...into", "transforms to"
   - Issue: Current implementation looks for exact phrase matches
   - Needed: Pattern-based detection across multiple words

4. **Purpose Detection**
   - Not finding purpose markers: "to generate", "to achieve", "in order to"
   - Issue: Infinitive phrases not being parsed correctly
   - Needed: Multi-word pattern recognition

## Root Causes

### 1. Fundamental Misunderstanding of Database Structure
**The database only contains letters, phonemes, and morphemes - NOT complete words!**

The system is designed to build up from base components:
- Letters (a, b, c...)
- Phonemes (sound units)
- Morphemes (prefixes, roots, suffixes like "un-", "manage", "-er")

Words are meant to be **composed** from these building blocks, not looked up directly.

```rust
// Current approach - WRONG, looking for complete words/sentences
let query = QueryBuilder::find("The manager organized the meeting")
    .layer_down()
    .compile();

// What's actually needed:
// 1. Break text into words: ["The", "manager", "organized", "the", "meeting"]
// 2. Break each word into morphemes: "manager" → ["manage", "-er"]
// 3. Look up each morpheme in the database
// 4. Build up the analysis from morpheme properties
```

### 2. Database Lookup Mismatches
- The database contains base forms (lemmas) but queries use inflected forms
- Example: Database has "organize" but query looks for "organized"
- Need lemmatization before querying

### 3. Executor State Issue
- Function creates a dummy database instead of using the initialized one
- This may cause executor to not have proper database state
- See line 57: `let dummy_database = LingoDatabase::open("english.lingo")`

### 4. Pattern Detection Logic
- Simple keyword matching instead of linguistic analysis
- Not leveraging the spatial relationships in the database
- Not using morphological decomposition effectively

## Recommended Improvements

### 1. Text Preprocessing Pipeline
```rust
fn preprocess_text(text: &str) -> Vec<String> {
    // 1. Tokenize into words
    // 2. Lemmatize to base forms
    // 3. Filter stop words
    // 4. Return clean word list
}
```

### 2. Improve Database Integration
- Use the database passed during initialization
- Store it as `Arc<LingoDatabase>` to share safely
- Remove the dummy database creation

### 3. Enhanced Pattern Detection

#### For Agency:
- Look for morphological markers (-er, -or, -ist suffixes)
- Check syntactic position (subject of sentence)
- Use the morpheme layer effectively

#### For Actions:
- Identify verbs through morphological analysis
- Check for verb inflections (-ed, -ing, -s)
- Use the database's etymology data

#### For Transformations:
- Implement multi-word pattern matching
- Look for state-change indicators
- Track input/output states properly

#### For Purpose:
- Detect infinitive markers ("to" + verb)
- Identify goal-oriented phrases
- Link to subsequent clauses

### 4. Leverage Spatial Relationships
- Use the 3D positions to find semantically related words
- Implement proper spatial coherence calculations
- Navigate layers more effectively

### 5. Query Optimization
```rust
// Better query pattern
let query = QueryBuilder::find(word)
    .layer(Layer::Words)  // Start at word layer
    .similar_threshold(0.8)  // Find similar words
    .layer_down()  // Then go to morphemes
    .compile();
```

## Summary

The core issue is that the detection algorithms are too simplistic and don't properly utilize the linguistic database's rich structure. The plugin architecture itself is perfect - it's just the linguistic analysis that needs to be more sophisticated.

The fact that Conditionality and Sequence detection work proves the system is functional. With proper text preprocessing and better database queries, all pattern types should be detectable.
# 📈 LingoDB Coverage Improvement Plan

## Current State: 45.9% Coverage

Our current database (`data/base.lingo`) contains 1,100 morphemes that excel at technical/scientific vocabulary but miss common everyday words. This document outlines our strategy to reach 90%+ coverage while maintaining quality.

## 🎯 Target: 90%+ Coverage of English

### Why 90% is the Sweet Spot
- **90-95%** = Covers virtually all common English text
- **95-100%** = Diminishing returns (rare words, proper nouns, slang)
- **Quality > Quantity**: Better to accurately classify 90% than poorly classify 100%

## 📊 Gap Analysis

### What We Have ✅
```
Strong Coverage:
- Technical roots: bio, tech, log, graph, phon, photo
- Common prefixes: un-, re-, pre-, dis-, anti-, sub-
- Productive suffixes: -tion, -er, -ness, -ment, -ity
- Scientific vocabulary: micro/macro, hyper/hypo, tele-
```

### What We're Missing ❌
```
Critical Gaps:
1. Common Germanic roots: happy, sad, good, bad, love, hate
2. Basic verbs: do, make, take, give, get, put, come, go
3. Everyday nouns: view, build, work, life, time, way
4. Bound morphemes: -y, -ical, -al, -ous, -ive, -able
5. Phonological variants: phone/phon, scope/scop
```

## 🗺️ Roadmap to 90% Coverage

### Phase 1: Core Vocabulary (45.9% → 70%)
**Add 300 morphemes focused on frequency**

#### 1.1 High-Frequency Germanic Roots (100 morphemes)
```
Emotions: happy, sad, glad, mad, love, hate, like, want
Actions: do, make, take, give, get, put, come, go, run, walk
Things: thing, way, life, time, work, home, food, water
Qualities: good, bad, big, small, hot, cold, new, old
```

#### 1.2 Essential Bound Morphemes (50 morphemes)
```
Adjective endings: -y, -ly, -ish, -ful, -less, -ous, -al, -ical
Verb endings: -ate, -ize, -ify, -en
Noun endings: -hood, -ship, -dom, -ism, -ist
```

#### 1.3 Common Latin/Greek Roots (150 morphemes)
```
Movement: mov, mot, port, duct, tract, ject
Mind: ment, psych, cogn, sci, mem
Communication: dict, scrib, graph, voc, claim
Vision: vid, vis, spec, scope, opt
Life: viv, vit, nat, gen
```

### Phase 2: Systematic Coverage (70% → 85%)
**Add 500 morphemes using corpus analysis**

#### 2.1 Analyze Common Word Families
- Extract morphemes from top 5,000 English words
- Group by semantic fields (education, business, daily life)
- Prioritize by productivity score

#### 2.2 Fill Etymological Gaps
```python
Priority Etymology Groups:
1. Norman French roots: court, just, peace, war
2. Old Norse roots: skill, want, take, they
3. Arabic loanwords: algebra, cotton, sugar
4. Spanish/Italian: plaza, piano, tempo
```

#### 2.3 Add Phonological Variants
- Map allomorphs: in-/im-/il-/ir-
- Handle vowel changes: receive/recept, conceive/concept
- Include spelling variants: -ise/-ize, -our/-or

### Phase 3: Fine-Tuning (85% → 90%+)
**Add 200 morphemes based on coverage testing**

#### 3.1 Coverage Testing Protocol
```python
def test_coverage(word_list, morpheme_db):
    """Test morphological coverage on word lists"""
    covered = 0
    total = 0
    missing_morphemes = set()
    
    for word in word_list:
        morphemes = decompose(word)
        if all(m in morpheme_db for m in morphemes):
            covered += 1
        else:
            missing_morphemes.update(
                m for m in morphemes if m not in morpheme_db
            )
        total += 1
    
    return covered/total, missing_morphemes
```

#### 3.2 Test Corpora
1. **General English**: Brown Corpus, BNC
2. **Contemporary**: News articles, Wikipedia
3. **Spoken**: Subtitle corpus, transcripts
4. **Academic**: Academic Word List
5. **Business**: Business English corpus

#### 3.3 Iterative Refinement
1. Run coverage tests
2. Identify top 50 missing morphemes
3. Add with proper spatial positioning
4. Retest and repeat

## 🏗️ Implementation Strategy

### 1. Data Collection Pipeline
```python
# Example: Morpheme extraction from frequency lists
sources = [
    "google-10000-english.txt",
    "academic-word-list.txt", 
    "spoken-corpus-5000.txt"
]

for source in sources:
    words = load_wordlist(source)
    morphemes = extract_morphemes(words)
    
    for morpheme in morphemes:
        if morpheme not in existing_db:
            new_morphemes.append({
                'morpheme': morpheme,
                'type': classify_type(morpheme),
                'etymology': infer_etymology(morpheme),
                'productivity': calculate_productivity(morpheme),
                'frequency': get_frequency(morpheme)
            })
```

### 2. Spatial Positioning Algorithm
```rust
fn calculate_morpheme_position(
    morpheme: &MorphemeData,
    existing_morphemes: &[MorphemeData]
) -> Coordinate3D {
    // X-axis: morpheme type
    let x = match morpheme.morph_type {
        Prefix => 0.2 + gaussian_noise(0.05),
        Root => 0.5 + gaussian_noise(0.1),
        Suffix => 0.8 + gaussian_noise(0.05),
    };
    
    // Y-axis: semantic/etymology with clustering
    let y = etymology_position(morpheme.etymology)
        + semantic_adjustment(morpheme.meaning);
    
    // Z-axis: layer (morphemes = 0.37)
    let z = MORPHEME_LAYER_Z;
    
    // Ensure minimum distance from existing morphemes
    adjust_for_spacing(Coordinate3D { x, y, z }, existing_morphemes)
}
```

### 3. Quality Assurance

#### Validation Criteria
- **Uniqueness**: No duplicate morphemes
- **Productivity**: Score based on ability to form new words
- **Spatial coherence**: Similar morphemes cluster appropriately
- **Etymology accuracy**: Verified against etymological dictionaries

#### Testing Protocol
```bash
# 1. Unit tests for individual morphemes
cargo test morpheme_validation

# 2. Integration tests for word decomposition
cargo test word_decomposition

# 3. Coverage benchmarks
cargo bench coverage_analysis

# 4. Spatial distribution visualization
cargo run --example visualize_morpheme_space
```

## 📋 Morpheme Priority List

### Immediate Additions (Top 100)
```
Rank  Morpheme  Type      Why Critical
----  --------  ----      ------------
1     happy     Root      High frequency emotion
2     -ly       Suffix    Adverb formation
3     view      Root      Common noun/verb
4     -able     Suffix    Adjective formation
5     build     Root      Common action verb
6     -al       Suffix    Adjective formation
7     good      Root      Basic quality
8     -ize      Suffix    Verb formation
9     work      Root      High frequency noun/verb
10    -ical     Suffix    Technical adjectives
...
```

## 🔄 Maintenance Plan

### Quarterly Reviews
1. Run coverage tests on new corpora
2. Identify emerging morphemes (new tech terms, etc.)
3. Prune low-value morphemes if needed
4. Rebalance spatial distribution

### Version Control
```
v1.0: Base 1,100 morphemes (45.9% coverage)
v1.1: +300 core vocabulary (70% target)
v1.2: +500 systematic additions (85% target)
v1.3: +200 fine-tuning (90%+ target)
v2.0: Optimized spatial distribution
```

## 🎯 Success Metrics

### Primary Metrics
- **Coverage**: % of words fully decomposable
- **Accuracy**: % of correct decompositions
- **Spatial coherence**: Clustering quality score

### Secondary Metrics
- **Query performance**: Morpheme lookup speed
- **Memory efficiency**: Database size vs coverage
- **Semantic consistency**: Opposition pair distances

## 🚀 Next Steps

1. **Implement morpheme extraction pipeline**
   ```bash
   python scripts/extract_morphemes.py --source corpora/
   ```

2. **Generate priority morpheme list**
   ```bash
   python scripts/analyze_coverage_gaps.py
   ```

3. **Add morphemes in batches**
   ```bash
   cargo run --example add_morpheme_batch batch_1.json
   ```

4. **Test and iterate**
   ```bash
   cargo test coverage && cargo run --example coverage_report
   ```

## 📝 Notes

- Focus on **morphological productivity** over raw frequency
- Maintain **spatial organization** principles (prefixes left, roots center, suffixes right)
- Preserve **semantic gradients** (oppositions should maintain consistent distances)
- Quality checkpoints at 60%, 75%, and 85% coverage

Remember: **90% excellent coverage > 100% mediocre coverage**
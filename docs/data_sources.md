# Lingo Database - Data Sources for Enhanced English Database

## Current Base Data (Implemented)
- ✅ 26 English letters
- ✅ 44 IPA phonemes for English
- ✅ 50+ common prefixes (Greek, Latin, Germanic)
- ✅ 25+ common suffixes
- ✅ 25+ common roots

## Potential Data Sources to Scrape

### 1. **CMU Pronouncing Dictionary** (Free)
- URL: http://www.speech.cs.cmu.edu/cgi-bin/cmudict
- Contains: 134,000+ English words with phonetic transcriptions
- Use for: Accurate phoneme mappings for every word
- Format: Word → Phoneme sequence

### 2. **Wiktionary** (Free, CC-BY-SA)
- API: https://en.wiktionary.org/w/api.php
- Contains: Etymology, morphology, pronunciations
- Use for: 
  - Detailed etymology for each word
  - Morphological breakdowns
  - Cross-language connections
  
### 3. **WordNet** (Free, Academic)
- URL: https://wordnet.princeton.edu/
- Contains: Semantic relationships, synonyms, hypernyms
- Use for: Building semantic connections between words
- 155,000+ words organized in 175,000+ synsets

### 4. **Online Etymology Dictionary** (Free for non-commercial)
- URL: https://www.etymonline.com/
- Contains: Detailed etymology for 50,000+ words
- Use for: Accurate etymology classification
- Historical word development paths

### 5. **MorphoLex** (Free, Academic)
- URL: https://github.com/hugomailhot/MorphoLex-en
- Contains: Morphological segmentation for 70,000 English words
- Use for: Pre-computed morpheme boundaries
- Example: "unbelievable" → ["un", "believ", "able"]

### 6. **CELEX2** (Academic License)
- Contains: Orthographic, phonological, morphological, syntactic info
- 160,000+ word forms
- Use for: Comprehensive linguistic properties

### 7. **Google Books Ngrams** (Free)
- URL: http://storage.googleapis.com/books/ngrams/books/datasetsv3.html
- Contains: Word frequency data over time
- Use for: Setting IS_FREQUENT flags accurately

## Scraping Strategy

### Phase 1: Core Linguistic Data
```python
# 1. CMU Dict for all phonemes
word_to_phonemes = scrape_cmu_dict()

# 2. MorphoLex for morpheme boundaries  
word_to_morphemes = scrape_morpholex()

# 3. Basic Wiktionary for top 10,000 words
etymologies = scrape_wiktionary_etymology(top_10k_words)
```

### Phase 2: Semantic Connections
```python
# 4. WordNet for relationships
synonyms, hypernyms = extract_wordnet_relations()

# 5. Cross-domain connections
analogies = discover_analogies(word_embeddings)
```

### Phase 3: Enrichment
```python
# 6. Frequency data
frequencies = load_google_ngrams()

# 7. Technical vocabularies
domain_specific = scrape_domain_glossaries()
```

## Implementation Plan

### Step 1: Enhanced Seeder
```rust
pub struct EnhancedSeeder {
    cmu_dict: HashMap<String, Vec<String>>,      // word → phonemes
    morpholex: HashMap<String, Vec<String>>,     // word → morphemes
    etymologies: HashMap<String, Etymology>,     // word → origin
    frequencies: HashMap<String, f32>,           // word → frequency
}
```

### Step 2: Automated Import
```rust
impl EnhancedSeeder {
    pub fn import_cmu_dict(&mut self, path: &str) -> Result<()> {
        // Parse CMU format: WORD  P1 P2 P3
    }
    
    pub fn import_morpholex(&mut self, path: &str) -> Result<()> {
        // Parse: word<tab>morph1+morph2+morph3
    }
}
```

### Step 3: Smart Connection Discovery
```rust
pub fn discover_semantic_connections(&mut self) {
    // Use WordNet synsets
    // Calculate spatial positions based on semantics
    // Create cross-domain analogies
}
```

## Monetization Features (Built on Base)

### Free Tier (Base Database)
- All letters, phonemes, morphemes
- Basic word lookup
- Simple queries
- 10,000 common words

### Pro Features ($9.99/month)
- **AI-Powered Discovery**
  - Real-time morpheme discovery for new words
  - Semantic similarity search
  - Cross-language connections
  
- **Advanced Analytics**
  - Etymology paths visualization
  - Frequency analysis over time
  - Domain-specific vocabularies
  
- **API Access**
  - RESTful API for queries
  - Bulk import/export
  - Custom spatial indexing

### Enterprise ($99/month)
- Custom domain vocabularies
- Multi-language support
- On-premise deployment
- SLA guarantees

## Quick Scraping Example

```python
# Quick Wiktionary scrape for etymology
import requests
from bs4 import BeautifulSoup

def get_etymology(word):
    url = f"https://en.wiktionary.org/wiki/{word}"
    response = requests.get(url)
    soup = BeautifulSoup(response.text, 'html.parser')
    
    # Find etymology section
    etymology = soup.find('span', {'id': 'Etymology'})
    if etymology:
        # Extract origin language
        text = etymology.find_next('p').text
        if 'Latin' in text:
            return 'Latin'
        elif 'Greek' in text:
            return 'Greek'
        elif 'French' in text:
            return 'French'
        # etc...
    
    return 'Unknown'
```

## Next Steps

1. **Implement CMU Dict importer** (most bang for buck)
2. **Add Wiktionary scraper** for top 10k words  
3. **Create pricing tiers** in the API
4. **Build web demo** showing the power

The beauty is the base database is pure linguistics - no AI needed. The AI features are value-adds that justify the subscription! 🚀
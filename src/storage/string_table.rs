//! String table implementation with compression support

use crate::core::error::{LingoError, Result};
use std::collections::HashMap;

/// String table for efficient string storage
#[derive(Debug)]
pub struct StringTable {
    /// Raw string data
    data: Vec<u8>,
    /// String cache for deduplication
    string_cache: HashMap<String, u32>,
}

impl StringTable {
    /// Create a new empty string table
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            string_cache: HashMap::new(),
        }
    }
    
    /// Create from existing data
    pub fn from_data(data: Vec<u8>) -> Self {
        Self {
            data,
            string_cache: HashMap::new(), // Cache will be built on demand
        }
    }
    
    /// Create from byte slice (for memory-mapped access)
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        Ok(Self {
            data: data.to_vec(),
            string_cache: HashMap::new(),
        })
    }
    
    /// Add a string to the table, returning its offset
    pub fn add_string(&mut self, s: &str) -> Result<u32> {
        // Check cache first
        if let Some(&offset) = self.string_cache.get(s) {
            return Ok(offset);
        }
        
        // Check string length
        if s.len() > u16::MAX as usize {
            return Err(LingoError::InvalidFormat(
                format!("String too long: {} bytes", s.len())
            ));
        }
        
        // Store at current offset
        let offset = self.data.len() as u32;
        self.data.extend_from_slice(s.as_bytes());
        
        // Cache the string
        self.string_cache.insert(s.to_string(), offset);
        
        Ok(offset)
    }
    
    /// Get a string from the table
    pub fn get_string(&self, offset: u32, length: u16) -> Result<&str> {
        let start = offset as usize;
        let end = start + length as usize;
        
        if end > self.data.len() {
            return Err(LingoError::IndexOutOfBounds {
                index: end,
                max: self.data.len(),
            });
        }
        
        let bytes = &self.data[start..end];
        std::str::from_utf8(bytes).map_err(|_| LingoError::InvalidUtf8)
    }
    
    /// Get the total size of the string table
    pub fn size(&self) -> usize {
        self.data.len()
    }
    
    /// Get raw data for serialization
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
    
    /// Clear the string table
    pub fn clear(&mut self) {
        self.data.clear();
        self.string_cache.clear();
    }
}

impl Default for StringTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Compressed string table with dictionary encoding
pub struct CompressedStringTable {
    /// Dictionary of common substrings
    dictionary: Vec<String>,
    /// Dictionary offsets
    dict_offsets: HashMap<String, u16>,
    /// Main string table
    table: StringTable,
}

impl CompressedStringTable {
    /// Create a new compressed string table
    pub fn new() -> Self {
        Self {
            dictionary: Vec::new(),
            dict_offsets: HashMap::new(),
            table: StringTable::new(),
        }
    }
    
    /// Build dictionary from a corpus of strings
    pub fn build_dictionary(&mut self, strings: &[String], max_entries: usize) {
        // Count substring frequencies
        let mut freq_map: HashMap<String, usize> = HashMap::new();
        
        for s in strings {
            // Extract common prefixes and suffixes
            for len in 3..=8 {
                if s.len() >= len {
                    // Prefix
                    let prefix = &s[..len];
                    *freq_map.entry(prefix.to_string()).or_insert(0) += 1;
                    
                    // Suffix
                    let suffix = &s[s.len() - len..];
                    *freq_map.entry(suffix.to_string()).or_insert(0) += 1;
                }
            }
        }
        
        // Sort by frequency and select top entries
        let mut freq_vec: Vec<(String, usize)> = freq_map.into_iter().collect();
        freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Build dictionary
        self.dictionary.clear();
        self.dict_offsets.clear();
        
        for (i, (substr, _)) in freq_vec.iter().take(max_entries).enumerate() {
            self.dictionary.push(substr.clone());
            self.dict_offsets.insert(substr.clone(), i as u16);
        }
    }
    
    /// Encode a string using dictionary compression
    pub fn encode_string(&mut self, s: &str) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();
        let mut pos = 0;
        
        while pos < s.len() {
            let mut found = false;
            
            // Try to match dictionary entries
            for len in (3..=8).rev() {
                if pos + len <= s.len() {
                    let substr = &s[pos..pos + len];
                    if let Some(&dict_idx) = self.dict_offsets.get(substr) {
                        // Encode as dictionary reference
                        encoded.push(0xFF); // Dictionary marker
                        encoded.push((dict_idx >> 8) as u8);
                        encoded.push((dict_idx & 0xFF) as u8);
                        pos += len;
                        found = true;
                        break;
                    }
                }
            }
            
            if !found {
                // Encode as literal
                encoded.push(s.as_bytes()[pos]);
                pos += 1;
            }
        }
        
        Ok(encoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_string_table_basic() {
        let mut table = StringTable::new();
        
        let offset1 = table.add_string("hello").unwrap();
        let offset2 = table.add_string("world").unwrap();
        let offset3 = table.add_string("hello").unwrap(); // Duplicate
        
        assert_eq!(offset1, offset3); // Should reuse existing string
        assert_ne!(offset1, offset2);
        
        assert_eq!(table.get_string(offset1, 5).unwrap(), "hello");
        assert_eq!(table.get_string(offset2, 5).unwrap(), "world");
    }
    
    #[test]
    fn test_dictionary_compression() {
        let mut compressed = CompressedStringTable::new();
        
        let corpus = vec![
            "technical".to_string(),
            "technology".to_string(),
            "technician".to_string(),
            "technique".to_string(),
        ];
        
        compressed.build_dictionary(&corpus, 10);
        
        // Should have "tech" in dictionary
        assert!(compressed.dict_offsets.contains_key("tech"));
    }
}
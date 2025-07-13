//! Integration module to combine scraped data with base data

use crate::data::english_base::*;
use crate::data::scraped_data::*;

/// Combine base and scraped morpheme data
pub fn get_all_prefixes() -> Vec<&'static MorphemeData> {
    let mut all = Vec::new();
    all.extend(ENGLISH_PREFIXES);
    all.extend(SCRAPED_PREFIXES);
    all
}

pub fn get_all_suffixes() -> Vec<&'static MorphemeData> {
    let mut all = Vec::new();
    all.extend(ENGLISH_SUFFIXES);
    all.extend(SCRAPED_SUFFIXES);
    all
}

pub fn get_all_roots() -> Vec<&'static MorphemeData> {
    let mut all = Vec::new();
    all.extend(ENGLISH_ROOTS);
    all.extend(SCRAPED_ROOTS);
    all
}

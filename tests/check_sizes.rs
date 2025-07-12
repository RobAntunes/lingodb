//! Check struct sizes

#[test]
fn check_header_size() {
    use lingo::storage::file_format::LingoFileHeader;
    
    let size = std::mem::size_of::<LingoFileHeader>();
    println!("LingoFileHeader size: {} bytes", size);
    
    // Calculate expected size
    let expected = 8 + 2 + 2 + 4 +  // Magic & Version (16 bytes)
                   8 + 4 + 4 + 1 + 1 + 1 + 1 + // File Layout (24 bytes, should be 32)
                   8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + // Section Offsets (64 bytes)
                   8 + 8 + 8 + 8 + // Index Offsets (32 bytes)
                   8 + 8 + 8 + 8 + // Checksums (32 bytes)
                   8 + 8 + 16 + 32 + // Metadata (64 bytes)
                   64 + // Reserved
                   208; // Padding
    
    println!("Expected size: {} bytes", expected);
}
//! Unit tests for core components

#[cfg(test)]
mod tests {
    use crate::core::*;
    
    #[test]
    fn test_coordinate_3d_creation() {
        let coord = Coordinate3D { x: 0.5, y: 0.3, z: 0.4 };
        // Copy fields before comparison due to packed struct
        let x = coord.x;
        let y = coord.y;
        let z = coord.z;
        assert_eq!(x, 0.5);
        assert_eq!(y, 0.3);
        assert_eq!(z, 0.4);
    }
    
    #[test]
    fn test_layer_z_ranges() {
        // Each layer should have distinct Z ranges
        assert_eq!(Layer::Letters.z_range(), (0.0, 0.15));
        assert_eq!(Layer::Phonemes.z_range(), (0.15, 0.30));
        assert_eq!(Layer::Morphemes.z_range(), (0.30, 0.45));
        assert_eq!(Layer::Words.z_range(), (0.45, 0.60));
        assert_eq!(Layer::Phrases.z_range(), (0.60, 0.75));
        assert_eq!(Layer::Concepts.z_range(), (0.75, 0.90));
        assert_eq!(Layer::Domains.z_range(), (0.90, 1.0));
    }
    
    #[test]
    fn test_node_flags() {
        let flags = NodeFlags::IS_TECHNICAL | NodeFlags::IS_PRODUCTIVE;
        assert!(flags.contains(NodeFlags::IS_TECHNICAL));
        assert!(flags.contains(NodeFlags::IS_PRODUCTIVE));
        assert!(!flags.contains(NodeFlags::IS_ARCHAIC));
    }
    
    #[test]
    fn test_connection_types() {
        // Ensure all connection types have unique values
        let types = vec![
            ConnectionType::Synonymy as u8,
            ConnectionType::Antonymy as u8,
            ConnectionType::Hypernymy as u8,
            ConnectionType::Hyponymy as u8,
            ConnectionType::Meronymy as u8,
            ConnectionType::Causation as u8,
            ConnectionType::Derivation as u8,
            ConnectionType::Etymology as u8,
            ConnectionType::Analogy as u8,
        ];
        
        // Check for uniqueness
        let mut sorted = types.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(types.len(), sorted.len());
    }
    
    #[test]
    fn test_node_id_ordering() {
        let id1 = NodeId(1);
        let id2 = NodeId(2);
        let id3 = NodeId(10);
        
        assert!(id1 < id2);
        assert!(id2 < id3);
        assert_eq!(id1, NodeId(1));
    }
    
    #[test]
    fn test_linguistic_node_creation() {
        let node = LinguisticNode::new(
            NodeId(1),
            Layer::Words,
            Coordinate3D { x: 0.5, y: 0.3, z: 0.4 }
        );
        
        // Copy fields before comparison due to packed struct
        let id = node.id;
        let layer = node.layer;
        let x = node.position.x;
        assert_eq!(id, NodeId(1));
        assert_eq!(layer, Layer::Words);
        assert_eq!(x, 0.5);
    }
}
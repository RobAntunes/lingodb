// GRADIENT DISCOVERY: up ↔ down (confidence 0.99)

// Step 1: Get the 3D coordinates
let point_up = get_3d_coordinates("up");
let point_down = get_3d_coordinates("down");

// Step 2: Calculate the semantic line
let semantic_vector = point_down - point_up;
let semantic_distance = magnitude(semantic_vector);

// Step 3: Sample 9 intermediate points
for i in 1..=9 {
    let t = i as f32 / 10.0;  // 0.1, 0.2, 0.3, ... 0.9
    let sample_point = point_up + t * semantic_vector;
    
    // Step 4: Find concepts near this gradient point
    let nearby_concepts = find_concepts_within_radius(sample_point, 0.12);
    
    println!("Gradient position {:.1} ({}% toward 'down'):", t, (t * 100.0) as i32);
    for concept in nearby_concepts {
        let morphemes = get_morphemes(concept.word);
        let etymology = get_etymology_family(concept.word);
        let distance_from_line = distance_to_line(concept.coordinates, point_up, point_down);
        
        println!("  - {}: morphemes={:?}, etymology={}, distance_from_line={:.3}", 
                concept.word, morphemes, etymology, distance_from_line);
    }
    println!();
}
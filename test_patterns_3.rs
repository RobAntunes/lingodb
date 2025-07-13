let mut opposition_distances = Vec::new();

for opposition_pair in all_75_opposition_pairs {
    let point_a = get_3d_coordinates(opposition_pair.word1);
    let point_b = get_3d_coordinates(opposition_pair.word2);
    let distance = calculate_distance(point_a, point_b);
    
    opposition_distances.push((
        opposition_pair,
        distance,
        classify_opposition_type(opposition_pair)
    ));
}

// Sort by distance to see the natural clustering
opposition_distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

// Look for natural distance clusters
let clusters = find_distance_clusters(opposition_distances);
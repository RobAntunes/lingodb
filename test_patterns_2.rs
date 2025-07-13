let point_in = get_3d_coordinates("in");
let point_out = get_3d_coordinates("out");

for i in 1..=9 {
    let t = i as f32 / 10.0;
    let sample_point = point_in + t * (point_out - point_in);
    let nearby = find_concepts_within_radius(sample_point, 0.12);
    
    println!("Containment gradient {:.1}: {:?}", t, nearby);
}
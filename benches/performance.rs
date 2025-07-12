//! Performance benchmarks for Lingo database

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lingo::core::Coordinate3D;

fn benchmark_coordinate_distance(c: &mut Criterion) {
    let coord1 = Coordinate3D::new(0.1, 0.2, 0.3);
    let coord2 = Coordinate3D::new(0.7, 0.8, 0.9);
    
    c.bench_function("coordinate_distance", |b| {
        b.iter(|| {
            black_box(coord1.distance(black_box(coord2)))
        })
    });
}

fn benchmark_coordinate_distance_squared(c: &mut Criterion) {
    let coord1 = Coordinate3D::new(0.1, 0.2, 0.3);
    let coord2 = Coordinate3D::new(0.7, 0.8, 0.9);
    
    c.bench_function("coordinate_distance_squared", |b| {
        b.iter(|| {
            black_box(coord1.distance_squared(black_box(coord2)))
        })
    });
}

criterion_group!(benches, benchmark_coordinate_distance, benchmark_coordinate_distance_squared);
criterion_main!(benches);
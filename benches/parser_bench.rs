use cadstream::{stl_parser::StlParser, cad_data::Triangle};
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use glam::Vec3;

fn create_test_stl_data(triangle_count: usize) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(b"solid test_object\n");
    
    for i in 0..triangle_count {
        let triangle = format!(
            "  facet normal 0.0 0.0 1.0\n    outer loop\n      vertex {} 0.0 0.0\n      vertex {} 1.0 0.0\n      vertex {} 0.0 1.0\n    endloop\n  endfacet\n",
            i as f32 * 0.1, i as f32 * 0.1, i as f32 * 0.1
        );
        data.extend_from_slice(triangle.as_bytes());
    }
    
    data.extend_from_slice(b"endsolid test_object\n");
    data
}

fn create_binary_stl_data(triangles: &[Triangle]) -> Vec<u8> {
    let mut data = vec![0u8; 80]; // Header
    data.extend_from_slice(&(triangles.len() as u32).to_le_bytes());
    
    for triangle in triangles {
        // Normal vector
        data.extend_from_slice(&triangle.normal.x.to_le_bytes());
        data.extend_from_slice(&triangle.normal.y.to_le_bytes());
        data.extend_from_slice(&triangle.normal.z.to_le_bytes());
        
        // Vertices
        for vertex in &triangle.vertices {
            data.extend_from_slice(&vertex.x.to_le_bytes());
            data.extend_from_slice(&vertex.y.to_le_bytes());
            data.extend_from_slice(&vertex.z.to_le_bytes());
        }
        
        // Attribute byte count
        data.extend_from_slice(&0u16.to_le_bytes());
    }
    
    data
}

fn bench_ascii_parsing(c: &mut Criterion) {
    let triangle_counts = vec![100, 1000, 10000];
    
    let mut group = c.benchmark_group("ascii_stl_parsing");
    
    for &count in &triangle_counts {
        let data = create_test_stl_data(count);
        
        group.bench_with_input(
            BenchmarkId::new("triangles", count),
            &data,
            |b, data| {
                let parser = StlParser::new();
                b.iter(|| {
                    parser.parse_data(data, format!("test_{}.stl", count)).unwrap()
                });
            },
        );
    }
    
    group.finish();
}

fn bench_binary_parsing(c: &mut Criterion) {
    let triangle_counts = vec![100, 1000, 10000];
    
    let mut group = c.benchmark_group("binary_stl_parsing");
    
    for &count in &triangle_counts {
        // Create test triangles
        let triangles: Vec<Triangle> = (0..count).map(|i| Triangle {
            vertices: [
                Vec3::new(i as f32 * 0.1, 0.0, 0.0),
                Vec3::new(i as f32 * 0.1, 1.0, 0.0),
                Vec3::new(i as f32 * 0.1, 0.0, 1.0),
            ],
            normal: Vec3::new(0.0, 0.0, 1.0),
        }).collect();
        
        let data = create_binary_stl_data(&triangles);
        
        group.bench_with_input(
            BenchmarkId::new("triangles", count),
            &data,
            |b, data| {
                let parser = StlParser::new();
                b.iter(|| {
                    parser.parse_data(data, format!("test_{}.stl", count)).unwrap()
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_ascii_parsing, bench_binary_parsing);
criterion_main!(benches); 
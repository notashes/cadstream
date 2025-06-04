use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub vertices: [Vec3; 3],
    pub normal: Vec3,
}

#[derive(Debug, Clone)]
pub struct CadModel {
    pub name: String,
    pub triangles: Vec<Triangle>,
    pub bounds: BoundingBox,
    pub precision_info: PrecisionInfo,
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

#[derive(Debug, Clone)]
pub struct PrecisionInfo {
    pub max_error: f64,
    pub vertex_count: usize,
    pub triangle_count: usize,
    pub file_size_bytes: usize,
}

impl CadModel {
    pub fn new(name: String, triangles: Vec<Triangle>) -> Self {
        let bounds = Self::calculate_bounds(&triangles);
        let precision_info = PrecisionInfo {
            max_error: 0.0, // TODO: Calculate based on conversion
            vertex_count: triangles.len() * 3,
            triangle_count: triangles.len(),
            file_size_bytes: 0, // Will be set by parser
        };

        Self {
            name,
            triangles,
            bounds,
            precision_info,
        }
    }

    fn calculate_bounds(triangles: &[Triangle]) -> BoundingBox {
        if triangles.is_empty() {
            return BoundingBox {
                min: Vec3::ZERO,
                max: Vec3::ZERO,
            };
        }

        let mut min = triangles[0].vertices[0];
        let mut max = triangles[0].vertices[0];

        for triangle in triangles {
            for vertex in &triangle.vertices {
                min = min.min(*vertex);
                max = max.max(*vertex);
            }
        }

        BoundingBox { min, max }
    }

    pub fn center(&self) -> Vec3 {
        (self.bounds.min + self.bounds.max) * 0.5
    }

    pub fn size(&self) -> Vec3 {
        self.bounds.max - self.bounds.min
    }

    pub fn max_dimension(&self) -> f32 {
        let size = self.size();
        size.x.max(size.y).max(size.z)
    }
} 
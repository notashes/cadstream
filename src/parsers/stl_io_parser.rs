use super::{FileFormat, FileParser};
use crate::cad_data::{CadModel, Triangle};
use anyhow::{Context, Result};
use glam::Vec3;
use std::io::Cursor;

/// STL parser implementation using the stl_io crate
pub struct StlIoParser;

impl StlIoParser {
    pub fn new() -> Self {
        Self
    }
}

impl FileParser for StlIoParser {
    fn parse_data(&self, data: &[u8], name: String) -> Result<CadModel> {
        let mut cursor = Cursor::new(data);

        // Use stl_io to parse the STL data
        let stl = stl_io::read_stl(&mut cursor)
            .with_context(|| format!("Failed to parse STL data for {}", name))?;

        // Convert stl_io mesh to our Triangle format
        let triangles: Vec<Triangle> = stl
            .faces
            .into_iter()
            .map(|face| {
                // Get the three vertices for this face
                let v0 = stl.vertices[face.vertices[0]];
                let v1 = stl.vertices[face.vertices[1]];
                let v2 = stl.vertices[face.vertices[2]];

                let vertices = [
                    Vec3::new(v0[0], v0[1], v0[2]),
                    Vec3::new(v1[0], v1[1], v1[2]),
                    Vec3::new(v2[0], v2[1], v2[2]),
                ];

                // Use the face normal
                let normal = Vec3::new(face.normal[0], face.normal[1], face.normal[2]);

                Triangle { vertices, normal }
            })
            .collect();

        println!(
            "📐 Parsed {} triangles from {} (using stl_io)",
            triangles.len(),
            name
        );

        let mut model = CadModel::new(name, triangles);
        model.precision_info.file_size_bytes = data.len();

        Ok(model)
    }

    fn supported_formats(&self) -> &[FileFormat] {
        &[FileFormat::Stl]
    }

    fn parser_name(&self) -> &'static str {
        "stl_io Parser"
    }
}

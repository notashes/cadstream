use anyhow::{Result, anyhow, Context};
use glam::Vec3;
use std::io::{Read, Cursor};
use std::path::Path;

use crate::cad_data::{CadModel, Triangle};

pub struct StlParser;

impl StlParser {
    pub fn new() -> Self {
        Self
    }

    pub async fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<CadModel> {
        let data = tokio::fs::read(&path).await
            .with_context(|| format!("Failed to read file: {}", path.as_ref().display()))?;
        
        let file_name = path.as_ref()
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        self.parse_data(&data, file_name)
    }

    pub fn parse_data(&self, data: &[u8], name: String) -> Result<CadModel> {
        if data.len() < 80 {
            return Err(anyhow!("File too small to be a valid STL"));
        }

        // Check if it's ASCII STL by looking for "solid" at the beginning
        let is_ascii = data.starts_with(b"solid") && 
            data.iter().take(1024).all(|&b| b.is_ascii() && b != 0);

        let triangles = if is_ascii {
            self.parse_ascii_stl(data)?
        } else {
            self.parse_binary_stl(data)?
        };

        println!("📐 Parsed {} triangles from {}", triangles.len(), name);

        let mut model = CadModel::new(name, triangles);
        model.precision_info.file_size_bytes = data.len();
        
        Ok(model)
    }

    fn parse_ascii_stl(&self, data: &[u8]) -> Result<Vec<Triangle>> {
        let content = String::from_utf8_lossy(data);
        let mut triangles = Vec::new();
        let mut lines = content.lines().map(|l| l.trim()).filter(|l| !l.is_empty());

        // Skip the "solid" line
        if let Some(line) = lines.next() {
            if !line.starts_with("solid") {
                return Err(anyhow!("Invalid ASCII STL: missing 'solid' header"));
            }
        }

        while let Some(line) = lines.next() {
            if line.starts_with("endsolid") {
                break;
            }

            if line.starts_with("facet normal") {
                let normal = self.parse_normal_line(line)?;
                
                // Skip "outer loop"
                if let Some(loop_line) = lines.next() {
                    if !loop_line.starts_with("outer loop") {
                        return Err(anyhow!("Expected 'outer loop', found: {}", loop_line));
                    }
                }

                // Parse three vertices
                let mut vertices = [Vec3::ZERO; 3];
                for i in 0..3 {
                    if let Some(vertex_line) = lines.next() {
                        vertices[i] = self.parse_vertex_line(vertex_line)?;
                    } else {
                        return Err(anyhow!("Missing vertex in triangle"));
                    }
                }

                // Skip "endloop" and "endfacet"
                lines.next(); // endloop
                lines.next(); // endfacet

                triangles.push(Triangle { vertices, normal });
            }
        }

        Ok(triangles)
    }

    fn parse_binary_stl(&self, data: &[u8]) -> Result<Vec<Triangle>> {
        if data.len() < 84 {
            return Err(anyhow!("Binary STL too small"));
        }

        let mut cursor = Cursor::new(data);
        
        // Skip 80-byte header
        cursor.set_position(80);
        
        // Read triangle count
        let triangle_count = self.read_u32_le(&mut cursor)?;
        
        if data.len() < 84 + (triangle_count as usize * 50) {
            return Err(anyhow!("Binary STL data truncated"));
        }

        let mut triangles = Vec::with_capacity(triangle_count as usize);
        
        for _ in 0..triangle_count {
            // Read normal
            let normal = Vec3::new(
                self.read_f32_le(&mut cursor)?,
                self.read_f32_le(&mut cursor)?,
                self.read_f32_le(&mut cursor)?,
            );

            // Read vertices
            let mut vertices = [Vec3::ZERO; 3];
            for i in 0..3 {
                vertices[i] = Vec3::new(
                    self.read_f32_le(&mut cursor)?,
                    self.read_f32_le(&mut cursor)?,
                    self.read_f32_le(&mut cursor)?,
                );
            }

            // Skip attribute byte count
            cursor.set_position(cursor.position() + 2);

            triangles.push(Triangle { vertices, normal });
        }

        Ok(triangles)
    }

    fn parse_normal_line(&self, line: &str) -> Result<Vec3> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 5 || parts[0] != "facet" || parts[1] != "normal" {
            return Err(anyhow!("Invalid normal line: {}", line));
        }

        Ok(Vec3::new(
            parts[2].parse::<f32>()?,
            parts[3].parse::<f32>()?,
            parts[4].parse::<f32>()?,
        ))
    }

    fn parse_vertex_line(&self, line: &str) -> Result<Vec3> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 4 || parts[0] != "vertex" {
            return Err(anyhow!("Invalid vertex line: {}", line));
        }

        Ok(Vec3::new(
            parts[1].parse::<f32>()?,
            parts[2].parse::<f32>()?,
            parts[3].parse::<f32>()?,
        ))
    }

    fn read_u32_le(&self, cursor: &mut Cursor<&[u8]>) -> Result<u32> {
        let mut buf = [0u8; 4];
        cursor.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    fn read_f32_le(&self, cursor: &mut Cursor<&[u8]>) -> Result<f32> {
        let mut buf = [0u8; 4];
        cursor.read_exact(&mut buf)?;
        Ok(f32::from_le_bytes(buf))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_ascii_stl() {
        let stl_data = b"solid test
facet normal 0.0 0.0 1.0
outer loop
vertex 0.0 0.0 0.0
vertex 1.0 0.0 0.0
vertex 0.0 1.0 0.0
endloop
endfacet
endsolid test";

        let parser = StlParser::new();
        let model = parser.parse_data(stl_data, "test.stl".to_string()).unwrap();
        
        assert_eq!(model.triangles.len(), 1);
        assert_eq!(model.triangles[0].vertices[0], Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(model.triangles[0].vertices[1], Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(model.triangles[0].vertices[2], Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(model.triangles[0].normal, Vec3::new(0.0, 0.0, 1.0));
    }
} 
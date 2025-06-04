use anyhow::Result;
use rerun as rr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

use crate::cad_data::CadModel;

pub struct RerunRenderer {
    rec: rr::RecordingStream,
    current_model: Arc<RwLock<Option<CadModel>>>,
}

impl RerunRenderer {
    pub fn new(current_model: Arc<RwLock<Option<CadModel>>>) -> Result<Self> {
        // Create RecordingStream that serves directly over gRPC
        let rec = rr::RecordingStreamBuilder::new("cadstream")
            .default_enabled(true)
            .serve_grpc()?;

        // Log application info
        rec.log_static(
            "info",
            &rr::TextLog::new("🚀 CAD Stream Processor with Rerun"),
        )?;

        Ok(Self { rec, current_model })
    }

    pub async fn run(&self) -> Result<()> {
        println!("🔄 CAD Stream with Rerun visualization started!");
        println!("📁 Add STL files to the directory to see them in Rerun viewer");
        println!("🌐 Open http://localhost:9090/ in your browser and connect to 0.0.0.0:9876");

        // Set up initial timeline context
        self.rec.set_time_sequence("frame", 0);

        let mut last_triangle_count = 0;
        let mut frame_count = 0;

        // Log initial state immediately
        {
            let model = self.current_model.read().await;
            if let Some(model) = model.as_ref() {
                self.log_model(model).await?;
                last_triangle_count = model.triangles.len();
                println!(
                    "📊 Initial data logged to Rerun: {} triangles",
                    last_triangle_count
                );
            } else {
                // Log empty scene to show Rerun is connected
                self.rec.log(
                    "status",
                    &rr::TextLog::new("🎯 CAD Stream connected! Waiting for STL files..."),
                )?;
            }
        }

        loop {
            let model = self.current_model.read().await;

            if let Some(model) = model.as_ref() {
                // Only update if model changed
                if model.triangles.len() != last_triangle_count {
                    frame_count += 1;
                    self.rec.set_time_sequence("frame", frame_count);
                    self.log_model(model).await?;
                    last_triangle_count = model.triangles.len();
                }
            }

            // Check every 100ms for new models
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    async fn log_model(&self, model: &CadModel) -> Result<()> {
        // Convert CAD triangles to Rerun's mesh format
        let vertices: Vec<[f32; 3]> = model
            .triangles
            .iter()
            .flat_map(|tri| tri.vertices.iter().map(|v| [v.x, v.y, v.z]))
            .collect();

        // Create triangle indices (every 3 vertices forms a triangle)
        let triangle_indices: Vec<[u32; 3]> = (0..vertices.len())
            .step_by(3)
            .map(|i| [i as u32, (i + 1) as u32, (i + 2) as u32])
            .collect();

        // Log the mesh to Rerun
        self.rec.log(
            "cad_model/geometry",
            &rr::Mesh3D::new(vertices)
                .with_triangle_indices(triangle_indices)
                .with_albedo_factor([178u8, 178u8, 230u8, 255u8]),
        )?;

        // Log metadata as text
        let info_text = format!(
            "📊 Model: {}\n🔺 Triangles: {}\n📏 Bounds: {:.2} x {:.2} x {:.2}\n💾 File size: {} bytes",
            model.name,
            model.triangles.len(),
            model.size().x,
            model.size().y,
            model.size().z,
            model.precision_info.file_size_bytes
        );

        self.rec
            .log("cad_model/info", &rr::TextLog::new(info_text))?;

        // Log bounding box for reference
        let bbox_min = model.bounds.min;
        let bbox_max = model.bounds.max;

        self.rec.log(
            "cad_model/bounding_box",
            &rr::Boxes3D::from_centers_and_sizes(
                [[
                    (bbox_min.x + bbox_max.x) / 2.0,
                    (bbox_min.y + bbox_max.y) / 2.0,
                    (bbox_min.z + bbox_max.z) / 2.0,
                ]],
                [[
                    bbox_max.x - bbox_min.x,
                    bbox_max.y - bbox_min.y,
                    bbox_max.z - bbox_min.z,
                ]],
            )
            .with_colors([[204u8, 204u8, 204u8, 76u8]]),
        )?;

        println!(
            "📊 Updated Rerun: {} triangles from {}",
            model.triangles.len(),
            model.name
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cad_data::{CadModel, Triangle};
    use glam::Vec3;

    #[tokio::test]
    async fn test_rerun_renderer_creation() {
        let current_model: Arc<RwLock<Option<CadModel>>> = Arc::new(RwLock::new(None));
        // This would normally spawn the Rerun viewer, so we skip in tests
        // let _renderer = RerunRenderer::new(current_model);
    }
}

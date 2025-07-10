mod cad_data;
mod file_watcher;
mod parsers;
mod rerun_renderer;
mod stl_parser;

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{cad_data::CadModel, file_watcher::FileWatcher, rerun_renderer::RerunRenderer};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Create a test STL file if no STL files exist
    let current_dir = std::env::current_dir()?;
    let mut has_stl_files = false;
    let mut entries = tokio::fs::read_dir(&current_dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        if entry.path().extension().and_then(|ext| ext.to_str()) == Some("stl") {
            has_stl_files = true;
            break;
        }
    }

    if !has_stl_files {
        if let Err(e) = file_watcher::create_test_stl_file().await {
            eprintln!("Failed to create test STL file: {}", e);
        }
    }

    // Setup shared model state
    let current_model = Arc::new(RwLock::new(None));
    let _file_watcher = FileWatcher::new(current_model.clone()).await?;

    // Start Rerun visualization
    run_rerun_mode(current_model).await
}

async fn run_rerun_mode(current_model: Arc<RwLock<Option<CadModel>>>) -> Result<()> {
    println!("🌐 Starting CAD Stream Processor with Rerun visualization");
    println!("----------------------------------------------------------");
    println!("🚀 Rerun gRPC server is starting...");

    let rerun_renderer = RerunRenderer::new(current_model)?;

    println!("✅ Rerun server is running on port 9876");
    println!("📱 To view the data:");
    println!("   1. Install Rerun viewer: pip install rerun-sdk");
    println!("   2. Connect with: rerun --connect 127.0.0.1:9876");
    println!("   3. Or open Rerun app and connect to: 127.0.0.1:9876");
    println!("----------------------------------------------------------");

    rerun_renderer.run().await
}

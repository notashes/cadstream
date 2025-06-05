use anyhow::Result;
use notify::{
    event::CreateKind, event::ModifyKind, Event, EventKind, RecommendedWatcher, RecursiveMode,
    Watcher,
};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};

use crate::cad_data::CadModel;
use crate::parsers::{parse_file, FileFormat, ParserFactory};

pub struct FileWatcher {
    _watcher: RecommendedWatcher,
    current_model: Arc<RwLock<Option<CadModel>>>,
}

impl FileWatcher {
    pub async fn new(current_model: Arc<RwLock<Option<CadModel>>>) -> Result<Self> {
        let (tx, mut rx) = mpsc::channel::<PathBuf>(32);
        let current_model_clone = current_model.clone();

        // Start file processing task
        tokio::spawn(async move {
            while let Some(path) = rx.recv().await {
                if let Err(e) = Self::process_file(&path, &current_model_clone).await {
                    eprintln!("❌ Failed to process file {}: {}", path.display(), e);
                }
            }
        });

        let watcher =
            notify::recommended_watcher(move |res: Result<Event, notify::Error>| match res {
                Ok(event) => {
                    let should_process = match event.kind {
                        EventKind::Create(CreateKind::File) => true,
                        EventKind::Modify(ModifyKind::Data(_)) => true,
                        _ => false,
                    };

                    if should_process {
                        for path in event.paths {
                            if Self::is_supported_file(&path) {
                                println!("📁 Detected CAD file: {}", path.display());
                                if let Err(e) = tx.try_send(path.to_path_buf()) {
                                    eprintln!("Failed to queue file for processing: {}", e);
                                }
                            }
                        }
                    }
                }
                Err(e) => eprintln!("File watcher error: {:?}", e),
            })?;

        let mut file_watcher = Self {
            _watcher: watcher,
            current_model,
        };

        file_watcher.start_watching().await?;

        Ok(file_watcher)
    }

    async fn start_watching(&mut self) -> Result<()> {
        let current_dir = std::env::current_dir()?;
        println!("👀 Watching directory: {}", current_dir.display());

        self._watcher
            .watch(&current_dir, RecursiveMode::NonRecursive)?;

        // Check for existing CAD files in the directory
        let mut entries = tokio::fs::read_dir(&current_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if Self::is_supported_file(&path) {
                println!("📄 Found existing CAD file: {}", path.display());
                if let Err(e) = Self::process_file(&path, &self.current_model).await {
                    eprintln!(
                        "❌ Failed to process existing file {}: {}",
                        path.display(),
                        e
                    );
                } else {
                    // Only load the first file found for now
                    break;
                }
            }
        }

        Ok(())
    }

    async fn process_file(
        path: &Path,
        current_model: &Arc<RwLock<Option<CadModel>>>,
    ) -> Result<()> {
        // Add a small delay to ensure file is fully written
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Detect file format and create appropriate parser
        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

        let format = FileFormat::from_extension(extension)
            .ok_or_else(|| anyhow::anyhow!("Unsupported file format: {}", extension))?;

        let parser = ParserFactory::create_parser(format)?;
        let model = parse_file(parser.as_ref(), path).await?;

        println!(
            "✅ Successfully loaded: {} (using {})",
            model.name,
            parser.parser_name()
        );
        println!("   📊 {} triangles", model.precision_info.triangle_count);
        println!(
            "   📏 Size: {:.2} x {:.2} x {:.2}",
            model.size().x,
            model.size().y,
            model.size().z
        );
        println!(
            "   💾 File size: {} bytes",
            model.precision_info.file_size_bytes
        );

        let mut current = current_model.write().await;
        *current = Some(model);

        Ok(())
    }

    fn is_supported_file(path: &Path) -> bool {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            let supported_extensions = ParserFactory::supported_extensions();
            supported_extensions.contains(&extension.to_lowercase().as_str())
        } else {
            false
        }
    }
}

// Helper function to create a simple test STL file
pub async fn create_test_stl_file() -> Result<()> {
    let test_stl = r#"solid test_cube
facet normal 0.0 0.0 1.0
outer loop
vertex 0.0 0.0 1.0
vertex 1.0 0.0 1.0
vertex 1.0 1.0 1.0
endloop
endfacet
facet normal 0.0 0.0 1.0
outer loop
vertex 0.0 0.0 1.0
vertex 1.0 1.0 1.0
vertex 0.0 1.0 1.0
endloop
endfacet
facet normal 0.0 0.0 -1.0
outer loop
vertex 0.0 0.0 0.0
vertex 1.0 1.0 0.0
vertex 1.0 0.0 0.0
endloop
endfacet
facet normal 0.0 0.0 -1.0
outer loop
vertex 0.0 0.0 0.0
vertex 0.0 1.0 0.0
vertex 1.0 1.0 0.0
endloop
endfacet
endsolid test_cube"#;

    tokio::fs::write("test_cube.stl", test_stl).await?;
    println!("📝 Created test_cube.stl - a simple cube for demonstration");

    Ok(())
}

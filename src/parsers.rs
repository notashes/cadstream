use crate::cad_data::CadModel;
use anyhow::{Context, Result};
use std::path::Path;

/// Supported CAD file formats
#[derive(Debug, Clone, PartialEq)]
pub enum FileFormat {
    Stl,
    // Future formats can be added here
    // Obj,
    // Ply,
    // Step,
}

impl FileFormat {
    /// Detect file format from file extension
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension.to_lowercase().as_str() {
            "stl" => Some(FileFormat::Stl),
            // "obj" => Some(FileFormat::Obj),
            // "ply" => Some(FileFormat::Ply),
            // "step" | "stp" => Some(FileFormat::Step),
            _ => None,
        }
    }

    /// Get file extensions for this format
    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            FileFormat::Stl => &["stl"],
            // FileFormat::Obj => &["obj"],
            // FileFormat::Ply => &["ply"],
            // FileFormat::Step => &["step", "stp"],
        }
    }
}

/// Trait for parsing CAD files into our common CadModel format
pub trait FileParser: Send + Sync {
    /// Parse raw file data
    fn parse_data(&self, data: &[u8], name: String) -> Result<CadModel>;

    /// Get supported file formats
    fn supported_formats(&self) -> &[FileFormat];

    /// Get parser name/description
    fn parser_name(&self) -> &'static str;
}

/// Async wrapper for file parsing
pub async fn parse_file<P: AsRef<Path>>(parser: &dyn FileParser, path: P) -> Result<CadModel> {
    let data = tokio::fs::read(&path)
        .await
        .with_context(|| format!("Failed to read file: {}", path.as_ref().display()))?;

    let file_name = path
        .as_ref()
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    parser.parse_data(&data, file_name)
}

/// Factory for creating appropriate parsers based on file format and feature flags
pub struct ParserFactory;

impl ParserFactory {
    /// Create a parser for the given file format using available implementations
    pub fn create_parser(format: FileFormat) -> Result<Box<dyn FileParser>> {
        match format {
            FileFormat::Stl => Self::create_stl_parser(),
            // Future: add other formats here
        }
    }

    /// Create the best available STL parser based on enabled features
    fn create_stl_parser() -> Result<Box<dyn FileParser>> {
        #[cfg(feature = "stl-io-parser")]
        {
            log::info!("Using stl_io parser for STL files");
            Ok(Box::new(crate::parsers::stl_io_parser::StlIoParser::new()))
        }

        #[cfg(all(feature = "custom-stl-parser", not(feature = "stl-io-parser")))]
        {
            log::info!("Using custom STL parser for STL files");
            Ok(Box::new(
                crate::parsers::custom_stl_parser::CustomStlParser::new(),
            ))
        }

        #[cfg(not(any(feature = "stl-io-parser", feature = "custom-stl-parser")))]
        {
            anyhow::bail!("No STL parser implementation available. Enable either 'stl-io-parser' or 'custom-stl-parser' feature.")
        }
    }

    /// Get all supported file extensions across all available parsers
    pub fn supported_extensions() -> Vec<&'static str> {
        let mut extensions = Vec::new();

        // Add STL if any STL parser is available
        #[cfg(any(feature = "stl-io-parser", feature = "custom-stl-parser"))]
        {
            extensions.extend_from_slice(FileFormat::Stl.extensions());
        }

        // Future: add other format extensions here

        extensions
    }
}

// Sub-modules for different parser implementations
#[cfg(feature = "custom-stl-parser")]
pub mod custom_stl_parser;

#[cfg(feature = "stl-io-parser")]
pub mod stl_io_parser;

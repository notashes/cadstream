use super::{FileFormat, FileParser};
use crate::cad_data::CadModel;
use crate::stl_parser::StlParser;
use anyhow::Result;

/// Custom STL parser implementation using our own parsing logic
pub struct CustomStlParser {
    parser: StlParser,
}

impl CustomStlParser {
    pub fn new() -> Self {
        Self {
            parser: StlParser::new(),
        }
    }
}

impl FileParser for CustomStlParser {
    fn parse_data(&self, data: &[u8], name: String) -> Result<CadModel> {
        self.parser.parse_data(data, name)
    }

    fn supported_formats(&self) -> &[FileFormat] {
        &[FileFormat::Stl]
    }

    fn parser_name(&self) -> &'static str {
        "Custom STL Parser"
    }
}

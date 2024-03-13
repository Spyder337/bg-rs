pub mod rust;

use std::error::Error;

pub struct RustGenerator;
pub struct PythonGenerator;
pub struct Build2Generator;

pub trait Generator {
    fn create(
        &self,
        name: String,
        p_type: ProjectType,
        libs: Vec<String>
    ) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectBuilder {
    Build2 = 0,
    Python = 1,
    Rust = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectType {
    Bin = 0,
    Lib = 1,
    NestedBin = 2,
    Nested = 3,
}
use std::error::Error;

pub mod generators;
pub mod libtree;

pub type Result<E> = core::result::Result<(), E>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectType {
    Empty,
    Bin,
    Lib,
    NestedBin,
    Nested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectBuilder {
    Build2,
    Python,
    Rust,
}

pub trait Generator {
    fn validate_input();
    fn create(
        &self,
        name: String,
        p_type: ProjectType,
        libs: Vec<String>
    ) -> Result<Box<dyn Error>>;
}
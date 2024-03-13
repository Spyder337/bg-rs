use std::error::Error;

use clap::ValueEnum;

pub mod generators;
pub mod libtree;

pub type Result<E> = core::result::Result<(), E>;

/// Base project types to build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ProjectType {
    Empty,     //  Empty project
    Bin,       //  Binary executable
    Lib,       //  Library
    NestedBin, //  Root project with sub-projects
    Nested,    //  Root project with library sub-projects
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ProjectBuilder {
    Build2,
    Python,
    Rust,
}

pub fn get_file_ext(p_type: ProjectBuilder) -> String {
    match p_type {
        ProjectBuilder::Build2 => ".xpp".to_string(),
        ProjectBuilder::Python => ".py".to_string(),
        ProjectBuilder::Rust => ".rs".to_string(),
    }
}

pub trait Generator {
    /**
    * Validate input
    * Summary:
       Validates that project types that require sub-projects have them.
    * @param p_type ProjectType
    * @param libs Vec<String>
    * @return Result<Box<dyn Error>>
    */
    fn validate_input(&self, p_type: ProjectType, libs: &Vec<String>) -> Result<Box<dyn Error>> {
        match p_type {
            ProjectType::Nested | ProjectType::NestedBin => {
                if libs.is_empty() {
                    return Err("No subprojects specified for nested project".into());
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
    /**
    # Summary:

    Create the root directory of the project. There are two cases:
    1. The project type is [`ProjectType::Virtual`] or [`ProjectType::VirtualBin`]
        - This case requres sub-project names and sub-directories to be provided.
        - The root directory contains a root buildfile that links to the sub-projects.
    2. Otherwise:
        - The project root is the main build directory. There are no sub-projects.
    */
    fn create_root(&self, p_type: ProjectType, name: &String) -> Result<Box<dyn Error>>;
    /**
    # Summary:

    Create the library files and sub-directories for the project. There are two main cases:
    1. The project type is [`ProjectType::Virtual`] or [`ProjectType::VirtualBin`]
        - Library value roots are treated as seperate sub-projects.
        - The files are then generated in those roots if specified.
    2. Otherwise:
        - The values are paresed as relative paths. If the path ends in a '/' then it is treated as a directory.
        Otherwise, it is treated as a file to be generated.
     */
    fn create_libs(
        &self,
        p_type: ProjectType,
        name: &String,
        libs: &Vec<String>,
    ) -> Result<Box<dyn Error>>;
    fn create(
        &self,
        p_type: ProjectType,
        name: &String,
        libs: &Vec<String>,
    ) -> Result<Box<dyn Error>> {
        // Validate input
        // If there is an error, bubble it up.
        if let Err(e) = self.validate_input(p_type, libs) {
            return Err(e);
        }
        // Create root project
        if let Err(e) = self.create_root(p_type, name) {
            return Err(e);
        }
        // Create sub-projects
        if let Err(e) = self.create_libs(p_type, name, libs) {
            return Err(e);
        }
        Ok(())
    }
}

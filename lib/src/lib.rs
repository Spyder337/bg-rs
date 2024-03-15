use std::{collections::HashMap, env, error::Error, path::{Path, PathBuf}, process::Command};

use clap::ValueEnum;
use generators::RustGenerator;

pub mod generators;
mod tests;

pub type GenResult<E> = core::result::Result<(), E>;

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

pub fn copy_directory(src_path: &Path, dest_path: &Path) -> Result<(), Box<dyn Error>> {
    let res = Command::new("cp")
        .args(vec!["-r", src_path.to_str().unwrap(), dest_path.to_str().unwrap()])
        .spawn();

    if let Err(_e) = res {
        return Err("Failed to copy directory.".into());
    }

    Ok(())
}

pub fn get_generator<T>(p_type: ProjectBuilder) -> impl Generator {
    match p_type {
        ProjectBuilder::Build2 => todo!(),
        ProjectBuilder::Python => todo!(),
        ProjectBuilder::Rust => RustGenerator,
    }
}

pub trait Generator {
    /**
    ### Validate input
    ### Summary:
       Validates that project types that require sub-projects have them.
    * @param p_type ProjectType
    * @param libs Vec<String>
    * @return Result<Box<dyn Error>>
    */
    fn validate_input(&self, p_type: ProjectType, libs: &Vec<String>) -> GenResult<Box<dyn Error>> {
        match p_type {
            ProjectType::Nested | ProjectType::NestedBin => {
                if libs.is_empty() {
                    return Err("No subprojects specified for nested project".into());
                } else {
                    Ok(())
                }
            }
            ProjectType::Empty => {
                if libs.is_empty() {
                    Ok(())
                } else {
                    return Err("Empty projects have no libraries attached.".into());
                }
            }
            _ => Ok(()),
        }
    }

    /**
    ### Summary:
    Converts the array of lib paths into an ordered set of paths under a common root.
    */
    fn parse_lib_str(&self, libs: &Vec<String>) -> HashMap<String, Vec<String>> {
        let mut lib_dict: HashMap<String, Vec<String>> = HashMap::new();
        println!("Lib Count: {}", libs.len());

        for lib in libs {
            //  Get the root name
            let pieces: Vec<&str> = lib.split('/').collect();
            let name = pieces[0].to_string();
            println!("{}", name);
            //  Find the library path and add it to the dictionary.
            let root_end = name.len() + 1;
            let lib_path = &lib[root_end..];
            if !lib_dict.contains_key(&name) {
                if !lib_path.is_empty() {
                    lib_dict.insert(name.to_string(), vec![lib_path.to_string()]);
                } else {
                    lib_dict.insert(name.to_string(), vec![]);
                }
            } else {
                if !lib_path.is_empty() {
                    lib_dict.get_mut(&name).unwrap().push(lib_path.to_string());
                }
            }
        }

        lib_dict
    }

    /**
    Generate an individual project.
     */
    fn create_project(
        &self,
        is_root: bool,
        root: &mut PathBuf,
        p_type: ProjectType,
        p_name: &str,
        libs: &Vec<String>,
    ) -> GenResult<Box<dyn Error>>;

    fn create_root_folder<P>(&self, path: &P) -> GenResult<Box<dyn Error>> where P: AsRef<std::path::Path> ;

    fn create(
        &self,
        p_type: ProjectType,
        name: &String,
        libs: &Vec<String>,
    ) -> GenResult<Box<dyn Error>> {
        // Validate input
        // If there is an error, bubble it up.
        if let Err(e) = self.validate_input(p_type, libs) {
            return Err(e);
        }

        //  Store path for later commands.
        let mut path = env::current_dir()?;

        match p_type {
            ProjectType::Empty => {
                self.create_project(true, &mut path, p_type, name.as_str(), &libs)?
            }
            ProjectType::Bin | ProjectType::Lib => {
                self.create_project(true, &mut path, p_type, name.as_str(), &libs)?;
            }
            ProjectType::NestedBin | ProjectType::Nested => {
                let root_path = path.clone();
                path.push(name);

                //  Try to create the root directory for the project
                //  If it fails return the error.
                if let Err(e) = self.create_root_folder(&path){
                    return Err(e);
                }
                
                env::set_current_dir(path.as_path())?;
                let lib_dict = self.parse_lib_str(libs);
                let _subdir = path.clone();

                for (lib_root, lib_paths) in lib_dict {
                    self.create_project(
                        false,
                        &mut path,
                        ProjectType::Lib,
                        lib_root.as_str(),
                        &lib_paths,
                    )?;
                }

                if p_type == ProjectType::NestedBin {
                    self.create_project(
                        false,
                        &mut path.clone(),
                        ProjectType::Bin,
                        name.as_str(),
                        &vec![],
                    )?;
                }

                env::set_current_dir(root_path.as_path())?;
            }
        }

        Ok(())
    }
}

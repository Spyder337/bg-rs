use std::{ collections::HashMap, error::Error, path, process::Command };
use crate::generators::ProjectType;
use crate::Result;
use super::{ Generator, RustGenerator };

pub enum RustProjectType {
    Bin,
    Lib,
    Virtual,
}

impl RustGenerator {
    fn handle_libs(libs: Vec<String>, r_type: RustProjectType) -> Result<Box<dyn Error>> {
        let mut lib_dict:HashMap<String, Vec<String>> = HashMap::new();

        //  Create a tree of the libraries.
        if !libs.is_empty() {
            for lib in &libs {
                let lib_path = path::Path::new(lib);
                let key = lib_path.parent().unwrap().to_str().unwrap().to_string();
                let value = lib_path.file_name().unwrap().to_str().unwrap().to_string();
                
                if !lib_dict.contains_key(&key) {
                    lib_dict.insert(key, vec![value]);
                } else {
                    lib_dict.get_mut(&key).unwrap().push(value);
                }
            }
        }
        
        match r_type {
            RustProjectType::Bin | RustProjectType::Lib => {
                for lib in libs {
                    let is_dir = lib.ends_with('/');
                    if is_dir {
                        // Create folder and mod.rs
                    }
                    else {
                        //
                    }
                }
                Ok(())
            }
            RustProjectType::Virtual => { Ok(()) }
        }
    }

    fn create_lib(
        project_name: String,
        r_type: RustProjectType,
        lib_names: Vec<String>
    ) -> Result<Box<dyn Error>> {
        // Cargo Create
        // If libs mk dirs and files
        //  Generate mod files as necessary
        let c = Command::new("cargo")
            .arg("new")
            .arg(project_name)
            .arg("--lib")
            .spawn()
            .expect("Failed to create project");

        c.wait_with_output().unwrap();

        if !lib_names.is_empty() {
            if let Ok(_) = RustGenerator::handle_libs(lib_names, r_type) {
                return Ok(());
            } else {
                return Err("Failed to generate library files".into());
            }
        }

        Ok(())
    }

    fn create_bin(
        project_name: String,
        r_type: RustProjectType,
        lib_names: Vec<String>
    ) -> Result<Box<dyn Error>> {
        let c = Command::new("cargo")
            .arg("new")
            .arg(project_name)
            .arg("--bin")
            .spawn()
            .expect("Failed to create project");

        c.wait_with_output().unwrap();

        if !lib_names.is_empty() {
            if let Ok(_) = RustGenerator::handle_libs(lib_names, r_type) {
                return Ok(());
            } else {
                return Err("Failed to generate library files".into());
            }
        }

        Ok(())
    }

    fn create_virtual(
        project_name: String,
        r_type: RustProjectType,
        lib_names: Vec<String>
    ) -> Result<Box<dyn Error>> {
        Ok(())
    }
}

impl Generator for RustGenerator {
    fn create(
        &self,
        name: String,
        p_type: ProjectType,
        libs: Vec<String>
    ) -> Result<Box<dyn Error>> {
        let p_name = name.clone();
        let l_names: Vec<String> = libs.clone();
        let r_type: RustProjectType;

        //  The way a project is generated is based on how the libraries
        //  are handled. If there is only one name provided then the
        //  exe and lib cases are handled the same way.
        match p_type {
            ProjectType::Bin => {
                r_type = RustProjectType::Bin;
            }
            ProjectType::Lib => {
                r_type = RustProjectType::Lib;
            }
            ProjectType::Nested |
            ProjectType::NestedBin => {
                r_type = {
                    if l_names.is_empty() {
                        return Err("No libraries provided for combined project".into());
                    } else {
                        RustProjectType::Virtual
                    }
                };
            }
        }

        match r_type {
            RustProjectType::Bin => RustGenerator::create_bin(p_name, r_type, l_names),
            RustProjectType::Lib => RustGenerator::create_lib(p_name, r_type, l_names),
            RustProjectType::Virtual => RustGenerator::create_virtual(p_name, r_type, l_names),
        }
    }
}

use std::{
    cell::RefCell,
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{GenResult, Generator, ProjectType};

use super::RustGenerator;

fn create_crate(name: &str, init_type: &str) -> GenResult<Box<dyn Error>> {
    let res = Command::new("cargo")
        .args(vec!["new", init_type, name])
        .spawn();

    if let Err(e) = res {
        return Err(Box::new(e));
    }
    Ok(())
}

fn import_package_json(_file_path: &Path) -> GenResult<Box<dyn Error>> {
    todo!("Create package imports for the application.")
}

fn handle_libs(src_path: &Path, libs: &Vec<String>) -> GenResult<Box<dyn Error>> {
    if src_path.exists() {
        let libs = libs.clone();
        for mut lib in libs {
            let is_dir = lib.ends_with('/');
            let mut path: RefCell<PathBuf> = RefCell::new(PathBuf::new());
            //  Create directory or library file.
            if is_dir {
                path = RefCell::new(src_path.join(&lib));
                if let Err(e) = fs::create_dir_all(path.borrow().as_path()) {
                    return Err(Box::new(e));
                }

                let file_path = path.borrow().clone().join("mod.rs");
                if let Err(e) = fs::File::create(file_path) {
                    return Err(Box::new(e));
                }
            } else {
                //  Append file extension to lib string
                lib.push_str(".rs");
                //  Update path to be the full path to the file
                path = RefCell::new(src_path.join(&lib));
                //  Create the library file.
                if let Err(e) = fs::File::create(path.borrow().as_path()) {
                    return Err(Box::new(e));
                }
            }
            //  Include the new directory or crate.

            //  Add the file to mod.rs for the library.
            //  crate name
            let lib_name = path
                .borrow()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            //  path to the lib/mod.rs file
            let mut lib_rs_file = path.borrow().parent().clone().unwrap().join("mod.rs");
            //  If the mod.rs file doesn't exist look for the lib.rs file.
            if !lib_rs_file.exists() {
                let mut new_buff = lib_rs_file.parent().unwrap().to_path_buf();
                new_buff = new_buff.join("lib.rs");
                lib_rs_file = new_buff;
            }
            //  text to add to the lib/mod.rs file
            let file_str = format!("mod {};", lib_name);

            // Try to write the file_str to the lib/mod.rs file.
            if let Ok(mut f) = OpenOptions::new()
                .write(true)
                .append(true)
                .open(lib_rs_file)
            {
                if let Err(e) = f.write(file_str.as_bytes()) {
                    return Err(Box::new(e));
                }
            }
        }
        Ok(())
    } else {
        return Err("Src path for library files not found.".into());
    }
}

fn get_type_str(p_type: ProjectType) -> String {
    let init_type = match p_type {
        crate::ProjectType::Empty => "--empty".to_string(),
        crate::ProjectType::Bin => "--bin".to_string(),
        crate::ProjectType::Lib => "--lib".to_string(),
        _ => "--lib".to_string(),
    };

    init_type
}

impl Generator for RustGenerator {
    fn create_project(
        &self,
        _is_root: bool,
        root: &mut std::path::PathBuf,
        p_type: crate::ProjectType,
        p_name: &str,
        libs: &Vec<String>,
    ) -> crate::GenResult<Box<dyn std::error::Error>> {
        let src_path = &mut root.clone();
        //  Get the rust crate type from the project type.
        let init_type = &get_type_str(p_type);

        //  Try to create the crate.
        let mut res = create_crate(p_name, init_type);
        if let Err(e) = res {
            return Err(e);
        }
        //  Try to add any library files.
        res = handle_libs(src_path, libs);
        if let Err(e) = res {
            return Err(e);
        }
        Ok(())
    }
}

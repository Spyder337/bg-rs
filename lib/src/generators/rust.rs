use crate::Generator;

use super::RustGenerator;

impl Generator for RustGenerator {
    fn create_project(
        &self,
        is_root: bool,
        root: &mut std::path::PathBuf,
        p_type: crate::ProjectType,
        p_name: &str,
        libs: &Vec<String>,
    ) -> crate::GenResult<Box<dyn std::error::Error>> {
        todo!()
    }
}

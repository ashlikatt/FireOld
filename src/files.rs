use std::{path::{Path, PathBuf}, fmt::Display, fs::{self}};

use crate::{parser::{FireLocation, StructuredFireProject}, compiler::CompileException};


pub fn verify_project_format(path: &Path) -> Result<(), ProjectStructureException> {
    if !path.is_dir() { // Make sure the path is a directory
        return Err(ProjectStructureException::NotDir);
    };
    let mut src_check = path.to_path_buf();
    src_check.push("src");
    if !src_check.exists() || !src_check.is_dir() { // Make sure the src folder exists
        return Err(ProjectStructureException::NoSrc);
    };
    if src_check.read_dir().unwrap().count() == 0 { // Make sure the src folder has something in it
        return Err(ProjectStructureException::EmptySrc)
    }
    let mut target_check = path.to_path_buf();
    target_check.push("target");
    if !target_check.exists() || !target_check.is_dir() { // Make sure the target folder exists
        return Err(ProjectStructureException::NoTarget);
    };
    Ok(())
}

// Assumes that verify_project_format has previous been called on this dir
pub fn get_all_project_files(path: &Path) -> Vec<UncompiledFireFile> {
    let mut src_folder = path.to_path_buf();
    src_folder.push("src");
    let mut check_files = src_folder.read_dir().unwrap();

    let mut out = Vec::new();
    scan_load_files(&mut out, src_folder, FireLocation::new());
    out
}

fn scan_load_files(files: &mut Vec<UncompiledFireFile>, path: PathBuf, loc: FireLocation) {
    for uf in path.read_dir().unwrap() {
        if let Ok(f) = uf {
            if f.path().is_dir() {
                scan_load_files(files, f.path().to_path_buf(), loc.with(f.file_name().to_str().unwrap().to_string()));
            } else if f.path().is_file() {
                if let Some(n) = f.path().extension() {
                    if n == "fire" {
                        files.push(UncompiledFireFile::new(loc.with(f.path().file_stem().unwrap().to_str().unwrap().to_string()), f.path()));
                    };
                };
            };
        };
    };
}












// Represents a fire file with raw, uncompiled text.
#[derive(Debug)]
pub struct UncompiledFireFile {
    location: FireLocation,
    file_location: PathBuf
}
impl UncompiledFireFile {
    pub fn new(location: FireLocation, file_location: PathBuf) -> UncompiledFireFile {
        UncompiledFireFile { location, file_location }
    }
    pub fn read_all(&self) -> Result<String, CompileException> {
        if let Ok(s) = fs::read_to_string(self.file_location.clone()) {
            Ok(s)
        } else {
            Err(CompileException::NoFileAccess(&self.file_location))
        }
    }
    pub fn structure_into(self, proj: StructuredFireProject) -> Result<StructuredFireProject, CompileException<'static>> {
        unimplemented!();
    }
}

// A fire file that contains uncompiled headers. 
#[derive(Debug)]
pub struct StructuredFireFile {
    location: FireLocation
}

pub enum ProjectStructureException {
    NotDir, NoSrc, NoTarget, EmptySrc
}
impl Display for ProjectStructureException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectStructureException::NotDir => f.write_str("Project must be a directory.")?,
            ProjectStructureException::NoSrc => f.write_str("Project does not contain a src subdirectory.")?,
            ProjectStructureException::NoTarget => f.write_str("Project does not contain a target subdirectory.")?,
            ProjectStructureException::EmptySrc => f.write_str("Project src folder does not contain any modules.")?,
        };
        Ok(())
    }
}
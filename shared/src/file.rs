use walkdir::WalkDir;
use std::path::PathBuf;
use std::io::{Write, Result};
use std::fs::{self, File};

pub fn delete_file(file: &PathBuf) -> Result<()> {
    let mut handle = File::create(file)?;
    handle.write_all("".as_bytes())?;

    fs::remove_file(file)?;
    Ok(())
}

pub fn get_files(dir: String) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.path().to_path_buf())
        .collect()
}
use walkdir::WalkDir;
use std::path::PathBuf;
use std::io::{Write, Result};
use std::fs::{self, File};

pub fn delete_file(path: &PathBuf) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all("".as_bytes())?;
    fs::remove_file(path)?;

    Ok(())
}

pub fn get_files(dir: String) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|res| res.ok())
        .filter(|item| item.path().is_file())
        .map(|item| item.path().to_path_buf())
        .collect()
}

use std::env;
use std::path::PathBuf;
use std::io::{Write, Result};
use std::fs::{self, File};

use lib::crypto::decrypt;

fn main() {
    let args = env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .join(" ");

    let verbose = lib::misc::get_flag(&args, "verbose");

    let master = lib::misc::get_val(&args, "master");

    let folder = lib::misc::get_val(&args, "folder");
    let files = lib::file::get_files(folder);

    for file in &files {
        _ = process_file(
            file,
            &master
        );

        if verbose {
            println!("done for: {:?}", file);
        }
    }
}

pub fn process_file(file: &PathBuf, master: &str) -> Result<()> {
    let name = file
        .file_name()
        .map(|f| f.to_string_lossy())
        .unwrap_or_default();

    let suffix = if let Some(pos) = name.rfind(".bm.") {
        let suffix = &name[pos + 4..];
        if suffix.len() == 10 {
            suffix
        } else {
            return Ok(());
        }
    } else {
        return Ok(());
    };

    let content = fs::read(file)?;
    if content.len() < 100 {
        return Ok(());
    }

    let key = String::from_utf8_lossy(&content[..100]);
    let real_content = &content[100..];

    let full_key = format!("{}{}{}", key, suffix, master);
    let new_content = decrypt(&real_content.to_vec(), &full_key);

    let new_file = {
        let new_name = name
            .strip_suffix(&format!(".bm.{}", suffix))
            .unwrap_or(&name);

        let mut path = PathBuf::from(file);
        path.set_file_name(new_name);

        path
    };

    let mut handle = File::create(new_file)?;
    handle.write_all(&new_content)?;

    lib::file::delete_file(file)?;

    Ok(())
}
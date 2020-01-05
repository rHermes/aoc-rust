use std::io;
use std::io::Write;

use std::path::PathBuf;

use std::fs;
use std::fs::OpenOptions;

// To find cache path
use dirs;

// For getting the url
use bytes::Bytes;
use reqwest;

// For sha256 digest
use sha2::{Digest, Sha256};

// Check if we have it in cache, if not then we fetch it and write it to the file
pub async fn get_input(
    c: &reqwest::Client,
    sess: &String,
    year: i32,
    day: i32,
) -> Result<Bytes, String> {
    let hid = format!("{:x}", Sha256::digest(sess.as_bytes()));

    // Create the dir
    let base_dir: PathBuf = [
        dirs::cache_dir().unwrap().to_str().unwrap(),
        "raoc",
        hid.as_str(),
        "inputs",
    ]
    .iter()
    .collect();

    let kkk: std::io::Result<()> = fs::create_dir_all(base_dir.clone());

    if let Err(e) = kkk {
        if e.kind() != io::ErrorKind::AlreadyExists {
            return Err(e.to_string());
        }
    }

    // Create the input file path
    let input_file = base_dir.join(format!("{}-{:02}.txt", year, day));

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(input_file.clone());

    if let Err(e) = file {
        if e.kind() == io::ErrorKind::AlreadyExists {
            return match fs::read(input_file) {
                Ok(v) => Ok(Bytes::from(v)),
                Err(e) => Err(e.to_string()),
            };
        } else {
            return Err(e.to_string());
        }
    }

    let mut fi = file.unwrap();

    let ss = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let www = c.get(&ss).send().await;
    match www {
        Ok(bb) => match bb.bytes().await {
            Ok(b) => match fi.write_all(&b) {
                Ok(_) => Ok(b),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

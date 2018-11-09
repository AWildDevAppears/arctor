extern crate clap;

use clap::{Arg, App};
use std::ffi::OsStr;
use std::path::Path;

mod archiver;

fn main() {
    let matches = App::new("Arctor")
        .version("0.1.0")
        .author("Josh Burgess <jburgess@whiteoctober.co.uk>")
        .about("Achive reactor tool - many archive types, one command")
        .arg(
            Arg::with_name("FILE")
                .required(true)
        )
        .args_from_usage(
            "-x 'Extract archive'
            -t, --type==[FILE] 'Sets file type for archive'"
        )
        .get_matches();
    let path = Path::new(matches.value_of("FILE").unwrap());
    let file_type = matches.value_of("type").unwrap_or("");

    match file_type {
        "zip" => handle_zip_archive(path),
        "7zip" => handle_7zip_archive(path),
        "7z" => handle_7zip_archive(path),
        "tar" => handle_tar_archive(path),
        "gz" => handle_tar_archive(path),
        "gzip" => handle_tar_archive(path),
        _ => {
            match path.extension().and_then(OsStr::to_str) {
                Some("zip") => handle_zip_archive(path),
                _ => println!("You forgot to specify this case!"),
            };
        }
    }

    println!("{}", file_type);
}

fn handle_zip_archive(path: &Path) {
    archiver::zip::extract(path);
}

fn handle_tar_archive(path: &Path) {
    match archiver::tar::extract(path) {
        Ok(v) => v,
        Err(err) => println!("Error: {:?}", err),
    }
}

fn handle_7zip_archive(path: &Path) {
    println!("7Zip not implemented yet")
}


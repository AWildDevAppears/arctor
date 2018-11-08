extern crate clap;

use clap::{Arg, App};
use std::ffi::OsStr;

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
    let path = std::path::Path::new(matches.value_of("FILE").unwrap());
    let file_type = matches.value_of("type").unwrap_or("");

    match file_type {
        "zip" => println!("Zip not implemented yet"),
        _ => {
            match path.extension().and_then(OsStr::to_str) {
                Some("rs") => println!("Rust!"),
                _ => println!("You forgot to specify this case!"),
            };
        }
    }

    println!("{}", file_type);
}


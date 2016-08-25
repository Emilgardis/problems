//! Binary
//!


#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

use lang_interp::ToHistogram;
extern crate lang_interp;

use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::{ErrorKind, Read};

fn open_file<T: AsRef<Path>>(file_path: T) -> String {
    let mut buf = String::new();
    File::open(file_path.as_ref()).unwrap().read_to_string(&mut buf).unwrap();
    buf
}

fn main() {
    let matches = App::new("Language interpreter/guesser")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Tries to guess what language your text is written in.")
        .subcommand(SubCommand::with_name("learn")
                    .about("Teach the program about a new language")
                    .arg(Arg::with_name("language")
                         .required(true))
                    .arg(Arg::with_name("file")
                         //.min_values(1)
                         .validator(validate_txt)
                         .required(true))
                    )
        .subcommand(SubCommand::with_name("guess")
                    .about("The program will try to guess the language.")
                    .arg(Arg::with_name("file")
                         .validator(validate_txt)
                         .required(true))
                    )
        .get_matches();

    match matches.subcommand_name() {
        Some("learn") => {
            
        },
        
        _ => ()
    }
}

fn validate_txt(v: String) -> Result<(), String> {
    match File::open(&v) {
        Ok(_) => Ok(()),
        Err(e) =>
            Err(format!("{}{}",
                    match e.kind() {
                        ErrorKind::NotFound =>
                            "File not found: ",
                        ErrorKind::PermissionDenied =>
                            "No permission to open: ",
                        _ => e.description(),
                    },
                    v)
                )
    }
}

//! Binary
//!


#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

use lang_interp::{ToHistogram, Language};
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

    match matches.subcommand() {
        ("learn", Some(sub_m)) => {
            let mut buf = String::new();

            File::open(sub_m.value_of("file").unwrap())
                .unwrap_or_else(|_| panic!("Couldn't open file."))
                .read_to_string(&mut buf).unwrap();
            // Implement blacklist!
            //
            
            {
                let lang = Language { 
                    language: sub_m.value_of("language").unwrap().into(),
                    histogram: buf.to_histogram(),
                };
                lang.write_lang(format!("{}.lang", sub_m.value_of("language").unwrap()));
            }
        },
        ("guess", Some(sub_m)) => {
            let mut buf = String::new();

            File::open(sub_m.value_of("file").unwrap())
                .unwrap_or_else(|_| panic!("Couldn't open file."))
                .read_to_string(&mut buf).unwrap();
            
            let lang_test = Language::open_lang(format!("{}/assets/svenska.lang", env!("CARGO_MANIFEST_DIR")).histogram.to_ranking();
            let example = buf.to_histogram().to_ranking();
            
            println!("{}", lang_test.similarity(&example))
        } 
        _ => {
            
        }
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

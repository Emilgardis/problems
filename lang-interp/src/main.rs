#![feature(question_mark)]
//! Binary
//!

mod find_lang;

#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

pub use lang_interp::{ToNGram, Language};
extern crate lang_interp;

use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::{ErrorKind, Read, Error as IoErr};
use std::collections::BTreeMap;

fn open_file<T: AsRef<Path>>(file_path: T) -> Result<String, IoErr> {
    let mut buf = String::new();
    File::open(file_path.as_ref())?.read_to_string(&mut buf)?;
    Ok(buf)
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
            let mut buf = open_file(sub_m.value_of("file").unwrap()).unwrap();
            let mut ngrams = BTreeMap::new();
            ngrams.insert(1, buf.clone().to_ngram(1));
            ngrams.insert(2, buf.clone().to_ngram(2));
            {
                let lang = Language { 
                    language: sub_m.value_of("language").unwrap().into(),
                    ngrams: ngrams,
                };
                lang.write_lang(format!("{}.lang", sub_m.value_of("language").unwrap()));
            }
        },
        ("guess", Some(sub_m)) => {
            let mut buf = open_file(sub_m.value_of("file").unwrap()).unwrap();
            
            let langs = find_lang::find_languages(format!(
                    "{}/assets/", env!("CARGO_MANIFEST_DIR")), 1).unwrap();
            if langs.len() == 0 {
                println!("No languages found, please add \".lang\" files via the learn command\n\
                         See learn --help")
            }
            let example1 = buf.clone().to_ngram(1);
            let example2 = buf.to_ngram(2);
            println!("Calculating levhenstein fitness.");
            for lang in &langs {
                println!("{}:", lang.language);
                println!("1: {}", lang.ngrams.get(&2u8).unwrap().as_ranking()
                         .similarity(&example1.as_ranking()));
                println!("2: {}", lang.ngrams.get(&2u8).unwrap().as_ranking()
                         .similarity(&example2.as_ranking()));
            }
            let mut highest = ("", -1.0);
            println!("Calculating cosine simularity for");
            for lang in &langs {
                let mut combined: Option<f64> = Some(0.0);
                //println!("{:#?}", lang);
                println!("{}:", lang.language);
                match lang.ngrams.get(&1u8).unwrap().cos_simularity(&example1){
                    Some(r) => {
                        println!("\t1: {} %", r);
                        combined = Some(r/2.0);
                    },
                    None => {
                        println!("\t2: Not enough data.");
                        combined = None;
                    },
                }
                match lang.ngrams.get(&2u8).unwrap().cos_simularity(&example2){
                    Some(r) => {
                        println!("\t2: {} %", r);
                        if let Some(comb) = combined {
                            combined = Some(comb + r/2.0);
                        }
                    },
                    None => {
                        println!("\t2: Not enough data.");
                    },
                }
                if combined.is_some() && combined.unwrap() > highest.1 {
                    highest = (lang.language.as_ref(), combined.unwrap());
                }
            }
            println!("Best match was {:?} with {}%", highest.0, highest.1*100.0);
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

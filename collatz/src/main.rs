//! A collatz max length thing.
//!
//! # TODO
//! * Use less clones, currently, the program clones pretty 
//! much every u64 that is passed around. It is better to use references.
//! This will make the program much less memory-hungry
//! * Implement logging.
//! * Put stuff in Boxes, don't know if this will fix much of the memory usage though.
//! * Make interactive, probably with a CLI or passing flags.
//! * Make sieve and the for-loop threaded.

mod collatz;
use collatz::{CollatzSieve, Collatz};
#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

use std::error::Error;

fn validate_nums(v: String) -> Result<(), String> {
    for num in v.split_whitespace() {
        match num.parse::<f64>() {
            Ok(num_f64) => {
                if !num_f64.is_sign_positive() || num_f64 == 0.0 {
                    return Err(String::from(format!("\"{}\" is not a positive number.", num)));
                }
                if !num_f64.is_normal() || num_f64.fract() != 0.0 {
                    return Err(String::from(format!("\"{}\" is not a natural number.", num)));
                }
            },
            Err(e) => {
                return Err(String::from(format!("Couldn't parse number \"{}\": {:?}", num, e.description())));
            }
        }
    }
    return Ok(())
}

fn parse_nums(v: String) -> Vec<u64> {
    v.split_whitespace().map(|num| num.parse::<f64>().unwrap() as u64).collect()
}

fn main() {
    let matches = App::new("Collatz Sequence Generator")
                    //.title("Collatz Sequence Generator")
                    .version(crate_version!())
                    .author(crate_authors!())
                    .about("Collatz discovered this in 1937!")
                    .arg(Arg::with_name("disable-sieve")
                         .short("d")
                         .long("disable-sieve")
                         .help("Tells the program to disable the sieve.\n\
                               Ignored with `get` as it already disables it.")
                        )
                    .arg(Arg::with_name("verbose")
                         .short("v")
                         .help("Enables full output.\n\
                               Use with `--do-twos` for more complete output.")
                         )
                    .subcommand(SubCommand::with_name("bound")
                        .about("Calculate highest sequence of a number.")
                        .setting(AppSettings::AllowLeadingHyphen)
                        .arg(Arg::with_name("bound")
                            .min_values(1)
                            .max_values(2)
                                .validator(validate_nums)
                                .required(true)
                            )
                        )
                    .subcommand(SubCommand::with_name("get")
                        .about("Get the collatz sequence of given number.")
                        .setting(AppSettings::AllowLeadingHyphen)
                        .arg(Arg::with_name("number")
                            .required(true)
                            .validator(validate_nums)
                            )
                        )
                    .get_matches();

    match matches.subcommand() {
        ("bound", Some(sub_m)) => {
            let bound = parse_nums(sub_m.value_of("bound").unwrap().into());
            let range = if bound.len() == 2 {
                (bound[0]..(bound[1] + 1))
            } else {
                1..(bound[0] + 1)
            };
            println!("Calculating collatz sequences between {:?}", range);
            let mut sieve = CollatzSieve::new();
            let mut max = (1, 1); // Longest chain, length.
            for i in range {
                let mut coll =
                    if !matches.is_present("disable-sieve") {
                        Collatz::with_sieve(i, &mut sieve) 
                    } else {
                        Collatz::new(i)
                    };
                if matches.is_present("do-twos") {
                    coll.skip_twos(false);
                }
                while let Some(_) = coll.next() {
                    //print!("{},", num);
                }
                // println!("{:?}", coll);
                if coll.count > max.1 {
                    max = (i, coll.count.clone());
                }
            }

            println!("Longest chain was ({}, {}).", max.0, max.1);
            
            if !matches.is_present("disable-sieve") { 
                println!("Length of sieve is {}\n", sieve.sieve.len());
            }
        },
        ("get", Some(sub_m)) => {
            let num = parse_nums(sub_m.value_of("number").unwrap().into())[0];
            let mut coll = Collatz::new(num);
            if matches.is_present("do-twos") {
                coll.skip_twos(false);
            }
            let verbose = matches.is_present("verbose");
            if verbose {
                println!(" 0# {}", coll.orig);
            }
            while let Some(_) = coll.next() {
                if verbose {
                    println!("{:2}# {}", coll.count, coll.curr);
                }
            }
            println!("Length of collatz sequence of {}: {}", num, coll.count);
        },
        (_, _) => {
            println!("Nothing to do");
        }
    }
}

//! Our binary for collatz
extern crate collatz;
use collatz::{CollatzSieve, Collatz};
#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand, Values};

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
    Ok(())
}

fn parse_nums(v: Values) -> Vec<u64> {
    v.map(|num| num.parse::<f64>().unwrap() as u64).collect()
}

fn main() {
    let matches = App::new("Collatz Sequence Generator")
                    //.title("Collatz Sequence Generator")
                    .version(crate_version!())
                    .author(crate_authors!())
                    .about("Collatz discovered this in 1937!")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .setting(AppSettings::UnifiedHelpMessage)
                    .arg(Arg::with_name("enable-sieve")
                         .short("e")
                         .long("enable-sieve")
                         .help("Tells the program to enable the sieve.{n}\
                               Ignored with `get` as it doesn't need it.")
                        )
                    .arg(Arg::with_name("verbose")
                         .short("v")
                         .help("Enables more output.{n}\
                               Usable with `--do-twos` and `--enable-sieve` for more complete output.")
                         )
                    .arg(Arg::with_name("do-twos")
                        .long("do-twos")
                        .short("2")
                        .help("Disable check for powers of two.")
                        )
                    .subcommand(SubCommand::with_name("bound")
                        .about("Calculate highest sequence of a number.")
                        .setting(AppSettings::AllowLeadingHyphen)
                        .arg(Arg::with_name("bound")
                            .min_values(1)
                            .max_values(2)
                                .validator(validate_nums)
                                .use_delimiter(true)
                                .required(true)
                            )
                        )
                    .subcommand(SubCommand::with_name("get")
                        .about("Get the collatz sequence of given number.")
                        .setting(AppSettings::AllowLeadingHyphen)
                        .arg(Arg::with_name("number")
                            .required(true)
                            .max_values(1)
                            .validator(validate_nums)
                            )
                        )
                    .get_matches();
    
    let verbose = matches.is_present("verbose");
    let do_twos = matches.is_present("do-twos");
    match matches.subcommand() {
        ("bound", Some(sub_m)) => {
            let bound = parse_nums(sub_m.values_of("bound").unwrap());
            println!("Got: {:?}", bound);
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
                    if matches.is_present("enable-sieve") {
                        Collatz::with_sieve(i, &mut sieve) 
                    } else {
                        Collatz::new(i)
                    };
                if do_twos {
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

            if matches.is_present("enable-sieve") && matches.is_present("verbose"){ 
                println!("Length of sieve is {}\n", sieve.sieve.len());
                println!("Sieve:\n{:?}", sieve.sieve);
                println!("-----------Sieve data:\n{:?}", sieve.sieve_data);
            }
        },
        ("get", Some(sub_m)) => {
            let num = parse_nums(sub_m.values_of("number").unwrap())[0].into();
            let mut coll = Collatz::new(num);
            if do_twos {
                coll.skip_twos(false);
            }
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
        }
    }
}

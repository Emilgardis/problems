//! EULER 22
//! https://projecteuler.net/problem=22
//!
//! TODO:
//! * Should my list store the names? Takes much memory
#![feature(sort_unstable)]
extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;

quick_main!(run);

pub mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        } 
    }
}

use errors::*;

use regex::Regex;

use std::io::prelude::*;
use std::fs::File;
fn run() -> Result<()> {
    let path = std::env::args().nth(1).ok_or("Please provide a file path")?;
    let mut string = String::new();
    File::open(&path).chain_err(|| format!("while opening file '{}'", &path))?.read_to_string(&mut string)?;
    let names = NameList::new(&string)?;
    let mut sum = 0;
    for (i, name) in names.into_iter().enumerate() {
        sum += (i+1)*name.value();
    }
    println!("{}", sum);
    Ok(())
}

struct NameList<'a> {
    pub sorted: Vec<Name<'a>>,
}

impl<'a> NameList<'a> {
    /// Takes a comma separated file with qouted names
    pub fn new(source: &'a str) -> Result<NameList<'a>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r#""(.*?)""#).unwrap(); // Cannot fail
        }
        let mut vec: Vec<_> = RE.captures_iter(source).map(|ca| Name(ca.get(1).ok_or("No match for a name found").unwrap().as_str())).collect(); // handle error
        vec.sort_unstable();
        Ok(NameList {
            sorted: vec,
        })
    }
}

impl<'a> IntoIterator for NameList<'a> {
    type Item = Name<'a>;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.sorted.into_iter()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Name<'a>(pub &'a str);

impl<'a> Name<'a> {
    pub fn value(&self) -> usize {
        self.0.chars().fold(0, |v, ch| v + (ch as u8 - 64) as usize )
    }
}


#[cfg(test)]
mod tests{

    use super::*;
    static NAMES: &'static str = r#""MARY","PATRICIA","LINDA","BARBARA","ELIZABETH""#;
    

    #[test]
    fn sort_names() {
        let names = NameList::new(NAMES).unwrap();
        println!("{}", names.sorted.len());
        assert_eq!(
            names.sorted.iter().map(|name| name.0).collect::<Vec<_>>(),
            vec!["BARBARA","ELIZABETH","LINDA","MARY","PATRICIA"]
        );
    }

    #[test]
    fn get_value() {
        assert_eq!(Name("COLIN").value(), 53);
    }
}

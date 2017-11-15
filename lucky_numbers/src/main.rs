#![feature(drain_filter)]

#[macro_use]
extern crate clap;

use std::iter::Iterator;
use clap::{Arg, App};

fn validate_integer(s: String) -> Result<(), String> {
    match s.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}", e))
    }
}

fn main() {
    let matches = App::new("Lucky integer")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Computes nearest lucky integer")
        .arg(Arg::with_name("near")
             .required(true)
             .validator(validate_integer)
             .takes_value(true))
        .get_matches();
    let mut ln = LuckyIter::new(matches.value_of("near").unwrap().parse().unwrap());
    println!();
    for n in ln {
        println!("{}", n);
    }
}

pub struct LuckyIter {
    list: Vec<usize>,
    current: usize,
}

impl LuckyIter {
    fn new(max: usize) -> LuckyIter {
        LuckyIter {
            list: (1..max).collect(),
            current: 0,
        }
    }
}

impl Iterator for LuckyIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 0 {
            let mut i = 0;
            self.list.drain_filter(|x| reduce(&2, &mut i, x));
            //println!("{:?}", self.list);
            self.current = 1;
            Some(1)
        } else {
            //println!("list: {:?}, current: {}", self.list, self.current);
            if let Some(every) = self.list.get(self.current).cloned() {
                //println!("removing every: {}", every);
                let mut i = 0;
                self.list.drain_filter(|x| reduce(&every, &mut i, x));
                //println!("result {:?}\n", self.list);
                self.current += 1;
                self.list.get(self.current-1).cloned()
            } else {
                None
            } 
        }
    }
}

fn reduce(every: &usize, i: &mut usize, curr: &usize) -> bool {
    let res =  (*i+1) % every == 0;
    *i  = *i+1;
    res
}

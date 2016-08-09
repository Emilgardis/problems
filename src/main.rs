//! A collatz max length thing.
//!
use std::iter::Iterator;
use std::collections::BTreeMap;
// use std::default::Default;
// extern crate copperline as cl;


// Holds all known values that go to 1, except for powers of two. TODO: Add a closure for
// powers.
struct CollatzSieve(BTreeMap<u64, u64>, BTreeMap<u64, u64>, bool); // TODO: Make generic over all Sized and Eq

impl CollatzSieve {
    fn new(dummy: bool) -> CollatzSieve {
        CollatzSieve(BTreeMap::new(), BTreeMap::new(), dummy)
    }

    fn insert(&mut self, val: u64, orig: u64) {
        if !self.2 {
            self.0.insert(val, orig);
        }
    }
    /// Checks if this entry exists, if it does, how many steps to tumble down?
    fn get_previous(&self, val: &u64) -> Option<u64> {
        match self.0.get(val) {
            Some(ref orig) => {
                match self.1.get(orig) {
                    Some(counted) => Some(counted.clone()),
                    None => panic!("Expected to find a result."),
                }
            },
            None => None
        }
    }
}

struct Collatz<'a> {
    orig: u64,
    curr: u64,
    //walked: Vec<u64>,
    count: u64,
    sieve: &'a mut CollatzSieve,
}



impl<'a> Iterator for Collatz<'a> {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        // println!("{0}\t{1} - {1:#b}", self.orig, self.curr);
        if self.curr == 1 {
            return None;
        //} else if self.sieve.0.contains(&self.curr) {
            // idk how to do this shit. Maybe a map?
            // FIXME: Add proper count for sieved values.
        } else if self.curr.is_power_of_two() {
            // FIXME: Speed?
            // Should not cause any problems.
            self.count += self.curr.trailing_zeros() as u64 - 1;
            return None;
        } else if self.curr % 2 == 0 {
            self.count += 1;
            //self.walked.push(self.curr);
            self.sieve.insert(self.curr, self.orig);
            self.curr = self.curr / 2;
        } else {
            self.count += 1;
            //self.walked.push(self.curr);
            self.sieve.insert(self.curr, self.orig);
            self.curr = (3 * self.curr)+1; // We know from math that odd*odd + 1 is even. 
            // But that will rest for now. Add a function to do the first condition.
        }
        Some(self.curr)
    }
}

impl<'a> Collatz<'a> {
    fn new(start: u64, sieve: &'a mut CollatzSieve) -> Collatz {
        Collatz {
            orig: start,
            curr: start,
            //walked: vec![],
            count: 1, // We start don't we.
            sieve: sieve,
        }
    }
}

fn main() {
    let mut sieve = CollatzSieve::new(true);
    let mut max = (1, 1); // Longest chain, length.
    for i in 1..(1000000 as u64) {
        let mut coll = Collatz::new(i, &mut sieve);
        while let Some(_) = coll.next() {
            //print!("{},", num);
        }
        //println!("{} {}", i, coll.count);
        if coll.count > max.1 {
            max = (i, coll.count);
            // print!("::: {:?}", coll.sieve.0);
        }
    }

    println!("Longest chain was ({}, {}). \nLength of sieve is {}", max.0, max.1, sieve.0.len())
}

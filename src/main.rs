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
use std::iter::Iterator;
use std::collections::BTreeMap;
// use std::default::Default;
// extern crate copperline as cl;

/// A sieve for our number crunching
///
/// # TODO
/// * Make thread-safe (?), it may actually already be.
pub struct CollatzSieve {
    /// Holds all known values that go to 1, except for powers of two.
    /// The tuple includes `(steps, origin)`.
    pub sieve: BTreeMap<u64, (u64, u64)>,
    /// Holds the total steps required to get to 1, together with `sieve`
    /// this allows us to compute the steps required to get to 1.
    /// This is register easily by computing `total_steps` **-** `steps`
    pub sieve_data: BTreeMap<u64, u64>,
    /// Set to `true` to disable the sieve functionality.
    pub dummy: bool,
    /// Unimplemented.
    ///
    /// FIXME
    /// This will be used as a way of seeing how the `BTreeMap`s are called.
    /// This way we can probably find a better solution.
    _access_debug: BTreeMap<u64, u64>,
}// TODO: Make generic over all Sized and Eq

impl CollatzSieve {
    /// Make a new sieve. Set dummy to true to disable any actual functionality.
    pub fn new(dummy: bool) -> CollatzSieve {
        CollatzSieve {
            sieve: BTreeMap::new(),
            sieve_data: BTreeMap::new(),
            dummy: dummy,
            _access_debug: BTreeMap::new(),
        }
    }
    /// Insert data about a certain value. 
    ///
    /// # Fields
    /// * `val` is the number we are handling
    /// * `steps` is the amount of steps taken from `orig` to get to `val`
    /// * `orig` is the starting point, used as a kind of has for `sieve_data`
    pub fn insert(&mut self, val: u64, steps: u64, orig: u64) {
        if !self.dummy { 
            self.sieve.insert(val, (steps, orig));
        }
    }
    
    /// Adds `orig` to `sieve_data`. This means we now have complete knowledge of
    /// any new number we've gone throuqh.
    pub fn add_result(&mut self, orig: u64, total_steps: u64) {
        if !self.dummy { 
            self.sieve_data.insert(orig, total_steps);
        }
    }

    /// Checks if this entry exists, if it does, how many steps to tumble down?
    ///
    /// # Panics
    /// Panics if we know about a value, but not about it's `orig`. This is probably
    /// something that hinders thread-safety.
    pub fn in_sieve(&self, val: &u64) -> Option<u64> {
        if !self.dummy {
            match self.sieve.get(val) {
                Some(&(steps, orig)) => {
                    match self.sieve_data.get(&orig) {
                        Some(total_steps) => {
                            Some(total_steps - steps)
                        },
                        None => panic!("Expected to find a result."),
                    }
                },
                None => None
            }
        } else {
            None
        }
    }
}

/// Used to iterate over the sequence.
///
/// # Fields
pub struct Collatz<'a> {
    /// The starting point
    pub orig: u64,
    /// The current value we are processing
    pub curr: u64,
    //walked: Vec<u64>,
    /// The amount of steps we have taken to get to `curr`
    pub count: u64,
    /// The actual sieve we are using.
    pub sieve: &'a mut CollatzSieve,
}



impl<'a> Iterator for Collatz<'a> {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        //if self.orig == 8400511 { println!("{0}\t{1} - {1:#b} n^2: {2:?}", self.count, self.curr, self.curr.is_power_of_two()); }
        match self.sieve.in_sieve(&self.curr) {
            Some(steps) => {
                //println!("{} found {} in sieve, added {} steps", self.orig, self.curr, steps+1);
                self.count += steps+1;
                self.register();
                return None;
            },
            None => {},
        }
        if self.curr == 1 {
            self.register();
            return None;
        } else if self.curr.is_power_of_two() {
            self.count += self.curr.trailing_zeros() as u64 - 1;
            self.register();
            return None;
        } else if self.curr % 2 == 0 {
            self.count += 1;
            //self.walked.push(self.curr);
            self.sieve.insert(self.curr, self.count, self.orig);
            self.curr = self.curr / 2;
        } else {
            self.count += 1;
            //self.walked.push(self.curr);
            self.sieve.insert(self.curr, self.count, self.orig);
            self.curr = (3 * self.curr)+1; // We know from math that odd*odd + 1 is even. 
            // But that will rest for now.
        }
        Some(self.curr)
    }
}

impl<'a> Collatz<'a> {
    pub fn new(start: u64, sieve: &'a mut CollatzSieve) -> Collatz {
        Collatz {
            orig: start,
            curr: start,
            //walked: vec![],
            count: 1, // We start don't we.
            sieve: sieve,
        }
    }
    /// Called before the iterator returns `None`. Sets the appropriate values
    /// `orig` and `total_steps` to the sieve's `sieve_data`
    /// 
    /// # TODO
    /// Make this more seamless, currently calling `register` before returning `None`
    /// is registere only because I'm not sure how to implement this. Possibly with `Drop`?
    pub fn register(&mut self) {
        self.sieve.add_result(self.orig, self.count);
    }
}

fn main() {
    let mut sieve = CollatzSieve::new(true);
    let mut max = (1, 1); // Longest chain, length.
    for i in 1..(1e6 as u64 + 1) {
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

    println!("Longest chain was ({}, {}). \nLength of sieve is {}", max.0, max.1, sieve.sieve.len());
    //println!(":::\n{:?}\n:::\n{:?}", sieve.0, sieve.1);
}

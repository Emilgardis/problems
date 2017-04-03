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
use std::collections::{HashMap, BTreeMap};
// use std::default::Default;

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
    /// This is done easily by computing `total_steps` **-** `steps`
    pub sieve_data: BTreeMap<u64, u64>,
    /// Unimplemented.
    ///
    /// FIXME
    /// This will be used as a way of seeing how the `BTreeMap`s are called.
    /// This way we can probably find a better solution.
    _access_debug: BTreeMap<u64, u64>,
}// TODO: Make generic over all Sized and Eq

impl CollatzSieve {
    /// Make a new sieve.
    pub fn new() -> CollatzSieve {
        CollatzSieve {
            sieve: BTreeMap::new(),
            sieve_data: BTreeMap::new(),
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
        self.sieve.insert(val, (steps, orig));
    }
    
    /// Adds `orig` to `sieve_data`. This means we now have complete knowledge of
    /// any new number we've gone throuqh.
    pub fn add_result(&mut self, orig: u64, total_steps: u64) {
        self.sieve_data.insert(orig, total_steps);
    }

    /// Checks if this entry exists, if it does, how many steps to tumble down?
    ///
    /// # Panics
    /// Panics if we know about a value, but not about it's `orig`. This is a 
    /// problem that probably is something that hinders thread-safety.
    pub fn in_sieve(&self, val: &u64) -> Option<u64> {
        match self.sieve.get(val) {
            Some(&(steps, orig)) => {
                match self.sieve_data.get(&orig) {
                    Some(total_steps) => {
                        Some(total_steps - steps)
                    },
                    None => panic!("Expected to find a result in sieve for number {}.", orig),
                }
            },
            None => None
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
    _skip_twos: bool,
    /// The actual sieve we are using.
    pub sieve: Option<&'a mut CollatzSieve>,
}

impl<'a> Drop for Collatz<'a> {
    fn drop(&mut self) {
        self.register();
    }
}



impl<'a> Iterator for Collatz<'a> {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        //if self.orig == 8400511 { println!("{0}\t{1} - {1:#b} n^2: {2:?}", self.count, self.curr, self.curr.is_power_of_two()); }
        if let Some(steps) = self.sieve.as_ref().map_or(None, |ref mut sieve| sieve.in_sieve(&self.curr)) {
    //println!("{} found {} in sieve, added {} steps", self.orig, self.curr, steps+1);
    self.count += steps+1;
    return None;
}
        if self.curr == 1 {
            return None;
        } else if self._skip_twos && self.curr.is_power_of_two() {
            self.count += self.curr.trailing_zeros() as u64 + 0;
            return None;
        } else if self.curr % 2 == 0 {
            self.count += 1;
            //self.walked.push(self.curr);
            //self.sieve.insert(self.curr, self.count, self.orig);
            if let Some(ref mut sieve) = self.sieve { sieve.insert(self.curr, self.count, self.orig) };
            self.curr /= 2;
        } else {
            self.count += 1;
            //self.walked.push(self.curr);
            if let Some(ref mut sieve) = self.sieve { sieve.insert(self.curr, self.count, self.orig) };
            self.curr = (3 * self.curr)+1; // We know from math that odd*odd + 1 is even. 
            // But that will rest for now.
        }
        Some(self.curr)
    }
}

impl<'a> Collatz<'a> {
    pub fn with_sieve(start: u64, sieve: &'a mut CollatzSieve) -> Collatz {
        Collatz {
            orig: start,
            curr: start,
            //walked: vec![],
            _skip_twos: true,
            count: 0,
            sieve: Some(sieve),
        }
    }
    pub fn new(start: u64) -> Collatz<'a> {
        Collatz {
            orig: start,
            curr: start,
            //walked: vec![],
            _skip_twos: true,
            count: 0, // We start don't we.
            sieve: None,
        }
    }
    /// Called before the iterator returns `None`. Sets the appropriate values
    /// `orig` and `total_steps` to the sieve's `sieve_data`
    /// 
    /// # TODO
    /// Make this more seamless, currently calling `register` before returning `None`
    /// is done only because I'm not sure how to implement this. Possibly with `Drop`?
    pub fn register(&mut self) {
        if let Some(ref mut sieve) = self.sieve { sieve.add_result(self.orig, self.count) };
    }

    pub fn skip_twos(&mut self, flag: bool) {
        self._skip_twos = flag;
    }
}



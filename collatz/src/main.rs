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
fn main() {
    let mut sieve = CollatzSieve::new();
    let mut max = (1, 1); // Longest chain, length.
    for i in 1..(1000 as u64 + 1) {
        let mut coll = Collatz::with_sieve(i, &mut sieve);
        while let Some(_) = coll.next() {
            //print!("{},", num);
        }
        // println!("{:?}", coll);
        if coll.count > max.1 {
            max = (i, coll.count.clone());
        }
    }

    println!("Longest chain was ({}, {}). \nLength of sieve is {}\n",
            max.0, max.1, sieve.sieve.len());
}

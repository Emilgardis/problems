#![feature(test)]

extern crate collatz;
use collatz::{Collatz, CollatzSieve};

extern crate test;
use std::ops::Range;
use test::Bencher;

fn gen_collatz_bound_with_sieve(bound: Range<u64>) {
    let mut sieve = CollatzSieve::new(); 
    for i in bound {
        let mut coll = Collatz::with_sieve(i, &mut sieve);
        while let Some(_) = coll.next() {
            // Stuff
        }
    }
}

fn gen_collatz_bound_without_sieve(bound: Range<u64>) {
    for i in bound {
        let mut coll = Collatz::new(i);
        while let Some(_) = coll.next() {
            // stuff
        }
    }
}

#[bench]
fn collatz_1_to_1e6_with_sieve(b: &mut Bencher) {
    b.iter(|| gen_collatz_bound_with_sieve(1..(1e6 as u64)))
}
#[bench]
fn collatz_1_to_1e6_without_sieve(b: &mut Bencher) {
    b.iter(|| gen_collatz_bound_without_sieve(1..(1e6 as u64)))
}

#[bench]
fn collatz_8500411_for(b: &mut Bencher) {
    b.iter(|| { let mut collatz = Collatz::new(8500411);
        for _ in collatz {}
    })
}

#[bench]
fn collatz_8500411_while_let(b: &mut Bencher) {
    b.iter(|| { let mut collatz = Collatz::new(8500411);
        while let Some(_) = collatz.next() {}
    })
}

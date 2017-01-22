#![feature(test)]

extern crate test;
extern crate prime;

use prime::naive::sieve;

pub fn primes(limit: u64) -> Vec<sieve::Prime> {
    let sieve = sieve::ArSieve::new(limit);
    let mut result = Vec::new();
    for prime in sieve {
        result.push(prime);
    }
    result
}

#[cfg(test)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn prime_to_1e6(b: &mut Bencher) {
        b.iter(|| primes(1e6 as u64))
    }
}

#![feature(test)]

extern crate test;
extern crate prime;

pub fn primes(limit: u64) -> Vec<prime::sieve::Prime> {
    let sieve = prime::sieve::ArSieve::new(limit);
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

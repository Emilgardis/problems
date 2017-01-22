#![feature(test)]

extern crate test;
extern crate slow_primes;

pub fn primes(limit: u64) -> slow_primes::Primes {
    slow_primes::Primes::sieve(limit as usize)
}

#[cfg(test)]
mod bench {
    use super::*;
    use test::Bencher;
    use slow_primes::PrimeIterator;

    #[bench]
    fn extern_slow_primes_to_1e6(b: &mut Bencher) {
        b.iter(|| primes(1e6 as u64))
    }
}

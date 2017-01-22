//! A crate for generating primes in different fashions and ways.
//!
//! # TODO:
//! * Sieve
//! * Bruteforce
//! * Iterator with range,

pub mod naive_sieve;

pub mod naive {
    pub use naive_sieve as sieve;
}

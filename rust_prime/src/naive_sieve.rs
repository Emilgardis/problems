//! An implementation of Ar* Sieve
//!
//! # TODO
//! * Implement a future structure for the iterator.
//! * Parallelize (if possible)
use std::iter;
use std::collections::BTreeMap;
// TODO: Use proper name.
/// An iterator for primes.
pub type Prime = u64;
pub type Composite = u64;

#[derive(Debug, Clone)]
pub struct ArSieve {
    pub sieve: Box<BTreeMap<Composite, Vec<Prime>>>,
    _current: u64,
    limit: u64,
}

impl ArSieve {
    pub fn new(limit: u64) -> ArSieve {
        let sieve = Box::new(BTreeMap::new());
        ArSieve {
            sieve: sieve,
            _current: 2,
            limit: limit,
        }
    }
}

impl iter::Iterator for ArSieve {
    type Item= Prime;

    fn next(&mut self) -> Option<Prime> {
        if self._current == 2 {
            self._current += 1;
            return Some(2);
        }
        while self._current < self.limit {
            if !self.sieve.contains_key(&(self._current as Prime)) {
                self.sieve.insert((self._current * self._current) as Composite, vec![self._current as Prime]);  
                self._current += 2;
                return Some((self._current - 2) as Prime);
            } else {
                for prime in self.sieve.get(&self._current).unwrap().clone() {
                    self.sieve.entry(prime+self._current).or_insert(vec![]).push(prime);
                }
                self.sieve.remove(&self._current);
            }
            self._current += 2;
        }
        None
    }
}


#[cfg(test)]
mod sieve_tests {
    use super::ArSieve;
    #[test]
    fn it_works() {
        let mut sieve = ArSieve::new(10 as u64);
        let mut primes = vec![];
        for p in &mut sieve {
            primes.push(p);
        }
        assert_eq!(primes, vec![2,3,5,7]);
    }
}

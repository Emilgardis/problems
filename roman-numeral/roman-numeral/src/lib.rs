#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate num;
#[macro_use]
extern crate num_derive;
extern crate itertools;

use itertools::Itertools;

use std::str::FromStr;
use num::traits::{CheckedSub, CheckedMul, CheckedDiv};
use std::ops::{Add, Sub};

#[repr(u16)]
// Store as u8, u16 is uneeded.
#[derive(FromPrimitive, Ord, Eq, PartialOrd, PartialEq, Debug, Clone)]
pub enum RomanNumeral {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

impl RomanNumeral {
    /// Gives the biggest possible numeral, excluding with vinculums or ()
    ///
    /// TODO: Make into proc macro
    fn biggest(u: &usize) -> Result<RomanNumeral, ()> {
        use self::RomanNumeral::*;
        Ok(match () {
               _ if u >= &1000 => M,
               _ if u >= &500 => D,
               _ if u >= &100 => C,
               _ if u >= &50 => L,
               _ if u >= &10 => X,
               _ if u >= &5 => V,
               _ if u >= &1 => I,
               _ => return Err(()),
           })
    }

    // Really bad name-
    fn step_up(&self) -> Result<RomanNumeral, ()> {
        use self::RomanNumeral::*;
        Ok(match self {
               &I | &X | &C | &M => return Err(()),
               &V => X,
               &L => C,
               &D => M,
           })
    }
}

impl FromStr for RomanNumeral {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::RomanNumeral::*;
        Ok(match s.to_uppercase().as_str() {
               "I" => I,
               "V" => V,
               "X" => X,
               "L" => L,
               "C" => C,
               "D" => D,
               "M" => M,
               _ => return Err(()),

           })
    }
}

pub struct Roman(Vec<RomanNumeral>);

impl FromStr for Roman {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vec = vec![];

        for ch in s.chars() {
            vec.push(ch.to_string().parse()?)
        }
        Ok(Roman::new(vec))
    }
}

impl Add for Roman {
    type Output = Roman;

    fn add(self, v: Roman) -> Roman {
        unimplemented!()
    }
}

impl Sub for Roman {
    type Output = Roman;

    fn sub(self, v: Roman) -> Roman {
        let k = self.as_usize().sub(v.as_usize());
        Roman::from_usize(k).unwrap()
    }
}
impl CheckedSub for Roman {
    fn checked_sub(&self, v: &Roman) -> Option<Roman> {
        // Formatting is lost, is this acceptable?
        let k = self.as_usize().checked_sub(v.as_usize());
        match k {
            Some(0) | None => None,
            Some(res) => Roman::from_usize(res).ok(),
        }
    }
}

impl Roman {
    pub fn new(content: Vec<RomanNumeral>) -> Roman {
        Roman(content)
    }
    #[doc(hidden)]
    pub fn _inner(self) -> Vec<RomanNumeral> {
        self.0
    }
    /// Makes an roman numeral from an usize
    pub fn as_usize(&self) -> usize {
        let mut sum = 0;
        // Make an iterator over (RomanNumeral, usize)
        // where the usize is times repeated consecutively
        let mut iter = self.0
            .clone()
            .into_iter()
            .map(|e| (e, 1))
            .coalesce(|x, y| if x.0 == y.0 {
                          Ok((x.0, x.1 + y.1))
                      } else {
                          Err((x, y))
                      })
            .peekable();
        while let Some((numeral, reps)) = iter.next() {
            // Unwrap should never panic
            if iter.peek().is_none() || iter.peek().unwrap().0 < numeral {
                println!("{}", numeral.clone() as usize);
                sum += numeral as usize * reps;
            } else {
                println!("{:?} - {}",
                         iter.peek().unwrap().clone().0 as usize,
                         numeral.clone() as usize);
                let (n_next, n_reps) = iter.next().unwrap();
                sum += (n_next as usize * n_reps) - (numeral as usize * reps);
            }
        }
        sum
    }

    pub fn as_string(&self) -> Result<String, ::std::fmt::Error> {
        use std::fmt::Write;
        let mut buf = String::new();
        for numeral in &self.0 {
            write!(buf, "{:?}", numeral)?;
        }
        Ok(buf)
    }

    // Condenses into subtractions
    pub fn condense(&mut self) {
        // TODO: Consider if we should condense with/to double subtraction?
        // FIXME: DONT USE STRINGS!
        let mut stred = self.as_string().unwrap();

        let mut res = stred
            .replace("VIIII", "IX")
            .replace("DCCCC", "CM")
            .replace("LXXXX", "XC")
            .replace("CCCC", "CD")
            .replace("XXXX", "XL")
            .replace("IIII", "IV");
        self.0 = res.parse::<Roman>().unwrap().0
    }

    pub fn expand(&mut self) {
        let mut stred = self.as_string().unwrap();

        let mut res = stred
            .replace("IV", "IIII")
            .replace("XL", "XXXX")
            .replace("CD", "CCCC")
            .replace("XC", "LXXXX")
            .replace("CM", "DCCCC")
            .replace("IX", "VIIII");

        self.0 = res.parse::<Roman>().unwrap().0

    }

    pub fn from_usize(u: usize) -> Result<Roman, ()> {
        let mut vec = vec![];
        let mut rest = u;
        while rest != 0 {
            let res = RomanNumeral::biggest(&rest)?;
            println!("{:?}", res);
            rest -= res.clone() as usize;
            vec.push(res);

        }
        Ok(Roman::new(vec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::RomanNumeral::*;
    #[test]
    fn iiii() {
        let n = Roman::new(vec![I, I, I, I]);
        assert_eq!(4, n.as_usize());
    }

    #[test]
    fn iv() {
        let n = Roman::new(vec![I, V]);
        assert_eq!(4, n.as_usize());
    }
    #[test]
    fn ordering() {
        assert!(I < X);
        assert!(X <= L);
        assert!(X != C);
        assert!(C < D);

    }

    #[test]
    fn cdxcix() {
        let n = Roman::new(vec![C, D, X, C, I, X]);
        assert_eq!(499, n.as_usize());
    }

    #[test]
    fn cccclxxxxviiii() {
        let n = Roman::new(vec![C, C, C, C, L, X, X, X, X, V, I, I, I, I]);
        assert_eq!(499, n.as_usize());
    }

    #[test]
    fn iix() {
        let n = Roman::new(vec![I, I, X]);
        assert_eq!(8, n.as_usize());
    }

    #[test]
    fn xiix() {
        let n = Roman::new(vec![X, I, I, X]);
        assert_eq!(18, n.as_usize());
    }

    #[test]
    fn from_str() {
        let n = Roman::from_str("MMXVII").unwrap();
        assert_eq!(2017, n.as_usize());
    }
    #[test]
    fn from_usize() {
        let n = Roman::from_usize(2017).unwrap();
        assert_eq!(2017, n.as_usize());
    }

    #[test]
    fn condense_499() {
        let mut n = Roman::new(vec![C, C, C, C, L, X, X, X, X, V, I, I, I, I]);
        n.condense();
        assert_eq!(499, n.as_usize());
        assert_eq!(vec![C, D, X, C, I, X], n._inner());
    }

    #[test]
    fn condense_viiii() {
        let mut n = Roman::new(vec![V, I, I, I, I]);
        n.condense();
        assert_eq!(vec![I, X], n._inner());
    }

    #[test]
    fn condense_cccc() {
        let mut n = Roman::new(vec![C, C, C, C]);
        n.condense();
        assert_eq!(vec![C, D], n._inner());
    }

    #[test]
    fn expand() {
        let mut n = Roman::new(vec![C, D, X, C, I, X]);
        n.expand();
        assert_eq!(vec![C, C, C, C, L, X, X, X, X, V, I, I, I, I], n._inner());
    }
}

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate num;
#[macro_use]
extern crate num_derive;
extern crate itertools;
#[macro_use]
extern crate error_chain;

mod errors;
use errors::*;

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
    fn biggest(u: &usize) -> Result<RomanNumeral> {
        use self::RomanNumeral::*;
        Ok(match () {
               _ if u >= &1000 => M,
               _ if u >= &500 => D,
               _ if u >= &100 => C,
               _ if u >= &50 => L,
               _ if u >= &10 => X,
               _ if u >= &5 => V,
               _ if u >= &1 => I,
               _ => return Err(ErrorKind::NoZeroNumeral.into()),
           })
    }

    // Really bad name-
    fn step_up(&self) -> Result<RomanNumeral> {
        use self::RomanNumeral::*;
        Ok(match self {
               &I | &X | &C | &M => return Err(format!("Cannot step_up from {:?}", self).into()),
               &V => X,
               &L => C,
               &D => M,
           })
    }
}

impl FromStr for RomanNumeral {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use self::RomanNumeral::*;
        Ok(match s.to_uppercase().as_str() {
               "I" => I,
               "V" => V,
               "X" => X,
               "L" => L,
               "C" => C,
               "D" => D,
               "M" => M,
               _ => return Err(format!("No roman numeral corresponding to {}", s).into()),

           })
    }
}

#[derive(Clone)]
pub struct Roman(Vec<RomanNumeral>);

impl FromStr for Roman {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut vec = vec![];
        if s.len() == 0 {
            return Err(ErrorKind::NoZeroNumeral.into());
        }
        for ch in s.chars() {
            vec.push(ch.to_string().parse().chain_err(|| "While parsing roman numeral")?)
        }
        Ok(Roman::new(vec).chain_err(|| "While making new roman numeral after parsing")?)
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
        let k = self.as_usize().unwrap().sub(v.as_usize().unwrap());
        Roman::from_usize(k).unwrap()
    }
}
impl CheckedSub for Roman {
    fn checked_sub(&self, v: &Roman) -> Option<Roman> {
        // Formatting is lost, is this acceptable?
        let k = self.as_usize().unwrap().checked_sub(if let Ok(v_) = v.as_usize() {
            v_
        } else {
            return None;
        });
        match k {
            Some(0) | None => None,
            Some(res) => Roman::from_usize(res).ok(),
        }
    }
}

impl Roman {
    pub fn new(content: Vec<RomanNumeral>) -> Result<Roman> {
        Roman::validate(content).chain_err(|| "While validating the sequence of roman numerals")
    }
    #[doc(hidden)]
    pub fn _inner(&self) -> &Vec<RomanNumeral> {
       &self.0
    }
    /// Makes an roman numeral from an usize
    pub fn as_usize(&self) -> Result<usize> {
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
                sum += numeral as usize * reps;
            } else {
                let (n_next, n_reps) = iter.next().unwrap(); // Doesn't panic.
                sum += (n_next as usize * n_reps).checked_sub((numeral as usize * reps)).ok_or::<Error>("Underflow error".into())?;
            }
        }
        Ok(sum)
    }

    fn validate(vec: Vec<RomanNumeral>) -> Result<Roman> {
        let ro = Roman(vec);
        ro.as_usize()?;
        let stred = ro.as_string()?;
        // TODO: Check for invalid Sequences like LXC
        //
        // LXC Is invalid because
        // L = 50
        // XC = 90 (100 - 10)
        //let mut iter = ro._inner()
        //    .clone()
        //    .into_iter()
        //    .map(|e| (e, 1))
        //    .coalesce(|x, y| if x.0 == y.0 {
        //                  Ok((x.0, x.1 + y.1))
        //              } else {
        //                  Err((x, y))
        //              })
        //    .peekable();
        //let mut prev: Option<& (RomanNumeral, usize)> = None;
        //let mut current: (RomanNumeral, usize) = iter.next().unwrap(); // Shouldn't panic


        // Check for invalid 2 five*10^n numerals.
        if stred.contains("VV") {
            Err(ErrorKind::InvalidSequence("VV").into())
        } else if stred.contains("LL") { 
            Err(ErrorKind::InvalidSequence("LL").into())
        } else if stred.contains("DD") {
            Err(ErrorKind::InvalidSequence("DD").into())
        } else {
            Ok(ro)
        }
    } 
    pub fn as_string(&self) -> Result<String> {
        use std::fmt::Write;
        let mut buf = String::new();
        for numeral in &self.0 {
            write!(buf, "{:?}", numeral)?;
        }
        Ok(buf)
    }

    // Condenses into subtractions
    pub fn condense(&mut self) -> Result<()> {
        // TODO: Consider if we should condense with/to double subtraction?
        // FIXME: DONT USE STRINGS!
        let stred = self.as_string().unwrap();

        let res = stred
            .replace("IIIIII", "VI") // FIXME: Add other for 10 and 100
            .replace("VIIII", "IX")
            .replace("DCCCC", "CM")
            .replace("LXXXX", "XC")
            .replace("CCCC", "CD")
            .replace("XXXX", "XL")
            .replace("IIII", "IV");

        self.0 = res.parse::<Roman>().chain_err(|| "After condensing")?.0;
        Ok(())
    }

    pub fn expand(&mut self) -> Result<()> {
        let stred = self.as_string()?;
        println!("{}", stred);
        let res = stred
            .replace("IV", "IIII")
            .replace("XL", "XXXX")
            .replace("CD", "CCCC")
            .replace("XC", "LXXXX")
            .replace("CM", "DCCCC")
            .replace("IX", "VIIII");

        self.0 = res.parse::<Roman>().chain_err(|| "After expanding")?.0;
        Ok(())

    }

    pub fn from_usize(u: usize) -> Result<Roman> {
        let mut vec = vec![];
        let mut rest = u;
        while rest != 0 {
            let res = RomanNumeral::biggest(&rest)?;
            rest -= res.clone() as usize;
            vec.push(res);

        }
        Ok(Roman::new(vec)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::RomanNumeral::*;
    #[test]
    fn iiii() {
        let n = Roman::new(vec![I, I, I, I]).unwrap();
        assert_eq!(4, n.as_usize().unwrap());
    }

    #[test]
    fn iv() {
        let n = Roman::new(vec![I, V]).unwrap();
        assert_eq!(4, n.as_usize().unwrap());
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
        let n = Roman::new(vec![C, D, X, C, I, X]).unwrap();
        assert_eq!(499, n.as_usize().unwrap());
    }

    #[test]
    fn cccclxxxxviiii() {
        let n = Roman::new(vec![C, C, C, C, L, X, X, X, X, V, I, I, I, I]).unwrap();
        assert_eq!(499, n.as_usize().unwrap());
    }

    #[test]
    fn iix() {
        let n = Roman::new(vec![I, I, X]).unwrap();
        assert_eq!(8, n.as_usize().unwrap());
    }

    #[test]
    fn xiix() {
        let n = Roman::new(vec![X, I, I, X]).unwrap();
        assert_eq!(18, n.as_usize().unwrap());
    }

    #[test]
    fn from_str() {
        let n = Roman::from_str("MMXVII").unwrap();
        assert_eq!(2017, n.as_usize().unwrap());
    }
    #[test]
    fn from_usize() {
        let n = Roman::from_usize(2017).unwrap();
        assert_eq!(2017, n.as_usize().unwrap());
    }

    #[test]
    fn condense_499() {
        let mut n = Roman::new(vec![C, C, C, C, L, X, X, X, X, V, I, I, I, I]).unwrap();
        n.condense().unwrap();
        assert_eq!(499, n.as_usize().unwrap());
        assert_eq!(&vec![C, D, X, C, I, X], n._inner());
    }

    #[test]
    fn condense_viiii() {
        let mut n = Roman::new(vec![V, I, I, I, I]).unwrap();
        n.condense().unwrap();
        assert_eq!(&vec![I, X], n._inner());
    }

    #[test]
    fn condense_cccc() {
        let mut n = Roman::new(vec![C, C, C, C]).unwrap();
        n.condense().unwrap();
        assert_eq!(&vec![C, D], n._inner());
    }

    #[test]
    fn expand() {
        let mut n = Roman::new(vec![C, D, X, C, I, X]).unwrap();
        n.expand().unwrap();
        assert_eq!(&vec![C, C, C, C, L, X, X, X, X, V, I, I, I, I], n._inner());
    }
    #[test]
    fn sixteen() {
        let mut n = Roman::new(vec![X, I, I, I, I, I, I]).unwrap();
        n.condense().unwrap();
        assert_eq!(&vec![X, V, I], n._inner());
    }
}

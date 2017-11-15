//! EULER 145
//! https://projecteuler.net/problem=145
//! 
//! A reversible number n has the property that the sum [n + reverse(n)] only includes odd digits.

extern crate num;

use num::{Integer, NumCast};
use num::traits::CheckedMul;

fn main() {
    let limit: usize = ::std::env::args().nth(1).expect("Please provide a number for the limit").parse::<f64>().unwrap() as usize;
    let mut n = 0;
    for i in 11..limit {
        if i % 10 == 0 {
            continue;
        }
        if (i + i.reverse().unwrap()).is_odd_digits().unwrap() {
            //print!("{},", i);
            n += 1;
        }
    }
    //if !n.is_multiple_of(&2) {
    //    panic!("Error, n can't be uneven");
    //}
    println!("{}", n);
}

pub trait IntegerExt: Integer {
    fn reverse(&self) -> Result<Self, &'static str>;
    fn push_back(&mut self, other: &Self) -> Result<(), &'static str>;
    fn is_odd_digits(&self) -> Result<bool, &'static str>;
}

impl<T> IntegerExt for T where T: Integer + NumCast + CheckedMul + Copy {
    fn reverse(&self) -> Result<T, &'static str> {
        let mut i: usize = NumCast::from(*self).ok_or("Number is to large for usize")?;
        let mut res;

        {
            let (qu, rem) = i.div_rem(&10);
            i = qu;
            res = rem;
        }
        //let mut place = 10;
        loop {
            match i.div_rem(&10) {
                (0, rem) => {
                    res.push_back(&rem)?;
                    break;
                }
                (qu, rem) => {
                    res.push_back(&rem)?;
                    i = qu;
                }
            }
        }
        Ok(NumCast::from(res).ok_or("value doesn't fit inside T")?)
    }
    /// Pushes a digit to the ones place.
    fn push_back(&mut self, other: &T) -> Result<(), &'static str> {
        *self = self.checked_mul(&NumCast::from(10).unwrap()).ok_or("Value doesn't fit")? + other.clone(); 
        Ok(())
    }
    fn is_odd_digits(&self) -> Result<bool, &'static str> {
        let mut i: usize = NumCast::from(*self).ok_or("Number is to large for usize")?;

        loop {
            match i.div_rem(&10) {
                (0, rem) => {
                    if rem.is_multiple_of(&2) {
                        return Ok(false);
                    } else {
                        return Ok(true);
                    }
                }
                (qu, rem) => {
                    if rem.is_multiple_of(&2) {
                        return Ok(false);
                    }
                    i = qu;
                }
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(123.reverse().unwrap(), 321);
        assert_eq!(987654321.reverse().unwrap(), 123456789);
    }
    
    #[test]
    fn test_is_odd_digits() {
        assert!(13579.is_odd_digits().unwrap());
        assert!(99.is_odd_digits().unwrap());
        assert!(!929.is_odd_digits().unwrap());
    }
}

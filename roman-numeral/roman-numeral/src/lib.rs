#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate num;
#[macro_use]
extern crate num_derive;

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

pub struct Roman(Vec<RomanNumeral>);

impl Roman {
    pub fn new(content: Vec<RomanNumeral>) -> Roman {
        Roman(content)
    }
    // Currently does addition and ones subtraction,
    //
    // TODO: Multiple subtraction.
    pub fn as_usize(&self) -> usize {
        let mut sum = 0;
        let mut iter = self.0.clone().into_iter().peekable();
        while let Some(numeral) = iter.next() {

            if iter.peek().is_none() || iter.peek().unwrap() <= &numeral {
                // Unwrap should never panic
                println!("{}", numeral.clone() as usize);
                sum += numeral as usize;
            } else {

                println!("{} - {}",
                         iter.peek().unwrap().clone() as usize,
                         numeral.clone() as usize);
                //while iter.peek().is_some() && iter.peek().unwrap() >
                sum += iter.next().unwrap() as usize - numeral as usize;
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

    pub fn condense(&mut self) -> Roman {
        unimplemented!()
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

}

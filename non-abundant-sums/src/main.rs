/// EULER 23
/// https://projecteuler.net/problem=23
///
/// Find the sum of all the positive integers which cannot be written as the sum of two abundant
/// numbers.
///
/// TODO: 
/// * Make into library, could be useful.
/// * Implement Aliquot for all positive integer types. One thing to notice though is if the
///   value of an abundant number can overflow what is holding it. Actually I don't think it can.
///   Maybe the values inside the enum should be the offset.
/// * Add 'num' crate for traits.
/// 
use std::collections::BTreeSet;
fn main() {
    let limit = 28123;

    let abundant_numbers: Vec<usize> = (1usize..limit).filter(|v| v.is_abundant()).collect();
    println!("Got abundant numbers");
    println!("All abundant numbers: {:?}", abundant_numbers);
    //println!("{:?}", abundant_numbers);
    // Find sum of all integers that cannot be written as the sum of two abundant_numbers.
    // Sum of the two numbers has to be under under our 'limit'
     
    let mut sums = BTreeSet::new();

    'outer: for x in abundant_numbers.iter().rev() {
        for y in &abundant_numbers {
            if x+y > limit {
                continue 'outer;
            }
            sums.insert(x+y);
        }
    }
    println!("Calculated sums");
    println!("all numbers that can be expressed as the sum of two abundant numbers under 28123: {:?}", sums);
    
    let unchecked = (1usize..limit).collect::<BTreeSet<_>>();
    let all = unchecked.difference(&sums).collect::<Vec<_>>();
    println!("numbers that can't: {:?}", all);
    println!("{}", all.into_iter().fold(0, ::std::ops::Add::add))
}

pub trait Divisors {
    //fn divisors_buffed(&self, buf: &mut [u64]) -> usize;
    fn divisors(&self) -> Vec<usize>;
    fn divisors_sum(&self) -> usize;
}


pub trait Aliquot {
    fn aliquot_sum(&self) -> AliquotSum;
    
    fn is_perfect(&self) -> bool {
        match self.aliquot_sum() {
            AliquotSum::Perfect => true,
            _ => false,
        }
    }

    fn is_deficient(&self) -> bool {
        match self.aliquot_sum() {
            AliquotSum::Deficient(_) => true,
            _ => false
        }
    }

    fn is_abundant(&self) -> bool {
        match self.aliquot_sum() {
            AliquotSum::Abundant(_) => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum AliquotSum {
    Perfect,
    Deficient(usize),
    Abundant(usize),
}

//impl<T> Aliquot for T where T: Divisors {
impl Aliquot for usize {
    fn aliquot_sum(&self) -> AliquotSum {
        let sum = self.divisors_sum();
        match sum {
            _ if sum == *self => AliquotSum::Perfect,
            _ if sum < *self => AliquotSum::Deficient(sum),
            _ if sum > *self => AliquotSum::Abundant(sum),
            _ => unreachable!()
        }
    }
}
impl Divisors for usize {
    fn divisors(&self) -> Vec<usize> {
        if *self == 0 {
            return vec![];
        }
        let mut vec = vec![1];
        // TODO: Can we optimise this for even numbers
        // TODO: Use some kind of seive if number is lare enough
        // TODO: Use suggestions from https://stackoverflow.com/q/3545259/4284367
        // FIXME: Use sqrt
        for i in 2..(*self/2 + 1) {
            if self % i == 0 {
                vec.push(i);
            }
        }
        vec
    }

    fn divisors_sum(&self) -> usize {
        if *self == 0 {
            return 0;
        }
        let mut sum = 1;
        // TODO: Can we optimise this for even numbers
        // TODO: Use some kind of seive if number is lare enough
        // TODO: Use suggestions from https://stackoverflow.com/q/3545259/4284367
        // FIXME: Use sqrt
        for i in 2..(*self/2 + 1) {
            if self % i == 0 {
                sum += i;
            }
        }
        sum

    }
} 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_perfect() {
        println!("{:?}", 28usize.divisors());
        assert!(28usize.is_perfect());
    }

    #[test]
    fn prime_is_deficient() {
        println!("{:?}", 17usize.divisors());
        assert!(17usize.is_deficient());
    }
    #[test]
    fn is_abundant() {
        println!("{:?}", 12usize.divisors());
        assert!(12usize.is_abundant());
    }

}

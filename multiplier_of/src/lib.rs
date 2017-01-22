extern crate permutahedron;

use std::ops::Range;

// TODO: Better name.
pub fn get_sum_(multipliers: &[u64], max: u64) -> u64 {
    let mut sum = 0;
    for mult in multipliers {
        sum += (max-1)/mult * (mult + max) / 2;
        println!("{}", mult)
    }
    
    sum
}

#[cfg(test)]
mod multiplier_test {
    use super::*;

    #[test]
    fn multiplers_of_3_5_to_1000() {
        assert_eq!(233168, get_sum_(&[3,5], 1000));
    }
}

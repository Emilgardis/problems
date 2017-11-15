#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate roman_numeral;

use std::str::{self, FromStr};

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(s) = str::from_utf8(data) {
        if let Ok(mut num) = roman_numeral::Roman::from_str(s) {
            println!("{:?}", num._inner());
            num.as_usize().unwrap();
            num.as_string().unwrap();
            assert_ne!(num._inner().len(), 0);
            num.clone().condense().unwrap();
            num.expand().unwrap();
        }
    }
});

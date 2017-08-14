#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate garland_words;
use garland_words::Garland;
use std::str;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        if s.chars().all(|c| c.is_alphabetic()) {
            s.garland();
        }
    }
});

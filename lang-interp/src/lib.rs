#![feature(proc_macro)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod ngram;
mod language;

pub use ngram::{ToNGram, NGram};
pub use language::Language;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

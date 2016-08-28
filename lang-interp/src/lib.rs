#![feature(plugin)]
#![plugin(serde_macros)]
#![feature(custom_derive)]
extern crate serde;
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

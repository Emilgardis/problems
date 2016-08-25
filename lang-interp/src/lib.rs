#![feature(plugin)]
#![plugin(serde_macros)]
#![feature(custom_derive)]
extern crate serde;
extern crate serde_yaml;

mod histogram;
mod language;

pub use histogram::{ToHistogram, Histogram};
pub use language::Language;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

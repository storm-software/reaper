#![doc = include_str!("../README.md")]
#![deny(rust_2018_idioms, unsafe_code)]
#![allow(clippy::needless_collect)] // the implementation of that rule is way too eager, it rejects necessary collects
#![allow(clippy::derive_partial_eq_without_eq)]

pub mod cashflow;
pub mod instruments;
pub mod iso;
pub mod telemetry;
pub mod time;
pub mod trading;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}

//! [Problem 014](https://projecteuler.net/problem=14)

#[macro_use(problem)]
extern crate euler_lib;

use euler_lib::problems::set_one::{longest_collatz_seq};

fn compute(n: u64) -> u64 {
  longest_collatz_seq(n)
}

fn solve() -> String {
  compute(1_000_000).to_string()
}

problem!(&837_799.to_string(), solve);

#[cfg(test)]
mod tests {
  #[test]
  fn p014_example() {
    // The example provides the sequence for thirteen as an example, though nine
    // has the longest sequence for numbers <= thirteen.
    assert_eq!(9, super::compute(13));
  }

  #[test]
  fn p014_solution() {
    assert_eq!(837_799, super::compute(1_000_000));
  }
}

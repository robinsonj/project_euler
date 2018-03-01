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

problem!(&1.to_string(), solve);

#[cfg(test)]
mod tests {
  #[test]
  fn p014_example() {
    assert_eq!(10, super::compute(13));
  }
}

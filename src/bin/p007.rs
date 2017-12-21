//! [Problem 007](https://projecteuler.net/problem=7)

#[macro_use(problem)]
extern crate euler_lib;

use euler_lib::problems::set_one::{nth_prime};

fn compute(n: u64) -> u64 {
  nth_prime(n)
}

fn solve() -> String {
  compute(10_001).to_string()
}

problem!(&104_743.to_string(), solve);

#[cfg(test)]
mod tests {
  #[test]
  fn p007_example() {
    assert_eq!(13, super::compute(6));
  }

  #[test]
  fn p007_solution() {
    assert_eq!(104_743, super::compute(10_001));
  }
}

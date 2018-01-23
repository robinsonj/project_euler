//! [Problem 010](https://projecteuler.net/problem=10)

#[macro_use(problem)]
extern crate euler_lib;

use euler_lib::problems::set_one::{sum_of_primes};

fn compute(n: u64) -> u64 {
  sum_of_primes(n)
}

fn solve() -> String {
  compute(2_000_000).to_string()
}

problem!(&142_913_828_922u64.to_string(), solve);

#[cfg(test)]
mod tests {
  #[test]
  fn p010_example() {
    assert_eq!(17, super::compute(10));
  }

  #[test]
  fn p010_solution() {
    assert_eq!(142_913_828_922, super::compute(2_000_000));
  }
}

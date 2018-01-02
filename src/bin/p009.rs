//! [Problem 009](https://projecteuler.net/problem=9)

#[macro_use(problem)]
extern crate euler_lib;

use euler_lib::problems::set_one::{pythagorean_triplets};

fn compute(n: u64) -> u64 {
  for (a, b, c) in pythagorean_triplets(n) {
    return a * b * c
  }

  0
}

fn solve() -> String {
  compute(1_000).to_string()
}

problem!(&31_875_000.to_string(), solve);

#[cfg(test)]
mod tests {
  #[test]
  fn p009_example() {
    assert_eq!(60, super::compute(12));
  }

  #[test]
  fn p009_solution() {
    assert_eq!(31_875_000, super::compute(1_000));
  }
}

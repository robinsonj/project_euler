//! [Problem 006](https://projecteuler.net/problem=6)

#[macro_use(problem)]
extern crate euler_lib;

use euler_lib::problems::set_one::{sum_square_difference};

fn compute(n: u64) -> u64 {
  sum_square_difference(n)
}

fn solve() -> String {
  compute(100).to_string()
}

problem!(&25_164_150.to_string(), solve);

#[cfg(test)]
mod tests {
  #[test]
  fn p006_example() {
    assert_eq!(2_640, super::compute(10));
  }

  #[test]
  fn p006_solution() {
    assert_eq!(25_164_150, super::compute(100));
  }
}

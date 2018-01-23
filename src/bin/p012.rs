//! [Problem 012](https://projecteuler.net/problem=12)

#[macro_use(problem)]
extern crate euler_lib;

use euler_lib::problems::set_one::{divis_triangular_number};

fn compute(n: u64) -> u64 {
  divis_triangular_number(n)
}

fn solve() -> String {
  compute(500).to_string()
}

problem!(&76_576_500.to_string(), solve);

#[cfg(test)]
mod tests {
  #[test]
  fn p012_example() {
    assert_eq!(28, super::compute(5));
  }

  #[test]
  fn p012_solution() {
    assert_eq!(76_576_500, super::compute(500));
  }
}

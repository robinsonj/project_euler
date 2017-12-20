//! [Problem 5](https://projectuler.net/problem=5)

#[macro_use(problem)]
extern crate euler_lib;

use euler_lib::problems::set_one::{smallest_multiple};

fn compute(n: u64) -> u64 {
  smallest_multiple(n)
}

fn solve() -> String {
  compute(20).to_string()
}

problem!("232792560", solve);

#[cfg(test)]
mod tests {
  #[test]
  fn p005_example() {
    assert_eq!(2520, super::compute(10));
  }
}

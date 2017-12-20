//! [Problem 5](https://projectuler.net/problem=5)

#[macro_use(problem)]
extern crate euler_lib;

use euler_lib::problems::set_one::{smallest_multiple};

fn compute(n: &[u64]) -> u64 {
  smallest_multiple(n)
}

fn solve() -> String {
  let bases: Vec<u64> = (2..20).collect();
  compute(bases.as_slice()).to_string()
}

problem!(&232_792_560.to_string(), solve);

#[cfg(test)]
mod tests {
  #[test]
  fn p005_example() {
    let bases: Vec<u64> = (2..10).collect();
    assert_eq!(2520, super::compute(bases.as_slice()));
  }

  #[test]
  fn p005_solution() {
    let bases: Vec<u64> = (2..20).collect();
    assert_eq!(232_792_560, super::compute(bases.as_slice()));
  }
}

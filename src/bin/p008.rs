//! [Problem 008](https://projecteuler.net/problem=8)

#[macro_use(problem)]
extern crate euler_lib;

use euler_lib::problems::set_one::{largest_product_in_series};
use euler_lib::problems::input::{P008};

fn compute(n: u64, s: &'static str) -> u64 {
  largest_product_in_series(n, s)
}

fn solve() -> String {
  compute(13, P008).to_string()
}

problem!(&23_514_624_000u64.to_string(), solve);

#[cfg(test)]
mod tests {
  #[test]
  fn p008_example() {
    assert_eq!(5_832, super::compute(4, super::P008));
  }

  #[test]
  fn p008_solution() {
    assert_eq!(23_514_624_000, super::compute(13, super::P008));
  }
}

/// Compute the square of the sum of all numbers 1..n.
pub fn sq_sum(n: u64) -> u64 {
  let sum = n * (n + 1) / 2;

  sum * sum
}

/// Compute the sum of the squares of all numbers 1..n.
pub fn sum_sq(n: u64) -> u64 {
  (2 * n + 1) * (n + 1) * n / 6
}

#[cfg(test)]
mod tests {
  #[test]
  fn sq_sum() {
    assert_eq!(3_025, super::sq_sum(10));
  }

  #[test]
  fn sum_sq() {
    assert_eq!(385, super::sum_sq(10));
  }
}

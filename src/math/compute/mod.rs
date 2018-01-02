/// Compute the square of the sum of all numbers 1..n.
pub fn sq_sum(n: u64) -> u64 {
  let sum = n * (n + 1) / 2;

  sum * sum
}

/// Compute the sum of the squares of all numbers 1..n.
pub fn sum_sq(n: u64) -> u64 {
  (2 * n + 1) * (n + 1) * n / 6
}

/// Compute the greatest common denominator.
pub fn gcd(a: u64, b: u64) -> u64 {
  if a == 0 || b == 0 {
    return 0
  }

  if a == b {
    return a
  }

  if a > b {
    return gcd(a - b , b)
  }

  gcd(a, b - a)
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

  #[test]
  fn gcd() {
    assert_eq!(10,  super::gcd(10,  20));
    assert_eq!(3,   super::gcd(6,   15));
    assert_eq!(1,   super::gcd(13,  20));
    assert_eq!(0,   super::gcd(0,   1));
  }
}

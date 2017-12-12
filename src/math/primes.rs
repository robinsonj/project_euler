pub fn is_prime(p: u64) -> bool {
  factors(p) == vec![p]
}

/// Generate a vector of the prime factors of a given integer.
///
/// # Arguments
///
/// * `f` - An unsigned integer to be factorized.
///
/// # Examples
///
/// ```
/// use euler_lib::math::primes::factors;
/// assert_eq!(factors(10), vec![2, 5]);
/// ```
pub fn factors(mut f: u64) -> Vec<u64> {
  let mut primes: Vec<u64> = Vec::new();
  let mut candidate: u64 = 2;

  while f > 1 {
    while f % candidate == 0 {
      primes.push(candidate);
      f /= candidate;
    }

    candidate += 1;
  }

  primes
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_prime() {
    assert_eq!(is_prime(894119), true);
    assert_eq!(is_prime(894120), false);
  }

  #[test]
  fn test_factors() {
    assert_eq!(factors(6), vec![2, 3]);
    assert_eq!(factors(8), vec![2, 2, 2]);
    assert_eq!(factors(9), vec![3, 3]);
    assert_eq!(factors(29), vec![29]);
    assert_eq!(factors(100), vec![2, 2, 5, 5]);
    assert_eq!(factors(901255), vec![5, 17, 23, 461]);
    assert_eq!(factors(93819012551), vec![11, 9539, 894119]);
  }
}

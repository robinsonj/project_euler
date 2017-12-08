/// Provide solutions to Euler problems 1-25.

use math::primes;
use math::palindromes::{is_palindrome};

#[allow(dead_code)]
fn multiples_sum(max: i64) -> i64 {
  let mut sum: i64 = 0;

  for i in 0..max {
    if i % 3 == 0 || i % 5 == 0 {
      sum += i;
    }
  }

  return sum
}

/// Sum the even numbers in the fibonacci sequence below the max.
#[allow(dead_code)]
fn even_fibs(max: i64) -> i64 {
  let mut sum: i64 = 0;
  let mut fib1 = 0;
  let mut fib2 = 1;
  let mut c = 0;

  while c < max {
    c = fib1 + fib2;
    fib1 = fib2;
    fib2 = c;

    if c % 2 == 0 {
      sum += c;
    }
  }

  return sum
}

/// Find the largest prime factor of target num.
#[allow(dead_code)]
fn largest_prime_factor(i: u64) -> u64 {
  let mut factor: u64 = 0;

  for n in 2..((i as f64).sqrt()).floor() as u64 {
    if i % n == 0 && n > factor && primes::is_prime(n) {
      factor = n;
    }
  }

  return factor
}

/// Find the largest palindrome product of two numbers in range.
#[allow(dead_code)]
fn palindrome_product(i: u64, i_max: u64, j: u64, j_max: u64) -> u64 {
  let mut largest: u64 = 0;

  for a in i..i_max {
    for b in j..j_max {
      if is_palindrome(a * b) && a * b > largest {
        largest = a * b;
      }
    }
  }

  return largest
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn problem_1() {
    assert_eq!(multiples_sum(1_000), 233_168);
  }

  #[test]
  fn problem_2() {
    assert_eq!(even_fibs(4_000_000), 4_613_732);
  }

  #[test]
  fn problem_3() {
    assert_eq!(largest_prime_factor(600_851_475_143), 6_857)
  }

  #[test]
  fn problem_4() {
    assert_eq!(palindrome_product(100, 1000, 100, 1000), 906609);
  }
}

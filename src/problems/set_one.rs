//! Provide solutions to Project Euler problems 001-025.

use math::primes;
use math::compute::{sum_sq, sq_sum};
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

  for a in (i..i_max).rev() {
    for b in (j..j_max).rev() {
      if is_palindrome(a * b) && a * b > largest {
        largest = a * b;
      }
    }
  }

  largest
}

/// Find the smallest multiple of all ints in the range arg.
pub fn smallest_multiple(bases: &[u64]) -> u64 {
  let mut f1 = primes::Set::new();

  for d in bases.iter() {
    f1.join(primes::Set::of(*d));
  }

  f1.to_int()
}

pub fn sum_square_difference(n: u64) -> u64 {
  sq_sum(n) - sum_sq(n)
}

/// Find the nth prime number.
pub fn nth_prime(n: u64) -> u64 {
  let mut count: u64 = 1;
  let mut candidate: u64 = 1;

  while count < n {
    candidate += 2;
    if primes::is_prime2(candidate) {
      count += 1;
    }
  }

  candidate
}

/// Find the largest product in the series `s` of lenth `n`.
pub fn largest_product_in_series(n: u64, s: &'static str) -> u64 {
  s
    // Get an iterator for each `char` in the string.
    .chars()

    // Convert the str chars in to numeric types where possible.
    // Non-numeric chars (anything not "0" - "9") or ignored.
    .filter_map(|c| c.to_digit(10))

    // Convert all elements in the collection to u64s.
    .map(|n| n as u64)

    // Collect the iteractor in to a vector of u64s.
    // Uses the special 'turbofish' syntax (`::<>`).
    .collect::<Vec<_>>()

    // Create an iterator over all contiguous windows in the slice.
    .windows(n as usize)

    // Transfor the iterator of windows in to a vector of the contiquous
    // products.
    //
    // `1u64` is the initial value for the accumulator `acc`.
    .map(|w| w.iter().fold(1u64, |acc, &next| acc * next))

    // Find the maximum value in the vector.
    .max()

    // Unwrap and return the max value.
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  use problems::input::{P008};

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
    assert_eq!(palindrome_product(100, 1_000, 100, 1_000), 906_609);
  }

  #[test]
  fn problem_5() {
    let bases: Vec<u64> = (2..20).collect();
    assert_eq!(smallest_multiple(bases.as_slice()), 232_792_560);
  }

  #[test]
  fn problem_6() {
    assert_eq!(25_164_150, super::sum_square_difference(100));
  }

  #[test]
  fn problem_7() {
    assert_eq!(104_743, super::nth_prime(10_001));
  }

  #[test]
  fn problem_8() {
    assert_eq!(23_514_624_000, super::largest_product_in_series(13, P008));
  }
}

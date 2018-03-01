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

/// Compute the product of an integer triplet.
pub fn tripprod(t: (u64, u64, u64)) -> u64 {
  t.0 * t.1 * t.2
}

/// Compute the sum of all whole numbers `0..n`.
pub fn gauss_sum(n: u64) -> u64 {
  (n * (n + 1)) / 2
}

/// Compute the list of all the factors of `n`.
pub fn factors(n: u64) -> Vec<u64> {
  let mut f: Vec<u64> = vec![];

  for factor in 1..((n as f64).sqrt()).floor() as u64 + 1 {
    if n % factor == 0 {
      f.push(factor);
      f.push(n / factor);
    }
  }

  f.sort();
  f
}

/// Compute the Collatz sequence of given integer `n`.
pub fn collatz(mut n: u64) -> Vec<u64> {
  let mut c: Vec<u64> = vec![];

  while n > 1 {
    c.push(n);

    if n % 2 == 0 {
      n = n / 2;
    } else {
      n = n * 3 + 1;
    }
  }

  c.push(n);

  c
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

  #[test]
  fn tripprod() {
    assert_eq!(6,   super::tripprod((1, 2, 3)));
    assert_eq!(100, super::tripprod((1, 2, 50)));
    assert_eq!(144, super::tripprod((2, 6, 12)));
  }

  #[test]
  fn gauss_sum() {
    assert_eq!(5050, super::gauss_sum(100));
    assert_eq!(5151, super::gauss_sum(101));
  }

  #[test]
  fn factors() {
    assert_eq!(vec![1, 2, 3, 6], super::factors(6));
    assert_eq!(vec![1, 2, 4, 7, 14, 28], super::factors(28));
  }

  #[test]
  fn collatz() {
    let collatz_9 = super::collatz(9);
    let collatz_13 = super::collatz(13);

    assert_eq!(collatz_9, vec![9, 28, 14, 7, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1]);
    assert_eq!(collatz_13, vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1]);
  }
}

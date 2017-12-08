pub fn is_palindrome(n: u64) -> bool {
  return n == reverse(n)
}

fn reverse(mut n: u64) -> u64 {
  let mut u: u64 = 0;

  while n > 0 {
    u = 10 * u + n % 10;
    n = n / 10;
  }

  return u;
}

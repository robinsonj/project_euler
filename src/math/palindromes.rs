pub fn is_palindrome(n: u64) -> bool {
  n == reverse(n)
}

fn reverse(mut n: u64) -> u64 {
  let mut u: u64 = 0;

  while n > 0 {
    u = 10 * u + n % 10;
    n = n / 10;
  }

  u
}

#[cfg(test)]
mod tests {
  #[test]
  fn is_palindrome() {
    assert_eq!(true,  super::is_palindrome(1));
    assert_eq!(true,  super::is_palindrome(121));
    assert_eq!(true,  super::is_palindrome(1000000001));

    assert_eq!(false, super::is_palindrome(010));
    assert_eq!(false, super::is_palindrome(100));
    assert_eq!(false, super::is_palindrome(123456789));
  }

  #[test]
  fn reverse() {
    assert_eq!(1,         super::reverse(1));
    assert_eq!(987654321, super::reverse(123456789));
  }
}

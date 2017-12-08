pub fn is_prime(p: u64) -> bool {
  for i in 2..(p / 2 + 1) {
    if p % i == 0 {
      return false;
    }
  }

  return true
}

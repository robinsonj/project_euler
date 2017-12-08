pub fn multiples_sum(max: i64) -> i64 {
  let mut sum: i64 = 0;

  for i in 0..max {
    if i % 3 == 0 || i % 5 == 0 {
      sum += i;
    }
  }

  return sum
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn id_1() {
    assert_eq!(multiples_sum(1000), 233168);
  }
}

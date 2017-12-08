/// Provide solutions to Euler problems 1-25.

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn id_1() {
    assert_eq!(multiples_sum(1_000), 233_168);
  }

  #[test]
  fn id_2() {
    assert_eq!(even_fibs(4_000_000), 4_613_732);
  }
}

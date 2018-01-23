pub fn less_than(bound: u64) -> Vec<bool> {
  let mut primes: Vec<bool> = (0..bound + 1).map( |n| n == 2 || n & 1 != 0).collect();
  let mut num = 3u64;

  while num * num <= bound {
    let mut j = num * num;
    while j <= bound {
      primes[j as usize] = false;
      j += num;
    }

    num += 2;
  }

  primes[2..primes.len()].to_vec()
}

pub fn list(bound: u64) -> Vec<u64> {
  less_than(bound)
    .into_iter()
    .enumerate()
    .filter_map(|(i, p)| if p { Some(i as u64 + 2) } else { None })
    .collect::<Vec<u64>>()
}

#[cfg(test)]
mod tests {
  static TEST_DATA: &'static [bool; 19] = &[true, true, false, true, false, true, false,
    false, false, true, false, true, false, false, false, true, false, true, false];

  #[test]
  fn less_than() {
    let s = super::less_than(20);

    assert_eq!(19, s.len());
    assert_eq!(TEST_DATA.to_vec(), s);
  }

  #[test]
  fn list() {
    let l20 = super::list(20);
    let l25 = super::list(25);

    assert_eq!(vec![2, 3, 5, 7, 11, 13, 17, 19], l20);
    assert_eq!(vec![2, 3, 5, 7, 11, 13, 17, 19, 23], l25);
  }
}

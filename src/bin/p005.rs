//! [Problem 5](https://projectuler.net/problem=5)

#[macro_use(problem)]
extern crate euler_lib;

fn compute(n: u64) -> u64 {
  let mut m = n;

  loop {
    let mut found: bool = true;

    for d in (2..n).rev() {
      if m % d != 0 {
        found = false;
        break;
      }
    }

    if found {
      break;
    }

    m += 1;
  }

  m
}

fn solve() -> String {
  compute(20).to_string()
}

problem!("232792560", solve);

#[cfg(test)]
mod tests {
  #[test]
  fn p005_example() {
    assert_eq!(2520, super::compute(10));
  }
}

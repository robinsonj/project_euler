use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

#[derive(Debug)]
pub struct Factors {
  num: u64,
  factors: HashMap<u64, u32>,
  factorized: bool
}

impl Factors {
  pub fn new(num: u64) -> Factors {
    Factors {
      num: num,
      factors: HashMap::new(),
      factorized: false
    }
  }

  pub fn join(&mut self, j: Factors) {
    for (i, c) in j.factors {
      match self.factors.entry(i) {
        Occupied(ref entry) if entry.get() >= &c => {
        },
        Occupied(mut entry) => {
          *entry.get_mut() = c;
        },
        Vacant(entry) => {
          entry.insert(c);
        }
      }
    }
  }

  /// Generate a vector of the prime factors of a given integer.
  pub fn factorize(&mut self) {
    let mut f: u64 = self.num;
    let mut candidate: u64 = 2;

    while f > 1 {
      while f % candidate == 0 {
        self.add(candidate);
        f /= candidate;
      }

      candidate += 1;
    }

    self.factorized = true
  }

  fn add(&mut self, i: u64) {
    let count = self.factors.entry(i).or_insert(0);
    *count += 1;
  }

  pub fn to_int(&mut self) -> u64 {
    if !&self.factorized {
      &self.factorize();
    }

    let mut num: u64 = 1;

    for (i, c) in &self.factors {
      num *= i.pow(*c);
    }

    num
  }

  pub fn to_vec(&mut self) -> Vec<u64> {
    if !&self.factorized {
      &self.factorize();
    }

    let mut factors: Vec<u64> = vec![];

    for (i, c) in &self.factors {
      for _ in 0..*c {
        factors.push(*i)
      }
    }

    factors
  }

  pub fn is_prime(&self) -> bool {
    self.factorized && self.factors.iter().len() == 1
  }
}

pub fn is_prime(p: u64) -> bool {
  let mut factors = Factors::new(p);

  factors.to_vec() == vec![p]
}

#[cfg(test)]
mod tests {
  #[test]
  fn is_prime() {
    assert_eq!(super::is_prime(894119), true);
    assert_eq!(super::is_prime(894120), false);
  }

  #[test]
  fn factors_struct_new() {
    let num = 100;
    let f = super::Factors::new(num);

    assert_eq!(f.factorized, false);
    assert_eq!(f.num, num);
    assert_eq!(f.factors.is_empty(), true);
  }

  #[test]
  fn factors_struct() {
    let mut factors = super::Factors::new(24);
    let mut f_vec = factors.to_vec();
    f_vec.sort();

    assert_eq!(factors.to_int(), 24);
    assert_eq!(f_vec, vec![2, 2, 2, 3]);
  }

  #[test]
  fn factors_struct_join() {
    let mut f1 = super::Factors::new(12);
    let mut f2 = super::Factors::new(13);
    let mut f3 = super::Factors::new(14);

    f2.factorize();
    f3.factorize();
    f1.join(f2);

    let mut f1_vec = f1.to_vec();
    f1_vec.sort();

    assert_eq!(f1_vec, vec![2, 2, 3, 13]);

    f1.join(f3);
    f1_vec = f1.to_vec();
    f1_vec.sort();

    assert_eq!(f1_vec, vec![2, 2, 3, 7, 13]);
  }
}

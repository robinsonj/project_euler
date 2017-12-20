use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

#[derive(Debug)]
pub struct Set {
  factors: HashMap<u64, u32>,
}

impl Set {
  pub fn new() -> Set {
    Set {
      factors: HashMap::new()
    }
  }

  pub fn of(f: u64) -> Set {
    let mut set = Set {
      factors: HashMap::new()
    };

    set.factorize(&mut f.clone());

    set
  }

  pub fn join(&mut self, j: Set) {
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
  pub fn factorize(&mut self, f: &mut u64) {
    let mut candidate: u64 = 2;

    while *f > 1 {
      while *f % candidate == 0 {
        self.add(candidate);
        *f /= candidate;
      }

      candidate += 1;
    }
  }

  fn add(&mut self, i: u64) {
    let count = self.factors.entry(i).or_insert(0);
    *count += 1;
  }

  pub fn to_int(&mut self) -> u64 {
    let mut num: u64 = 1;

    for (i, c) in &self.factors {
      num *= i.pow(*c);
    }

    num
  }

  pub fn to_vec(&mut self) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];

    for (i, c) in &self.factors {
      for _ in 0..*c {
        factors.push(*i)
      }
    }

    factors
  }

  pub fn is_prime(&self) -> bool {
    self.factors.iter().len() == 1
  }
}

pub fn is_prime(p: u64) -> bool {
  let mut factors = Set::of(p);

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
    let f = super::Set::new();

    assert_eq!(f.factors.is_empty(), true);
  }

  #[test]
  fn factors_struct_of() {
    let num = 100;
    let f = super::Set::of(num);

    assert_eq!(f.factors.is_empty(), false);
  }

  #[test]
  fn factors_struct() {
    let mut factors = super::Set::of(24);
    let mut f_vec = factors.to_vec();
    f_vec.sort();

    assert_eq!(factors.to_int(), 24);
    assert_eq!(f_vec, vec![2, 2, 2, 3]);
  }

  #[test]
  fn factors_struct_join() {
    let mut f1 = super::Set::of(12);
    let f2 = super::Set::of(13);
    let f3 = super::Set::of(14);

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

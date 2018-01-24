pub mod sieve;

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
    let mut set = Set::new();
    set.factorize(&mut f.clone());

    set
  }

  /// Compute the number of divisors the factored set would produce.
  ///
  /// An integers number of divisors can be calculated with the formula:
  ///   The integer `N` can be expressed with the formula:
  ///      `N = p_1^a1 * p_2^a2 * p_3^a3 * ...`
  ///   where p_n is a distinct prime number and a_n is its exponent.
  ///
  ///   The number of divisors of integer N can be calculated with:
  ///     `D = (a1 + 1) * (a2 + 1) * (a3 + 1) * ...`
  ///   where a_n are the exponents of the distinct prime factors of N.
  ///
  /// # Examples
  ///
  /// ```
  /// use euler_lib::math::primes::Set;
  ///
  /// let d1 = Set::of(28).num_divisors();
  /// let d2 = Set::of(100).num_divisors();
  ///
  /// assert_eq!(6, d1);
  /// assert_eq!(9, d2);
  /// ```
  pub fn num_divisors(self) -> u64 {
    let mut num_divisors: u64 = 1;

    for (_, c) in self.factors {
      num_divisors *= (c + 1) as u64
    }

    num_divisors
  }

  /// Join two factorization sets.
  ///
  /// This mutates the calling set and adds the set 'j' factors to itself.
  ///
  /// # Examples
  ///
  /// ```
  /// use euler_lib::math::primes::Set;
  ///
  /// let mut a = Set::of(10);
  /// let     b = Set::of(20);
  ///
  /// let mut a_vec = a.to_vec();
  /// a_vec.sort();
  ///
  /// assert_eq!(vec![2, 5], a_vec);
  ///
  /// // Join b to a.
  /// a.join(b);
  ///
  /// a_vec = a.to_vec();
  /// a_vec.sort();
  ///
  /// assert_eq!(vec![2, 2, 5], a_vec);
  /// ```
  pub fn join(&mut self, j: Set) {
    for (i, c) in j.factors {
      match self.factors.entry(i) {
        Occupied(ref entry) if entry.get() >= &c => {},
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

pub fn is_prime2(n: u64) -> bool {
  match n {
    1               => false,
    2 ... 3         => true,
    x if x % 2 == 0 => false,
    x if x < 9      => true,
    x if x % 3 == 0 => false,
    x               => {
      let r: u64 = (x as f64).sqrt() as u64;
      let mut f: u64 = 5;

      while f <= r {
        match x {
          xx if xx % f == 0       => return false,
          xx if xx % (f + 2) == 0 => return false,
          _ => { f += 6 }
        }
      }

      true
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn is_prime() {
    assert_eq!(super::is_prime(894119), true);
    assert_eq!(super::is_prime(894120), false);
  }

  #[test]
  fn is_prime2() {
    assert_eq!(super::is_prime2(894119), true);
    assert_eq!(super::is_prime2(894120), false);
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
  fn factors_num_divisors() {
    assert_eq!(6, super::Set::of(28).num_divisors());
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

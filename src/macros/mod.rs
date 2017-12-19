#[macro_export]
macro_rules! problem {
  ($answer:expr, $solver:expr) => {
    fn main() {
      $crate::solver::Solver::new($answer, $solver).run();
    }
  }
}

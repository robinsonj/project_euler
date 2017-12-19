enum SolverFunction<'a> {
  Function(fn() -> String),
  FunctionWithFile(&'a str, fn() -> String)
}

#[derive(Debug)]
pub struct SolverResult<T> {
  pub result: T,
  pub correct: bool
}

pub struct Solver<'a> {
  answer: &'a str,
  solver: SolverFunction<'a>
}

impl<'a> Solver<'a> {
  pub fn new(answer: &'a str, solver: fn() -> String) -> Solver<'a> {
    Solver {
      answer: answer,
      solver: SolverFunction::Function(solver)
    }
  }

  pub fn run(self) {
    match self.solve() {
      Ok(result) => {
        println!("Result: {:?}", result);
      },
      Err(_) => {
        println!("Something went wrong!");
      }
    }
  }

  pub fn solve(&self) -> Result<SolverResult<String>, &'static str> {
    let answer = match self.solver {
      SolverFunction::Function(solve) => solve(),
      SolverFunction::FunctionWithFile(file_name, solve) => {
        solve()
      }
    };

    let result = SolverResult {
      correct: answer == self.answer,
      result: answer
    };

    Ok(result)
  }
}

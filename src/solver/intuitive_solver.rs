use crate::types::{SudokuIntuitiveSolveResult, CellPosition, SolutionStep};
use crate::solver::Solver;

mod naked_singles;
mod hidden_singles;
mod thermo_steps;

impl Solver {
  pub fn intuitive_solve(&mut self) -> SudokuIntuitiveSolveResult {
    let mut empty_cell_count = self.constraints.grid_size.pow(2) as u32 - self.constraints.fixed_numbers.len() as u32;

    println!("{}", empty_cell_count);

    let mut full_solution = true;
    let mut no_solution = false;
    let mut steps: Vec<SolutionStep> = vec![];
    while empty_cell_count > 0 {
      if self.is_cell_with_no_candidates() {
        full_solution = false;
        no_solution = true;
        break;
      }

      let step = self.find_step_raw();
      if step.is_none() {
        full_solution = false;
        break
      }

      let step = step.unwrap();
      let pos = &step.cells[0];
      let CellPosition { row, col } = *pos;
      let value = step.values[0];

      self.grid[row][col] = value;
      empty_cell_count -= 1;

      println!("{} {} {} {:?}", row, col, value, step.rule);

      steps.push(step);
    }

    let res = SudokuIntuitiveSolveResult {
      full_solution,
      no_solution,
      solution: self.grid.to_vec(),
      steps,
    };
    res
  }

  fn find_step_raw(&self) -> Option<SolutionStep> {
    let step = self.find_naked_singles();
    if step.is_some() {
      return step
    }

    let step = self.find_hidden_singles();
    if step.is_some() {
      return step
    }

    let step = self.find_thermo_steps();
    if step.is_some() {
      return step
    }

    // TODO: implement other rules

    None
  }

  fn is_cell_with_no_candidates(&self) -> bool {
    for row in 0..self.constraints.grid_size {
      for col in 0..self.constraints.grid_size {
        if self.grid[row][col] == 0 {
          let cell_candidates = self.compute_cell_candidates(row, col);
          if cell_candidates.is_empty() {
            return true
          }
        }
      }
    }

    false
  }
}

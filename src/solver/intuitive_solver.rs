use crate::types::{SudokuIntuitiveSolveResult, CellPosition, SolutionStep};
use crate::solver::Solver;

mod naked_singles;
mod hidden_singles;
mod thermo_steps;

impl Solver {
  pub fn intuitive_solve(&mut self) -> SudokuIntuitiveSolveResult {
    let mut empty_cell_count = self.constraints.grid_size.pow(2) as u32 - self.constraints.fixed_numbers.len() as u32;

    println!("{}", empty_cell_count);

    let mut solution_count = 1;
    let mut steps: Vec<SolutionStep> = vec![];
    while empty_cell_count > 0 {
      let step = self.find_step_raw();
      if step.is_none() {
        solution_count = 0;
        break
      }

      let step = step.unwrap();
      let pos = &step.cells[0];
      let CellPosition { row, col } = *pos;
      let value = step.values[0];

      self.grid[row][col] = value;
      empty_cell_count -= 1;

      println!("{} {} {}", row, col, value);

      steps.push(step);
    }

    let res = SudokuIntuitiveSolveResult {
      solution_count,
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
}

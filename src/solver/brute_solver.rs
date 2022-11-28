use std::collections::HashSet;
use crate::solver::Solver;
use crate::types::{SudokuBruteSolveResult, Grid};

impl Solver {
  pub fn brute_solve(&mut self, use_intuition: bool) -> SudokuBruteSolveResult {
    let mut solution_count = 0;
    self.recursive_check(&mut solution_count, use_intuition);

    let res = SudokuBruteSolveResult {
      solution_count,
      solution: Some(self.solution.as_ref().unwrap().to_vec()),
    };
    res
  }

  pub fn recursive_check(&mut self, solution_count: &mut u32, use_intuition: bool) {
    let mut best_row = usize::MAX;
    let mut best_col = usize::MAX;
    let mut best_candidates: HashSet<u32> = HashSet::new();

    let mut original_grid: Option<Grid> = None;
    if use_intuition {
      original_grid = Some(self.grid.to_vec());
      let result = self.intuitive_solve();
      self.grid = result.solution;
    }

    for row in 0..self.constraints.grid_size {
      for col in 0..self.constraints.grid_size {
        if self.grid[row][col] == 0 {
          let cell_candidates = self.compute_cell_candidates(row, col);
          if best_row == usize::MAX || cell_candidates.len() < best_candidates.len() {
            best_row = row;
            best_col = col;
            best_candidates = cell_candidates;
          }
        }
      }
    }

    if best_row == usize::MAX {
      self.solution = Some(self.grid.to_vec());
      *solution_count += 1;
      if use_intuition {
        self.grid = original_grid.unwrap();
      }
      return
    }

    for value in &best_candidates {
      self.grid[best_row][best_col] = *value;
      self.recursive_check(solution_count, use_intuition);
      self.grid[best_row][best_col] = 0;
      if *solution_count > 1 {
        break
      }
    }

    if use_intuition {
      self.grid = original_grid.unwrap();
    }
  }
}

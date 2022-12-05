use std::collections::HashSet;
use crate::solver::Solver;
use crate::types::{SudokuBruteSolveResult, Grid};

impl Solver {
  pub fn brute_solve(&mut self, use_intuition: bool) -> SudokuBruteSolveResult {
    // Not sure if there is value in running it without intuition
    assert!(use_intuition);

    let mut solution_count = 0;
    self.recursive_check(&mut solution_count, use_intuition, 1);

    let res = SudokuBruteSolveResult {
      solution_count,
      solution: if let Some(grid) = &self.solution { Some(grid.to_vec()) } else { None },
    };
    res
  }

  pub fn recursive_check(&mut self, solution_count: &mut u32, use_intuition: bool, depth: u32) {
    let mut best_row = usize::MAX;
    let mut best_col = usize::MAX;
    let mut best_candidates: HashSet<u32> = HashSet::new();

    let mut original_grid: Option<Grid> = None;
    if use_intuition {
      original_grid = Some(self.grid.to_vec());
      // No need to store candidates, we will recompute
      // Storing would complicate things and we need to do it for each candidate at each depth
      self.candidates_active = false;
      let result = self.intuitive_solve();
      if result.no_solution {
        self.grid = original_grid.unwrap();
        return
      }
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
    } else if !best_candidates.is_empty() {
      for value in best_candidates.into_iter() {
        self.grid[best_row][best_col] = value;

        // Currently we only run the solver with intuition and because of
        // rules that restrict candidates to valid ones we know <value> is valid
        self.recursive_check(solution_count, use_intuition, depth + 1);

        self.grid[best_row][best_col] = 0;
        if *solution_count > 1 {
          break
        }
      }
    }

    if use_intuition {
      self.grid = original_grid.unwrap();
    }
  }
}

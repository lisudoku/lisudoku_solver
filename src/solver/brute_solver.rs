use std::collections::HashSet;
use crate::solver::Solver;
use crate::types::{SudokuBruteSolveResult, Grid, SolutionType, CellPosition};

impl Solver {
  pub fn brute_solve(&mut self, use_logical: bool) -> SudokuBruteSolveResult {
    // Not sure if there is value in running it without logical
    assert!(use_logical);

    let mut solution_count = 0;
    self.recursive_check(&mut solution_count, use_logical, 1);

    let res = SudokuBruteSolveResult {
      solution_count,
      solution: if let Some(grid) = &self.solution { Some(grid.to_vec()) } else { None },
    };
    res
  }

  pub fn recursive_check(&mut self, solution_count: &mut u32, use_logical: bool, depth: u32) {
    let mut best_cell: Option<CellPosition> = None;
    let mut best_candidates: HashSet<u32> = HashSet::new();

    let mut original_grid: Option<Grid> = None;
    if use_logical {
      original_grid = Some(self.grid.to_vec());
      // No need to store candidates, we will recompute
      // Storing would complicate things and we need to do it for each candidate at each depth
      self.candidates_active = false;
      let result = self.logical_solve();
      if result.solution_type == SolutionType::None {
        self.grid = original_grid.unwrap();
        return
      }
    }

    for cell in self.get_all_empty_cells() {
      let cell_candidates = self.compute_cell_candidates(&cell);
      if best_cell.is_none() || cell_candidates.len() < best_candidates.len() {
        best_cell = Some(cell);
        best_candidates = cell_candidates;
      }
    }

    if best_cell.is_none() {
      self.solution = Some(self.grid.to_vec());
      *solution_count += 1;
    } else if !best_candidates.is_empty() {
      for value in best_candidates.into_iter() {
        let CellPosition { row: best_row, col: best_col } = best_cell.unwrap();
        self.grid[best_row][best_col] = value;

        // Currently we only run the solver with logical and because of
        // rules that restrict candidates to valid ones we know <value> is valid
        self.recursive_check(solution_count, use_logical, depth + 1);

        self.grid[best_row][best_col] = 0;
        if *solution_count > 1 {
          break
        }
      }
    }

    if use_logical {
      self.grid = original_grid.unwrap();
    }
  }
}

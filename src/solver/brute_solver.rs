use crate::solver::Solver;
use crate::solver::types::CandidateSet;
use crate::types::{SudokuBruteSolveResult, Grid, SolutionType, CellPosition};

impl Solver {
  pub fn brute_solve(&mut self, use_logical: bool) -> SudokuBruteSolveResult {
    // Not sure if there is value in running it without logical
    assert!(use_logical);

    let mut solution_count = 0;
    let mut solution: Option<Grid> = None;
    self.recursive_check(1, use_logical, &mut solution_count, &mut solution);

    let res = SudokuBruteSolveResult {
      solution_count,
      solution,
    };
    res
  }

  fn recursive_check(&mut self, depth: u32, use_logical: bool, solution_count: &mut u32, solution: &mut Option<Grid>) {
    let mut best_cell: Option<CellPosition> = None;
    let mut best_candidates = CandidateSet::new();

    let mut original_grid: Option<Grid> = None;
    if use_logical {
      original_grid = Some(self.grid.clone());
      // No need to store candidates, we will recompute
      // Storing would complicate things and we need to do it for each candidate at each depth
      self.candidates_active = false;
      let result = self.logical_solve();
      if result.solution_type == SolutionType::None {
        self.grid = original_grid.unwrap();
        return
      }
    }

    for cell in self.all_empty_cells() {
      let cell_candidates = self.compute_cell_candidates(&cell);
      if best_cell.is_none() || cell_candidates.len() < best_candidates.len() {
        best_cell = Some(cell);
        best_candidates = cell_candidates;
      }
    }

    if best_cell.is_none() {
      *solution = Some(self.grid.clone());
      *solution_count += 1;
    } else if !best_candidates.is_empty() {
      for value in best_candidates.into_iter() {
        let CellPosition { row: best_row, col: best_col } = best_cell.unwrap();
        self.grid[best_row][best_col] = value;

        // Currently we only run the solver with logical and because of
        // rules that restrict candidates to valid ones we know <value> is valid
        self.recursive_check(depth + 1, use_logical, solution_count, solution);

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

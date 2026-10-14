use crate::solver::Solver;
use crate::solver::types::CandidateSet;
use crate::types::{SudokuBruteSolveResult, Grid, SolutionType, CellPosition};
use rand::seq::SliceRandom;

pub struct BruteSolveSettings {
  pub use_logic: bool,
  pub randomize: bool,
  pub solution_count_limit: Option<u32>,
}

impl Solver {
  pub fn brute_solve(&mut self, settings: BruteSolveSettings) -> SudokuBruteSolveResult {
    let mut solution_count = 0;
    let mut solution: Option<Grid> = None;
    self.recursive_check(1, &settings, &mut solution_count, &mut solution);

    let res = SudokuBruteSolveResult {
      solution_count,
      solution,
    };
    res
  }

  fn recursive_check(&mut self, depth: u32, settings: &BruteSolveSettings, solution_count: &mut u32, solution: &mut Option<Grid>) {
    let mut best_cell: Option<CellPosition> = None;
    let mut best_candidates = CandidateSet::new();

    let mut original_grid: Option<Grid> = None;
    if settings.use_logic {
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

    let cells = self.all_empty_cells();
    // TODO: try cells with most empty peers?
    for cell in cells {
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
      let mut values: Vec<u32> = best_candidates.into_iter().collect();
      if settings.randomize {
        values.shuffle(&mut rand::rng());
      }
      for value in values {
        let CellPosition { row: best_row, col: best_col } = best_cell.unwrap();
        self.grid[best_row][best_col] = value;

        // Currently we only run the solver with logical and because of
        // rules that restrict candidates to valid ones we know <value> is valid
        self.recursive_check(depth + 1, settings, solution_count, solution);

        self.grid[best_row][best_col] = 0;

        if *solution_count >= settings.solution_count_limit.unwrap_or(2) {
          break
        }
      }
    }

    if settings.use_logic {
      self.grid = original_grid.unwrap();
    }
  }
}

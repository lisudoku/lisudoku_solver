use crate::{solver::{SolveState, logical::logical_solver::LogicalSolver, types::CandidateSet}, types::{CellPosition, Grid, SolutionType, SudokuBruteSolveResult, SudokuConstraints}};
use rand::seq::SliceRandom;

pub struct BruteSolveSettings {
  pub use_logic: bool,
  pub randomize: bool,
  pub solution_count_limit: Option<u32>,
}

pub struct BruteSolver {
  state: SolveState,
  // TODO: add settings
}

impl BruteSolver {
  pub fn new(input_constraints: SudokuConstraints) -> BruteSolver {
    BruteSolver{
      state: SolveState::new(input_constraints),
    }
  }

  pub fn solve(&mut self, settings: BruteSolveSettings) -> SudokuBruteSolveResult {
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
      let mut logical_solver = LogicalSolver::from_solve_state(self.state.clone());
      let result = logical_solver.solve();
      if result.solution_type == SolutionType::None {
        return
      }
      original_grid = Some(self.state.grid.clone());
      self.state.grid = result.solution.unwrap();
    }

    let cells = self.state.all_empty_cells();
    for cell in cells {
      let cell_candidates = self.state.compute_cell_candidates(&cell);

      if best_cell.is_none() || cell_candidates.len() < best_candidates.len() {
        best_cell = Some(cell);
        best_candidates = cell_candidates;
      }
    }

    if best_cell.is_none() {
      *solution = Some(self.state.grid.clone());
      *solution_count += 1;
    } else if !best_candidates.is_empty() {
      let mut values: Vec<u32> = best_candidates.into_iter().collect();
      if settings.randomize {
        values.shuffle(&mut rand::rng());
      }
      for value in values {
        let CellPosition { row: best_row, col: best_col } = best_cell.unwrap();
        self.state.grid[best_row][best_col] = value;

        // Currently we only run the solver with logical and because of
        // rules that restrict candidates to valid ones we know <value> is valid
        self.recursive_check(depth + 1, settings, solution_count, solution);

        self.state.grid[best_row][best_col] = 0;

        if *solution_count >= settings.solution_count_limit.unwrap_or(2) {
          break
        }
      }
    }

    if settings.use_logic {
      self.state.grid = original_grid.unwrap();
    }
  }
}

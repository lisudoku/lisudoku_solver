extern crate console_error_panic_hook;

use types::SolutionType;
use wasm_bindgen::prelude::*;
use crate::{solver::{SolveState, GridCheckResult, brute::{BruteSolveSettings, BruteSolver}, logical::logical_solver::LogicalSolver}, types::{Grid, SudokuBruteSolveResult, SudokuConstraints, SudokuLogicalSolveResult}};

pub mod solver;
pub mod types;

#[wasm_bindgen]
pub fn wasm_check_solved(constraints: SudokuConstraints, grid: Grid) -> GridCheckResult {
  // panic::set_hook(Box::new(console_error_panic_hook::hook));
  // Try this instead
  console_error_panic_hook::set_once();

  let solver = SolveState::from_constraints_and_grid(constraints, grid);
  solver.check_solved()
}

#[wasm_bindgen]
pub fn wasm_logical_solve(constraints: SudokuConstraints) -> SudokuLogicalSolveResult {
  // panic::set_hook(Box::new(console_error_panic_hook::hook));
  // Try this instead
  console_error_panic_hook::set_once();

  let mut solver = LogicalSolver::new(constraints).with_enriched_steps(true);
  solver.solve()
}

#[wasm_bindgen]
pub fn wasm_brute_solve(constraints: SudokuConstraints) -> SudokuBruteSolveResult {
  // panic::set_hook(Box::new(console_error_panic_hook::hook));
  // Try this instead
  console_error_panic_hook::set_once();

  let mut solver = BruteSolver::new(constraints);
  solver.solve(BruteSolveSettings{ use_logic: true, randomize: false, solution_count_limit: None })
}

// TODO: Try changing return type to Result<SudokuLogicalSolveResult, JsError> ?
#[wasm_bindgen]
pub fn wasm_logical_hint(constraints: SudokuConstraints) -> SudokuLogicalSolveResult {
  console_error_panic_hook::set_once();

  let mut solver = LogicalSolver::new(constraints.clone());
  let result = solver.solve();

  // Make sure there is a valid solution first
  if result.solution_type != SolutionType::Full {
    return result;
  }

  // Get the first relevant steps
  let mut solver = LogicalSolver::new(constraints.clone())
    .with_hint_mode(true);
  solver.solve()
}

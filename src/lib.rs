extern crate console_error_panic_hook;

use types::SolutionType;
use wasm_bindgen::prelude::*;
use crate::{solver::checker::SolvedState, types::{Grid, SudokuBruteSolveResult, SudokuConstraints, SudokuLogicalSolveResult}};

pub mod solver;
pub mod types;

#[wasm_bindgen]
pub fn wasm_check_solved(constraints: SudokuConstraints, grid: Grid) -> SolvedState {
  // panic::set_hook(Box::new(console_error_panic_hook::hook));
  // Try this instead
  console_error_panic_hook::set_once();

  let solver = solver::Solver::new(constraints, Some(grid));
  solver.check_solved()
}

#[wasm_bindgen]
pub fn wasm_logical_solve(constraints: SudokuConstraints) -> SudokuLogicalSolveResult {
  // panic::set_hook(Box::new(console_error_panic_hook::hook));
  // Try this instead
  console_error_panic_hook::set_once();

  let mut solver = solver::Solver::new(constraints, None);
  solver.logical_solve()
}

#[wasm_bindgen]
pub fn wasm_brute_solve(constraints: SudokuConstraints) -> SudokuBruteSolveResult {
  // panic::set_hook(Box::new(console_error_panic_hook::hook));
  // Try this instead
  console_error_panic_hook::set_once();

  let mut solver = solver::Solver::new(constraints, None);
  solver.brute_solve(true)
}

#[wasm_bindgen]
pub fn wasm_logical_hint(constraints: SudokuConstraints) -> SudokuLogicalSolveResult {
  // panic::set_hook(Box::new(console_error_panic_hook::hook));
  // Try this instead
  console_error_panic_hook::set_once();

  let mut solver = solver::Solver::new(constraints.clone(), None);
  let result = solver.logical_solve();

  // Make sure there is a valid solution first
  if result.solution_type != SolutionType::Full {
    return result;
  }

  // Get the first relevant steps
  let mut solver = solver::Solver::new(constraints.clone(), None)
    .with_hint_mode(true);
  solver.logical_solve()
}

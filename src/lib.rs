extern crate console_error_panic_hook;

use strum::IntoEnumIterator;
use types::SolutionType;
use wasm_bindgen::prelude::*;
use crate::{solver::{GridCheckResult, SolveState, brute::BruteSolver, logical::{logical_solver::LogicalSolver, techniques::TechniqueKind}}, types::{Grid, SudokuBruteSolveResult, SudokuConstraints, SudokuLogicalSolveResult}};

pub mod solver;
pub mod types;

#[wasm_bindgen(js_name = checkSolved)]
pub fn check_solved(constraints: SudokuConstraints, grid: Grid) -> GridCheckResult {
  // panic::set_hook(Box::new(console_error_panic_hook::hook));
  // Try this instead
  console_error_panic_hook::set_once();

  let solver = SolveState::from_constraints_and_grid(constraints, grid);
  solver.check_solved()
}

#[wasm_bindgen(js_name = logicalSolve)]
pub fn logical_solve(
  constraints: SudokuConstraints,
  #[wasm_bindgen(js_name = techniqueKinds)] technique_kinds: Vec<TechniqueKind>,
) -> SudokuLogicalSolveResult {
  // panic::set_hook(Box::new(console_error_panic_hook::hook));
  // Try this instead
  console_error_panic_hook::set_once();

  let mut solver = LogicalSolver::new(constraints)
    .with_techniques(technique_kinds)
    .with_enriched_steps(true);
  solver.solve()
}

#[wasm_bindgen(js_name = bruteSolve)]
pub fn brute_solve(constraints: SudokuConstraints) -> SudokuBruteSolveResult {
  // panic::set_hook(Box::new(console_error_panic_hook::hook));
  // Try this instead
  console_error_panic_hook::set_once();

  let mut solver = BruteSolver::new(constraints).with_logic();
  solver.solve()
}

// TODO: Try changing return type to Result<SudokuLogicalSolveResult, JsError> ?
#[wasm_bindgen(js_name = logicalHint)]
pub fn logical_hint(constraints: SudokuConstraints) -> SudokuLogicalSolveResult {
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

#[wasm_bindgen(js_name = availableTechniques)]
pub fn available_techniques() -> Vec<TechniqueKind> {
  TechniqueKind::iter().collect()
}

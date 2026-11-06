extern crate console_error_panic_hook;

use strum::IntoEnumIterator;
use tsify::{Ts, Tsify};
use types::SolutionType;
use wasm_bindgen::prelude::*;
use crate::{solver::{GridCheckResult, SolveState, brute::BruteSolver, logical::{logical_solver::LogicalSolver, techniques::TechniqueKind}}, types::{Grid, SudokuBruteSolveResult, SudokuConstraints, SudokuLogicalSolveResult}};

pub mod solver;
pub mod types;

#[wasm_bindgen(js_name = checkSolved)]
pub fn check_solved(constraints: Ts<SudokuConstraints>, grid: Ts<Grid>) -> Result<Ts<GridCheckResult>, JsError> {
  console_error_panic_hook::set_once();

  let solver = SolveState::from_constraints_and_grid(
    constraints.to_rust()?,
    grid.to_rust()?,
  );
  Ok(solver.check_solved().into_ts()?)
}

#[wasm_bindgen(js_name = logicalSolve)]
pub fn logical_solve(
  constraints: Ts<SudokuConstraints>,
  #[wasm_bindgen(js_name = techniqueKinds)] technique_kinds: Vec<Ts<TechniqueKind>>,
) -> Result<Ts<SudokuLogicalSolveResult>, JsError> {
  console_error_panic_hook::set_once();

  let technique_kinds = technique_kinds
    .into_iter()
    .map(|kind| kind.to_rust())
    .collect::<Result<Vec<_>, _>>()?;

  let mut solver = LogicalSolver::new(constraints.to_rust()?)
    .with_techniques(technique_kinds)
    .with_enriched_steps(true);
  Ok(solver.solve().into_ts()?)
}

#[wasm_bindgen(js_name = bruteSolve)]
pub fn brute_solve(constraints: Ts<SudokuConstraints>) -> Result<Ts<SudokuBruteSolveResult>, JsError> {
  console_error_panic_hook::set_once();

  let mut solver = BruteSolver::new(constraints.to_rust()?).with_logic();
  Ok(solver.solve().into_ts()?)
}

// TODO: Try changing return type to Result<SudokuLogicalSolveResult, JsError> ?
#[wasm_bindgen(js_name = logicalHint)]
pub fn logical_hint(constraints: Ts<SudokuConstraints>) -> Result<Ts<SudokuLogicalSolveResult>, JsError> {
  console_error_panic_hook::set_once();

  let mut solver = LogicalSolver::new(constraints.to_rust()?);
  let result = solver.solve();

  // Make sure there is a valid solution first
  if result.solution_type != SolutionType::Full {
    return Ok(result.into_ts()?);
  }

  // Get the first relevant steps
  let mut solver = LogicalSolver::new(constraints.to_rust()?)
    .with_hint_mode(true);
  Ok(solver.solve().into_ts()?)
}

#[wasm_bindgen(js_name = availableTechniques)]
pub fn available_techniques() -> Result<Vec<Ts<TechniqueKind>>, JsError> {
  Ok(
    TechniqueKind::iter()
      .map(|kind| kind.into_ts())
      .collect::<Result<Vec<_>, _>>()?
  )
}

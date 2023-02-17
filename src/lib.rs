extern crate console_error_panic_hook;
use std::panic;

use types::SolutionType;
use wasm_bindgen::prelude::*;
use gloo_utils::format::JsValueSerdeExt;

mod solver;
mod types;

#[wasm_bindgen]
pub fn wasm_check_solved(js_constraints: JsValue, js_grid: JsValue) -> JsValue {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  let constraints: types::SudokuConstraints = js_constraints.into_serde().unwrap();
  let grid: types::SudokuGrid = js_grid.into_serde().unwrap();
  let solver = solver::Solver::new(constraints, Some(grid));
  let result = solver.check_solved();
  JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn wasm_logical_solve(js_constraints: JsValue) -> JsValue {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  let constraints: types::SudokuConstraints = js_constraints.into_serde().unwrap();
  let mut solver = solver::Solver::new(constraints, None);
  let result = solver.logical_solve();
  JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn wasm_brute_solve(js_constraints: JsValue) -> JsValue {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  let constraints: types::SudokuConstraints = js_constraints.into_serde().unwrap();
  let mut solver = solver::Solver::new(constraints, None);
  let result = solver.brute_solve(true);
  JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn wasm_logical_hint(js_constraints: JsValue) -> JsValue {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  let constraints: types::SudokuConstraints = js_constraints.into_serde().unwrap();
  let mut solver = solver::Solver::new(constraints.clone(), None);
  let result = solver.logical_solve();

  // Make sure there is a valid solution first
  if result.solution_type != SolutionType::Full {
    return JsValue::from_serde(&result).unwrap();
  }

  // Get the first relevant steps
  let mut solver = solver::Solver::new(constraints.clone(), None)
    .with_hint_mode();
  let result = solver.logical_solve();

  JsValue::from_serde(&result).unwrap()
}

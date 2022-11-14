extern crate console_error_panic_hook;
use std::panic;

use wasm_bindgen::prelude::*;
use gloo_utils::format::JsValueSerdeExt;

mod solver;
mod types;

#[wasm_bindgen]
pub fn wasm_check_solved(js_constraints: JsValue, js_grid: JsValue) -> JsValue {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  let constraints: types::SudokuConstraints = js_constraints.into_serde().unwrap();
  let grid: types::SudokuGrid = js_grid.into_serde().unwrap();
  let solver = solver::Solver {
    constraints: constraints,
    grid: Some(grid),
  };
  let result = solver.check_solved();
  JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn wasm_intuitive_solve(js_constraints: JsValue) -> JsValue {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  let constraints: types::SudokuConstraints = js_constraints.into_serde().unwrap();
  let solver = solver::Solver {
    constraints: constraints,
    grid: None,
  };
  let result = solver.intuitive_solve();
  JsValue::from_serde(&result).unwrap()
}

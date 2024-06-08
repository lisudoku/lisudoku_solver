use crate::solver::Solver;
use crate::solver::logical_solver::combinations::cell_combinations_runner::CellCombinationsRunner;
use crate::types::{SolutionStep, Rule, Area, Arrow, CellPosition};
use super::combinations::cell_combination_logic::{CellCombinationLogic, CellsCacheKey};
use super::combinations::cell_combinations_runner::{State, CellCombinationsRunnerResult};
use super::technique::Technique;
use std::collections::HashMap;

// X can't be a candidate in this cell because it violates the arrow sum
pub struct ArrowCandidates;

impl Technique for ArrowCandidates {
  fn is_candidate_validity_update_step(&self) -> bool { true }
  fn get_rule(&self) -> Rule { Rule::ArrowCandidates }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }

    solver.constraints.arrows.iter().enumerate().flat_map(|(arrow_index, arrow)| {
      let cells = arrow.all_cells();

      // Running the algorithm for really long arrows will take too much time, so
      // wait for better opportunities
      if solver.count_empty_cells_in_list(&cells) > 8 {
        return vec![]
      }

      let mut arrow_combinatons_logic_factory = solver.arrow_combinatons_logic_factory.borrow_mut();
      let combination_logic = arrow_combinatons_logic_factory.create(arrow, solver);
      let mut runner = CellCombinationsRunner::new(solver, Box::new(combination_logic));
      let (valid_candidates, _) = runner.run();
      let invalid_candidates = solver.cell_candidates_diff(&cells, valid_candidates);

      invalid_candidates.into_iter().map(|(cell, invalid_values)| {
        self.build_simple_solution_step(
          invalid_values,
          vec![ Area::Arrow(arrow_index) ],
          vec![ cell ]
        )
      }).collect::<Vec<_>>()
    }).collect()
  }
}

// A little confusing since the below logic doesn't strictly apply to arrow_candidates, but also advanced candidates...
// basically everything that uses arrow_combinatons_logic_factory

pub struct ArrowCombinationLogic<'a> {
  arrow: &'a Arrow,
  cache: &'a mut HashMap<CellsCacheKey, CellCombinationsRunnerResult>,
  solver: &'a Solver,
  arrow_cells_count: usize,
  arrow_cells_sum: u32,
  circle_sum_min: u32,
  circle_sum_max: u32,
}

impl<'a> ArrowCombinationLogic<'_> {
  fn min_max_circle_sum(cells: &Vec<CellPosition>, solver: &Solver) -> (u32, u32) {
    let mut circle_sum_min: u32 = 0;
    let mut circle_sum_max: u32 = 0;

    for cell in cells {
      let value = solver.grid[cell.row][cell.col];
      // Could be improved to check actual candidates and don't repeat them
      let max_value = if value == 0 { 9 } else { value };
      let min_value = if value == 0 { 1 } else { value };
      circle_sum_min = 10 * circle_sum_min + min_value;
      circle_sum_max = 10 * circle_sum_max + max_value;
    }

    (circle_sum_min, circle_sum_max)
  }

  fn new(arrow: &'a Arrow, solver: &'a Solver, cache: &'a mut ArrowCombinationLogicCache) -> ArrowCombinationLogic<'a> {
    let (circle_sum_min, circle_sum_max) = Self::min_max_circle_sum(&arrow.circle_cells, solver);

    ArrowCombinationLogic {
      arrow,
      cache,
      solver,
      arrow_cells_count: arrow.arrow_cells.len(),
      arrow_cells_sum: 0,
      circle_sum_min,
      circle_sum_max,
    }
  }

  fn cache_key(&self) -> CellsCacheKey {
    self.solver.cells_to_cache_key(&self.cells())
  }
}

impl CellCombinationLogic for ArrowCombinationLogic<'_> {
  fn cells(&self) -> Vec<CellPosition> {
    self.arrow.all_cells()
  }

  fn is_value_valid_candidate_in_cell(&self, runner: &CellCombinationsRunner, value: u32, index: usize) -> bool {
    if index >= self.arrow_cells_count {
      // We are placing circle cells, so we only allow cells that match we correct sum
      // Note: we are assuming that circle cells are in order!

      // Note: precomputing powers didn't improve performance
      let correct_digit = self.arrow_cells_sum / 10_u32.pow(runner.cells.len() as u32 - index as u32 - 1) % 10;

      return value == correct_digit;
    }

    // We are placing arrow cells

    // Checking conflicts between this cell and previous cells is done in the runner

    // Make sure the final arrow sum will have the correct range
    // Note: tried running dp to guarantee we are moving to a valid arrow_sum, but not much difference

    let cell_count_left = self.arrow_cells_count - index - 1;

    // For the minimum assume the rest will all be 1s
    let min_arrow_sum = self.arrow_cells_sum + value + cell_count_left as u32 * 1;
    // For the maximum assume the rest will all be 9s
    let max_arrow_sum = self.arrow_cells_sum + value + cell_count_left as u32 * runner.solver.constraints.grid_size as u32;

    return max_arrow_sum >= self.circle_sum_min && min_arrow_sum <= self.circle_sum_max
  }

  fn should_check_value_conflict(&self) -> bool { true }

  fn advance_state(&mut self, _state: &mut State, value: u32, index: usize) {
    if index < self.arrow_cells_count {
      self.arrow_cells_sum += value;
    }
  }

  fn restore_state(&mut self, _state: &mut State, value: u32, index: usize) {
    if index < self.arrow_cells_count {
      self.arrow_cells_sum -= value;
    }
  }

  fn cached_result(&self) -> Option<&CellCombinationsRunnerResult> {
    self.cache.get(&self.cache_key())
  }

  fn cache_result(&mut self, result: &CellCombinationsRunnerResult) {
    self.cache.insert(self.cache_key(), result.to_owned());
  }
}

type ArrowCombinationLogicCache = HashMap<CellsCacheKey, CellCombinationsRunnerResult>;

pub struct ArrowCombinationLogicFactory {
  cache: ArrowCombinationLogicCache,
}

impl<'a> ArrowCombinationLogicFactory {
  pub fn new() -> ArrowCombinationLogicFactory {
    ArrowCombinationLogicFactory {
      cache: HashMap::new(),
    }
  }

  pub fn create(&'a mut self, arrow: &'a Arrow, solver: &'a Solver) -> ArrowCombinationLogic<'a> {
    ArrowCombinationLogic::new(arrow, solver, &mut self.cache)
  }
}

use std::cmp::{min, max};
use crate::solver::Solver;
use crate::solver::logical_solver::combinations::cell_combinations_runner::CellCombinationsRunner;
use crate::types::{Area, CellPosition, Renban, Rule, SolutionStep};
use super::combinations::cell_combination_logic::CellCombinationLogic;
use super::combinations::cell_combinations_runner::State;
use super::technique::Technique;

// X can't be a candidate in this cell because it violates the renban rule:
// it must be filled by a set of consecutive digits
pub struct RenbanCandidates;

impl Technique for RenbanCandidates {
  fn is_candidate_validity_update_step(&self) -> bool { true }
  fn get_rule(&self) -> Rule { Rule::RenbanCandidates }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }

    solver.constraints.renbans.iter().enumerate().flat_map(|(renban_index, renban)| {
      let cells = renban;

      let combination_logic = RenbanCombinationLogic::new(renban);
      let mut runner = CellCombinationsRunner::new(solver, Box::new(combination_logic));
      let (valid_candidates, _) = runner.run();
      let invalid_candidates = solver.cell_candidates_diff(&cells, valid_candidates);

      invalid_candidates.into_iter().map(|(cell, invalid_values)| {
        self.build_simple_solution_step(
          invalid_values,
          vec![ Area::Renban(renban_index) ],
          vec![ cell ]
        )
      }).collect::<Vec<_>>()
    }).collect()
  }
}

pub struct RenbanCombinationLogic<'a> {
  renban: &'a Renban,
  min_used_value: u32,
  max_used_value: u32,
}

impl<'a> RenbanCombinationLogic<'_> {
  fn new(renban: &'a Renban) -> RenbanCombinationLogic<'a> {
    RenbanCombinationLogic {
      renban,
      min_used_value: u32::MAX,
      max_used_value: 0,
    }
  }
}

impl CellCombinationLogic for RenbanCombinationLogic<'_> {
  fn cells(&self) -> Vec<CellPosition> {
    self.renban.to_vec()
  }

  fn is_value_valid_candidate_in_cell(&self, _runner: &CellCombinationsRunner, value: u32, index: usize) -> bool {
    if index >= self.renban.len() {
      // We filled in all of the renban digits and checked conditions along the way, so all good
      return true
    }

    // Check if range gaps can be filled
    let next_max_used_value = max(self.max_used_value, value);
    let next_min_used_value = min(self.min_used_value, value);
    let range = next_max_used_value - next_min_used_value + 1;
    let range_left_to_fill = range - index as u32 - 1;
    let cell_count_left = self.renban.len() - index - 1;

    return cell_count_left as u32 >= range_left_to_fill
  }

  fn should_check_value_conflict(&self) -> bool { true }

  fn advance_state(&mut self, _state: &mut State, value: u32, index: usize) {
    if index < self.renban.len() {
      // TODO: Any way to stop these from repeating as above?
      self.max_used_value = max(self.max_used_value, value);
      self.min_used_value = min(self.min_used_value, value);
    }
  }

  fn restore_state(&mut self, state: &mut State, _value: u32, index: usize) {
    if index < self.renban.len() {
      // TODO: could keep a stack, but meh... it's fast enough
      self.max_used_value = *state.used_candidates[0..index].iter().max().unwrap_or(&0);
      self.min_used_value = *state.used_candidates[0..index].iter().min().unwrap_or(&u32::MAX);
    }
  }
}

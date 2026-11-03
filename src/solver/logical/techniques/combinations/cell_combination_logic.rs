use crate::types::CellPosition;

use super::cell_combinations_runner::{State, CellCombinationsRunner, CellCombinationsRunnerResult};

pub(crate) type CellsCacheKey = Vec<(u32, u32)>;

pub(crate) trait CellCombinationLogic {
  fn cells(&self) -> Vec<CellPosition>;

  /// Checks special validity, runner checks regular peer conflicts
  fn is_value_valid_candidate_in_cell(&self, runner: &CellCombinationsRunner, value: u32, index: usize) -> bool;

  fn should_check_all_cells_in_set(&self) -> bool { false }

  fn should_check_value_conflict(&self) -> bool { false }

  fn advance_state(&mut self, _state: &mut State, _value: u32, _index: usize) {}

  fn restore_state(&mut self, _state: &mut State, _value: u32, _index: usize) {}

  fn cached_result(&self) -> Option<&CellCombinationsRunnerResult> { None }

  fn cache_result(&mut self, _result: &CellCombinationsRunnerResult) {}
}

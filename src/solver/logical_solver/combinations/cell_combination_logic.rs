use super::cell_combinations_runner::{State, CellCombinationsRunner};

pub trait CellCombinationLogic {
  fn is_value_valid_candidate_in_cell(&self, runner: &CellCombinationsRunner, value: u32, index: usize) -> bool;

  fn should_check_all_cells_in_set(&self) -> bool { false }

  fn should_check_value_conflict(&self) -> bool { false }

  fn advance_state(&mut self, _state: &mut State, _value: u32, _index: usize) {}

  fn restore_state(&mut self, _state: &mut State, _value: u32, _index: usize) {}
}

pub struct DefaultCellCombinationLogic;

impl CellCombinationLogic for DefaultCellCombinationLogic {
  fn is_value_valid_candidate_in_cell(&self, _runner: &CellCombinationsRunner, _value: u32, _index: usize) -> bool { true }
}

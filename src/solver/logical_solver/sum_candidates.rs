use crate::solver::Solver;
use crate::types::CellPosition;
use super::combinations::cell_combination_logic::CellCombinationLogic;
use super::combinations::cell_combinations_runner::{CellCombinationsRunner, State};

impl Solver {
  pub fn detect_invalid_sum_candidates(&self, cells: &Vec<CellPosition>, sum: u32) -> Vec<(CellPosition, Vec<u32>)> {
    let mut combinations_runner = CellCombinationsRunner::new(
      &self, Box::new(SumCombinationsLogic::new(cells, sum))
    );
    let (valid_candidates, _) = combinations_runner.run();
    self.cell_candidates_diff(cells, valid_candidates)
  }
}

struct SumCombinationsLogic<'a> {
  sum_left: u32,
  cells: &'a Vec<CellPosition>,
}

impl SumCombinationsLogic<'_> {
  pub fn new<'a>(cells: &'_ Vec<CellPosition>, sum: u32) -> SumCombinationsLogic {
    SumCombinationsLogic {
      cells,
      sum_left: sum,
    }
  }
}

impl CellCombinationLogic for SumCombinationsLogic<'_> {
  fn cells(&self) -> Vec<CellPosition> {
    self.cells.to_owned()
  }

  fn is_value_valid_candidate_in_cell(&self, runner: &CellCombinationsRunner, value: u32, index: usize) -> bool {
    // TODO: adapt to grid_size
    // 9 + 8 + 7 + ... + (9 - x + 1) = x * (19 - x) / 2
    let cells_left_count: u32 = runner.cells.len() as u32 - index as u32 - 1;
    let max_sum_left = cells_left_count * (19 - cells_left_count) / 2;

    if value > self.sum_left {
      return false
    }
    if self.sum_left - value > max_sum_left {
      return false
    }

    true
  }

  fn should_check_all_cells_in_set(&self) -> bool { true }

  fn advance_state(&mut self, _state: &mut State, value: u32, _index: usize) {
    self.sum_left -= value;
  }

  fn restore_state(&mut self, _state: &mut State, value: u32, _index: usize) {
    self.sum_left += value;
  }
}

use crate::{types::{CellPosition, Grid}, solver::Solver};
use std::{collections::HashSet, ops::{BitXorAssign, BitAnd}};
use super::cell_combination_logic::CellCombinationLogic;

pub struct CellCombinationsRunner<'a> {
  // Input
  pub cells: Vec<CellPosition>,
  pub solver: &'a Solver,
  combinations_logic: Box<dyn CellCombinationLogic + 'a>,
  // State
  pub state: State,
  // Output
  combinations_list: Vec<Vec<u32>>,
  valid_candidates: Vec<HashSet<u32>>,
}

pub struct State {
  pub used_candidates: Vec<u32>,
  pub temp_grid: Grid,
  // For each index a mask of previous positions that affect it
  pub affected_by: Vec<u32>,
  // For each value a mask of positions that have it
  pub used_candidates_at: Vec<u32>,
}

pub type CellCombinationsRunnerResult = (Vec<HashSet<u32>>, Vec<Vec<u32>>);

impl<'a> CellCombinationsRunner<'a> {
  pub fn new(solver: &'a Solver, combinations_logic: Box<dyn CellCombinationLogic + 'a>) -> CellCombinationsRunner<'a> {
    let cells = combinations_logic.cells();
    let cell_count = cells.len();

    let affected_by: Vec<u32> = cells.iter().enumerate().map(|(cell_index, cell)| {
      let mut mask = 0;
      for (prev_cell_index, prev_cell) in cells[0..cell_index].iter().enumerate() {
        if solver.cells_affect_eachother(prev_cell, cell) {
          mask.bitxor_assign(1 << prev_cell_index);
        }
      }
      mask
    }).collect();

    CellCombinationsRunner {
      cells,
      solver,
      combinations_logic,
      state: State {
        used_candidates: vec![ 0; cell_count ],
        temp_grid: solver.grid.to_vec(),
        affected_by,
        used_candidates_at: vec![ 0; solver.constraints.grid_size + 1 ],
      },
      valid_candidates: vec![ HashSet::new(); cell_count ],
      combinations_list: vec![],
    }
  }

  pub fn run(&mut self) -> CellCombinationsRunnerResult {
    // The cache is never cleared, so the results could become outdated.
    // However, it doesn't cause issues because the invalid combinations can be later ignored.
    if let Some(cached_result) = self.combinations_logic.cached_result() {
      return cached_result.to_owned()
    }

    self.run_recursive(0);

    let result = (self.valid_candidates.to_vec(), self.combinations_list.to_vec());

    self.combinations_logic.cache_result(&result);

    result
  }

  fn run_recursive(&mut self, index: usize) {
    if index == self.cells.len() {
      for (cell_index, &candidate) in self.state.used_candidates.iter().enumerate() {
        self.valid_candidates[cell_index].insert(candidate);
      }
      self.combinations_list.push(self.state.used_candidates.to_vec());

      return
    }

    let cell = self.cells[index].to_owned();
    let cell_prev_value = self.state.temp_grid[cell.row][cell.col];
    
    let mut prev_value_candidate = HashSet::new();
    let cell_candidates = if cell_prev_value == 0 {
      self.solver.candidates[cell.row][cell.col].iter()
    } else {
      prev_value_candidate.insert(cell_prev_value);
      prev_value_candidate.iter()
    };

    for &value in cell_candidates {
      if cell_prev_value != 0 && value != cell_prev_value {
        continue
      }

      if self.combinations_logic.should_check_all_cells_in_set() &&
         self.state.used_candidates_at[value as usize] != 0 {
        continue
      }

      if self.combinations_logic.should_check_value_conflict() {
        if self.state.affected_by[index].bitand(self.state.used_candidates_at[value as usize]) != 0 {
          continue
        }
      }

      if !self.combinations_logic.is_value_valid_candidate_in_cell(&self, value, index) {
        continue
      }

      self.state.temp_grid[cell.row][cell.col] = value;
      self.state.used_candidates[index] = value;
      self.state.used_candidates_at[value as usize].bitxor_assign(1 << index);

      self.combinations_logic.advance_state(&mut self.state, value, index);

      self.run_recursive(index + 1);

      self.state.temp_grid[cell.row][cell.col] = cell_prev_value;
      self.state.used_candidates_at[value as usize].bitxor_assign(1 << index);

      self.combinations_logic.restore_state(&mut self.state, value, index);
    }
  }
}

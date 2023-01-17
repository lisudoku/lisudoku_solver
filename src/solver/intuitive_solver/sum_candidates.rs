use crate::solver::Solver;
use crate::types::CellPosition;
use std::collections::HashSet;
use std::ops::{BitXorAssign, BitAnd};

impl Solver {
  pub fn detect_invalid_sum_candidates(&self, cells: &Vec<CellPosition>, sum: u32) -> Vec<(CellPosition, Vec<u32>)> {
    let valid_candidates = self.mark_valid_candidates(cells, sum);

    self.cell_candidates_diff(cells, valid_candidates)
  }

  fn mark_valid_candidates(&self, cells: &Vec<CellPosition>, sum: u32) -> Vec<HashSet<u32>> {
    let mut valid_candidates: Vec<HashSet<u32>> = vec![ HashSet::new(); cells.len() ];
    let mut used_candidates: Vec<u32> = vec![ 0; cells.len() ];
    let mut used_candidates_set: u32 = 0;

    self.generate_candidate_combinations(0, sum, &mut used_candidates, &mut used_candidates_set, cells, &mut valid_candidates);

    valid_candidates
  }

  fn generate_candidate_combinations(
    &self, index: usize, sum_left: u32,
    used_candidates: &mut Vec<u32>, used_candidates_set: &mut u32,
    cells: &Vec<CellPosition>, valid_candidates: &mut Vec<HashSet<u32>>
  ) {
    if index == cells.len() {
      if sum_left != 0 {
        return
      }

      for (cell_index, candidate) in used_candidates.iter().enumerate() {
        valid_candidates[cell_index].insert(*candidate);
      }

      return
    }

    // 9 + 8 + 7 + ... + (9 - x + 1) = x * (19 - x) / 2
    let cells_left_count: u32 = cells.len() as u32 - index as u32 - 1;
    let max_sum_left = cells_left_count * (19 - cells_left_count) / 2;

    let cell = cells[index];
    let candidates = &self.candidates[cell.row][cell.col];

    for value in candidates {
      if *value > sum_left {
        continue
      }
      let new_sum_left = sum_left - value;
      if new_sum_left > max_sum_left {
        continue
      }
      if used_candidates_set.bitand(1 << value) > 0 {
        continue
      }

      used_candidates[index] = *value;
      used_candidates_set.bitxor_assign(1 << value);

      self.generate_candidate_combinations(
        index + 1,
        new_sum_left,
        used_candidates,
        used_candidates_set,
        cells,
        valid_candidates
      );

      used_candidates_set.bitxor_assign(1 << value);
    }
  }
}

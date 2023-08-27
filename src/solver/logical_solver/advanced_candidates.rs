use crate::solver::Solver;
use crate::types::CellPosition;
use std::collections::{HashSet, HashMap};
use std::ops::{BitOrAssign, BitAnd};
use super::combinations::cell_combination_logic::CellsCacheKey;

pub type CellEliminationsResult = Vec<(CellPosition, Vec<u32>)>;

impl Solver {
  // Eliminate combinations that remove all candidates from any cells
  // Returns cells that need to be updated
  pub fn eliminate_combinations(&self, combinations: &Vec<Vec<u32>>, cells: &Vec<CellPosition>) -> CellEliminationsResult {
    let cache_key: CellsCacheKey = self.cells_to_cache_key(cells);
    if let Some(cached_result) = self.cell_eliminations_cache.borrow().get(&cache_key) {
      return cached_result.to_owned()
    }

    let cell_peers: Vec<Vec<CellPosition>> = cells.iter().map(|cell| {
      self.get_cell_peers(cell, true)
    }).collect();

    let gs = self.constraints.grid_size;
    let cand_sets: Vec<Vec<u32>> = (0..gs).map(|row| {
      (0..gs).map(|col| {
        self.candidates_to_set(CellPosition::new(row, col))
      }).collect()
    }).collect();

    let valid_combinations: Vec<&Vec<u32>> = combinations.iter().filter(|combination| {
      let mut changed_cells: HashMap<CellPosition, u32> = HashMap::new();
      for cell_index in 0..cells.len() {
        let cell_value = combination[cell_index];
        for peer_cell in &cell_peers[cell_index] {
          if cand_sets[peer_cell.row][peer_cell.col].bitand(1 << cell_value) != 0 {
            let entry = changed_cells.entry(*peer_cell).or_default();
            entry.bitor_assign(1 << cell_value);
          }
        }
      }
      changed_cells.into_iter().all(|(cell, candidate_updates)| {
        candidate_updates != cand_sets[cell.row][cell.col]
      })
    }).collect();

    let mut valid_candidates: Vec<HashSet<u32>> = vec![ HashSet::new(); cells.len() ];
    for combination in valid_combinations {
      for (cell_index, &candidate) in combination.iter().enumerate() {
        valid_candidates[cell_index].insert(candidate);
      }
    }

    let result = self.cell_candidates_diff(cells, valid_candidates);

    self.cell_eliminations_cache.borrow_mut().insert(cache_key, result.to_vec());

    result
  }
}

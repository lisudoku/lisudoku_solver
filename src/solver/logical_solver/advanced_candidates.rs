use crate::solver::Solver;
use crate::types::CellPosition;
use std::collections::{HashSet, HashMap};
use std::ops::{BitOrAssign, BitOr, BitAnd};

impl Solver {
  // Eliminate combinations that remove all candidates from any cells
  // Returns cells that need to be updated
  pub fn eliminate_combinations(&self, combinations: &Vec<Vec<u32>>, cells: &Vec<CellPosition>) -> Vec<(CellPosition, Vec<u32>)> {
    // TODO: cache result

    let cell_peers: Vec<Vec<CellPosition>> = cells.iter().map(|cell| {
      self.get_cell_peers(cell, true)
    }).collect();

    let gs = self.constraints.grid_size;
    let cand_sets: Vec<Vec<u32>> = (0..gs).map(|row| {
      (0..gs).map(|col| {
        self.candidates[row][col].iter().fold(0, |acc, e| {
          acc.bitor(1 << e)
        })
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

    self.cell_candidates_diff(cells, valid_candidates)
  }
}

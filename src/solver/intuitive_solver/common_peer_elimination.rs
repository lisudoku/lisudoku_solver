use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule, Area};
use std::collections::HashSet;
use itertools::Itertools;

// Cell can’t be X because it eliminates all X candidates from a region
// It is a more general version of the Locked Candidates
// For diagonal puzzles the rule applied on the is called Crossover
// For antiknight puzzles the rule is called L technique
impl Solver {
  pub fn find_common_peer_elimination(&self) -> Option<SolutionStep> {
    if !self.candidates_active {
      return None
    }

    for area in self.get_all_areas(false, false, false) {
      let cells_by_value = self.compute_cells_by_value_in_area(&area, &self.candidates);
      for (value, cells) in cells_by_value.into_iter().sorted() {
        let step = self.find_common_peer_elimination_cells_with_value(&area, cells, value);
        if step.is_some() {
          return step
        }
      }
    }

    None
  }

  pub fn find_common_peers_for_cells(&self, cells: &Vec<CellPosition>) -> Vec<CellPosition> {
    let mut peer_counts = vec![ vec![ 0; self.constraints.grid_size ]; self.constraints.grid_size ];
    for cell in cells {
      for CellPosition { row, col } in self.get_cell_peers(cell) {
        peer_counts[row][col] += 1;
      }
    }
    for &CellPosition { row, col } in cells {
      peer_counts[row][col] = 0;
    }
    let common_peers: Vec<CellPosition> = self.get_empty_area_cells(&Area::Grid)
      .into_iter()
      .filter(|&cell| peer_counts[cell.row][cell.col] == cells.len())
      .collect();

    common_peers
  }

  fn find_common_peers_for_cells_with_values(&self, cells: &Vec<CellPosition>, values: &HashSet<u32>) -> Vec<CellPosition> {
    let common_peers = self.find_common_peers_for_cells(cells);
    let common_peers_with_values: Vec<CellPosition> = self.filter_cells_with_any_candidates(
      &common_peers, &values
    );
    common_peers_with_values
  }

  fn find_common_peer_elimination_cells_with_value(&self, area: &Area, cells: Vec<CellPosition>, value: u32) -> Option<SolutionStep> {
    let values: HashSet<u32> = HashSet::from([value]);
    let affected_cells: Vec<CellPosition> = self.find_common_peers_for_cells_with_values(&cells, &values);

    if affected_cells.is_empty() {
      return None
    }

    Some(
      SolutionStep {
        rule: Rule::CommonPeerElimination,
        cells,
        values: vec![ value ],
        areas: vec![ *area ],
        affected_cells,
        candidates: None,
      }
    )
  }
}

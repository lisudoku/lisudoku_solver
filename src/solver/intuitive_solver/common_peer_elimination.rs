use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule, Area};
use itertools::Itertools;

// Cell can’t be X because it eliminates all X candidates from a region
impl Solver {
  pub fn find_common_peer_elimination(&self) -> Option<SolutionStep> {
    if !self.candidates_active {
      return None
    }

    for area in self.get_all_areas(false, false) {
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

  fn find_common_peer_elimination_cells_with_value(&self, area: &Area, cells: Vec<CellPosition>, value: u32) -> Option<SolutionStep> {
    let mut peer_counts = vec![ vec![ 0; self.constraints.grid_size ]; self.constraints.grid_size ];
    for cell in &cells {
      for CellPosition { row, col } in self.get_cell_peers(cell) {
        peer_counts[row][col] += 1;
      }
    }
    for &CellPosition { row, col } in &cells {
      peer_counts[row][col] = 0;
    }
    let affected_cells: Vec<CellPosition> = self.get_all_cells_with_candidate(value)
      .into_iter()
      .filter(|&CellPosition { row, col }| peer_counts[row][col] == cells.len())
      .collect();

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

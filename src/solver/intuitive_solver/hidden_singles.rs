use std::collections::HashSet;
use itertools::Itertools;

use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule, Area};

impl Solver {
  pub fn find_hidden_singles(&self) -> Option<SolutionStep> {
    let mut candidates: Vec<Vec<HashSet<u32>>> = vec![
      vec![ HashSet::new(); self.constraints.grid_size ];
      self.constraints.grid_size
    ];
    for cell in &self.get_all_empty_cells() {
      candidates[cell.row][cell.col] = self.compute_cell_candidates(cell);
    }

    for area in self.get_all_areas(false) {
      let hidden_single = self.find_hidden_single_in_area(&area, &candidates);

      if hidden_single.is_none() {
        continue
      }

      let (found_cell, value) = hidden_single.unwrap();

      let mut cells: Vec<CellPosition> = vec![ found_cell ];

      if !self.candidates_active {
        let other_cells = self.find_hidden_single_covering_cells(&area, &found_cell, value);
        cells.extend(other_cells.into_iter());
      }

      return Some(
        SolutionStep {
          rule: Rule::HiddenSingle,
          cells,
          values: vec![ value ],
          areas: vec![ area ],
          affected_cells: vec![],
          candidates: None,
        }
      )
    }

    None
  }

  fn find_hidden_single_in_area(&self, area: &Area, candidates: &Vec<Vec<HashSet<u32>>>) -> Option<(CellPosition, u32)> {
    let value_cells = self.compute_cells_by_value_in_area(area, candidates);
    let hidden_single = value_cells.iter().find(|(_value, cells)| cells.len() == 1);
    if let Some((&value, cells)) = hidden_single {
      return Some((cells[0], value));
    }

    None
  }

  fn find_hidden_single_covering_cells(&self, area: &Area, found_cell: &CellPosition, value: u32) -> Vec<CellPosition> {
    let mut covered_cells = vec![ vec![ false; self.constraints.grid_size ]; self.constraints.grid_size ];
    let mut cells: Vec<CellPosition> = vec![];
    for cell in &self.get_empty_area_cells(area) {
      if cell.eq(found_cell) || covered_cells[cell.row][cell.col] {
        continue
      }

      for peer in self.get_cell_peers(cell) {
        let peer_value = self.grid[peer.row][peer.col];
        if peer_value != value {
          continue
        }

        for CellPosition { row, col } in self.get_cell_peers(&peer) {
          covered_cells[row][col] = true;
        }
        cells.push(peer);
      }
    }
    cells.into_iter().unique().collect()
  }
}

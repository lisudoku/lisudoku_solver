use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule, Area};
use std::collections::HashSet;
use itertools::Itertools;
use super::technique::Technique;

// Cell can’t be X because it eliminates all X candidates from a region
// It is a more general version of the Locked Candidates
// For diagonal puzzles the rule applied on the is called Crossover
// For antiknight puzzles the rule is called L technique
pub struct CommonPeerElimination;

impl Technique for CommonPeerElimination {
  fn get_rule(&self) -> Rule { Rule::CommonPeerElimination }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }

    for area in solver.get_all_areas(false, false, false) {
      let cells_by_value = solver.compute_cells_by_value_in_area(&area, &solver.candidates);
      for (value, cells) in cells_by_value.into_iter().sorted() {
        let step = self.find_common_peer_elimination_cells_with_value(solver, &area, cells, value);
        if step.is_some() {
          return vec![ step.unwrap() ]
        }
      }
    }

    vec![]
  }
}

impl CommonPeerElimination {
  fn find_common_peer_elimination_cells_with_value(&self, solver: &Solver, area: &Area, cells: Vec<CellPosition>, value: u32) -> Option<SolutionStep> {
    let values: HashSet<u32> = HashSet::from([value]);
    let affected_cells: Vec<CellPosition> = self.find_common_peers_for_cells_with_values(solver, &cells, &values);

    if affected_cells.is_empty() {
      return None
    }

    Some(
      SolutionStep {
        rule: self.get_rule(),
        cells,
        values: vec![ value ],
        areas: vec![ *area ],
        affected_cells,
        candidates: None,
      }
    )
  }

  fn find_common_peers_for_cells_with_values(&self, solver: &Solver, cells: &Vec<CellPosition>, values: &HashSet<u32>) -> Vec<CellPosition> {
    let common_peers = Self::find_common_peers_for_cells(solver, cells);
    let common_peers_with_values: Vec<CellPosition> = solver.filter_cells_with_any_candidates(
      &common_peers, &values
    );
    common_peers_with_values
  }

  pub fn find_common_peers_for_cells(solver: &Solver, cells: &Vec<CellPosition>) -> Vec<CellPosition> {
    let mut peer_counts = vec![ vec![ 0; solver.constraints.grid_size ]; solver.constraints.grid_size ];
    for cell in cells {
      for CellPosition { row, col } in solver.get_cell_peers(cell) {
        peer_counts[row][col] += 1;
      }
    }
    for &CellPosition { row, col } in cells {
      peer_counts[row][col] = 0;
    }
    let common_peers: Vec<CellPosition> = solver.get_empty_area_cells(&Area::Grid)
      .into_iter()
      .filter(|&cell| peer_counts[cell.row][cell.col] == cells.len())
      .collect();

    common_peers
  }
}

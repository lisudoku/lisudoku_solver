use std::collections::HashSet;
use itertools::Itertools;
use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule, Area};
use super::technique::Technique;

pub struct HiddenSingles;

impl Technique for HiddenSingles {
  fn is_grid_step(&self) -> bool { true }
  fn get_rule(&self) -> Rule { Rule::HiddenSingle }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    let mut candidates: Vec<Vec<HashSet<u32>>> = vec![
      vec![ HashSet::new(); solver.constraints.grid_size ];
      solver.constraints.grid_size
    ];
    for cell in &solver.get_all_empty_cells() {
      candidates[cell.row][cell.col] = solver.compute_cell_candidates(cell);
    }
    // dbg!(&candidates);

    for area in solver.get_all_areas(false, false, false) {
      let hidden_single = self.find_hidden_single_in_area(solver, &area, &candidates);

      if hidden_single.is_none() {
        continue
      }

      let (found_cell, value) = hidden_single.unwrap();

      let mut cells: Vec<CellPosition> = vec![ found_cell ];

      if !solver.candidates_active {
        let other_cells = self.find_hidden_single_covering_cells(solver, &area, &found_cell, value);
        cells.extend(other_cells.into_iter());
      }

      return vec![
        SolutionStep {
          rule: self.get_rule(),
          cells,
          values: vec![ value ],
          areas: vec![ area ],
          affected_cells: vec![],
          candidates: None,
        }
      ]
    }

    vec![]
  }
}

impl HiddenSingles {
  fn find_hidden_single_in_area(&self, solver: &Solver, area: &Area, candidates: &Vec<Vec<HashSet<u32>>>) -> Option<(CellPosition, u32)> {
    let value_cells = solver.compute_cells_by_value_in_area(area, candidates);
    let hidden_single = value_cells.iter().find(|(_value, cells)| cells.len() == 1);
    if let Some((&value, cells)) = hidden_single {
      return Some((cells[0], value));
    }

    None
  }

  fn find_hidden_single_covering_cells(&self, solver: &Solver, area: &Area, found_cell: &CellPosition, value: u32) -> Vec<CellPosition> {
    let mut covered_cells = vec![ vec![ false; solver.constraints.grid_size ]; solver.constraints.grid_size ];
    let mut cells: Vec<CellPosition> = vec![];
    for cell in &solver.get_empty_area_cells(area) {
      if cell.eq(found_cell) || covered_cells[cell.row][cell.col] {
        continue
      }

      for peer in solver.get_cell_peers(cell, true) {
        let peer_value = solver.grid[peer.row][peer.col];
        if peer_value != value {
          continue
        }

        for CellPosition { row, col } in solver.get_cell_peers(&peer, true) {
          covered_cells[row][col] = true;
        }
        cells.push(peer);
      }
    }
    cells.into_iter().unique().collect()
  }
}

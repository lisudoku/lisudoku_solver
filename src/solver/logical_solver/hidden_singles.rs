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

    solver.get_all_proper_areas().into_iter().flat_map(|area| {
      let hidden_singles = self.find_hidden_singles_in_area(solver, &area, &candidates);

      hidden_singles.into_iter().map(move |(found_cell, value)| {
        let mut cells: Vec<CellPosition> = vec![ found_cell ];

        if !solver.candidates_active {
          let other_cells = self.find_hidden_single_covering_cells(solver, &area, &found_cell, value);
          cells.extend(other_cells.into_iter());
        }

        return self.build_grid_solution_step(
          cells,
          vec![ value ],
          vec![ area ],
          &solver,
        )
      })
    }).unique_by(|step| step.cells[0]).collect()
  }
}

impl HiddenSingles {
  fn find_hidden_singles_in_area(&self, solver: &Solver, area: &Area, candidates: &Vec<Vec<HashSet<u32>>>) -> Vec<(CellPosition, u32)> {
    let value_cells = solver.compute_cells_by_value_in_area(area, candidates);
    value_cells.iter().filter(|(_value, cells)| cells.len() == 1).map(|(&value, cells)| (cells[0], value)).collect()
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

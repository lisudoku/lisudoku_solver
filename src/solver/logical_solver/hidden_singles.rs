use itertools::Itertools;
use crate::solver::Solver;
use crate::solver::types::CandidateSet;
use crate::types::{SolutionStep, CellPosition, Rule, Area};
use super::technique::Technique;

// Within a house, X is a candidate in only 1 cell, so you must assign it there.
pub struct HiddenSingles;

impl Technique for HiddenSingles {
  fn is_grid_step(&self) -> bool { true }
  fn get_rule(&self) -> Rule { Rule::HiddenSingle }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    let candidates = solver.compute_all_cell_candidates();

    solver.puzzle.all_house_areas().into_iter().flat_map(|area| {
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
          vec![ area.clone() ],
          &solver,
        )
      })
    }).unique_by(|step| step.cells[0]).collect()
  }
}

impl HiddenSingles {
  fn find_hidden_singles_in_area(&self, solver: &Solver, area: &Area, candidates: &Vec<Vec<CandidateSet>>) -> Vec<(CellPosition, u32)> {
    let value_cells = solver.compute_cells_by_value_in_area(area, candidates);
    value_cells.iter().filter(|(_value, cells)| cells.len() == 1).sorted().map(|(&value, cells)| (cells[0], value)).collect()
  }

  fn find_hidden_single_covering_cells(&self, solver: &Solver, area: &Area, found_cell: &CellPosition, value: u32) -> Vec<CellPosition> {
    let mut covered_cells = vec![ vec![ false; solver.puzzle.grid_size ]; solver.puzzle.grid_size ];
    let mut cells: Vec<CellPosition> = vec![];
    for cell in &solver.area_empty_cells(area) {
      if cell.eq(found_cell) || covered_cells[cell.row][cell.col] {
        continue
      }

      for peer in solver.puzzle.all_cell_peers_unconstrained(cell) {
        let peer_value = solver.grid[peer.row][peer.col];
        if peer_value != value {
          continue
        }

        for CellPosition { row, col } in solver.puzzle.all_cell_peers_unconstrained(&peer) {
          covered_cells[row][col] = true;
        }
        cells.push(peer);
      }
    }
    cells.into_iter().unique().collect()
  }
}

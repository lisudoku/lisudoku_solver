use crate::solver::logical::logical_solver::LogicalSolver;
use crate::solver::logical::techniques::{StepApplication, TechniqueDefinition, TechniqueKind, TechniquePhase};
use crate::solver::types::CandidateSet;
use crate::types::{SolutionStep, CellPosition, Area};
use itertools::Itertools;

// Within a house, X is a candidate in only 1 cell, so you must assign it there.

#[derive(Clone, Debug)]
pub struct HiddenSingle;

impl TechniqueDefinition for HiddenSingle {
  fn kind(&self) -> TechniqueKind { TechniqueKind::HiddenSingle }

  fn phase(&self) -> TechniquePhase { TechniquePhase::Grid }

  fn step_application(&self) -> StepApplication { StepApplication::SetCellValue }

  fn complexity(&self, step: &SolutionStep, solver: &LogicalSolver) -> u32 {
    solver.state.area_empty_cells(&step.areas[0]).len() as u32 / 2 + step.cells.len() as u32 - 1
  }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    let candidates = solver.state.compute_all_cell_candidates();

    let all_hidden_singles = solver.state.puzzle.all_house_areas().flat_map(|area| {
      let hidden_singles = self.find_hidden_singles_in_area(solver, &area, &candidates);

      hidden_singles.into_iter().map(move |(found_cell, value)| {
        let mut cells: Vec<CellPosition> = vec![ found_cell ];

        if !solver.performance_mode && !solver.state.candidates_active {
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
    });

    if solver.performance_mode {
      return all_hidden_singles.take(1).collect()
    }
    all_hidden_singles.sorted_by_key(|step| self.complexity(step, solver)).unique_by(|step| step.cells[0]).collect()
  }
}

impl HiddenSingle {
  fn find_hidden_singles_in_area<'a>(&'a self, solver: &'a LogicalSolver, area: &Area, candidates: &Vec<Vec<CandidateSet>>) -> impl Iterator<Item = (CellPosition, u32)> + 'a {
    let value_cells = solver.state.compute_cells_by_value_in_area(area, candidates);
    value_cells.filter_map(|(value, cells)| {
      if cells.len() == 1 { Some((cells[0], value)) } else { None }
    })
  }

  fn find_hidden_single_covering_cells(&self, solver: &LogicalSolver, area: &Area, found_cell: &CellPosition, value: u32) -> Vec<CellPosition> {
    let mut covered_cells = vec![ vec![ false; solver.state.puzzle.grid_size ]; solver.state.puzzle.grid_size ];
    let mut cells: Vec<CellPosition> = vec![];
    for cell in &solver.state.area_empty_cells(area) {
      if cell.eq(found_cell) || covered_cells[cell.row][cell.col] {
        continue
      }

      for peer in solver.state.puzzle.all_cell_peers_unconstrained(cell) {
        let peer_value = solver.state.value_at(&peer);
        if peer_value != value {
          continue
        }

        for CellPosition { row, col } in solver.state.puzzle.all_cell_peers_unconstrained(&peer) {
          covered_cells[row][col] = true;
        }
        cells.push(peer);
      }
    }
    cells.into_iter().unique().collect()
  }
}

use crate::solver::logical::logical_solver::LogicalSolver;
use crate::solver::logical::techniques::{TechniqueDefinition, TechniqueKind, TechniquePhase};
use crate::solver::types::CandidateSet;
use crate::types::{Area, CellPosition, SolutionStep};
use itertools::Itertools;

// Within a house, 2/3 cells have the same set of 2/3 candidates, remove those from all
// other cells in the house

#[derive(Clone, Debug)]
pub(crate) struct NakedSet {
  set_size: usize,
}

impl TechniqueDefinition for NakedSet {
  fn kind(&self) -> TechniqueKind { if self.set_size == 2 { TechniqueKind::NakedPair } else { TechniqueKind::NakedTriple } }

  fn phase(&self) -> TechniquePhase { TechniquePhase::CandidateElimination }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    if !solver.state.candidates_active {
      return vec![]
    }

    for area in solver.state.puzzle.all_region_areas() {
      let steps = self.run_in_area(area, solver);
      if !steps.is_empty() {
        return steps;
      }
    }

    vec![]
  }
}

impl NakedSet {
  pub(crate) fn new(set_size: usize) -> NakedSet {
    NakedSet { set_size }
  }

  pub(crate) fn run_in_area(&self, area: Area, solver: &LogicalSolver) -> Vec<SolutionStep> {
    let area_cells = solver.state.area_empty_cells(&area);
    // TODO: filter cells with CandidateSet size > self.set_size

    let cell_combinations = area_cells.iter().sorted_unstable().combinations(self.set_size);

    for cells in cell_combinations {
      let mut combined_candidates = CandidateSet::new();
      for cell in &cells {
        let candidates = &solver.state.candidates[cell.row][cell.col];
        combined_candidates = combined_candidates.union(candidates);
        if combined_candidates.len() > self.set_size {
          break
        }
      }

      if combined_candidates.len() != self.set_size {
        continue
      }

      let affected_cells: Vec<CellPosition> = solver
        .state
        .filter_cells_with_any_candidates(&area_cells, &combined_candidates)
        .into_iter()
        .filter(|cell| !cells.contains(&cell))
        .collect();
      if !solver.any_cells_with_candidates(&affected_cells, &combined_candidates) {
        continue
      }

      return vec![
        self.build_solution_step(
          cells.into_iter().cloned().collect(),
          combined_candidates.into_iter().sorted().collect(),
          vec![ area ],
          affected_cells,
        )
      ]
    }

    vec![]
  }
}

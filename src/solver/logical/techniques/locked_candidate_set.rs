use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{TechniqueDefinition, TechniqueKind, TechniquePhase}}, types::{CellPosition, SolutionStep}};

// In an area A, X candidate cells are all included in area B, so remove X from all other cells in B.
// Example: In column C3, digit 3 must be in square S1. Therefore, 3 cannot be a candidate of any
// other cells in the same square (S1).

#[derive(Clone, Debug)]
pub(crate) struct LockedCandidateSet {
  set_size: usize,
}

impl TechniqueDefinition for LockedCandidateSet {
  fn kind(&self) -> TechniqueKind { if self.set_size == 2 { TechniqueKind::LockedCandidatePair } else { TechniqueKind::LockedCandidateTriple } }

  fn phase(&self) -> TechniquePhase { TechniquePhase::CandidateElimination }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    if !solver.state.candidates_active {
      return vec![]
    }

    for area in solver.state.puzzle.all_house_areas() {
      let value_cells = solver.state.compute_cells_by_value_in_area(&area, &solver.state.candidates);

      for (value, cells) in value_cells {
        if cells.len() != self.set_size {
          continue
        }

        let other_areas = solver.state.puzzle.find_common_region_areas_except(&cells, area.clone());
        if other_areas.is_empty() {
          continue
        }

        for other_area in other_areas {
          let affected_cells: Vec<CellPosition> = solver
            .state
            .area_cells_with_candidate(&other_area, value)
            .into_iter()
            .filter(|cell| !cells.contains(cell))
            .collect();
          if !affected_cells.is_empty() {
            return vec![
              self.build_solution_step(
                cells, // set of cells in area with value
                vec![ value ],
                vec![ area, other_area ],
                affected_cells
              )
            ]
          }
        }
      }
    }

    vec![]
  }
}

impl LockedCandidateSet {
  pub(crate) fn new(set_size: usize) -> LockedCandidateSet {
    LockedCandidateSet { set_size }
  }
}

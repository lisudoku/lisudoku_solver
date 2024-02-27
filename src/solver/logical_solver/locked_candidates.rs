use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule};
use super::technique::Technique;

// In an area A, X candidate cells are all included in area B, so remove X from all other cells in B.
// Example: In column C3, digit 3 must be in square S1. Therefore, 3 cannot be a candidate of any
// other cells in the same square (S1).
pub struct LockedCandidates {
  set_size: usize,
}

impl Technique for LockedCandidates {
  fn get_rule(&self) -> Rule { if self.set_size == 2 { Rule::LockedCandidatesPairs } else { Rule::LockedCandidatesTriples } }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }

    let areas = solver.get_all_areas(false, false, false);
    for area in areas {
      let value_cells = solver.compute_cells_by_value_in_area(&area, &solver.candidates);

      for (value, cells) in value_cells {
        if cells.len() != self.set_size {
          continue
        }

        let other_areas = solver.find_common_areas_except(&cells, area);
        if other_areas.is_empty() {
          continue
        }

        for other_area in other_areas {
          let affected_cells: Vec<CellPosition> = solver
            .get_area_cells_with_candidate(&other_area, value)
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

impl LockedCandidates {
  pub fn new(set_size: usize) -> LockedCandidates {
    LockedCandidates { set_size }
  }
}

use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule};

// In an area A, X candidate cells are all included in area B, so remove X from all other cells in B.
// Example: In column C3, digit 3 must be in square S1. Therefore, 3 cannot be a candidate of any
// other cells in the same square (S1).
impl Solver {
  pub fn find_locked_candidates_pairs(&self) -> Option<SolutionStep> {
    self.find_locked_candidates_set(2)
  }

  pub fn find_locked_candidates_triples(&self) -> Option<SolutionStep> {
    self.find_locked_candidates_set(3)
  }

  pub fn find_locked_candidates_set(&self, set_size: usize) -> Option<SolutionStep> {
    if !self.candidates_active {
      return None
    }

    let areas = self.get_all_areas(false, false, false);
    for area in areas {
      let value_cells = self.compute_cells_by_value_in_area(&area, &self.candidates);

      for (value, cells) in value_cells {
        if cells.len() != set_size {
          continue
        }

        let other_areas = self.find_common_areas_except(&cells, area);
        if other_areas.is_empty() {
          continue
        }

        for other_area in other_areas {
          let affected_cells: Vec<CellPosition> = self.get_area_cells_with_candidate(&other_area, value)
                                                      .into_iter()
                                                      .filter(|cell| !cells.contains(cell))
                                                      .collect();
          if !affected_cells.is_empty() {
            return Some(
              SolutionStep {
                rule: if set_size == 2 { Rule::LockedCandidatesPairs } else { Rule::LockedCandidatesTriples },
                cells, // set of cells in area with value
                values: vec![ value ],
                areas: vec![ area, other_area ],
                affected_cells,
                candidates: None,
              }
            )
          }
        }
      }
    }

    return None
  }
}

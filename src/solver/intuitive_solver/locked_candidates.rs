use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule};

impl Solver {
  pub fn find_locked_candidates(&self) -> Option<SolutionStep> {    
    if !self.candidates_active {
      return None
    }

    let areas = self.get_all_areas();
    for area in areas {
      let value_cells = self.compute_cells_by_value_in_area(&area, &self.candidates);

      for (value, cells) in value_cells {
        // TODO: also check for triples
        if cells.len() != 2 {
          continue
        }

        let other_area = self.find_common_area_except(&cells, area);
        if other_area.is_none() {
          continue
        }
        let other_area = other_area.unwrap();

        let affected_cells: Vec<CellPosition> = self.get_area_cells_with_candidate(&other_area, value)
                                                    .into_iter()
                                                    .filter(|cell| !cells.contains(cell))
                                                    .collect();
        if !affected_cells.is_empty() {
          return Some(
            SolutionStep {
              rule: Rule::LockedCandidates,
              cells,
              values: vec![ value ],
              areas: vec![ other_area ],
              affected_cells,
              candidates: None,
            }
          )
        }
      }
    }

    return None
  }
}

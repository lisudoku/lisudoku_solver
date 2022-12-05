use std::collections::HashSet;
use crate::solver::Solver;
use crate::types::{SolutionStep, Rule};
use combinations::Combinations;

impl Solver {
  pub fn find_naked_pairs(&self) -> Option<SolutionStep> {
    self.find_naked_set(2)
  }

  pub fn find_naked_triples(&self) -> Option<SolutionStep> {
    self.find_naked_set(3)
  }

  pub fn find_naked_set(&self, set_size: usize) -> Option<SolutionStep> {
    if !self.candidates_active {
      return None
    }

    let areas = self.get_all_areas();
    for area in areas {
      let area_cells = self.get_empty_area_cells(&area);
      let cell_combinations: Vec<_> = if set_size < area_cells.len() {
        Combinations::new(area_cells, set_size).collect()
      } else {
        // Has to be handled separately because of stupid crate
        vec![area_cells]
      };

      for cells in cell_combinations {
        let mut combined_candidates: HashSet<u32> = HashSet::new();
        for cell in &cells {
          let candidates = &self.candidates[cell.row][cell.col];
          combined_candidates = combined_candidates.union(candidates).cloned().collect();
          if combined_candidates.len() > set_size {
            break
          }
        }

        if combined_candidates.len() != set_size {
          continue
        }

        let affected_cells = self.get_affected_by_area_cells(&area, &cells, &combined_candidates);
        if !self.any_cells_with_candidates(&affected_cells, &combined_candidates) {
          continue
        }

        return Some(
          SolutionStep {
            rule: if set_size == 2 { Rule::NakedPairs } else { Rule::NakedTriples },
            cells,
            values: combined_candidates.into_iter().collect(),
            areas: vec![area],
            affected_cells,
            candidates: None,
          }
        )
      }
    }

    None
  }
}

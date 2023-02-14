use std::collections::HashSet;
use crate::solver::Solver;
use crate::types::{SolutionStep, Rule};
use combinations::Combinations;
use super::technique::Technique;

// Within a house, 2/3 cells have the same set of 2/3 candidates, remove those from all
// other cells in the house
pub struct NakedSet {
  set_size: usize,
}

impl Technique for NakedSet {
  fn get_rule(&self) -> Rule { if self.set_size == 2 { Rule::NakedPairs } else { Rule::NakedTriples } }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }

    let areas = solver.get_all_areas(false, true, false);
    for area in areas {
      let area_cells = solver.get_empty_area_cells(&area);
      let cell_combinations: Vec<_> = if self.set_size < area_cells.len() {
        Combinations::new(area_cells, self.set_size).collect()
      } else {
        // Has to be handled separately because of stupid crate
        vec![area_cells]
      };

      for cells in cell_combinations {
        let mut combined_candidates: HashSet<u32> = HashSet::new();
        for cell in &cells {
          let candidates = &solver.candidates[cell.row][cell.col];
          combined_candidates = combined_candidates.union(candidates).cloned().collect();
          if combined_candidates.len() > self.set_size {
            break
          }
        }

        if combined_candidates.len() != self.set_size {
          continue
        }

        let affected_cells = solver.get_affected_by_area_cells(&area, &cells, &combined_candidates);
        if !solver.any_cells_with_candidates(&affected_cells, &combined_candidates) {
          continue
        }

        return vec![
          SolutionStep {
            rule: self.get_rule(),
            cells,
            values: combined_candidates.into_iter().collect(),
            areas: vec![area],
            affected_cells,
            candidates: None,
          }
        ]
      }
    }

    vec![]
  }
}

impl NakedSet {
  pub fn new(set_size: usize) -> NakedSet {
    NakedSet { set_size }
  }
}

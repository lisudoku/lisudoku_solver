use std::collections::HashSet;
use crate::solver::Solver;
use crate::types::{SolutionStep, Rule, CellPosition};
use combinations::Combinations;
use itertools::Itertools;

// Within a house, X and Y are candidates in only 2 cells, so you can remove any other
// candidate from those 2 cells.
impl Solver {
  pub fn find_hidden_pairs(&self) -> Option<SolutionStep> {
    self.find_hidden_set(2)
  }

  pub fn find_hidden_triples(&self) -> Option<SolutionStep> {
    self.find_hidden_set(3)
  }

  pub fn find_hidden_set(&self, set_size: usize) -> Option<SolutionStep> {
    if !self.candidates_active {
      return None
    }

    let areas = self.get_all_areas(false, false);
    for area in areas {
      let value_cells = self.compute_cells_by_value_in_area(&area, &self.candidates);

      let area_values: Vec<u32> = self.compute_area_candidates_union(&area).into_iter().sorted().collect();
      let value_combinations: Vec<_> = if set_size < area_values.len() {
        Combinations::new(area_values, set_size).collect()
      } else {
        // Has to be handled separately because of stupid crate
        vec![area_values]
      };

      for values in value_combinations {
        let mut combined_cells: HashSet<CellPosition> = HashSet::new();
        for value in &values {
          let cells: HashSet<CellPosition> = value_cells[value].iter().copied().collect();
          combined_cells = combined_cells.union(&cells).cloned().collect();
          if combined_cells.len() > set_size {
            break
          }
        }

        if combined_cells.len() != set_size {
          continue
        }

        let cells_array: Vec<CellPosition> = combined_cells.into_iter().sorted().collect();
        let values_set: HashSet<u32> = values.iter().copied().collect();
        if !self.any_cells_with_other_candidates(&cells_array, &values_set) {
          continue
        }

        return Some(
          SolutionStep {
            rule: if set_size == 2 { Rule::HiddenPairs } else { Rule::HiddenTriples },
            cells: cells_array,
            values,
            areas: vec![area],
            affected_cells: vec![],
            candidates: None,
          }
        )
      }
    }

    None
  }
}

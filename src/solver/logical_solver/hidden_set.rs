use std::collections::HashSet;
use crate::solver::Solver;
use crate::types::{SolutionStep, Rule, CellPosition};
use combinations::Combinations;
use itertools::Itertools;
use super::technique::Technique;

// Within a house, X and Y are candidates in only 2 cells, so you can remove any other
// candidate from those 2 cells.
pub struct HiddenSet {
  set_size: usize,
}

impl Technique for HiddenSet {
  fn get_rule(&self) -> Rule { if self.set_size == 2 { Rule::HiddenPairs } else { Rule::HiddenTriples } }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }

    let areas = solver.get_all_areas(false, false, false);
    for area in areas {
      let value_cells = solver.compute_cells_by_value_in_area(&area, &solver.candidates);

      let area_values: Vec<u32> = solver.compute_area_candidates_union(&area).into_iter().sorted().collect();
      let value_combinations: Vec<_> = if self.set_size < area_values.len() {
        Combinations::new(area_values, self.set_size).collect()
      } else {
        // Has to be handled separately because of stupid crate
        vec![area_values]
      };

      for values in value_combinations {
        let mut combined_cells: HashSet<CellPosition> = HashSet::new();
        for value in &values {
          let cells: HashSet<CellPosition> = value_cells[value].iter().copied().collect();
          combined_cells = combined_cells.union(&cells).cloned().collect();
          if combined_cells.len() > self.set_size {
            break
          }
        }

        if combined_cells.len() != self.set_size {
          continue
        }

        let cells_array: Vec<CellPosition> = combined_cells.into_iter().sorted().collect();
        let values_set: HashSet<u32> = values.iter().copied().collect();
        if !solver.any_cells_with_other_candidates(&cells_array, &values_set) {
          continue
        }

        return vec![
          self.build_solution_step(
            cells_array,
            values,
            vec![area],
            vec![],
          )
        ]
      }
    }

    vec![]
  }

  fn apply(&self, step: &SolutionStep, solver: &mut Solver) {
    for &CellPosition { row, col } in &step.cells {
      let value_set: HashSet<u32> = step.values.iter().copied().collect();
      solver.candidates[row][col] = solver.candidates[row][col].intersection(&value_set).copied().collect();
    }
  }
}

impl HiddenSet {
  pub fn new(set_size: usize) -> HiddenSet {
    HiddenSet { set_size }
  }
}

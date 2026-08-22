use crate::solver::Solver;
use crate::solver::types::CandidateSet;
use crate::types::{Area, Rule, SolutionStep};
use combinations::Combinations;
use itertools::Itertools;
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

    let areas = solver.puzzle.all_region_areas();
    for area in areas {
      let steps = self.run_in_area(area, solver);
      if !steps.is_empty() {
        return steps;
      }
    }

    vec![]
  }
}

impl NakedSet {
  pub fn new(set_size: usize) -> NakedSet {
    NakedSet { set_size }
  }

  pub fn run_in_area(&self, area: Area, solver: &Solver) -> Vec<SolutionStep> {
    let area_cells = solver.area_empty_cells(&area);
    // TODO: filter cells with CandidateSet size > self.set_size

    let cell_combinations: Vec<_> = if self.set_size < area_cells.len() {
      Combinations::new(area_cells.to_vec(), self.set_size).collect()
    } else {
      // Has to be handled separately because of stupid crate
      vec![area_cells.to_vec()]
    };

    for cells in cell_combinations {
      let mut combined_candidates = CandidateSet::new();
      for cell in &cells {
        let candidates = &solver.candidates[cell.row][cell.col];
        combined_candidates = combined_candidates.union(candidates);
        if combined_candidates.len() > self.set_size {
          break
        }
      }

      if combined_candidates.len() != self.set_size {
        continue
      }

      let affected_cells = solver
        .filter_cells_with_any_candidates(&area_cells, &combined_candidates)
        .into_iter()
        .filter(|cell| !cells.contains(cell))
        .collect();
      if !solver.any_cells_with_candidates(&affected_cells, &combined_candidates) {
        continue
      }

      return vec![
        self.build_solution_step(
          cells,
          combined_candidates.into_iter().sorted().collect(),
          vec![ area ],
          affected_cells,
        )
      ]
    }

    vec![]
  }
}

use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule, Area};
use itertools::Itertools;
use std::collections::HashSet;
use super::technique::Technique;

// Area A's cells must be 45 and after subtracting killer cages there is Y left.
pub struct Killer45;

impl Technique for Killer45 {
  fn get_rule(&self) -> Rule { Rule::Killer45 }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      // TODO: could make it work without candidates for cases with 1 candidate left
      return vec![]
    }
    if solver.constraints.killer_cages.is_empty() {
      return vec![]
    }

    for area in &solver.get_all_areas(false, false, false) {
      let steps = self.find_killer45_in_area(solver, area);
      if !steps.is_empty() {
        return steps
      }
    }

    vec![]
  }
}

impl Killer45 {
  // Note: we assume there are no overlapping cages
  fn find_killer45_in_area(&self, solver: &Solver, area: &Area) -> Vec<SolutionStep> {
    let mut area_cells_set: HashSet<CellPosition> = solver.get_area_cells(area).into_iter().collect();
    let mut region_sum_left = solver.constraints.grid_size as u32 * (solver.constraints.grid_size as u32 + 1) / 2;

    for (killer_cage_index, killer_cage) in solver.constraints.killer_cages.iter().enumerate() {
      if killer_cage.sum.is_some() && solver.is_empty_area_subset(&Area::KillerCage(killer_cage_index), area) {
        region_sum_left -= solver.get_subset_area_sum(killer_cage, area);
        let killer_cage_set: HashSet<CellPosition> = killer_cage.region.iter().copied().collect();
        area_cells_set = area_cells_set.difference(&killer_cage_set).copied().collect();
      }
    }

    // Mark fixed digits
    for cell in &area_cells_set.iter().copied().collect_vec() {
      let value = solver.grid[cell.row][cell.col];
      if value != 0 && area_cells_set.remove(cell) {
        region_sum_left -= value;
      }
    }

    // TODO: think of a more sophisticated way of pruning
    if area_cells_set.len() > 3 {
      return vec![]
    }

    let empty_cells = area_cells_set.iter().sorted().copied().collect();
    let invalid_sum_candidates = solver.detect_invalid_sum_candidates(&empty_cells, region_sum_left);

    if invalid_sum_candidates.is_empty() {
      return vec![]
    }

    invalid_sum_candidates.into_iter().map(|(cell, invalid_values)| {
      self.build_simple_solution_step(
        invalid_values,
        vec![ *area ],
        vec![ cell ],
      )
    }).collect()
  }
}

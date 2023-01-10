use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule, Area};
use itertools::Itertools;
use std::collections::HashSet;

// Area A's cells must be 45 and after subtracting killer cages there is Y left.
impl Solver {
  pub fn find_killer45(&self) -> Option<SolutionStep> {
    if !self.candidates_active {
      // TODO: could make it work without candidates for cases with 1 candidate left
      return None
    }

    for area in &self.get_all_areas(false, false) {
      let step = self.find_killer45_in_area(area);
      if step.is_some() {
        return step
      }
    }

    None
  }

  // Note: we assume there are no overlapping cages
  fn find_killer45_in_area(&self, area: &Area) -> Option<SolutionStep> {
    let mut area_cells_set: HashSet<CellPosition> = self.get_area_cells(area).into_iter().collect();
    let mut region_sum_left = self.constraints.grid_size as u32 * (self.constraints.grid_size as u32 + 1) / 2;

    for (killer_cage_index, killer_cage) in self.constraints.killer_cages.iter().enumerate() {
      if killer_cage.sum.is_some() && self.is_empty_area_subset(&Area::KillerCage(killer_cage_index), area) {
        region_sum_left -= self.get_subset_area_sum(killer_cage, area);
        let killer_cage_set: HashSet<CellPosition> = killer_cage.region.iter().copied().collect();
        area_cells_set = area_cells_set.difference(&killer_cage_set).copied().collect();
      }
    }

    // Mark fixed digits
    for cell in &area_cells_set.iter().copied().collect_vec() {
      let value = self.grid[cell.row][cell.col];
      if value != 0 && area_cells_set.remove(cell) {
        region_sum_left -= value;
      }
    }

    // TODO: think of a more sophisticated way of pruning
    if area_cells_set.len() > 3 {
      return None
    }

    let empty_cells = area_cells_set.iter().sorted().copied().collect();
    let invalid_sum_candidates = self.detect_invalid_sum_candidates(&empty_cells, region_sum_left);

    if invalid_sum_candidates.is_empty() {
      return None
    }

    // TODO: take all
    let (cell, invalid_values) = invalid_sum_candidates.into_iter().next().unwrap();

    return Some(
      SolutionStep {
        rule: Rule::Killer45,
        cells: vec![],
        values: invalid_values,
        areas: vec![ *area ],
        affected_cells: vec![ cell ],
        candidates: None,
      }
    )
  }
}

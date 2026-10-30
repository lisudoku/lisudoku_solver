use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::Technique}, types::{Area, CellPosition, KillerCage, Rule, SolutionStep}};
use itertools::Itertools;
use std::collections::HashSet;

// Area A's cells must be 45 and after subtracting killer cages there is Y left.
// After trying value combinations that sum up to Y eliminate values that do no appear
// in any valid sum.
pub struct Killer45;

impl Technique for Killer45 {
  fn get_rule(&self) -> Rule { Rule::Killer45 }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    if !solver.state.candidates_active {
      // TODO: could make it work without candidates for cases with 1 candidate left
      return vec![]
    }

    let Some(constraint) = solver.state.puzzle.killer() else { return vec![] };
    if constraint.killer_cages.is_empty() {
      return vec![]
    }

    for area in solver.state.puzzle.all_house_areas() {
      let steps = self.find_killer45_in_area(solver, &area);
      if !steps.is_empty() {
        return steps
      }
    }

    vec![]
  }
}

impl Killer45 {
  // TODO: make it work for overlapping cages, currently it doesn't
  fn find_killer45_in_area(&self, solver: &LogicalSolver, area: &Area) -> Vec<SolutionStep> {
    // TODO: could replace HashSet with CellPositionSet or CandidateSet with indices
    let mut area_cells_set: HashSet<CellPosition> = solver.state.puzzle.area_cells(area).into_iter().collect();
    let mut region_sum_left = solver.state.puzzle.grid_size as u32 * (solver.state.puzzle.grid_size as u32 + 1) / 2;

    let Some(constraint) = solver.state.puzzle.killer() else { return vec![] };
    for (killer_cage_index, killer_cage) in constraint.killer_cages.iter().enumerate() {
      if killer_cage.sum.is_some() && Self::is_empty_area_subset(&Area::KillerCage(killer_cage_index), area, &solver) {
        // TODO: subset logic implemented twice, is_empty_area_subset could return the cells
        region_sum_left -= Self::compute_subset_area_sum(killer_cage, area, solver);
        let killer_cage_set: HashSet<CellPosition> = killer_cage.region.iter().copied().collect();
        area_cells_set = area_cells_set.difference(&killer_cage_set).copied().collect();
      }
    }

    // Mark fixed digits
    for cell in &area_cells_set.iter().copied().collect_vec() {
      let value = solver.state.value_at(cell);
      if value != 0 && area_cells_set.remove(cell) {
        region_sum_left -= value;
      }
    }

    // TODO: think of a more sophisticated way of pruning
    if area_cells_set.len() > 3 {
      return vec![]
    }

    let empty_cells = area_cells_set.into_iter().sorted().collect();
    let invalid_sum_candidates = solver.detect_invalid_sum_candidates(&empty_cells, region_sum_left);

    if invalid_sum_candidates.is_empty() {
      return vec![]
    }

    invalid_sum_candidates.into_iter().map(|(cell, invalid_values)| {
      self.build_simple_solution_step(
        invalid_values,
        vec![ area.clone() ],
        vec![ cell ],
      )
    }).collect()
  }

  /// Returns the sum of digits of cells from <killer_cage> inside <big_area>
  fn compute_subset_area_sum(killer_cage: &KillerCage, big_area: &Area, solver: &LogicalSolver) -> u32 {
    let big_set: HashSet<CellPosition> = solver.state.puzzle.area_cells(big_area).into_iter().collect();
    let killer_cells_sum: u32 = killer_cage.region.iter().map(|&cell| {
      if !big_set.contains(&cell) {
        solver.state.value_at(&cell)
      } else {
        0
      }
    }).sum();
    killer_cage.sum.unwrap() - killer_cells_sum
  }


  /// Returns true if the part of <small_area> inside <big_area> is empty
  fn is_empty_area_subset(small_area: &Area, big_area: &Area, solver: &LogicalSolver) -> bool {
    let small_set: HashSet<CellPosition> = solver.state.area_empty_cells(small_area).into_iter().collect();
    if small_set.is_empty() {
      return false
    }

    let big_set: HashSet<CellPosition> = solver.state.puzzle.area_cells(big_area).into_iter().collect();
    big_set.is_superset(&small_set)
  }
}

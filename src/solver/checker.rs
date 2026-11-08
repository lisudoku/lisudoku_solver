use crate::solver::constraints::{ConstraintDefinition};
use crate::solver::solve_state::SolveState;
use crate::solver::types::{CandidateGrid, CandidateSet};
use crate::types::{Area, InvalidStateReason, InvalidStateType};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Serialize, Deserialize, Debug, PartialEq, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct GridCheckResult {
  pub solved: bool,
  #[tsify(optional)]
  pub invalid_state_reason: Option<InvalidStateReason>,
}

impl GridCheckResult {
  pub fn solved() -> GridCheckResult {
    GridCheckResult {
      solved: true,
      invalid_state_reason: None,
    }
  }

  pub fn unsolved(invalid_state_reason: InvalidStateReason) -> GridCheckResult {
    GridCheckResult {
      solved: false,
      invalid_state_reason: Some(invalid_state_reason),
    }
  }
}

impl SolveState {
  pub fn check_solved(&self) -> GridCheckResult {
    self.check_grid_valid(true)
  }

  pub fn check_partially_solved(&self) -> GridCheckResult {
    self.check_grid_valid(false)
  }

  fn check_grid_valid(&self, check_full: bool) -> GridCheckResult {
    let mut cached_candidates = CandidateGrid::new(self.puzzle.grid_size);

    if check_full {
      for cell in self.puzzle.all_grid_cells() {
        let value = self.value_at(&cell);
        if value == 0 {
          return GridCheckResult::unsolved(
            InvalidStateReason {
              state_type: InvalidStateType::CellEmpty,
              area: Area::Cell(cell.row, cell.col),
              values: vec![],
            }
          )
        } else if value < 1 || value > self.puzzle.grid_size as u32 {
          return GridCheckResult::unsolved(
            InvalidStateReason {
              state_type: InvalidStateType::CellInvalidValue,
              area: Area::Cell(cell.row, cell.col),
              values: vec![value],
            }
          )
        }
      }
    }

    // Check that all empty cell values are valid and still have candidates
    for cell in self.all_empty_cells() {
      let cell_candidates = self.compute_cell_candidates(&cell);
      if cell_candidates.is_empty() {
        return GridCheckResult::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::CellNoCandidates,
            area: Area::Cell(cell.row, cell.col),
            values: vec![],
          }
        )
      }
      cached_candidates[cell.row][cell.col] = cell_candidates;
    }

    // Check areas and special constraints
    for constraint in &self.puzzle.constraints {
      let constraint_region_areas = constraint.region_areas();
      for area in constraint_region_areas {
        let check = self.check_region_area_valid(&area);
        if !check.solved {
          return check
        }
      }

      let check = constraint.check_grid_special_validity(self.grid(), &cached_candidates);
      if !check.solved {
        return check
      }
    }

    GridCheckResult::solved()
  }

  fn check_region_area_valid(&self, area: &Area) -> GridCheckResult {
    let mut values = CandidateSet::new();
    let mut candidates_and_values = CandidateSet::new();
    let area_cells = self.puzzle.area_cells(area);
    let mut area_cell_candidates: Vec<CandidateSet> = Vec::with_capacity(area_cells.len());
    assert!(area_cells.len() < 16);

    // Check value duplicates and compute candidates
    for cell in &area_cells {
      let value = self.value_at(cell);
      if value == 0 {
        let cell_candidates = self.compute_cell_candidates(cell);
        candidates_and_values = candidates_and_values.union(&cell_candidates);
        area_cell_candidates.push(cell_candidates);
        continue
      }
      if values.contains(&value) {
        return GridCheckResult::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::AreaValueConflict,
            area: area.clone(),
            values: vec![value],
          }
        )
      }
      values.insert(value);
    }

    candidates_and_values = candidates_and_values.union(&values);

    // There's less candidate values than area cells so there is no solution
    if candidates_and_values.len() < area_cells.len() {
      let values = if area_cells.len() == self.puzzle.grid_size && !self.performance_mode {
        self.full_candidate_set()
          .difference(&candidates_and_values)
          .into_iter()
          .collect()
      } else {
        vec![]
      };

      // Can't place `values` in `area`
      return GridCheckResult::unsolved(
        InvalidStateReason {
          state_type: InvalidStateType::AreaCandidates,
          area: area.clone(),
          values,
        }
      )
    }

    if area_cells.len() == self.puzzle.grid_size {
      // `area` is a house area, so we need to place all values.
      // Check if all remaining candidate subsets fit into the remaining cells.

      let mut cell_indices_with_value: Vec<CandidateSet> = vec![
        CandidateSet::new();
        self.puzzle.grid_size
      ];
      for (index, cell_candidates) in area_cell_candidates.iter().enumerate() {
        for value in cell_candidates {
          cell_indices_with_value[value as usize - 1].insert(index as u32);
        }
      }

      let mut used_cell_indices_set = CandidateSet::new();

      // Sorting value groups by size in increasing order to try to maximize
      // the overlap between groups. It's just a heuristic, but good enough for now.
      let value_cell_entries = cell_indices_with_value
        .into_iter()
        .enumerate()
        .filter(|e| !e.1.is_empty())
        .sorted_by_key(|e| e.1.len());
      let mut values: Vec<u32> = vec![];

      for (value_index, (value, value_cells)) in value_cell_entries.enumerate() {
        used_cell_indices_set = used_cell_indices_set.union(&value_cells);
        if !self.performance_mode {
          values.push(value as u32 + 1);
        }

        if value_index + 1 > used_cell_indices_set.len() {
          // There are more values than target cells
          return GridCheckResult::unsolved(
            InvalidStateReason {
              state_type: InvalidStateType::AreaCandidates,
              area: area.clone(),
              values,
            }
          )
        }
      }
    }

    GridCheckResult::solved()
  }

  pub(crate) fn check_all_grid_special_validity(&self) -> GridCheckResult {
    for constraint in &self.puzzle.constraints {
      let check = constraint.check_grid_special_validity(self.grid(), &self.candidates);
      if !check.solved {
        return check
      }
    }
    GridCheckResult::solved()
  }
}

use crate::solver::Solver;
use crate::solver::constraints::{ConstraintDefinition};
use crate::solver::types::CandidateSet;
use crate::types::{Area, CellPosition, InvalidStateReason, InvalidStateType};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use std::collections::{HashMap, HashSet};
use std::{format};

#[derive(Serialize, Deserialize, Debug, PartialEq, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct SolvedState {
  pub solved: bool,
  #[tsify(optional)]
  pub invalid_state_reason: Option<InvalidStateReason>,
}

impl SolvedState {
  pub fn solved() -> SolvedState {
    SolvedState {
      solved: true,
      invalid_state_reason: None,
    }
  }

  pub fn unsolved(invalid_state_reason: InvalidStateReason) -> SolvedState {
    SolvedState {
      solved: false,
      invalid_state_reason: Some(invalid_state_reason),
    }
  }
}

impl Solver {
  pub(crate) fn check_solved(&self) -> SolvedState {
    self.check_grid_valid(false)
  }

  pub(crate) fn check_partially_solved(&self) -> SolvedState {
    self.check_grid_valid(true)
  }

  fn check_grid_valid(&self, allow_empty: bool) -> SolvedState {
    let mut cached_candidates: Vec<Vec<CandidateSet>> = vec![
      vec![ CandidateSet::new(); self.puzzle.grid_size ];
      self.puzzle.grid_size
    ];

    // Check that all cell values are valid and they still have candidates
    for cell in self.puzzle.all_grid_cells() {
      let value = self.grid[cell.row][cell.col];
      if value == 0 {
        if !allow_empty {
          return SolvedState::unsolved(
            InvalidStateReason {
              state_type: InvalidStateType::CellEmpty,
              area: Area::Cell(cell.row, cell.col),
              values: vec![],
            }
          )
        }

        let cell_candidates = self.compute_cell_candidates(&cell);
        cached_candidates[cell.row][cell.col] = cell_candidates.clone();
        if cell_candidates.is_empty() {
          return SolvedState::unsolved(
            InvalidStateReason {
              state_type: InvalidStateType::CellNoCandidates,
              area: Area::Cell(cell.row, cell.col),
              values: vec![],
            }
          )
        }
      } else if value < 1 || value > self.puzzle.grid_size as u32 {
        return SolvedState::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::CellInvalidValue,
            area: Area::Cell(cell.row, cell.col),
            values: vec![],
          }
        )
      }
    }

    // Check areas and special constraint rules
    for constraint in &self.puzzle.constraints {
      let constraint_region_areas = constraint.region_areas();
      for area in constraint_region_areas {
        let check = self.check_area_region_valid(&area);
        if !check.solved {
          return check
        }
      }

      let check = constraint.check_grid_special_validity(&self.grid, &cached_candidates);
      if !check.solved {
        return check
      }
    }

    SolvedState::solved()
  }

  fn check_area_region_valid(&self, area: &Area) -> SolvedState {
    let mut values = CandidateSet::new();
    let mut candidates = CandidateSet::new();

    let area_cells = self.puzzle.area_cells(area);
    for cell in &area_cells {
      let value = self.grid[cell.row][cell.col];
      if value == 0 {
        candidates.extend(self.compute_cell_candidates(cell));
        continue
      }
      if values.contains(&value) {
        return SolvedState::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::AreaValueConflict,
            area: area.clone(),
            values: vec![value],
          }
        )
      }
      values.insert(value);
    }

    candidates.extend(values);

    // There's less candidate values than area cells so there is no solution
    if candidates.len() < area_cells.len() {
      let values = if area_cells.len() == self.puzzle.grid_size {
        self.full_candidate_set()
          .difference(&candidates)
          .into_iter()
          .sorted()
          .collect()
      } else {
        vec![]
      };

      return SolvedState::unsolved(
        InvalidStateReason {
          state_type: InvalidStateType::AreaCandidates,
          area: area.clone(),
          values,
        }
      )
    }

    if area_cells.len() == self.puzzle.grid_size {
      // It's a full area, so we need to place all values
      // Check if all remaining candidate subsets fit into the remaining cells

      let mut value_cells: HashMap<u32, Vec<CellPosition>> = HashMap::new();
      for cell in self.filter_empty_cells(&area_cells) {
        for value in self.compute_cell_candidates(&cell) {
          let entry = value_cells.entry(value).or_insert(vec![]);
          entry.push(cell);
        }
      }

      let mut used_cells_set: HashSet<CellPosition> = HashSet::new();
      let mut values = vec![];
      for (value_index, (value, value_cells)) in value_cells.into_iter().sorted_by_key(|e| (e.1.len(), e.0)).enumerate() {
        used_cells_set.extend(value_cells);
        values.push(value);
        if value_index + 1 > used_cells_set.len() {
          return SolvedState::unsolved(
            InvalidStateReason {
              state_type: InvalidStateType::AreaCandidates,
              area: area.clone(),
              values,
            }
          )
        }
      }
    }

    SolvedState::solved()
  }
}

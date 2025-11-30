use crate::solver::Solver;
use crate::types::{Area, CellPosition, InvalidStateReason, InvalidStateType, KropkiDot, KropkiDotType};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use std::collections::{HashMap, HashSet};
use std::mem::swap;
use super::logical_solver::{technique::Technique, top_bottom_candidates::TopBottomCandidates};

#[derive(Serialize, Deserialize, Debug, PartialEq, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct SolvedState {
  pub solved: bool,
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
  pub fn check_solved(&self) -> SolvedState {
    self.check_grid_valid(false)
  }

  pub fn check_partially_solved(&self) -> SolvedState {
    self.check_grid_valid(true)
  }

  fn check_grid_valid(&self, allow_empty: bool) -> SolvedState {
    for cell in self.get_area_cells(&Area::Grid) {
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
        if cell_candidates.is_empty() {
          return SolvedState::unsolved(
            InvalidStateReason {
              state_type: InvalidStateType::CellNoCandidates,
              area: Area::Cell(cell.row, cell.col),
              values: vec![],
            }
          )
        }
      } else if value < 1 || value > self.constraints.grid_size as u32 {
        return SolvedState::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::CellInvalidValue,
            area: Area::Cell(cell.row, cell.col),
            values: vec![],
          }
        )
      }
    }

    for area in self.get_all_areas(true, true, true, true, true) {
      let check = self.check_area_valid(&area);
      if !check.solved {
        return check
      }
    }

    for arrow_index in 0..self.constraints.arrows.len() {
      let check = self.check_arrow_valid(arrow_index);
      if !check.solved {
        return check
      }
    }

    if self.constraints.anti_knight {
      let check = self.check_anti_knight_valid();
      if !check.solved {
        return check
      }
    }

    if self.constraints.anti_king {
      let check = self.check_anti_king_valid();
      if !check.solved {
        return check
      }
    }

    let check = self.check_odd_cells();
    if !check.solved {
      return check
    }

    let check = self.check_even_cells();
    if !check.solved {
      return check
    }

    if self.constraints.top_bottom {
      let check = self.check_top_bottom_valid();
      if !check.solved {
        return check
      }
    }

    SolvedState::solved()
  }

  fn check_area_valid(&self, area: &Area) -> SolvedState {
    match area {
      &Area::Row(_) | &Area::Column(_) | &Area::Region(_) |
        &Area::PrimaryDiagonal | &Area::SecondaryDiagonal => self.check_area_region_valid(area),
      &Area::KillerCage(killer_cage_index) => self.check_killer_area_valid(area, killer_cage_index),
      &Area::Thermo(_) => self.check_thermo_area_valid(area),
      &Area::KropkiDot(kropki_dot_index) => self.check_kropki_dot_valid(kropki_dot_index),
      &Area::Renban(_) => self.check_renban_valid(area),
      &Area::Palindrome(_) => self.check_palindrome_valid(area),
      &Area::Grid | &Area::Adhoc(_) | &Area::Cell(_, _) | &Area::Arrow(_) => unimplemented!(),
    }
  }

  fn check_area_region_valid(&self, area: &Area) -> SolvedState {
    let mut values = HashSet::new();
    let mut candidates = HashSet::new();

    let area_cells = self.get_area_cells(area);
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
      let values = if area_cells.len() == self.constraints.grid_size {
        self.compute_all_candidates().difference(&candidates).copied().sorted().collect()
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

    if area_cells.len() == self.constraints.grid_size {
      // It's a full area, so we need to place all values
      // Check if all remaining candidate subsets fit into the remaining cells

      let mut value_cells: HashMap<u32, Vec<CellPosition>> = HashMap::new();
      for cell in self.get_empty_area_cells(area) {
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

  fn check_thermo_area_valid(&self, area: &Area) -> SolvedState {
    let mut crt_max_value: u32 = 0;

    for CellPosition { row, col } in self.get_area_cells(area) {
      let value = self.grid[row][col];
      if value == 0 {
        continue
      }
      if value <= crt_max_value {
        return SolvedState::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::AreaConstraint,
            area: area.clone(),
            values: vec![crt_max_value, value],
          }
        )
      }
      crt_max_value = value
    }

    SolvedState::solved()
  }

  fn check_arrow_valid(&self, arrow_index: usize) -> SolvedState {
    let arrow = &self.constraints.arrows[arrow_index];
    let (arrow_sum, arrow_full) = self.arrow_arrow_sum(arrow);
    let (circle_number, circle_full) = self.arrow_circle_number(arrow);

    if arrow_full && circle_full {
      if arrow_sum == circle_number {
        return SolvedState::solved()
      }
      return SolvedState::unsolved(
        InvalidStateReason {
          state_type: InvalidStateType::AreaConstraint,
          area: Area::Arrow(arrow_index),
          values: vec![],
        }
      )
    }

    if circle_full {
      if arrow_sum <= circle_number {
        return SolvedState::solved()
      }

      return SolvedState::unsolved(
        InvalidStateReason {
          state_type: InvalidStateType::AreaConstraint,
          area: Area::Arrow(arrow_index),
          values: vec![],
        }
      )
    }

    SolvedState::solved()
  }

  fn check_renban_valid(&self, area: &Area) -> SolvedState {
    let check = self.check_area_region_valid(area);
    if !check.solved {
      return check
    }

    let mut min_value: u32 = self.constraints.grid_size as u32 + 1;
    let mut max_value: u32 = 0;
    let mut any_zero = false;
    let values = self.get_area_values(area);
    for &value in &values {
      if value == 0 {
        any_zero = true;
      }
      if value > 0 && value < min_value {
        min_value = value;
      }
      if value > max_value {
        max_value = value;
      }
    }

    if max_value == 0 {
      // All zeroes is valid (well, not necessarily, but it's not the checker's job)
      return SolvedState::solved()
    }

    if (!any_zero && max_value - min_value + 1 != values.len() as u32) ||
       (any_zero && max_value - min_value + 1 > values.len() as u32) {
      return SolvedState::unsolved(
        InvalidStateReason {
          state_type: InvalidStateType::AreaConstraint,
          area: area.clone(),
          values: vec![],
        }
      )
    }

    SolvedState::solved()
  }

  fn check_palindrome_valid(&self, area: &Area) -> SolvedState {
    let values = self.get_area_values(area);
    let mut left = 0;
    let mut right = values.len() - 1;
    while left < right {
      if values[left] != 0 && values[right] != 0 && values[left] != values[right] {
        return SolvedState::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::AreaConstraint,
            area: area.clone(),
            values: vec![left as u32, right as u32],
          }
        )
      }
      left += 1;
      right -= 1;
    }

    SolvedState::solved()
  }

  fn check_anti_knight_valid(&self) -> SolvedState {
    for cell in self.get_area_cells(&Area::Grid) {
      let value = self.grid[cell.row][cell.col];
      if value == 0 {
        continue
      }

      for peer in self.get_knight_peers(&cell) {
        let peer_value = self.grid[peer.row][peer.col];
        if peer_value == 0 {
          continue
        }
        if value == peer_value {
          return SolvedState::unsolved(
            InvalidStateReason {
              state_type: InvalidStateType::CellInvalidValue,
              area: Area::Cell(cell.row, cell.col),
              values: vec![value],
            }
          )
        }
      }
    }

    SolvedState::solved()
  }

  fn check_anti_king_valid(&self) -> SolvedState {
    for cell in self.get_area_cells(&Area::Grid) {
      let value = self.grid[cell.row][cell.col];
      if value == 0 {
        continue
      }

      for peer in self.get_king_peers(&cell) {
        let peer_value = self.grid[peer.row][peer.col];
        if peer_value == 0 {
          continue
        }
        if value == peer_value {
          return SolvedState::unsolved(
            InvalidStateReason {
              state_type: InvalidStateType::CellInvalidValue,
              area: Area::Cell(cell.row, cell.col),
              values: vec![value],
            }
          )
        }
      }
    }

    SolvedState::solved()
  }

  fn check_killer_area_valid(&self, area: &Area, killer_cage_index: usize) -> SolvedState {
    let check = self.check_area_region_valid(area);
    if !check.solved {
      return check
    }

    let mut sum: u32 = 0;
    let mut any_zero = false;
    for cell in self.get_area_cells(&area) {
      let value = self.grid[cell.row][cell.col];
      if value == 0 {
        any_zero = true;
      }
      sum += value;
    }

    let killer_cage = &self.constraints.killer_cages[killer_cage_index];
    if let Some(killer_sum) = killer_cage.sum {
      if sum != killer_sum && !any_zero || sum > killer_sum {
        return SolvedState::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::AreaConstraint,
            area: area.clone(),
            values: vec![],
          }
        )
      }
    }

    SolvedState::solved()
  }

  fn check_kropki_dot_valid(&self, kropki_dot_index: usize) -> SolvedState {
    let kropki_dot = &self.constraints.kropki_dots[kropki_dot_index];
    let KropkiDot { dot_type, cell_1, cell_2 } = kropki_dot;
    let mut value1 = self.grid[cell_1.row][cell_1.col];
    let mut value2 = self.grid[cell_2.row][cell_2.col];
    if value1 > value2 {
      swap(&mut value1, &mut value2);
    }
    if value1 == 0 {
      return SolvedState::solved()
    }

    let valid = match dot_type {
      KropkiDotType::Consecutive => {
        value1 + 1 == value2
      },
      KropkiDotType::Double => {
        value1 * 2 == value2
      },
      KropkiDotType::Negative => {
        value1 + 1 != value2 && value1 * 2 != value2
      },
    };

    if valid {
      return SolvedState::solved()
    }

    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::KropkiDot(kropki_dot_index),
        values: vec![],
      }
    )
  }

  fn check_odd_cells(&self) -> SolvedState {
    for cell in &self.constraints.odd_cells {
      let value = self.grid[cell.row][cell.col];
      if value != 0 && value % 2 == 0 {
        return SolvedState::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::CellInvalidValue,
            area: Area::Cell(cell.row, cell.col),
            values: vec![value],
          }
        )
      }
    }
    SolvedState::solved()
  }

  fn check_even_cells(&self) -> SolvedState {
    for cell in &self.constraints.even_cells {
      let value = self.grid[cell.row][cell.col];
      if value != 0 && value % 2 == 1 {
        return SolvedState::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::CellInvalidValue,
            area: Area::Cell(cell.row, cell.col),
            values: vec![value],
          }
        )
      }
    }
    SolvedState::solved()
  }

  fn check_top_bottom_valid(&self) -> SolvedState {
    let valid = TopBottomCandidates::new(true).run(&self).is_empty();

    if valid {
      return SolvedState::solved()
    }

    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Grid,
        values: vec![],
      }
    )
  }
}

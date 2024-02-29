use crate::solver::Solver;
use crate::types::{Area, CellPosition, InvalidStateReason, InvalidStateType, KropkiDot, KropkiDotType};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::mem::swap;
use super::logical_solver::{technique::Technique, top_bottom_candidates::TopBottomCandidates};

impl Solver {
  pub fn check_solved(&self) -> (bool, Option<InvalidStateReason>) {
    self.check_grid_valid(false)
  }

  pub fn check_partially_solved(&self) -> (bool, Option<InvalidStateReason>) {
    self.check_grid_valid(true)
  }

  fn check_grid_valid(&self, allow_empty: bool) -> (bool, Option<InvalidStateReason>) {
    for cell in self.get_area_cells(&Area::Grid) {
      let value = self.grid[cell.row][cell.col];
      if value == 0 {
        if !allow_empty {
          return (
            false,
            Some(InvalidStateReason {
              state_type: InvalidStateType::CellEmpty,
              area: Area::Cell(cell.row, cell.col),
              values: vec![],
            }),
          )
        }

        let cell_candidates = self.compute_cell_candidates(&cell);
        if cell_candidates.is_empty() {
          return (
            false,
            Some(InvalidStateReason {
              state_type: InvalidStateType::CellNoCandidates,
              area: Area::Cell(cell.row, cell.col),
              values: vec![],
            }),
          )
        }
      } else if value < 1 || value > self.constraints.grid_size as u32 {
        return (
          false,
          Some(InvalidStateReason {
            state_type: InvalidStateType::CellInvalidValue,
            area: Area::Cell(cell.row, cell.col),
            values: vec![],
          }),
        )
      }
    }

    for area in self.get_all_areas(true, true, true) {
      let check = self.check_area_valid(&area);
      if !check.0 {
        return check
      }
    }

    for arrow_index in 0..self.constraints.arrows.len() {
      let check = self.check_arrow_valid(arrow_index);
      if !check.0 {
        return check
      }
    }

    if self.constraints.anti_knight {
      let check = self.check_anti_knight_valid();
      if !check.0 {
        return check
      }
    }

    if self.constraints.anti_king {
      let check = self.check_anti_king_valid();
      if !check.0 {
        return check
      }
    }

    let check = self.check_odd_cells();
    if !check.0 {
      return check
    }

    let check = self.check_even_cells();
    if !check.0 {
      return check
    }

    if self.constraints.top_bottom {
      let check = self.check_top_bottom_valid();
      if !check.0 {
        return check
      }
    }

    (true, None)
  }

  fn check_area_valid(&self, area: &Area) -> (bool, Option<InvalidStateReason>) {
    match area {
      &Area::Row(_) | &Area::Column(_) | &Area::Region(_) |
        &Area::PrimaryDiagonal | &Area::SecondaryDiagonal => self.check_area_region_valid(area),
      &Area::KillerCage(killer_cage_index) => self.check_killer_area_valid(area, killer_cage_index),
      &Area::Thermo(_) => self.check_thermo_area_valid(area),
      &Area::KropkiDot(kropki_dot_index) => self.check_kropki_dot_valid(kropki_dot_index),
      &Area::Grid | &Area::Cell(_, _) | &Area::Arrow(_) => unimplemented!(),
    }
  }

  fn check_area_region_valid(&self, area: &Area) -> (bool, Option<InvalidStateReason>) {
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
        return (
          false,
          Some(InvalidStateReason {
            state_type: InvalidStateType::AreaValueConflict,
            area: *area,
            values: vec![value],
          }),
        )
      }
      values.insert(value);
    }

    candidates.extend(values);
    // Can't place some value in this area so there is no solution
    if candidates.len() < area_cells.len() {
      let values = if area_cells.len() == self.constraints.grid_size {
        self.compute_all_candidates().difference(&candidates).copied().sorted().collect()
      } else {
        vec![]
      };

      return (
        false,
        Some(InvalidStateReason {
          state_type: InvalidStateType::AreaCandidates,
          area: *area,
          values,
        }),
      )
    }

    if area_cells.len() == self.constraints.grid_size {
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
          return (
            false,
            Some(InvalidStateReason {
              state_type: InvalidStateType::AreaCandidates,
              area: *area,
              values,
            }),
          )
        }
      }
    }

    (true, None)
  }

  fn check_thermo_area_valid(&self, area: &Area) -> (bool, Option<InvalidStateReason>) {
    let mut crt_max_value: u32 = 0;

    for CellPosition { row, col } in self.get_area_cells(area) {
      let value = self.grid[row][col];
      if value == 0 {
        continue
      }
      if value <= crt_max_value {
        return (
          false,
          Some(InvalidStateReason {
            state_type: InvalidStateType::AreaConstraint,
            area: *area,
            values: vec![crt_max_value, value],
          }),
        )
      }
      crt_max_value = value
    }

    (true, None)
  }

  fn check_arrow_valid(&self, arrow_index: usize) -> (bool, Option<InvalidStateReason>) {
    let arrow = &self.constraints.arrows[arrow_index];
    let (arrow_sum, arrow_full) = self.arrow_arrow_sum(arrow);
    let (circle_number, circle_full) = self.arrow_circle_number(arrow);

    if arrow_full && circle_full {
      if arrow_sum == circle_number {
        return (true, None)
      }
      return (
        false,
        Some(InvalidStateReason {
          state_type: InvalidStateType::AreaConstraint,
          area: Area::Arrow(arrow_index),
          values: vec![],
        }),
      )
    }

    if circle_full {
      if arrow_sum <= circle_number {
        return (true, None)
      }

      return (
        false,
        Some(InvalidStateReason {
          state_type: InvalidStateType::AreaConstraint,
          area: Area::Arrow(arrow_index),
          values: vec![],
        }),
      )
    }

    (true, None)
  }

  fn check_anti_knight_valid(&self) -> (bool, Option<InvalidStateReason>) {
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
          return (
            false,
            Some(InvalidStateReason {
              state_type: InvalidStateType::CellInvalidValue,
              area: Area::Cell(cell.row, cell.col),
              values: vec![value],
            }),
          )
        }
      }
    }

    (true, None)
  }

  fn check_anti_king_valid(&self) -> (bool, Option<InvalidStateReason>) {
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
          return (
            false,
            Some(InvalidStateReason {
              state_type: InvalidStateType::CellInvalidValue,
              area: Area::Cell(cell.row, cell.col),
              values: vec![value],
            }),
          )
        }
      }
    }

    (true, None)
  }

  fn check_killer_area_valid(&self, area: &Area, killer_cage_index: usize) -> (bool, Option<InvalidStateReason>) {
    let check = self.check_area_region_valid(area);
    if !check.0 {
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
        return (
          false,
          Some(InvalidStateReason {
            state_type: InvalidStateType::AreaConstraint,
            area: *area,
            values: vec![],
          }),
        )
      }
    }

    (true, None)
  }

  fn check_kropki_dot_valid(&self, kropki_dot_index: usize) -> (bool, Option<InvalidStateReason>) {
    let kropki_dot = &self.constraints.kropki_dots[kropki_dot_index];
    let KropkiDot { dot_type, cell_1, cell_2 } = kropki_dot;
    let mut value1 = self.grid[cell_1.row][cell_1.col];
    let mut value2 = self.grid[cell_2.row][cell_2.col];
    if value1 > value2 {
      swap(&mut value1, &mut value2);
    }
    if value1 == 0 {
      return (true, None)
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
      return (true, None)
    }

    (
      false,
      Some(InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::KropkiDot(kropki_dot_index),
        values: vec![],
      }),
    )
  }

  fn check_odd_cells(&self) -> (bool, Option<InvalidStateReason>) {
    for cell in &self.constraints.odd_cells {
      let value = self.grid[cell.row][cell.col];
      if value != 0 && value % 2 == 0 {
        return (
          false,
          Some(InvalidStateReason {
            state_type: InvalidStateType::CellInvalidValue,
            area: Area::Cell(cell.row, cell.col),
            values: vec![value],
          }),
        )
      }
    }
    (true, None)
  }

  fn check_even_cells(&self) -> (bool, Option<InvalidStateReason>) {
    for cell in &self.constraints.even_cells {
      let value = self.grid[cell.row][cell.col];
      if value != 0 && value % 2 == 1 {
        return (
          false,
          Some(InvalidStateReason {
            state_type: InvalidStateType::CellInvalidValue,
            area: Area::Cell(cell.row, cell.col),
            values: vec![value],
          }),
        )
      }
    }
    (true, None)
  }

  fn check_top_bottom_valid(&self) -> (bool, Option<InvalidStateReason>) {
    let valid = TopBottomCandidates::new(true).run(&self).is_empty();

    if valid {
      return (true, None)
    }

    (
      false,
      Some(InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Grid,
        values: vec![],
      }),
    )
  }
}

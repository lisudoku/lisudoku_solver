use crate::solver::Solver;
use crate::types::{Area, CellPosition, KropkiDotType, KropkiDot, Arrow};
use std::collections::HashSet;
use std::mem::swap;
use super::logical_solver::{technique::Technique, top_bottom_candidates::TopBottomCandidates};

impl Solver {
  pub fn check_solved(&self) -> bool {
    self.check_grid_valid(false)
  }

  pub fn check_partially_solved(&self) -> bool {
    self.check_grid_valid(true)
  }

  fn check_grid_valid(&self, allow_empty: bool) -> bool {
    for CellPosition { row, col } in self.get_area_cells(&Area::Grid) {
      let value = self.grid[row][col];
      if value == 0 {
        if !allow_empty {
          return false
        }
      } else if value < 1 || value > self.constraints.grid_size as u32 {
        return false
      }
    }

    for area in self.get_all_areas(true, true, true) {
      if !self.check_area_valid(&area) {
        return false
      }
    }

    for arrow in &self.constraints.arrows {
      if !self.check_arrow_valid(arrow) {
        return false
      }
    }

    if self.constraints.anti_knight && !self.check_anti_knight_valid() {
      return false
    }

    if self.constraints.anti_king && !self.check_anti_king_valid() {
      return false
    }

    if !self.check_odd_cells() {
      return false
    }

    if !self.check_even_cells() {
      return false
    }

    if self.constraints.top_bottom && !self.check_top_bottom_valid() {
      return false
    }

    true
  }

  fn check_area_valid(&self, area: &Area) -> bool {
    match area {
      &Area::Row(_) | &Area::Column(_) | &Area::Region(_) |
        &Area::PrimaryDiagonal | &Area::SecondaryDiagonal => self.check_area_region_valid(area),
      &Area::KillerCage(killer_cage_index) => self.check_killer_area_valid(area, killer_cage_index),
      &Area::Thermo(_) => self.check_thermo_area_valid(area),
      &Area::KropkiDot(kropki_dot_index) => self.check_kropki_dot_valid(kropki_dot_index),
      &Area::Grid | &Area::Arrow(_) => unimplemented!(),
    }
  }

  fn check_area_region_valid(&self, area: &Area) -> bool {
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
        return false
      }
      values.insert(value);
    }

    candidates.extend(values);
    // Can't place some value in this area so there is no solution
    if candidates.len() < area_cells.len() {
      return false
    }

    true
  }

  fn check_thermo_area_valid(&self, area: &Area) -> bool {
    let mut crt_max_value: u32 = 0;

    for CellPosition { row, col } in self.get_area_cells(area) {
      let value = self.grid[row][col];
      if value == 0 {
        continue
      }
      if value <= crt_max_value {
        return false
      }
      crt_max_value = value
    }

    true
  }

  fn check_arrow_valid(&self, arrow: &Arrow) -> bool {
    let (arrow_sum, arrow_full) = self.arrow_arrow_sum(arrow);
    let (circle_number, circle_full) = self.arrow_circle_number(arrow);

    if arrow_full && circle_full {
      return arrow_sum == circle_number
    }

    if circle_full {
      return arrow_sum <= circle_number
    }

    true
  }

  fn check_anti_knight_valid(&self) -> bool {
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
          return false
        }
      }
    }

    true
  }

  fn check_anti_king_valid(&self) -> bool {
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
          return false
        }
      }
    }

    true
  }

  fn check_killer_area_valid(&self, area: &Area, killer_cage_index: usize) -> bool {
    if !self.check_area_region_valid(area) {
      return false
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
        return false
      }
    }

    true
  }

  fn check_kropki_dot_valid(&self, kropki_dot_index: usize) -> bool {
    let kropki_dot = &self.constraints.kropki_dots[kropki_dot_index];
    let KropkiDot { dot_type, cell_1, cell_2 } = kropki_dot;
    let mut value1 = self.grid[cell_1.row][cell_1.col];
    let mut value2 = self.grid[cell_2.row][cell_2.col];
    if value1 > value2 {
      swap(&mut value1, &mut value2);
    }
    if value1 == 0 {
      return true
    }

    match dot_type {
      KropkiDotType::Consecutive => {
        return value1 + 1 == value2
      },
      KropkiDotType::Double => {
        return value1 * 2 == value2
      },
      KropkiDotType::Negative => {
        return value1 + 1 != value2 && value1 * 2 != value2
      },
    }
  }

  fn check_odd_cells(&self) -> bool {
    self.constraints.odd_cells.iter().all(|cell| {
      let value = self.grid[cell.row][cell.col];
      value == 0 || value % 2 == 1
    })
  }

  fn check_even_cells(&self) -> bool {
    self.constraints.even_cells.iter().all(|cell| {
      let value = self.grid[cell.row][cell.col];
      value == 0 || value % 2 == 0
    })
  }

  fn check_top_bottom_valid(&self) -> bool {
    TopBottomCandidates::new(true).run(&self).is_empty()
  }
}

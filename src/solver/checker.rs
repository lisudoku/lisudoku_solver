use std::collections::HashSet;
use crate::solver::Solver;
use crate::types::{Area, CellPosition};

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

    for area in self.get_all_areas(true) {
      if !self.check_area_valid(&area) {
        return false
      }
    }

    true
  }

  fn check_area_valid(&self, area: &Area) -> bool {
    match area {
      Area::Row(_) | Area::Column(_) | Area::Region(_) => self.check_area_region_valid(area),
      Area::Thermo(_) => self.check_thermo_area_valid(area),
      Area::Grid => unimplemented!(),
    }
  }

  fn check_area_region_valid(&self, area: &Area) -> bool {
    let mut values = HashSet::new();

    for CellPosition { row, col } in self.get_area_cells(area) {
      let value = self.grid[row][col];
      if value == 0 {
        continue
      }
      if values.contains(&value) {
        return false
      }
      values.insert(value);
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
}

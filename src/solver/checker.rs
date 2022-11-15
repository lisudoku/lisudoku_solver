use std::collections::HashSet;
use crate::solver::Solver;
use crate::types::Region;

impl Solver {
  pub fn check_solved(&self) -> bool {
    for row in 0..self.constraints.grid_size {
      if !self.check_row_solved(row) {
        return false
      }
    }

    for col in 0..self.constraints.grid_size {
      if !self.check_col_solved(col) {
        return false
      }
    }

    for region in &self.constraints.regions {
      if !self.check_region_solved(region) {
        return false
      }
    }

    true
  }

  fn check_row_solved(&self, row: usize) -> bool {
    let mut values = HashSet::new();

    for col in 0..self.constraints.grid_size {
      let value = self.grid.as_ref().unwrap().values[row][col];
      if values.contains(&value) {
        return false
      }
      values.insert(value);
    }

    true
  }

  fn check_col_solved(&self, col: usize) -> bool {
    let mut values = HashSet::new();

    for row in 0..self.constraints.grid_size {
      let value = self.grid.as_ref().unwrap().values[row][col];
      if values.contains(&value) {
        return false
      }
      values.insert(value);
    }

    true
  }

  fn check_region_solved(&self, region: &Region) -> bool {
    let mut values = HashSet::new();

    for cell in region {
      let value = self.grid.as_ref().unwrap().values[cell.row][cell.col];
      if values.contains(&value) {
        return false
      }
      values.insert(value);
    }

    true
  }
}

use std::collections::HashSet;
use std::cmp::{min, max};
use crate::types::{SudokuConstraints, SudokuGrid, Grid, Area};

mod checker;
mod intuitive_solver;
mod brute_solver;

pub struct Solver {
  pub constraints: SudokuConstraints,
  pub grid: Grid,
  pub solution: Option<Grid>,
  grid_to_region: Vec<Vec<usize>>,
  grid_to_thermo: Vec<Vec<usize>>,
}

impl Solver {
  pub fn new(constraints: SudokuConstraints, input_grid: Option<SudokuGrid>) -> Solver {
    let mut grid_to_region = vec![ vec![ usize::MAX; constraints.grid_size ]; constraints.grid_size ];
    for (index, region) in constraints.regions.iter().enumerate() {
      for cell in region {
        grid_to_region[cell.row][cell.col] = index;
      }
    }

    let mut grid_to_thermo = vec![ vec![ usize::MAX; constraints.grid_size ]; constraints.grid_size ];
    for (index, thermo) in constraints.thermos.iter().enumerate() {
      for cell in thermo {
        grid_to_thermo[cell.row][cell.col] = index;
      }
    }

    let grid = if input_grid.is_some() {
      input_grid.unwrap().values
    } else {
      let mut initial_grid = vec![ vec![ 0; constraints.grid_size ]; constraints.grid_size ];
      for fixed_number in &constraints.fixed_numbers {
        initial_grid[fixed_number.position.row][fixed_number.position.col] = fixed_number.value;
      }
      initial_grid
    };

    Solver {
      constraints,
      grid,
      solution: None,
      grid_to_region,
      grid_to_thermo,
    }
  }

  fn compute_area_cell_candidates(&self, area: &Area, row: usize, col: usize) -> HashSet<u32> {
    match *area {
      Area::Row(row) => self.compute_row_values_candidates(row),
      Area::Column(col) => self.compute_col_values_candidates(col),
      Area::Region(region_index) => self.compute_region_values_candidates(region_index),
      Area::Thermo(thermo_index) => self.compute_thermo_cell_candidates(thermo_index, row, col),
    }
  }

  fn compute_row_values_candidates(&self, row: usize) -> HashSet<u32> {
    let mut set = self.compute_all_candidates();
    for col in 0..self.constraints.grid_size {
      if self.grid[row][col] != 0 {
        set.remove(&self.grid[row][col]);
      }
    }

    set
  }

  fn compute_col_values_candidates(&self, col: usize) -> HashSet<u32> {
    let mut set = self.compute_all_candidates();
    for row in 0..self.constraints.grid_size {
      if self.grid[row][col] != 0 {
        set.remove(&self.grid[row][col]);
      }
    }

    set
  }

  fn compute_region_values_candidates(&self, region_index: usize) -> HashSet<u32> {
    let mut set = self.compute_all_candidates();
    let region = &self.constraints.regions[region_index];
    for cell in region {
      if self.grid[cell.row][cell.col] != 0 {
        set.remove(&self.grid[cell.row][cell.col]);
      }
    }

    set
  }

  fn compute_thermo_cell_candidates(&self, thermo_index: usize, row: usize, col: usize) -> HashSet<u32> {
    let thermo = &self.constraints.thermos[thermo_index];

    let mut after = false;
    let mut max_before = 0;
    let mut min_after = self.constraints.grid_size as u32 + 1;

    for cell in thermo {
      if row == cell.row && col == cell.col {
        after = true;
        continue
      }
      let value = self.grid[cell.row][cell.col];
      if value == 0 {
        continue
      }

      if after {
        min_after = min(min_after, value);
      } else {
        max_before = max(max_before, value);
      }
    }

    let set: HashSet<u32> = (max_before+1..min_after).collect();

    set
  }

  fn compute_cell_candidates(&self, row: usize, col: usize) -> HashSet<u32> {
    let region_index = self.grid_to_region[row][col];
    let mut areas = vec![ Area::Row(row), Area::Column(col), Area::Region(region_index) ];
    let thermo_index = self.grid_to_thermo[row][col];
    if thermo_index != usize::MAX {
      areas.push(Area::Thermo(thermo_index));
    }
    // TODO: handle intersecting thermos

    let mut candidates = self.compute_all_candidates();
    for area in &areas {
      let area_set = self.compute_area_cell_candidates(area, row, col);
      candidates = candidates.intersection(&area_set).cloned().collect();
    }

    candidates
  }

  fn compute_all_candidates(&self) -> HashSet<u32> {
    (1..self.constraints.grid_size as u32 + 1).collect::<HashSet<u32>>()
  }
}

#[cfg(test)]
mod tests;

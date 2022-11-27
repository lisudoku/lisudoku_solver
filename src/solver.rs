use std::collections::HashSet;
use crate::types::{SudokuConstraints, SudokuSolveResult, SudokuGrid, CellPosition, Grid, SolutionStep, Area};

mod rules;
mod checker;

pub struct Solver {
  pub constraints: SudokuConstraints,
  pub grid: Grid,
  grid_to_region: Vec<Vec<usize>>,
}

impl Solver {
  pub fn new(constraints: SudokuConstraints, input_grid: Option<SudokuGrid>) -> Solver {
    let mut grid_to_region = vec![ vec![ 0; constraints.grid_size ]; constraints.grid_size ];
    for (index, region) in constraints.regions.iter().enumerate() {
      for cell in region {
        grid_to_region[cell.row][cell.col] = index;
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
      grid_to_region,
    }
  }

  pub fn intuitive_solve(&mut self) -> SudokuSolveResult {
    let mut empty_cell_count = self.constraints.grid_size.pow(2) as u32 - self.constraints.fixed_numbers.len() as u32;

    println!("{}", empty_cell_count);

    let mut steps: Vec<SolutionStep> = vec![];
    while empty_cell_count > 0 {
      let step = self.find_step_raw().unwrap();
      let pos = &step.cells[0];
      let CellPosition { row, col } = *pos;
      let value = step.values[0];

      self.grid[row][col] = value;
      empty_cell_count -= 1;

      println!("{} {} {}", row, col, value);

      steps.push(step);
    }

    let res = SudokuSolveResult {
      solution_count: 1,
      solution: self.grid.to_vec(),
      steps,
    };
    res
  }

  fn find_step_raw(&self) -> Option<SolutionStep> {
    let step = self.find_naked_singles();
    if step.is_some() {
      return step
    }

    let step = self.find_hidden_singles();
    if step.is_some() {
      return step
    }

    let step = self.find_thermo_steps();
    if step.is_some() {
      return step
    }

    // TODO: implement other rules

    None
  }

  fn compute_area_set(&self, area: &Area) -> HashSet<u32> {
    match *area {
      Area::Row(row) => self.compute_row_values_set(row),
      Area::Column(col) => self.compute_col_values_set(col),
      Area::Region(region_index) => self.compute_region_values_set(region_index),
      Area::Thermo(_) => todo!(),
    }
  }

  fn compute_row_values_set(&self, row: usize) -> HashSet<u32> {
    let mut set = HashSet::new();
    for col in 0..self.constraints.grid_size {
      if self.grid[row][col] != 0 {
        set.insert(self.grid[row][col]);
      }
    }

    set
  }

  fn compute_col_values_set(&self, col: usize) -> HashSet<u32> {
    let mut set = HashSet::new();
    for row in 0..self.constraints.grid_size {
      if self.grid[row][col] != 0 {
        set.insert(self.grid[row][col]);
      }
    }

    set
  }

  fn compute_region_values_set(&self, region_index: usize) -> HashSet<u32> {
    let mut set = HashSet::new();
    let region = &self.constraints.regions[region_index];
    for cell in region {
      if self.grid[cell.row][cell.col] != 0 {
        set.insert(self.grid[cell.row][cell.col]);
      }
    }

    set
  }

  fn compute_cell_candidates_set(&self, row: usize, col: usize) -> HashSet<u32> {
    let mut candidates = self.compute_all_candidates();
    let region_index = self.grid_to_region[row][col];
    let areas = [ Area::Row(row), Area::Column(col), Area::Region(region_index) ];

    for area in &areas {
      let area_set = self.compute_area_set(area);
      candidates = candidates.difference(&area_set).cloned().collect();
    }

    candidates
  }

  fn compute_all_candidates(&self) -> HashSet<u32> {
    (1..self.constraints.grid_size as u32 + 1).collect::<HashSet<u32>>()
  }
}

#[cfg(test)]
mod tests;

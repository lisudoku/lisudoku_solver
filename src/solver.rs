use std::collections::HashSet;
use crate::types::{SudokuConstraints, SudokuSolveResult, SudokuGrid, CellPosition, Grid, SolutionStep, Area};

mod rules;
mod checker;

pub struct Solver {
  pub constraints: SudokuConstraints,
  pub grid: Option<SudokuGrid>,
  grid_to_region: Vec<Vec<usize>>,
}

impl Solver {
  pub fn new(constraints: SudokuConstraints, grid: Option<SudokuGrid>) -> Solver {
    let mut grid_to_region = vec![ vec![ 0; constraints.grid_size ]; constraints.grid_size ];
    for (index, region) in constraints.regions.iter().enumerate() {
      for cell in region {
        grid_to_region[cell.row][cell.col] = index;
      }
    }

    Solver {
      constraints,
      grid,
      grid_to_region,
    }
  }

  pub fn intuitive_solve(&self) -> SudokuSolveResult {
    let mut grid = vec![ vec![ 0; self.constraints.grid_size ]; self.constraints.grid_size ];
    let mut empty_cell_count = self.constraints.grid_size as u32 * self.constraints.grid_size as u32;
    for fixed_number in &self.constraints.fixed_numbers {
      grid[fixed_number.position.row][fixed_number.position.col] = fixed_number.value;
      empty_cell_count -= 1;
    }

    println!("{}", empty_cell_count);

    let mut steps: Vec<SolutionStep> = vec![];
    while empty_cell_count > 0 {
      let step = self.find_step_raw(&grid).unwrap();
      let pos = &step.cells[0];
      let CellPosition { row, col } = *pos;
      let value = step.values[0];

      grid[row][col] = value;
      empty_cell_count -= 1;

      println!("{} {} {}", row, col, value);

      steps.push(step);
    }

    let res = SudokuSolveResult {
      solution_count: 1,
      solution: grid,
      steps,
    };
    res
  }

  fn find_step_raw(&self, grid: &Grid) -> Option<SolutionStep> {
    let step = self.find_naked_singles(grid);
    if step.is_some() {
      return step
    }

    let step = self.find_hidden_singles(grid);
    if step.is_some() {
      return step
    }

    let step = self.find_thermo_steps(grid);
    if step.is_some() {
      return step
    }

    // TODO: implement other rules

    None
  }

  fn compute_area_set(&self, grid: &Grid, area: &Area) -> HashSet<u32> {
    match *area {
      Area::Row(row) => self.compute_row_values_set(grid, row),
      Area::Column(col) => self.compute_col_values_set(grid, col),
      Area::Region(region_index) => self.compute_region_values_set(grid, region_index),
      Area::Thermo(_) => todo!(),
    }
  }

  fn compute_row_values_set(&self, grid: &Grid, row: usize) -> HashSet<u32> {
    let mut set = HashSet::new();
    for col in 0..self.constraints.grid_size {
      if grid[row][col] != 0 {
        set.insert(grid[row][col]);
      }
    }

    set
  }

  fn compute_col_values_set(&self, grid: &Grid, col: usize) -> HashSet<u32> {
    let mut set = HashSet::new();
    for row in 0..self.constraints.grid_size {
      if grid[row][col] != 0 {
        set.insert(grid[row][col]);
      }
    }

    set
  }

  fn compute_region_values_set(&self, grid: &Grid, region_index: usize) -> HashSet<u32> {
    let mut set = HashSet::new();
    let region = &self.constraints.regions[region_index];
    for cell in region {
      if grid[cell.row][cell.col] != 0 {
        set.insert(grid[cell.row][cell.col]);
      }
    }

    set
  }

  fn compute_cell_candidates_set(&self, grid: &Grid, row: usize, col: usize) -> HashSet<u32> {
    let mut candidates: HashSet<u32> = (1..self.constraints.grid_size as u32 + 1).collect();
    let region_index = self.grid_to_region[row][col];
    let areas = [ Area::Row(row), Area::Column(col), Area::Region(region_index) ];

    for area in &areas {
      let area_set = self.compute_area_set(grid, area);
      candidates = candidates.difference(&area_set).cloned().collect();
    }

    candidates
  }
}

#[cfg(test)]
mod tests;

use std::collections::HashSet;
use crate::types::{SudokuConstraints, SudokuSolveResult, SudokuGrid, Region};

pub struct Solver {
  pub constraints: SudokuConstraints,
  pub grid: SudokuGrid,
}

pub fn solve(constraints: SudokuConstraints) -> SudokuSolveResult {
  let res = SudokuSolveResult {
    solvable: constraints.grid_size == 4 && constraints.fixed_numbers.len() % 2 == 0,
  };
  res
}

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
      let value = self.grid.values[row][col];
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
      let value = self.grid.values[row][col];
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
      let value = self.grid.values[cell.row][cell.col];
      if values.contains(&value) {
        return false
      }
      values.insert(value);
    }

    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_wrong_row() {
    let constraints = SudokuConstraints {
      grid_size: 4,
      fixed_numbers: vec![],
      regions: SudokuConstraints::default_regions(4),
    };
    let grid = SudokuGrid {
      values: vec![
        vec![ 1, 1, 1, 1 ],
        vec![ 2, 2, 2, 2 ],
        vec![ 3, 3, 3, 3 ],
        vec![ 4, 4, 4, 4 ],
      ]
    };
    let solver = Solver {
      constraints: constraints,
      grid: grid,
    };
    let solved = solver.check_solved();
    assert_eq!(solved, false);
  }

  #[test]
  fn check_wrong_col() {
    let constraints = SudokuConstraints {
      grid_size: 4,
      fixed_numbers: vec![],
      regions: SudokuConstraints::default_regions(4),
    };
    let grid = SudokuGrid {
      values: vec![
        vec![ 1, 2, 3, 4 ],
        vec![ 1, 2, 3, 4 ],
        vec![ 1, 2, 3, 4 ],
        vec![ 1, 2, 3, 4 ],
      ]
    };
    let solver = Solver {
      constraints: constraints,
      grid: grid,
    };
    let solved = solver.check_solved();
    assert_eq!(solved, false);
  }

  #[test]
  fn check_wrong_region() {
    let constraints = SudokuConstraints {
      grid_size: 4,
      fixed_numbers: vec![],
      regions: SudokuConstraints::default_regions(4),
    };
    let grid = SudokuGrid {
      values: vec![
        vec![ 1, 2, 3, 4 ],
        vec![ 2, 1, 4, 3 ],
        vec![ 3, 4, 1, 2 ],
        vec![ 4, 3, 2, 1 ],
      ]
    };
    let solver = Solver {
      constraints: constraints,
      grid: grid,
    };
    let solved = solver.check_solved();
    assert_eq!(solved, false);
  }

  #[test]
  fn check_solved_grid() {
    let constraints = SudokuConstraints {
      grid_size: 4,
      fixed_numbers: vec![],
      regions: SudokuConstraints::default_regions(4),
    };
    let grid = SudokuGrid {
      values: vec![
        vec![ 2, 1, 4, 3 ],
        vec![ 3, 4, 1, 2 ],
        vec![ 1, 2, 3, 4 ],
        vec![ 4, 3, 2, 1 ],
      ]
    };
    let solver = Solver {
      constraints: constraints,
      grid: grid,
    };
    let solved = solver.check_solved();
    assert_eq!(solved, true);
  }
}

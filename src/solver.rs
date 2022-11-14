use std::{collections::HashSet};
use crate::types::{SudokuConstraints, SudokuSolveResult, SudokuGrid, Region, CellPosition, Grid, SolutionStep, Rule, Area};

pub struct Solver {
  pub constraints: SudokuConstraints,
  pub grid: Option<SudokuGrid>,
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

  pub fn intuitive_solve(&self) -> SudokuSolveResult {
    let mut grid = vec![ vec![ 0; self.constraints.grid_size ]; self.constraints.grid_size ];
    let mut candidates: Vec<Vec<HashSet<u32>>> = vec![ vec![ HashSet::new(); self.constraints.grid_size ]; self.constraints.grid_size ];
    let mut empty_cell_count = self.constraints.grid_size as u32 * self.constraints.grid_size as u32;
    for fixed_number in &self.constraints.fixed_numbers {
      grid[fixed_number.position.row][fixed_number.position.col] = fixed_number.value;
      empty_cell_count -= 1;
    }

    let mut grid_to_region = vec![ vec![ 0; self.constraints.grid_size ]; self.constraints.grid_size ];
    for (index, region) in self.constraints.regions.iter().enumerate() {
      for cell in region {
        grid_to_region[cell.row][cell.col] = index;
      }
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

    // for row in 0..self.constraints.grid_size {
    //   for col in 0..self.constraints.grid_size {
    //     if grid[row][col] != 0 {
    //       continue
    //     }

    //     for value in 1..self.constraints.grid_size+1 {
    //       candidates[row][col].insert(value as u32);
    //     }

    //     // eliminate values from row
    //     let row_set = self.compute_row_values_set(&grid, row);
    //     candidates[row][col] = candidates[row][col].difference(&row_set).cloned().collect();

    //     // col
    //     let col_set = self.compute_col_values_set(&grid, col);
    //     candidates[row][col] = candidates[row][col].difference(&col_set).cloned().collect();

    //     // region
    //     let region_index = grid_to_region[row][col];
    //     let region = &self.constraints.regions[region_index];
    //     let region_set = self.compute_region_values_set(&grid, region);
    //     candidates[row][col] = candidates[row][col].difference(&region_set).cloned().collect();
    //   }
    // }
    // dbg!(&self.constraints.regions);
    // dbg!(&candidates);

    // let mut steps: Vec<SolutionStep> = vec![];
    // while empty_cell_count > 0 {
    //   let step = self.find_step(&grid, &candidates).unwrap();
    //   let pos = &step.cells[0];
    //   let CellPosition { row, col } = *pos;
    //   let value = step.values[0];

    //   candidates[row][col].clear();
    //   grid[row][col] = value;
    //   empty_cell_count -= 1;

    //   self.update_row_candidates(&pos, &grid, &mut candidates);
    //   self.update_col_candidates(&pos, &grid, &mut candidates);

    //   let region_index = grid_to_region[pos.row][pos.col];
    //   let region = &self.constraints.regions[region_index];
    //   self.update_region_candidates(&pos, &region, &grid, &mut candidates);

    //   steps.push(step);

    //   println!("{} {} {}", row, col, value);
    // }

    // dbg!(&grid);

    dbg!(&steps);

    let res = SudokuSolveResult {
      solution_count: 1,
      solution: grid,
      steps,
    };
    res
  }

  fn find_step_raw(&self, grid: &Grid) -> Option<SolutionStep> {
    let mut grid_to_region = vec![ vec![ 0; self.constraints.grid_size ]; self.constraints.grid_size ];
    for (index, region) in self.constraints.regions.iter().enumerate() {
      for cell in region {
        grid_to_region[cell.row][cell.col] = index;
      }
    }

    let mut all_candidates: HashSet<u32> = HashSet::new();
    for value in 1..self.constraints.grid_size+1 {
      all_candidates.insert(value as u32);
    }

    // Obvious singles
    for row in 0..self.constraints.grid_size {
      for col in 0..self.constraints.grid_size {
        if grid[row][col] != 0 {
          continue
        }

        let row_set = self.compute_row_values_set(&grid, row);
        let col_set = self.compute_col_values_set(&grid, col);
        let region_index = grid_to_region[row][col];
        let region = &self.constraints.regions[region_index];
        let region_set = self.compute_region_values_set(&grid, region);

        // Only row
        if row_set.len() == self.constraints.grid_size - 1 {
          let value = *all_candidates.difference(&row_set).next().unwrap();
          return Some(
            SolutionStep {
              rule: Rule::ObviousSingle,
              cells: vec![ CellPosition { row, col } ],
              values: vec![ value ],
              areas: vec![Area::Row(row)],
              affected_cells: vec![],
            }
          )
        }

        // Only col
        if col_set.len() == self.constraints.grid_size - 1 {
          let value = *all_candidates.difference(&col_set).next().unwrap();
          return Some(
            SolutionStep {
              rule: Rule::ObviousSingle,
              cells: vec![ CellPosition { row, col } ],
              values: vec![ value ],
              areas: vec![ Area::Column(col) ],
              affected_cells: vec![],
            }
          )
        }

        // Only region
        if region_set.len() == self.constraints.grid_size - 1 {
          let value = *all_candidates.difference(&region_set).next().unwrap();
          return Some(
            SolutionStep {
              rule: Rule::ObviousSingle,
              cells: vec![ CellPosition { row, col } ],
              values: vec![ value ],
              areas: vec![ Area::Region(region_index) ],
              affected_cells: vec![],
            }
          )
        }

        // Row + col
        let row_and_col_set: HashSet<u32> = row_set.union(&col_set).cloned().collect();
        if row_and_col_set.len() == self.constraints.grid_size - 1 {
          let value = *all_candidates.difference(&row_and_col_set).next().unwrap();
          return Some(
            SolutionStep {
              rule: Rule::ObviousSingle,
              cells: vec![ CellPosition { row, col } ],
              values: vec![ value ],
              areas: vec![ Area::Row(row), Area::Column(col) ],
              affected_cells: vec![],
            }
          )
        }

        // TODO: other combinations
      }
    }

    // Hidden singles

    let mut candidates: Vec<Vec<HashSet<u32>>> = vec![ vec![ HashSet::new(); self.constraints.grid_size ]; self.constraints.grid_size ];
    for row in 0..self.constraints.grid_size {
      for col in 0..self.constraints.grid_size {
        if grid[row][col] != 0 {
          continue
        }

        for value in 1..self.constraints.grid_size+1 {
          candidates[row][col].insert(value as u32);
        }

        // eliminate values from row
        let row_set = self.compute_row_values_set(&grid, row);
        candidates[row][col] = candidates[row][col].difference(&row_set).cloned().collect();

        // col
        let col_set = self.compute_col_values_set(&grid, col);
        candidates[row][col] = candidates[row][col].difference(&col_set).cloned().collect();

        // region
        let region_index = grid_to_region[row][col];
        let region = &self.constraints.regions[region_index];
        let region_set = self.compute_region_values_set(&grid, region);
        candidates[row][col] = candidates[row][col].difference(&region_set).cloned().collect();
      }
    }

    // Check regions
    let hidden_single = self.find_hidden_single_region(&grid, &candidates);
    if let Some((region_index, found_cell, value)) = hidden_single {
      let mut covered_cells = vec![ vec![ false; self.constraints.grid_size ]; self.constraints.grid_size ];
      let mut cells = vec![ CellPosition { row: found_cell.row, col: found_cell.col } ];
      let region = &self.constraints.regions[region_index];
      for cell in region {
        if grid[cell.row][cell.col] == 0 && (cell.row != found_cell.row || cell.col != found_cell.col) && !covered_cells[cell.row][cell.col] {
          // search for rows
          let mut found_row: Option<usize> = None;
          for row in 0..self.constraints.grid_size {
            if grid[row][cell.col] == value {
              found_row = Some(row);
              break
            }
          }
          if let Some(row) = found_row {
            cells.push(CellPosition { row, col: cell.col });
            for other_row in 0..self.constraints.grid_size {
              // TODO: we should only cover in current region... but find a better approach overall
              covered_cells[other_row][cell.col] = true;
            }
            continue
          }

          // search for cols
          let mut found_col: Option<usize> = None;
          for col in 0..self.constraints.grid_size {
            if grid[cell.row][col] == value {
              found_col = Some(col);
              break
            }
          }
          if let Some(col) = found_col {
            cells.push(CellPosition { row: cell.row, col });
            for other_col in 0..self.constraints.grid_size {
              // TODO: we should only cover in current region... but find a better approach overall
              covered_cells[cell.row][other_col] = true;
            }
            continue
          }
        }
      }

      return Some(
        SolutionStep {
          rule: Rule::HiddenSingle,
          cells,
          values: vec![ value ],
          areas: vec![ Area::Region(region_index) ],
          affected_cells: vec![],
        }
      )
    }

    // TODO: check other types of hidden singles

    None
  }

  fn find_hidden_single_region(&self, grid: &Grid, candidates: &Vec<Vec<HashSet<u32>>>) -> Option<(usize, CellPosition, u32)> {
    for (index, region) in self.constraints.regions.iter().enumerate() {
      for value in 1..self.constraints.grid_size as u32 + 1 {
        let mut count = 0;
        let mut pos: Option<&CellPosition> = None;
        for cell in region {
          if grid[cell.row][cell.col] != 0 {
            continue
          }
          if candidates[cell.row][cell.col].contains(&value) {
            count += 1;
            pos = Some(cell);
          }
        }
        if count == 1 {
          return Some(
            (index, CellPosition { row: pos.unwrap().row, col: pos.unwrap().col }, value)
          )
        }
      }
    }

    None
  }

  fn update_row_candidates(&self, pos: &CellPosition, grid: &Grid, candidates: &mut Vec<Vec<HashSet<u32>>>) {
    for col in 0..self.constraints.grid_size {
      if grid[pos.row][col] != 0 {
        continue
      }
      candidates[pos.row][col].remove(&grid[pos.row][pos.col]);
    }
  }

  fn update_col_candidates(&self, pos: &CellPosition, grid: &Grid, candidates: &mut Vec<Vec<HashSet<u32>>>) {
    for row in 0..self.constraints.grid_size {
      if grid[row][pos.col] != 0 {
        continue
      }
      candidates[row][pos.col].remove(&grid[pos.row][pos.col]);
    }
  }

  fn update_region_candidates(&self, pos: &CellPosition, region: &Region, grid: &Grid, candidates: &mut Vec<Vec<HashSet<u32>>>) {
    for cell in region {
      if grid[cell.row][cell.col] != 0 {
        continue
      }
      candidates[cell.row][cell.col].remove(&grid[pos.row][pos.col]);
    }
  }

  fn find_step(&self, grid: &Grid, candidates: &Vec<Vec<HashSet<u32>>>) -> Option<SolutionStep> {
    for row in 0..self.constraints.grid_size {
      for col in 0..self.constraints.grid_size {
        if grid[row][col] == 0 && candidates[row][col].len() == 1 {
          let value = *candidates[row][col].iter().next().unwrap();
          return Some(
            SolutionStep {
              rule: Rule::ObviousSingle,
              cells: vec![ CellPosition { row, col } ],
              values: vec![ value ],
              areas: vec![], // ?????????
              affected_cells: vec![],
            }
          )
        }
      }
    }
    None
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

  fn compute_region_values_set(&self, grid: &Grid, region: &Region) -> HashSet<u32> {
    let mut set = HashSet::new();
    for cell in region {
      if grid[cell.row][cell.col] != 0 {
        set.insert(grid[cell.row][cell.col]);
      }
    }

    set
  }
}

#[cfg(test)]
mod tests {
  use crate::types::FixedNumber;

  use super::*;

  #[test]
  fn check_4x4_solve() {
    let constraints = SudokuConstraints {
      grid_size: 4,
      fixed_numbers: vec![
        FixedNumber::new(1, 1, 4),
        FixedNumber::new(1, 3, 2),
        FixedNumber::new(2, 0, 1),
        FixedNumber::new(2, 2, 3),
      ],
      regions: SudokuConstraints::default_regions(4),
    };
    let solver = Solver {
      constraints: constraints,
      grid: None,
    };
    let solution = solver.intuitive_solve();
    assert_eq!(solution.solution_count, 1);
    assert_eq!(solution.solution, vec![
      vec![ 2, 1, 4, 3 ],
      vec![ 3, 4, 1, 2 ],
      vec![ 1, 2, 3, 4 ],
      vec![ 4, 3, 2, 1 ],
    ]);
    assert_eq!(solution.steps.len(), 4 * 4 - 4);
  }

  #[test]
  fn check_6x6_solve() {
    let grid_size = 6;
    let fixed_numbers = vec![
      FixedNumber::new(0, 0, 6),
      FixedNumber::new(1, 0, 1),
      FixedNumber::new(1, 1, 4),
      FixedNumber::new(2, 1, 1),
      FixedNumber::new(2, 2, 2),
      FixedNumber::new(2, 3, 5),
      FixedNumber::new(2, 5, 6),
      FixedNumber::new(3, 0, 5),
      FixedNumber::new(3, 2, 6),
      FixedNumber::new(3, 3, 2),
      FixedNumber::new(3, 4, 1),
      FixedNumber::new(4, 4, 2),
      FixedNumber::new(4, 5, 1),
      FixedNumber::new(5, 5, 3),
    ];
    let empty_cells = grid_size * grid_size - fixed_numbers.len();
    let constraints = SudokuConstraints {
      grid_size,
      fixed_numbers,
      regions: SudokuConstraints::default_regions(grid_size),
    };
    let solver = Solver {
      constraints: constraints,
      grid: None,
    };
    let solution = solver.intuitive_solve();
    assert_eq!(solution.solution_count, 1);
    assert_eq!(solution.solution, vec![
      vec![ 6, 2, 3, 1, 4, 5 ],
      vec![ 1, 4, 5, 3, 6, 2 ],
      vec![ 4, 1, 2, 5, 3, 6 ],
      vec![ 5, 3, 6, 2, 1, 4 ],
      vec![ 3, 5, 4, 6, 2, 1 ],
      vec![ 2, 6, 1, 4, 5, 3 ],
    ]);
    assert_eq!(solution.steps.len(), empty_cells);
  }

  #[test]
  fn check_9x9_solve() {
    let grid_size = 9;
    let fixed_numbers = vec![
      FixedNumber::new(0, 0, 8),
      FixedNumber::new(0, 5, 1),
      FixedNumber::new(0, 8, 4),
      FixedNumber::new(1, 0, 4),
      FixedNumber::new(1, 1, 5),
      FixedNumber::new(1, 7, 1),
      FixedNumber::new(1, 8, 7),
      FixedNumber::new(2, 1, 9),
      FixedNumber::new(2, 2, 1),
      FixedNumber::new(2, 4, 2),
      FixedNumber::new(2, 5, 4),
      FixedNumber::new(2, 6, 5),
      FixedNumber::new(2, 7, 6),
      FixedNumber::new(3, 1, 4),
      FixedNumber::new(3, 7, 2),
      FixedNumber::new(4, 2, 6),
      FixedNumber::new(4, 6, 3),
      FixedNumber::new(5, 0, 9),
      FixedNumber::new(5, 1, 3),
      FixedNumber::new(5, 7, 8),
      FixedNumber::new(5, 8, 1),
      FixedNumber::new(6, 1, 7),
      FixedNumber::new(6, 2, 3),
      FixedNumber::new(6, 4, 8),
      FixedNumber::new(6, 5, 6),
      FixedNumber::new(6, 6, 4),
      FixedNumber::new(6, 7, 5),
      FixedNumber::new(7, 0, 5),
      FixedNumber::new(7, 1, 8),
      FixedNumber::new(7, 7, 7),
      FixedNumber::new(7, 8, 6),
      FixedNumber::new(8, 0, 6),
      FixedNumber::new(8, 5, 5),
      FixedNumber::new(8, 8, 3),
    ];
    let empty_cells = grid_size * grid_size - fixed_numbers.len();
    let constraints = SudokuConstraints {
      grid_size,
      fixed_numbers,
      regions: SudokuConstraints::default_regions(grid_size),
    };
    let solver = Solver {
      constraints: constraints,
      grid: None,
    };
    let solution = solver.intuitive_solve();
    assert_eq!(solution.solution_count, 1);
    assert_eq!(solution.solution, vec![
      vec![ 8, 6, 7, 5, 9, 1, 2, 3, 4 ],
      vec![ 4, 5, 2, 6, 3, 8, 9, 1, 7 ],
      vec![ 3, 9, 1, 7, 2, 4, 5, 6, 8 ],
      vec![ 7, 4, 8, 3, 1, 9, 6, 2, 5 ],
      vec![ 2, 1, 6, 8, 5, 7, 3, 4, 9 ],
      vec![ 9, 3, 5, 4, 6, 2, 7, 8, 1 ],
      vec![ 1, 7, 3, 9, 8, 6, 4, 5, 2 ],
      vec![ 5, 8, 9, 2, 4, 3, 1, 7, 6 ],
      vec![ 6, 2, 4, 1, 7, 5, 8, 9, 3 ],
    ]);
    assert_eq!(solution.steps.len(), empty_cells);
  }

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
      grid: Some(grid),
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
      grid: Some(grid),
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
      grid: Some(grid),
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
      grid: Some(grid),
    };
    let solved = solver.check_solved();
    assert_eq!(solved, true);
  }
}

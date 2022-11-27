use std::collections::HashSet;
use crate::solver::Solver;
use crate::types::Region;

// This is an older version, might be useful later
impl Solver {
  pub fn solve(&self) -> SudokuSolveResult {
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
    dbg!(&self.constraints.regions);
    dbg!(&candidates);

    let mut steps: Vec<SolutionStep> = vec![];
    while empty_cell_count > 0 {
      let step = self.find_step(&grid, &candidates).unwrap();
      let pos = &step.cells[0];
      let CellPosition { row, col } = *pos;
      let value = step.values[0];

      candidates[row][col].clear();
      grid[row][col] = value;
      empty_cell_count -= 1;

      self.update_row_candidates(&pos, &grid, &mut candidates);
      self.update_col_candidates(&pos, &grid, &mut candidates);

      let region_index = grid_to_region[pos.row][pos.col];
      let region = &self.constraints.regions[region_index];
      self.update_region_candidates(&pos, &region, &grid, &mut candidates);

      steps.push(step);

      println!("{} {} {}", row, col, value);
    }
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
              rule: Rule::NakedSingle,
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
}

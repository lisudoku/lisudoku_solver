use std::collections::HashSet;
use crate::solver::Solver;
use crate::types::{SolutionStep, Grid, CellPosition, Rule, Area};

impl Solver {
  pub fn find_obvious_singles(&self, grid: &Grid) -> Option<SolutionStep> {
    let mut all_candidates: HashSet<u32> = HashSet::new();
    for value in 1..self.constraints.grid_size+1 {
      all_candidates.insert(value as u32);
    }

    for row in 0..self.constraints.grid_size {
      for col in 0..self.constraints.grid_size {
        if grid[row][col] != 0 {
          continue
        }

        let row_set = self.compute_row_values_set(&grid, row);
        let col_set = self.compute_col_values_set(&grid, col);
        let region_index = self.grid_to_region[row][col];
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

    None
  }
}

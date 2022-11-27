use std::collections::HashSet;
use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule, Area};

impl Solver {
  pub fn find_hidden_singles(&self) -> Option<SolutionStep> {
    let mut candidates: Vec<Vec<HashSet<u32>>> = vec![ vec![ HashSet::new(); self.constraints.grid_size ]; self.constraints.grid_size ];
    for row in 0..self.constraints.grid_size {
      for col in 0..self.constraints.grid_size {
        if self.grid[row][col] != 0 {
          continue
        }

        for value in 1..self.constraints.grid_size+1 {
          candidates[row][col].insert(value as u32);
        }

        // eliminate values from row
        let row_set = self.compute_row_values_set(row);
        candidates[row][col] = candidates[row][col].difference(&row_set).cloned().collect();

        // col
        let col_set = self.compute_col_values_set(col);
        candidates[row][col] = candidates[row][col].difference(&col_set).cloned().collect();

        // region
        let region_index = self.grid_to_region[row][col];
        let region_set = self.compute_region_values_set(region_index);
        candidates[row][col] = candidates[row][col].difference(&region_set).cloned().collect();
      }
    }

    // Check regions
    let hidden_single = self.find_hidden_single_region(&candidates);
    if let Some((region_index, found_cell, value)) = hidden_single {
      let mut covered_cells = vec![ vec![ false; self.constraints.grid_size ]; self.constraints.grid_size ];
      let mut cells = vec![ CellPosition { row: found_cell.row, col: found_cell.col } ];
      let region = &self.constraints.regions[region_index];
      for cell in region {
        if self.grid[cell.row][cell.col] == 0 && (cell.row != found_cell.row || cell.col != found_cell.col) && !covered_cells[cell.row][cell.col] {
          // search for rows
          let mut found_row: Option<usize> = None;
          for row in 0..self.constraints.grid_size {
            if self.grid[row][cell.col] == value {
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
            if self.grid[cell.row][col] == value {
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

  fn find_hidden_single_region(&self, candidates: &Vec<Vec<HashSet<u32>>>) -> Option<(usize, CellPosition, u32)> {
    for (index, region) in self.constraints.regions.iter().enumerate() {
      for value in 1..self.constraints.grid_size as u32 + 1 {
        let mut count = 0;
        let mut pos: Option<&CellPosition> = None;
        for cell in region {
          if self.grid[cell.row][cell.col] != 0 {
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
}

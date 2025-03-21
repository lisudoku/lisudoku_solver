use std::mem::swap;
use std::collections::HashMap;
use itertools::Itertools;

use crate::solver::Solver;
use crate::types::{SolutionStep, Rule, Area, CellPosition};
use super::technique::Technique;

pub struct EmptyRectangles;

// Finds boxes X is on a row and col
impl Technique for EmptyRectangles {
  fn get_rule(&self) -> Rule { Rule::EmptyRectangles }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }

    let strong_links_by_value = solver.get_all_strong_links_by_value();

    for region_area in solver.get_region_areas() {
      let value_cells = solver.compute_cells_by_value_in_area(&region_area, &solver.candidates);

      for (value, region_cells) in value_cells.into_iter().sorted() {
        if region_cells.len() < 3 {
          continue
        }

        let (selected_row, selected_col) = self.find_most_frequent_row_col(&region_cells);

        // Check if all cells are on the row or column
        if region_cells.iter().any(|cell| cell.row != selected_row && cell.col != selected_col) {
          continue
        }
        // Check that not all of them are on the same row or col (that would be LockedCandidates)
        if region_cells.iter().all(|cell| cell.row == selected_row) ||
           region_cells.iter().all(|cell| cell.col == selected_col) {
          continue
        }

        let empty_strong_links = vec![];
        let strong_links = strong_links_by_value.get(&value).unwrap_or(&empty_strong_links);

        // Search for strong links that see selected_row
        for cell in solver.get_area_cells_with_candidate(&Area::Row(selected_row), value) {
          if region_cells.contains(&cell) {
            continue
          }
          let col = cell.col;

          for (area, _, mut c1, mut c2) in strong_links {
            if area != &Area::Column(col) || (c1.row != selected_row && c2.row != selected_row) {
              continue
            }

            if c2.row == selected_row {
              swap(&mut c1, &mut c2);
            }

            let common_cell = CellPosition::new(c2.row, selected_col);
            if solver.grid[common_cell.row][common_cell.col] == 0 &&
               solver.candidates[common_cell.row][common_cell.col].contains(&value) &&
               !region_cells.contains(&common_cell) {
              return vec![
                self.build_solution_step(
                  vec![ c1, c2 ], // the strong link
                  vec![ value ],
                  vec![ region_area, Area::Row(selected_row), Area::Column(selected_col) ],
                  vec![ common_cell ],
                )
              ]
            }
          }
        }

        // Search for strong links that see selected_col
        for cell in solver.get_area_cells_with_candidate(&Area::Column(selected_col), value) {
          if region_cells.contains(&cell) {
            continue
          }
          let row = cell.row;

          for (area, _, mut c1, mut c2) in strong_links {
            if area != &Area::Row(row) || (c1.col != selected_col && c2.col != selected_col) {
              continue
            }

            if c2.col == selected_col {
              swap(&mut c1, &mut c2);
            }

            let common_cell = CellPosition::new(selected_row, c2.col);
            if solver.grid[common_cell.row][common_cell.col] == 0 &&
               solver.candidates[common_cell.row][common_cell.col].contains(&value) &&
               !region_cells.contains(&common_cell) {
              return vec![
                self.build_solution_step(
                  vec![ c1, c2 ], // the strong link
                  vec![ value ],
                  vec![ region_area, Area::Row(selected_row), Area::Column(selected_col) ],
                  vec![ common_cell ],
                )
              ]
            }
          }
        }
      }
    }

    vec![]
  }
}

impl EmptyRectangles {
  fn find_most_frequent_row_col(&self, cells: &Vec<CellPosition>) -> (usize, usize) {
    let mut row_freq: HashMap<usize, u32> = HashMap::new();
    let mut col_freq: HashMap<usize, u32> = HashMap::new();
    for cell in cells {
      let row_entry = row_freq.entry(cell.row).or_default();
      *row_entry += 1;
      let col_entry = col_freq.entry(cell.col).or_default();
      *col_entry += 1;
    }

    let row = *row_freq.iter().max_by_key(|&(_, &freq)| freq).unwrap().0;
    let col = *col_freq.iter().max_by_key(|&(_, &freq)| freq).unwrap().0;

    (row, col)
  }
}

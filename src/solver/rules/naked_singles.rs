use std::collections::HashSet;
use crate::solver::Solver;
use crate::types::{SolutionStep, Grid, CellPosition, Rule, Area};
use combinations::Combinations;

impl Solver {
  pub fn find_naked_singles(&self, grid: &Grid) -> Option<SolutionStep> {
    for row in 0..self.constraints.grid_size {
      for col in 0..self.constraints.grid_size {
        if grid[row][col] != 0 {
          continue
        }

        let step = self.find_naked_single_in_cell(grid, row, col);
        if step.is_some() {
          return step
        }
      }
    }

    None
  }

  fn find_naked_single_in_cell(&self, grid: &Grid, row: usize, col: usize) -> Option<SolutionStep> {
    let region_index = self.grid_to_region[row][col];
    let candidate_areas = [ Area::Row(row), Area::Column(col), Area::Region(region_index) ];

    for area_count in 1..candidate_areas.len()+1 {
      let area_indexes: Vec<usize> = (0..candidate_areas.len()).collect();
      let area_combinations: Vec<_> = if area_count < area_indexes.len() {
        Combinations::new(area_indexes, area_count).collect()
      } else {
        // Has to be handled separately because of stupid crate
        vec![area_indexes]
      };

      for area_combination in area_combinations {
        let selected_areas = area_combination.iter().map(|index| &candidate_areas[*index]).collect();
        let step = self.find_naked_single_in_cell_and_areas(grid, row, col, selected_areas);
        if step.is_some() {
          return step
        }
      }
    }

    None
  }

  fn find_naked_single_in_cell_and_areas(&self, grid: &Grid, row: usize, col: usize, areas: Vec<&Area>) -> Option<SolutionStep> {
    let mut areas_set = HashSet::new();
    for area in &areas {
      let area_set = self.compute_area_set(grid, area);
      areas_set = areas_set.union(&area_set).cloned().collect();
    }
    if areas_set.len() == self.constraints.grid_size - 1 {
      let all_candidates: HashSet<u32> = (1..self.constraints.grid_size as u32 + 1).collect();
      let value = *all_candidates.difference(&areas_set).next().unwrap();
      return Some(
        SolutionStep {
          rule: Rule::NakedSingle,
          cells: vec![ CellPosition { row, col } ],
          values: vec![ value ],
          areas: areas.into_iter().map(|x| *x).collect(),
          affected_cells: vec![],
        }
      )
    }

    None
  }
}

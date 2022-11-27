use crate::solver::Solver;
use crate::types::{SolutionStep, Grid, Rule, Area};

impl Solver {
  pub fn find_thermo_steps(&self, grid: &Grid) -> Option<SolutionStep> {
    for thermo_index in 0..self.constraints.thermos.len() {
      let step = self.find_thermo_steps_for_thermo_min(grid, thermo_index);
      if step.is_some() {
        return step
      }

      let step = self.find_thermo_steps_for_thermo_max(grid, thermo_index);
      if step.is_some() {
        return step
      }
    }

    None
  }

  pub fn find_thermo_steps_for_thermo_min(&self, grid: &Grid, thermo_index: usize) -> Option<SolutionStep> {
    let thermo = &self.constraints.thermos[thermo_index];
    let mut current_min = 0;
    for cell in thermo {
      if grid[cell.row][cell.col] != 0 {
        current_min = grid[cell.row][cell.col];
        continue
      }

      let cell_candidates = self.compute_cell_candidates_set(grid, cell.row, cell.col);
      current_min += 1;

      let valid_candidates: Vec<_> = cell_candidates.iter().filter(|val| **val >= current_min).collect();
      if valid_candidates.len() == 1 {
        return Some(
          SolutionStep {
            rule: Rule::Thermo,
            cells: vec![ *cell ],
            values: vec![ *valid_candidates[0] ],
            areas: vec![ Area::Thermo(thermo_index) ],
            affected_cells: vec![],
          }
        )
      }

      let lowest_candidate = **valid_candidates.iter().min().unwrap();
      current_min = std::cmp::max(current_min, lowest_candidate);
    }

    None
  }

  pub fn find_thermo_steps_for_thermo_max(&self, grid: &Grid, thermo_index: usize) -> Option<SolutionStep> {
    let thermo = &self.constraints.thermos[thermo_index];
    let mut current_max = self.constraints.grid_size as u32 + 1;
    for cell in thermo.iter().rev() {
      if grid[cell.row][cell.col] != 0 {
        current_max = grid[cell.row][cell.col];
        continue
      }

      let cell_candidates = self.compute_cell_candidates_set(grid, cell.row, cell.col);
      current_max -= 1;

      let valid_candidates: Vec<_> = cell_candidates.iter().filter(|val| **val <= current_max).collect();
      if valid_candidates.len() == 1 {
        return Some(
          SolutionStep {
            rule: Rule::Thermo,
            cells: vec![ *cell ],
            values: vec![ *valid_candidates[0] ],
            areas: vec![ Area::Thermo(thermo_index) ],
            affected_cells: vec![],
          }
        )
      }

      let highest_candidate = **valid_candidates.iter().max().unwrap();
      current_max = std::cmp::min(current_max, highest_candidate);
    }

    None
  }
}

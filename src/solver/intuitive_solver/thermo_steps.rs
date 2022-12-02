use crate::solver::Solver;
use crate::types::{SolutionStep, Rule, Area};

impl Solver {
  pub fn find_thermo_steps(&self) -> Option<SolutionStep> {
    for thermo_index in 0..self.constraints.thermos.len() {
      let step = self.find_thermo_steps_for_thermo_min(thermo_index);
      if step.is_some() {
        return step
      }

      let step = self.find_thermo_steps_for_thermo_max(thermo_index);
      if step.is_some() {
        return step
      }
    }

    None
  }

  pub fn find_thermo_steps_for_thermo_min(&self, thermo_index: usize) -> Option<SolutionStep> {
    let thermo = &self.constraints.thermos[thermo_index];
    let mut current_min = 0;
    for cell in thermo {
      if self.grid[cell.row][cell.col] != 0 {
        current_min = self.grid[cell.row][cell.col];
        continue
      }

      let cell_candidates = self.compute_cell_candidates(cell.row, cell.col);
      current_min += 1;

      let valid_candidates: Vec<_> = cell_candidates.into_iter().filter(|val| *val >= current_min).collect();
      if valid_candidates.len() == 1 {
        return Some(
          SolutionStep {
            rule: Rule::Thermo,
            cells: vec![ *cell ],
            values: vec![ valid_candidates[0] ],
            areas: vec![ Area::Thermo(thermo_index) ],
            affected_cells: vec![],
            candidates: None,
          }
        )
      }
      if valid_candidates.is_empty() {
        return None
      }

      let lowest_candidate = valid_candidates.into_iter().min().unwrap();
      current_min = std::cmp::max(current_min, lowest_candidate);
    }

    None
  }

  pub fn find_thermo_steps_for_thermo_max(&self, thermo_index: usize) -> Option<SolutionStep> {
    let thermo = &self.constraints.thermos[thermo_index];
    let mut current_max = self.constraints.grid_size as u32 + 1;
    for cell in thermo.iter().rev() {
      if self.grid[cell.row][cell.col] != 0 {
        current_max = self.grid[cell.row][cell.col];
        continue
      }

      let cell_candidates = self.compute_cell_candidates(cell.row, cell.col);
      current_max -= 1;

      let valid_candidates: Vec<_> = cell_candidates.into_iter().filter(|val| *val <= current_max).collect();
      if valid_candidates.len() == 1 {
        return Some(
          SolutionStep {
            rule: Rule::Thermo,
            cells: vec![ *cell ],
            values: vec![ valid_candidates[0] ],
            areas: vec![ Area::Thermo(thermo_index) ],
            affected_cells: vec![],
            candidates: None,
          }
        )
      }
      if valid_candidates.is_empty() {
        return None
      }

      let highest_candidate = *valid_candidates.iter().max().unwrap();
      current_max = std::cmp::min(current_max, highest_candidate);
    }

    None
  }
}

use crate::solver::Solver;
use crate::types::{SolutionStep, Rule, Area, Thermo};

impl Solver {
  pub fn find_thermo_candidate_updates(&self) -> Vec<SolutionStep> {
    if !self.candidates_active {
      return vec![]
    }

    for (thermo_index, thermo) in self.constraints.thermos.iter().enumerate() {
      let lower_bounds = self.find_thermo_lower_bounds(&thermo);
      let upper_bounds = self.find_thermo_upper_bounds(&thermo);

      for (cell_index, cell) in thermo.iter().enumerate() {
        let invalid_values: Vec<u32> = self.candidates[cell.row][cell.col]
          .iter()
          .copied()
          .filter(|&value| value < lower_bounds[cell_index] || value > upper_bounds[cell_index])
          .collect();

        if !invalid_values.is_empty() {
          return vec![
            SolutionStep {
              rule: Rule::ThermoCandidates,
              cells: vec![],
              values: invalid_values,
              areas: vec![ Area::Thermo(thermo_index) ],
              affected_cells: vec![ *cell ],
              candidates: None,
            }
          ]
        }
      }
    }

    vec![]
  }

  pub fn find_thermo_lower_bounds(&self, thermo: &Thermo) -> Vec<u32> {
    let max_value: u32 = self.constraints.grid_size as u32 + 1;
    let mut current_min = 0;
    thermo.iter().map(|cell| {
      if self.grid[cell.row][cell.col] != 0 {
        current_min = self.grid[cell.row][cell.col];
      } else {
        let cell_candidates = self.compute_cell_candidates(cell);

        // If it reaches grid_sze it should result in a cell with no candidates
        if current_min < max_value {
          current_min += 1;
        }
  
        let valid_candidates: Vec<_> = cell_candidates.into_iter().filter(|&val| val >= current_min).collect();
  
        let lowest_candidate = valid_candidates.into_iter().min().unwrap_or(max_value);
        current_min = std::cmp::max(current_min, lowest_candidate);
      }
      current_min
    }).collect()
  }

  pub fn find_thermo_upper_bounds(&self, thermo: &Thermo) -> Vec<u32> {
    let mut current_max = self.constraints.grid_size as u32 + 1;
    thermo.iter().rev().map(|cell| {
      if self.grid[cell.row][cell.col] != 0 {
        current_max = self.grid[cell.row][cell.col];
      } else {
        let cell_candidates = self.compute_cell_candidates(cell);

        // If it reaches 0 it should result in a cell with no candidates
        if current_max > 0 {
          current_max -= 1;
        }
  
        let valid_candidates: Vec<_> = cell_candidates.into_iter().filter(|&val| val <= current_max).collect();
  
        let highest_candidate = valid_candidates.into_iter().max().unwrap_or(0);
        current_max = std::cmp::min(current_max, highest_candidate);
      }
      current_max
    }).collect::<Vec<u32>>().into_iter().rev().collect()
  }
}

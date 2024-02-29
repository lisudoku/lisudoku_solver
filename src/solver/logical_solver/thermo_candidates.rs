use crate::solver::Solver;
use crate::types::{SolutionStep, Rule, Area, Thermo};
use super::technique::Technique;

pub struct ThermoCandidates;

impl Technique for ThermoCandidates {
  fn is_candidate_validity_update_step(&self) -> bool { true }
  fn get_rule(&self) -> Rule { Rule::ThermoCandidates }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }

    for (thermo_index, thermo) in solver.constraints.thermos.iter().enumerate() {
      let lower_bounds = Self::find_thermo_lower_bounds(solver, &thermo);
      let upper_bounds = Self::find_thermo_upper_bounds(solver, &thermo);

      let steps: Vec<SolutionStep> = thermo.iter().enumerate().filter_map(|(cell_index, cell)| {
        let invalid_values: Vec<u32> = solver.candidates[cell.row][cell.col]
          .iter()
          .copied()
          .filter(|&value| value < lower_bounds[cell_index] || value > upper_bounds[cell_index])
          .collect();

        if invalid_values.is_empty() {
          return None
        }

        Some(self.build_simple_solution_step(
          invalid_values,
          vec![ Area::Thermo(thermo_index) ],
          vec![ *cell ],
        ))
      }).collect();

      if !steps.is_empty() {
        return steps
      }
    }

    vec![]
  }
}

impl ThermoCandidates {
  pub fn find_thermo_lower_bounds(solver: &Solver, thermo: &Thermo) -> Vec<u32> {
    let max_value: u32 = solver.constraints.grid_size as u32 + 1;
    let mut current_min = 0;
    thermo.iter().map(|cell| {
      if solver.grid[cell.row][cell.col] != 0 {
        current_min = solver.grid[cell.row][cell.col];
      } else {
        let cell_candidates = solver.compute_cell_candidates(cell);

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

  pub fn find_thermo_upper_bounds(solver: &Solver, thermo: &Thermo) -> Vec<u32> {
    let mut current_max = solver.constraints.grid_size as u32 + 1;
    thermo.iter().rev().map(|cell| {
      if solver.grid[cell.row][cell.col] != 0 {
        current_max = solver.grid[cell.row][cell.col];
      } else {
        let cell_candidates = solver.compute_cell_candidates(cell);

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

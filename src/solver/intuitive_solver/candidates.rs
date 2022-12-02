use std::collections::HashSet;
use crate::solver::Solver;
use crate::types::{SolutionStep, Rule};

impl Solver {
  pub fn find_candidates_step(&self) -> Option<SolutionStep> {
    if self.candidates_active {
      return None
    }

    let mut candidates: Vec<Vec<HashSet<u32>>> = vec![
      vec![ HashSet::new(); self.constraints.grid_size ];
      self.constraints.grid_size
    ];
    for row in 0..self.constraints.grid_size {
      for col in 0..self.constraints.grid_size {
        if self.grid[row][col] == 0 {
          candidates[row][col] = self.compute_cell_candidates(row, col);
        }
      }
    }

    return Some(
      SolutionStep {
        rule: Rule::Candidates,
        cells: vec![],
        values: vec![],
        areas: vec![],
        affected_cells: vec![],
        candidates: Some(candidates),
      }
    )
  }
}

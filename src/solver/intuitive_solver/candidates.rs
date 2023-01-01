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
    for cell in &self.get_all_empty_cells() {
      candidates[cell.row][cell.col] = self.compute_cell_candidates(cell);
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

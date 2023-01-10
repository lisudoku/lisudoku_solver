use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule, Area};

// X can't be a candidate in this cell because the other empty cells
// can't be assigned to make the sum Y
impl Solver {
  pub fn find_killer_candidate_updates(&self) -> Option<SolutionStep> {
    if !self.candidates_active {
      return None
    }

    for (killer_cage_index, killer_cage) in self.constraints.killer_cages.iter().enumerate() {
      if killer_cage.sum.is_none() {
        continue
      }
      let empty_cells: Vec<CellPosition> = killer_cage.region.iter().copied().filter(|cell| self.grid[cell.row][cell.col] == 0).collect();
      if empty_cells.is_empty() {
        continue
      }

      let sum: u32 = killer_cage.region.iter().map(|cell| self.grid[cell.row][cell.col]).sum();
      let total_sum = killer_cage.sum.unwrap();
      let rest_sum: u32 = total_sum - sum;

      let invalid_sum_candidates = self.detect_invalid_sum_candidates(&empty_cells, rest_sum);

      if invalid_sum_candidates.is_empty() {
        continue
      }

      // TODO: take all
      let (cell, invalid_values) = invalid_sum_candidates.into_iter().next().unwrap();

      return Some(
        SolutionStep {
          rule: Rule::KillerCandidates,
          cells: vec![],
          values: invalid_values,
          areas: vec![ Area::KillerCage(killer_cage_index) ],
          affected_cells: vec![ cell ],
          candidates: None,
        }
      )
    }

    None
  }
}

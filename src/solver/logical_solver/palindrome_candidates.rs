use crate::solver::Solver;
use crate::types::{Area, Rule, SolutionStep};
use super::technique::Technique;
use itertools::Itertools;

// We must keep in sync candidates for corresponding cells in a palindrome
pub struct PalindromeCandidates;

impl Technique for PalindromeCandidates {
  fn is_candidate_validity_update_step(&self) -> bool { true }

  fn get_rule(&self) -> Rule { Rule::PalindromeCandidates }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }

    solver.constraints.palindromes.iter().enumerate().flat_map(|(palindrome_index, palindrome)| {
      let area = Area::Palindrome(palindrome_index);
      let cells = palindrome;
      let mut left = 0;
      let mut right = cells.len() - 1;
      let mut steps: Vec<SolutionStep> = vec![];
      while left < right {
        let cell_left = cells[left];
        let cell_right = cells[right];
        if solver.grid[cell_left.row][cell_left.col] == 0 &&
           solver.grid[cell_right.row][cell_right.col] == 0 {
          let candidates_left = &solver.candidates[cell_left.row][cell_left.col];
          let candidates_right = &solver.candidates[cell_right.row][cell_right.col];

          let extra_candidates: Vec<u32> = candidates_left.difference(&candidates_right).sorted().copied().collect();
          if !extra_candidates.is_empty() {
            steps.push(
              self.build_simple_solution_step(
                extra_candidates,
                vec![area.clone()],
                vec![cell_left],
              )
            );
          }

          let extra_candidates: Vec<u32> = candidates_right.difference(&candidates_left).sorted().copied().collect();
          if !extra_candidates.is_empty() {
            steps.push(
              self.build_simple_solution_step(
                extra_candidates,
                vec![area.clone()],
                vec![cell_right],
              )
            );
          }
        }

        left += 1;
        right -= 1;
      }

      steps
    }).collect()
  }
}

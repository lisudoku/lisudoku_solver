use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::Technique}, types::{Area, Rule, SolutionStep}};
use itertools::Itertools;

// We must keep in sync candidates for corresponding cells in a palindrome
pub struct PalindromeCandidates;

impl Technique for PalindromeCandidates {
  fn is_candidate_validity_update_step(&self) -> bool { true }

  fn get_rule(&self) -> Rule { Rule::PalindromeCandidates }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    if !solver.state.candidates_active {
      return vec![]
    }

    let Some(constraint) = solver.state.puzzle.palindrome() else { return vec![] };
    constraint.palindromes.iter().enumerate().flat_map(|(palindrome_index, palindrome)| {
      let area = Area::Palindrome(palindrome_index);
      let cells = palindrome;
      let mut left = 0;
      let mut right = cells.len() - 1;
      let mut steps: Vec<SolutionStep> = vec![];
      while left < right {
        let cell_left = cells[left];
        let cell_right = cells[right];
        if solver.state.value_at(&cell_left) == 0 &&
           solver.state.value_at(&cell_right) == 0 {
          let candidates_left = &solver.state.candidates[cell_left.row][cell_left.col];
          let candidates_right = &solver.state.candidates[cell_right.row][cell_right.col];

          let extra_candidates: Vec<u32> = candidates_left.difference(&candidates_right).into_iter().sorted().collect();
          if !extra_candidates.is_empty() {
            steps.push(
              self.build_simple_solution_step(
                extra_candidates,
                vec![area.clone()],
                vec![cell_left],
              )
            );
          }

          let extra_candidates: Vec<u32> = candidates_right.difference(&candidates_left).into_iter().sorted().collect();
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

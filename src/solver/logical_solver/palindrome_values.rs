use crate::solver::Solver;
use crate::types::{Area, Rule, SolutionStep};
use super::technique::Technique;

// We must put X in this cell because the cell on the other side of the palindrome has the same value
pub struct PalindromeValues;

impl Technique for PalindromeValues {
  fn is_candidate_validity_update_step(&self) -> bool { true }

  fn is_grid_step(&self) -> bool { true }

  fn get_rule(&self) -> Rule { Rule::PalindromeValues }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    solver.constraints.palindromes.iter().enumerate().flat_map(|(palindrome_index, palindrome)| {
      let values = solver.get_area_values(&Area::Palindrome(palindrome_index));
      let mut left = 0;
      let mut right = values.len() - 1;
      let mut steps: Vec<SolutionStep> = vec![];
      while left < right {
        // one is 0 and the other is not
        if (values[left] == 0) != (values[right] == 0) {
          steps.push(
            self.build_grid_solution_step(
              vec![ if values[left] == 0 { palindrome[left] } else { palindrome[right] } ],
              vec![ if values[left] == 0 { values[right] } else { values[left] } ],
              vec![ Area::Palindrome(palindrome_index) ],
              &solver,
            )
          );
        }
        left += 1;
        right -= 1;
      }

      steps
    }).collect()
  }
}

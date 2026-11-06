use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{StepApplication, TechniqueDefinition, TechniqueKind, TechniquePhase}}, types::{Area, SolutionStep}};

// We must put X in this cell because the cell on the other side of the palindrome has the same value

#[derive(Clone, Debug)]
pub struct PalindromeValue;

impl TechniqueDefinition for PalindromeValue {
  fn kind(&self) -> TechniqueKind { TechniqueKind::PalindromeValue }

  // It would be awkward if it runs after PalindromeCandidateUpdate
  fn phase(&self) -> TechniquePhase { TechniquePhase::CandidateValidity }

  fn step_application(&self) -> StepApplication { StepApplication::SetCellValue }

  fn complexity(&self, _step: &SolutionStep, _solver: &LogicalSolver) -> u32 { 2 }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    let Some(constraint) = solver.state.puzzle.palindrome() else { return vec![] };

    constraint.palindromes.iter().enumerate().flat_map(|(palindrome_index, palindrome)| {
      let values = solver.state.area_values(&Area::Palindrome(palindrome_index));
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

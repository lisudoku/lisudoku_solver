use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::TechniqueKind}, types::{CellPosition, FixedNumber, Grid, Palindrome, SolutionType, SudokuConstraints}};

#[test]
fn check_palindrome_4x4_1_solve() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      [
        FixedNumber::new(0, 0, 1), FixedNumber::new(0, 2, 2),
        FixedNumber::new(1, 3, 1), FixedNumber::new(2, 1, 4),
      ]
    )
    .with_palindromes(
      [
        Palindrome(vec![
          CellPosition::new(1, 2), CellPosition::new(2, 2),
          CellPosition::new(3, 1), CellPosition::new(2, 0),
        ]),
      ]
    );

  let mut solver = LogicalSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert!(result.steps.iter().any(|step| step.technique_kind == TechniqueKind::PalindromeValue));
  assert!(result.steps.iter().all(|step| step.technique_kind != TechniqueKind::PalindromeCandidateUpdate));
  assert!(!solver.state.candidates_active);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 1, 3, 2, 4 ],
      vec![ 4, 2, 3, 1 ],
      vec![ 3, 4, 1, 2 ],
      vec![ 2, 1, 4, 3 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}

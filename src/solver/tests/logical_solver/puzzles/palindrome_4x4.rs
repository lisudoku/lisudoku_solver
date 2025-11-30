use crate::{solver::Solver, types::{CellPosition, FixedNumber, Grid, Palindrome, Rule, SolutionType, SudokuConstraints}};

#[test]
fn check_palindrome_4x4_1_solve() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 1), FixedNumber::new(0, 2, 2),
    FixedNumber::new(1, 3, 1), FixedNumber::new(2, 1, 4),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.palindromes = vec![
    Palindrome(vec![
      CellPosition::new(1, 2), CellPosition::new(2, 2),
      CellPosition::new(3, 1), CellPosition::new(2, 0),
    ]),
  ];

  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert!(result.steps.iter().any(|step| step.rule == Rule::PalindromeValues));
  assert!(result.steps.iter().all(|step| step.rule != Rule::PalindromeCandidates));
  assert!(!solver.candidates_active);
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

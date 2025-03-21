use std::rc::Rc;
use crate::{solver::{logical_solver::nishio_forcing_chains::NishioForcingChains, Solver}, types::{CellPosition, FixedNumber, Rule, SolutionType, SudokuConstraints}};

// https://github.com/lisudoku/lisudoku_solver/issues/65#issue-1926328789 WSC IB
#[test]
fn check_palindrome_9x9_1_solve() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 8), FixedNumber::new(0, 1, 2), FixedNumber::new(0, 2, 1),
    FixedNumber::new(0, 6, 9), FixedNumber::new(0, 7, 5), FixedNumber::new(0, 8, 3),
    FixedNumber::new(1, 3, 2), FixedNumber::new(1, 4, 3), FixedNumber::new(1, 5, 1),
    FixedNumber::new(4, 3, 1), FixedNumber::new(4, 5, 6),
    FixedNumber::new(6, 0, 5), FixedNumber::new(7, 0, 6), FixedNumber::new(8, 0, 1), FixedNumber::new(8, 1, 3),
    FixedNumber::new(6, 8, 4), FixedNumber::new(7, 8, 9), FixedNumber::new(8, 7, 8), FixedNumber::new(8, 8, 5),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.palindromes = vec![
    vec![
      CellPosition::new(3, 3), CellPosition::new(2, 2), CellPosition::new(1, 1),
      CellPosition::new(2, 0), CellPosition::new(3, 0), CellPosition::new(4, 0),
      CellPosition::new(5, 1), CellPosition::new(6, 2), CellPosition::new(7, 3),
      CellPosition::new(8, 4), CellPosition::new(7, 5), CellPosition::new(6, 6),
      CellPosition::new(5, 7),
    ],
    vec![
      CellPosition::new(4, 8), CellPosition::new(3, 8), CellPosition::new(2, 8),
      CellPosition::new(1, 7), CellPosition::new(2, 6), CellPosition::new(3, 5),
      CellPosition::new(3, 4), CellPosition::new(4, 4), CellPosition::new(5, 4),
    ],
  ];
  let mut solver = Solver::new(constraints, None).without_techniques(vec![Rc::new(NishioForcingChains)]);

  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 8, 2, 1, 4, 6, 7, 9, 5, 3 ],
    vec![ 9, 5, 7, 2, 3, 1, 8, 4, 6 ],
    vec![ 4, 6, 3, 8, 5, 9, 1, 7, 2 ],
    vec![ 3, 1, 8, 9, 2, 4, 5, 6, 7 ],
    vec![ 2, 9, 5, 1, 7, 6, 4, 3, 8 ],
    vec![ 7, 4, 6, 5, 8, 3, 2, 9, 1 ],
    vec![ 5, 7, 2, 6, 9, 8, 3, 1, 4 ],
    vec![ 6, 8, 4, 3, 1, 5, 7, 2, 9 ],
    vec![ 1, 3, 9, 7, 4, 2, 6, 8, 5 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
  let rules: Vec<_> = result.steps.iter().map(|step| step.rule).collect();
  assert!(rules.contains(&Rule::PalindromeValues));
  assert!(rules.contains(&Rule::PalindromeCandidates));
  insta::assert_yaml_snapshot!(result.steps);
}

// https://github.com/lisudoku/lisudoku_solver/issues/65#issuecomment-1928514241
#[test]
fn check_palindrome_9x9_2_solve() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 3, 4), FixedNumber::new(0, 5, 8), FixedNumber::new(1, 2, 3),
    FixedNumber::new(1, 6, 2), FixedNumber::new(2, 1, 2), FixedNumber::new(2, 7, 4),
    FixedNumber::new(3, 0, 1), FixedNumber::new(3, 8, 3), FixedNumber::new(5, 0, 7),
    FixedNumber::new(5, 8, 1), FixedNumber::new(6, 1, 6), FixedNumber::new(6, 7, 5),
    FixedNumber::new(7, 2, 5), FixedNumber::new(7, 6, 6), FixedNumber::new(8, 3, 3),
    FixedNumber::new(8, 5, 5),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.palindromes = vec![
    vec![
      CellPosition::new(4, 5), CellPosition::new(3, 4), CellPosition::new(4, 3),
      CellPosition::new(5, 4), CellPosition::new(6, 5), CellPosition::new(5, 6),
      CellPosition::new(4, 7), CellPosition::new(3, 6), CellPosition::new(2, 5),
      CellPosition::new(1, 4), CellPosition::new(2, 3), CellPosition::new(3, 2),
      CellPosition::new(4, 1), CellPosition::new(5, 2), CellPosition::new(6, 3),
      CellPosition::new(7, 4),
    ],
  ];
  let mut solver = Solver::new(constraints, None).without_techniques(vec![Rc::new(NishioForcingChains)]);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 5, 1, 9, 4, 2, 8, 7, 3, 6 ],
    vec![ 4, 8, 3, 1, 7, 6, 2, 9, 5 ],
    vec![ 6, 2, 7, 5, 3, 9, 1, 4, 8 ],
    vec![ 1, 5, 2, 7, 8, 4, 9, 6, 3 ],
    vec![ 3, 9, 8, 6, 5, 1, 4, 7, 2 ],
    vec![ 7, 4, 6, 2, 9, 3, 5, 8, 1 ],
    vec![ 9, 6, 1, 8, 4, 2, 3, 5, 7 ],
    vec![ 8, 3, 5, 9, 1, 7, 6, 2, 4 ],
    vec![ 2, 7, 4, 3, 6, 5, 8, 1, 9 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
  let rules: Vec<_> = result.steps.iter().map(|step| step.rule).collect();
  assert!(rules.contains(&Rule::PalindromeValues));
  assert!(rules.contains(&Rule::PalindromeCandidates));
  insta::assert_yaml_snapshot!(result.steps);
}

// https://github.com/lisudoku/lisudoku_solver/issues/65#issuecomment-2016619325
#[test]
fn check_palindrome_9x9_3_solve() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 6, 3), FixedNumber::new(0, 8, 5), FixedNumber::new(1, 5, 4),
    FixedNumber::new(2, 2, 8), FixedNumber::new(2, 4, 1), FixedNumber::new(2, 8, 4),
    FixedNumber::new(3, 1, 9), FixedNumber::new(3, 3, 8), FixedNumber::new(5, 3, 1),
    FixedNumber::new(5, 5, 3), FixedNumber::new(5, 7, 2), FixedNumber::new(6, 0, 6),
    FixedNumber::new(6, 8, 2), FixedNumber::new(7, 3, 7), FixedNumber::new(8, 2, 7),
    FixedNumber::new(8, 6, 9),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.palindromes = vec![
    vec![
      CellPosition::new(4, 6), CellPosition::new(3, 5), CellPosition::new(3, 4),
      CellPosition::new(4, 3), CellPosition::new(5, 4), CellPosition::new(6, 5),
      CellPosition::new(6, 6), CellPosition::new(6, 7), CellPosition::new(5, 8),
      CellPosition::new(4, 8), CellPosition::new(3, 8), CellPosition::new(2, 7),
      CellPosition::new(1, 6), CellPosition::new(0, 5), CellPosition::new(0, 4),
      CellPosition::new(0, 3), CellPosition::new(0, 2), CellPosition::new(1, 1),
      CellPosition::new(2, 0), CellPosition::new(3, 0), CellPosition::new(4, 0),
      CellPosition::new(5, 0), CellPosition::new(6, 1), CellPosition::new(7, 2),
      CellPosition::new(8, 3), CellPosition::new(8, 4), CellPosition::new(8, 5),
    ],
  ];
  let mut solver = Solver::new(constraints, None).without_techniques(vec![Rc::new(NishioForcingChains)]);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 4, 7, 1, 6, 8, 2, 3, 9, 5 ],
    vec![ 5, 6, 2, 9, 3, 4, 8, 1, 7 ],
    vec![ 9, 3, 8, 5, 1, 7, 2, 6, 4 ],
    vec![ 7, 9, 5, 8, 2, 6, 4, 3, 1 ],
    vec![ 1, 2, 3, 4, 7, 9, 5, 8, 6 ],
    vec![ 8, 4, 6, 1, 5, 3, 7, 2, 9 ],
    vec![ 6, 5, 9, 3, 4, 8, 1, 7, 2 ],
    vec![ 2, 8, 4, 7, 9, 1, 6, 5, 3 ],
    vec![ 3, 1, 7, 2, 6, 5, 9, 4, 8 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
  let rules: Vec<_> = result.steps.iter().map(|step| step.rule).collect();
  assert!(rules.contains(&Rule::PalindromeValues));
  assert!(rules.contains(&Rule::PalindromeCandidates));
  insta::assert_yaml_snapshot!(result.steps);
}

// https://github.com/lisudoku/lisudoku_solver/issues/65#issuecomment-2021598373
#[test]
fn check_palindrome_9x9_4_solve() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(1, 1, 4), FixedNumber::new(1, 3, 7), FixedNumber::new(1, 5, 1),
    FixedNumber::new(1, 7, 9), FixedNumber::new(2, 2, 1), FixedNumber::new(2, 6, 7),
    FixedNumber::new(3, 1, 1), FixedNumber::new(3, 3, 2), FixedNumber::new(3, 5, 4),
    FixedNumber::new(3, 7, 6), FixedNumber::new(5, 1, 6), FixedNumber::new(5, 3, 5),
    FixedNumber::new(5, 5, 8), FixedNumber::new(5, 7, 3), FixedNumber::new(6, 2, 4),
    FixedNumber::new(6, 6, 8), FixedNumber::new(7, 1, 9), FixedNumber::new(7, 3, 4),
    FixedNumber::new(7, 5, 2), FixedNumber::new(7, 7, 5),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.palindromes = vec![
    vec![
      CellPosition::new(5, 4), CellPosition::new(6, 5), CellPosition::new(7, 6),
      CellPosition::new(6, 7), CellPosition::new(5, 6), CellPosition::new(4, 5),
      CellPosition::new(3, 6), CellPosition::new(2, 7), CellPosition::new(1, 6),
      CellPosition::new(2, 5), CellPosition::new(3, 4), CellPosition::new(2, 3),
      CellPosition::new(1, 2), CellPosition::new(2, 1), CellPosition::new(3, 2),
      CellPosition::new(4, 3), CellPosition::new(5, 2), CellPosition::new(6, 1),
      CellPosition::new(7, 2), CellPosition::new(6, 3),
    ],
  ];
  let mut solver = Solver::new(constraints, None).without_techniques(vec![Rc::new(NishioForcingChains)]);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 6, 7, 9, 8, 4, 5, 2, 1, 3 ],
    vec![ 3, 4, 8, 7, 2, 1, 6, 9, 5 ],
    vec![ 2, 5, 1, 6, 9, 3, 7, 8, 4 ],
    vec![ 9, 1, 7, 2, 3, 4, 5, 6, 8 ],
    vec![ 5, 8, 3, 9, 6, 7, 1, 4, 2 ],
    vec![ 4, 6, 2, 5, 1, 8, 9, 3, 7 ],
    vec![ 7, 3, 4, 1, 5, 6, 8, 2, 9 ],
    vec![ 8, 9, 6, 4, 7, 2, 3, 5, 1 ],
    vec![ 1, 2, 5, 3, 8, 9, 4, 7, 6 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
  let rules: Vec<_> = result.steps.iter().map(|step| step.rule).collect();
  assert!(rules.contains(&Rule::PalindromeValues));
  assert!(rules.contains(&Rule::PalindromeCandidates));
  insta::assert_yaml_snapshot!(result.steps);
}

// https://github.com/lisudoku/lisudoku_solver/issues/65#issuecomment-2171772917
#[test]
fn check_palindrome_9x9_5_solve() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 3), FixedNumber::new(4, 4, 2), FixedNumber::new(8, 8, 1),
    FixedNumber::new(2, 6, 4), FixedNumber::new(2, 7, 5), FixedNumber::new(2, 8, 6),
    FixedNumber::new(3, 6, 7), FixedNumber::new(3, 7, 9), FixedNumber::new(3, 8, 8),
    FixedNumber::new(5, 0, 4), FixedNumber::new(5, 1, 6), FixedNumber::new(5, 2, 8),
    FixedNumber::new(6, 0, 5), FixedNumber::new(6, 1, 7), FixedNumber::new(6, 2, 3),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.palindromes = vec![
    vec![
      CellPosition::new(1, 3), CellPosition::new(0, 2), CellPosition::new(0, 1),
      CellPosition::new(1, 0), CellPosition::new(2, 0), CellPosition::new(3, 0),
      CellPosition::new(4, 1), CellPosition::new(4, 2), CellPosition::new(3, 3),
      CellPosition::new(2, 3), CellPosition::new(2, 2),
    ],
    vec![
      CellPosition::new(6, 6), CellPosition::new(6, 7), CellPosition::new(5, 8),
      CellPosition::new(4, 7), CellPosition::new(4, 6), CellPosition::new(5, 5),
      CellPosition::new(6, 5), CellPosition::new(7, 5), CellPosition::new(8, 5),
    ],
  ];
  let mut solver = Solver::new(constraints, None).without_techniques(vec![Rc::new(NishioForcingChains)]);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 3, 4, 1, 5, 8, 6, 9, 2, 7 ],
    vec![ 7, 5, 6, 2, 4, 9, 1, 8, 3 ],
    vec![ 9, 8, 2, 1, 3, 7, 4, 5, 6 ],
    vec![ 2, 3, 5, 4, 6, 1, 7, 9, 8 ],
    vec![ 1, 9, 7, 8, 2, 5, 6, 3, 4 ],
    vec![ 4, 6, 8, 7, 9, 3, 5, 1, 2 ],
    vec![ 5, 7, 3, 6, 1, 2, 8, 4, 9 ],
    vec![ 8, 1, 9, 3, 7, 4, 2, 6, 5 ],
    vec![ 6, 2, 4, 9, 5, 8, 3, 7, 1 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
  let rules: Vec<_> = result.steps.iter().map(|step| step.rule).collect();
  assert!(rules.contains(&Rule::PalindromeValues));
  assert!(rules.contains(&Rule::PalindromeCandidates));
  insta::assert_yaml_snapshot!(result.steps);
}

// https://github.com/lisudoku/lisudoku_solver/issues/65#issuecomment-2171781242
#[test]
fn check_palindrome_9x9_6_solve() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 1, 2), FixedNumber::new(1, 0, 1), FixedNumber::new(2, 3, 3),
    FixedNumber::new(2, 6, 4), FixedNumber::new(2, 8, 9), FixedNumber::new(3, 4, 5),
    FixedNumber::new(3, 5, 7), FixedNumber::new(3, 6, 6), FixedNumber::new(3, 8, 4),
    FixedNumber::new(5, 0, 3), FixedNumber::new(5, 2, 7), FixedNumber::new(5, 3, 8),
    FixedNumber::new(5, 4, 6), FixedNumber::new(6, 0, 7), FixedNumber::new(6, 2, 9),
    FixedNumber::new(6, 5, 1), FixedNumber::new(7, 8, 6), FixedNumber::new(8, 7, 7),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.palindromes = vec![
    vec![
      CellPosition::new(1, 3), CellPosition::new(1, 4), CellPosition::new(1, 5),
      CellPosition::new(1, 6), CellPosition::new(1, 7), CellPosition::new(2, 7),
      CellPosition::new(3, 7), CellPosition::new(4, 7), CellPosition::new(5, 7),
      CellPosition::new(6, 7), CellPosition::new(7, 7), CellPosition::new(8, 6),
      CellPosition::new(8, 5), CellPosition::new(8, 4), CellPosition::new(8, 3),
      CellPosition::new(8, 2), CellPosition::new(8, 1), CellPosition::new(7, 1),
      CellPosition::new(6, 1), CellPosition::new(5, 1), CellPosition::new(4, 1),
      CellPosition::new(3, 1), CellPosition::new(2, 1),
    ],
  ];
  let mut solver = Solver::new(constraints, None).without_techniques(vec![Rc::new(NishioForcingChains)]);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 9, 2, 8, 4, 1, 5, 7, 6, 3 ],
    vec![ 1, 4, 3, 7, 9, 6, 5, 8, 2 ],
    vec![ 5, 7, 6, 3, 8, 2, 4, 1, 9 ],
    vec![ 8, 9, 2, 1, 5, 7, 6, 3, 4 ],
    vec![ 4, 6, 1, 9, 2, 3, 8, 5, 7 ],
    vec![ 3, 5, 7, 8, 6, 4, 9, 2, 1 ],
    vec![ 7, 8, 9, 6, 3, 1, 2, 4, 5 ],
    vec![ 2, 1, 4, 5, 7, 8, 3, 9, 6 ],
    vec![ 6, 3, 5, 2, 4, 9, 1, 7, 8 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
  let rules: Vec<_> = result.steps.iter().map(|step| step.rule).collect();
  assert!(rules.contains(&Rule::PalindromeValues));
  assert!(rules.contains(&Rule::PalindromeCandidates));
  insta::assert_yaml_snapshot!(result.steps);
}

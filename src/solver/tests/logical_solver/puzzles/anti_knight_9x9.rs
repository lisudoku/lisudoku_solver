use crate::{types::{FixedNumber, SudokuConstraints, SolutionType}, solver::Solver};

// https://youtu.be/tHXXCW15bsk
#[test]
fn check_anti_knight_9x9_1_solve() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 6),
    FixedNumber::new(0, 7, 8),
    FixedNumber::new(0, 8, 9),
    FixedNumber::new(2, 2, 1),
    FixedNumber::new(2, 3, 2),
    FixedNumber::new(2, 4, 3),
    FixedNumber::new(3, 2, 4),
    FixedNumber::new(3, 3, 5),
    FixedNumber::new(3, 4, 6),
    FixedNumber::new(4, 2, 7),
    FixedNumber::new(4, 3, 8),
    FixedNumber::new(4, 4, 9),
    FixedNumber::new(5, 6, 4),
    FixedNumber::new(6, 5, 2),
    FixedNumber::new(7, 0, 3),
    FixedNumber::new(7, 7, 1),
    FixedNumber::new(7, 8, 2),
    FixedNumber::new(8, 0, 7),
    FixedNumber::new(8, 7, 4),
    FixedNumber::new(8, 8, 5),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.anti_knight = true;
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 6, 3, 5, 4, 7, 1, 2, 8, 9 ],
    vec![ 4, 7, 2, 9, 8, 5, 6, 3, 1 ],
    vec![ 9, 8, 1, 2, 3, 6, 5, 7, 4 ],
    vec![ 2, 1, 4, 5, 6, 3, 8, 9, 7 ],
    vec![ 5, 6, 7, 8, 9, 4, 1, 2, 3 ],
    vec![ 8, 9, 3, 1, 2, 7, 4, 5, 6 ],
    vec![ 1, 4, 9, 3, 5, 2, 7, 6, 8 ],
    vec![ 3, 5, 6, 7, 4, 8, 9, 1, 2 ],
    vec![ 7, 2, 8, 6, 1, 9, 3, 4, 5 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
  insta::assert_yaml_snapshot!(result.steps);
}

// https://www.funwithpuzzles.com/2018/02/no-knight-step-sudoku-puzzles.html
#[test]
fn check_anti_knight_9x9_2_solve() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 1),
    FixedNumber::new(0, 1, 4),
    FixedNumber::new(0, 2, 3),
    FixedNumber::new(1, 2, 5),
    FixedNumber::new(1, 3, 9),
    FixedNumber::new(1, 6, 3),
    FixedNumber::new(1, 8, 4),
    FixedNumber::new(2, 1, 2),
    FixedNumber::new(2, 3, 4),
    FixedNumber::new(2, 7, 8),
    FixedNumber::new(2, 8, 5),
    FixedNumber::new(3, 6, 2),
    FixedNumber::new(3, 7, 1),
    FixedNumber::new(5, 1, 3),
    FixedNumber::new(5, 2, 2),
    FixedNumber::new(6, 0, 3),
    FixedNumber::new(6, 1, 5),
    FixedNumber::new(6, 5, 7),
    FixedNumber::new(6, 7, 2),
    FixedNumber::new(7, 0, 2),
    FixedNumber::new(7, 2, 8),
    FixedNumber::new(7, 5, 9),
    FixedNumber::new(7, 6, 7),
    FixedNumber::new(8, 6, 8),
    FixedNumber::new(8, 7, 5),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.anti_knight = true;
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 1, 4, 3, 5, 7, 8, 9, 6, 2 ],
    vec![ 8, 6, 5, 9, 2, 1, 3, 7, 4 ],
    vec![ 9, 2, 7, 4, 6, 3, 1, 8, 5 ],
    vec![ 5, 9, 4, 7, 3, 6, 2, 1, 8 ],
    vec![ 7, 8, 1, 2, 9, 5, 4, 3, 6 ],
    vec![ 6, 3, 2, 1, 8, 4, 5, 9, 7 ],
    vec![ 3, 5, 9, 8, 4, 7, 6, 2, 1 ],
    vec![ 2, 1, 8, 6, 5, 9, 7, 4, 3 ],
    vec![ 4, 7, 6, 3, 1, 2, 8, 5, 9 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
  insta::assert_yaml_snapshot!(result.steps);
}

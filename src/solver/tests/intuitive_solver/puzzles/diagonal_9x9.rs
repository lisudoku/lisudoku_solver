use crate::{types::{FixedNumber, SudokuConstraints, SolutionType}, solver::Solver};

// TODO: uncomment after implementing more rules
// // https://ukpuzzles.org/file_download.php?fileid=247&md5=c200e06d8822177932d906103919ceba
// #[test]
// fn check_diagonal_9x9_1_hard_solve() {
//   let grid_size = 9;
//   let fixed_numbers = vec![
//     FixedNumber::new(0, 5, 3),
//     FixedNumber::new(0, 6, 8),
//     FixedNumber::new(1, 4, 4),
//     FixedNumber::new(1, 5, 7),
//     FixedNumber::new(2, 0, 6),
//     FixedNumber::new(2, 4, 2),
//     FixedNumber::new(3, 0, 8),
//     FixedNumber::new(3, 1, 9),
//     FixedNumber::new(4, 1, 2),
//     FixedNumber::new(4, 2, 3),
//     FixedNumber::new(4, 6, 7),
//     FixedNumber::new(4, 7, 8),
//     FixedNumber::new(5, 7, 3),
//     FixedNumber::new(5, 8, 6),
//     FixedNumber::new(6, 4, 3),
//     FixedNumber::new(6, 8, 4),
//     FixedNumber::new(7, 3, 1),
//     FixedNumber::new(7, 4, 6),
//     FixedNumber::new(8, 2, 7),
//     FixedNumber::new(8, 3, 8),
//   ];
//   let empty_cells = grid_size * grid_size - fixed_numbers.len();
//   let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
//   constraints.primary_diagonal = true;
//   constraints.secondary_diagonal = true;
//   let mut solver = Solver::new(constraints, None);
//   let result = solver.intuitive_solve();
//   assert_eq!(result.solution_type, SolutionType::Full);
//   assert_eq!(result.solution.unwrap(), vec![
//     vec![ 2, 5, 9, 6, 1, 3, 8, 4, 7 ],
//     vec![ 3, 8, 1, 9, 4, 7, 2, 6, 5 ],
//     vec![ 6, 7, 4, 5, 2, 8, 9, 1, 3 ],
//     vec![ 8, 9, 6, 3, 7, 1, 4, 5, 2 ],
//     vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9 ],
//     vec![ 7, 4, 5, 2, 8, 9, 1, 3, 6 ],
//     vec![ 5, 1, 8, 7, 3, 2, 6, 9, 4 ],
//     vec![ 9, 3, 2, 1, 6, 4, 5, 7, 8 ],
//     vec![ 4, 6, 7, 8, 9, 5, 3, 2, 1 ],
//   ]);
//   assert!(result.steps.len() >= empty_cells);
// }

// https://www.sudopedia.org/wiki/Sudoku-X
#[test]
fn check_diagonal_9x9_2_medium_solve() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(5, 0, 7),
    FixedNumber::new(5, 2, 4),
    FixedNumber::new(5, 4, 5),
    FixedNumber::new(5, 6, 3),
    FixedNumber::new(5, 8, 9),
    FixedNumber::new(6, 1, 5),
    FixedNumber::new(6, 3, 6),
    FixedNumber::new(6, 5, 7),
    FixedNumber::new(6, 7, 8),
    FixedNumber::new(7, 0, 9),
    FixedNumber::new(7, 2, 3),
    FixedNumber::new(7, 4, 4),
    FixedNumber::new(7, 6, 6),
    FixedNumber::new(7, 8, 7),
    FixedNumber::new(8, 1, 1),
    FixedNumber::new(8, 3, 2),
    FixedNumber::new(8, 5, 3),
    FixedNumber::new(8, 7, 4),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.primary_diagonal = true;
  constraints.secondary_diagonal = true;
  let mut solver = Solver::new(constraints, None);
  let result = solver.intuitive_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 3, 7, 1, 8, 6, 5, 2, 9, 4 ],
    vec![ 2, 6, 5, 9, 1, 4, 7, 3, 8 ],
    vec![ 8, 4, 9, 7, 3, 2, 5, 1, 6 ],
    vec![ 5, 3, 6, 4, 2, 9, 8, 7, 1 ],
    vec![ 1, 9, 8, 3, 7, 6, 4, 5, 2 ],
    vec![ 7, 2, 4, 1, 5, 8, 3, 6, 9 ],
    vec![ 4, 5, 2, 6, 9, 7, 1, 8, 3 ],
    vec![ 9, 8, 3, 5, 4, 1, 6, 2, 7 ],
    vec![ 6, 1, 7, 2, 8, 3, 9, 4, 5 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
}

// TODO: uncomment after implementing more rules
// // http://www.sudocue.net/minx.php
// #[test]
// fn check_diagonal_9x9_3_hard_solve() {
//   let grid_size = 9;
//   let fixed_numbers = vec![
//     FixedNumber::new(4, 8, 1),
//     FixedNumber::new(5, 2, 2),
//     FixedNumber::new(5, 3, 3),
//     FixedNumber::new(5, 4, 4),
//     FixedNumber::new(5, 5, 5),
//     FixedNumber::new(6, 0, 5),
//     FixedNumber::new(6, 1, 6),
//     FixedNumber::new(7, 0, 1),
//     FixedNumber::new(7, 7, 7),
//     FixedNumber::new(8, 3, 8),
//     FixedNumber::new(8, 6, 2),
//     FixedNumber::new(8, 7, 3),
//   ];
//   let empty_cells = grid_size * grid_size - fixed_numbers.len();
//   let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
//   constraints.primary_diagonal = true;
//   constraints.secondary_diagonal = true;
//   let mut solver = Solver::new(constraints, None);
//   let result = solver.intuitive_solve();
//   assert_eq!(result.solution_type, SolutionType::Full);
//   assert_eq!(result.solution.unwrap(), vec![
//     vec![ 3, 7, 1, 8, 6, 5, 2, 9, 4 ],
//   ]);
//   assert!(result.steps.len() >= empty_cells);
// }

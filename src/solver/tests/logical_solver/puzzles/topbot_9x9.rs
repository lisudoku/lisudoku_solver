use crate::{types::{FixedNumber, SudokuConstraints, SolutionType}, solver::Solver};

// German Sudoku Championship 2023 Qualification https://logic-masters.de/Wettbewerbe/CE/wettbewerb.php?id=216
#[test]
fn check_topbot_9x9_1() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(1, 1, 9), FixedNumber::new(1, 5, 7), FixedNumber::new(1, 7, 6),
    FixedNumber::new(2, 0, 4), FixedNumber::new(3, 2, 2), FixedNumber::new(3, 3, 8),
    FixedNumber::new(4, 4, 7), FixedNumber::new(4, 6, 1), FixedNumber::new(5, 8, 3),
    FixedNumber::new(6, 0, 5), FixedNumber::new(6, 3, 1), FixedNumber::new(7, 1, 4),
    FixedNumber::new(7, 8, 1),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers).with_top_bottom();
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 6, 7, 5, 4, 8, 1, 9, 3, 2 ],
    vec![ 1, 9, 3, 5, 2, 7, 8, 6, 4 ],
    vec![ 4, 2, 8, 6, 9, 3, 7, 1, 5 ],
    vec![ 3, 5, 2, 8, 1, 6, 4, 9, 7 ],
    vec![ 9, 6, 4, 3, 7, 5, 1, 2, 8 ],
    vec![ 8, 1, 7, 9, 4, 2, 6, 5, 3 ],
    vec![ 5, 8, 9, 1, 3, 4, 2, 7, 6 ],
    vec![ 7, 4, 6, 2, 5, 9, 3, 8, 1 ],
    vec![ 2, 3, 1, 7, 6, 8, 5, 4, 9 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
}

// WSC 2022 https://www.worldpuzzle.org/wpf-archive/2022/wsc-2022/
#[test]
fn check_topbot_9x9_2() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 6), FixedNumber::new(1, 1, 4), FixedNumber::new(2, 0, 9),
    FixedNumber::new(0, 3, 5), FixedNumber::new(1, 4, 1), FixedNumber::new(0, 5, 4),
    FixedNumber::new(0, 8, 2), FixedNumber::new(1, 7, 6), FixedNumber::new(2, 8, 5),
    FixedNumber::new(3, 1, 5), FixedNumber::new(5, 1, 9),
    FixedNumber::new(3, 7, 3), FixedNumber::new(5, 7, 5),
    FixedNumber::new(6, 0, 1), FixedNumber::new(7, 1, 6), FixedNumber::new(8, 0, 8),
    FixedNumber::new(8, 3, 4), FixedNumber::new(7, 4, 9), FixedNumber::new(8, 5, 2),
    FixedNumber::new(6, 8, 9), FixedNumber::new(7, 7, 7), FixedNumber::new(8, 8, 6),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers).with_top_bottom();
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![6, 3, 1, 5, 8, 4, 7, 9, 2],
    vec![7, 4, 5, 2, 1, 9, 8, 6, 3],
    vec![9, 8, 2, 3, 6, 7, 1, 4, 5],
    vec![2, 5, 7, 9, 4, 8, 6, 3, 1],
    vec![4, 1, 6, 7, 3, 5, 9, 2, 8],
    vec![3, 9, 8, 1, 2, 6, 4, 5, 7],
    vec![1, 2, 4, 6, 7, 3, 5, 8, 9],
    vec![5, 6, 3, 8, 9, 1, 2, 7, 4],
    vec![8, 7, 9, 4, 5, 2, 3, 1, 6],
  ]);
  assert!(result.steps.len() >= empty_cells);
}

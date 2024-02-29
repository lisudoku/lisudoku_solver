use crate::{types::{FixedNumber, SudokuConstraints, SolutionType, CellPosition}, solver::Solver};

// https://github.com/lisudoku/lisudoku_solver/issues/32
#[test]
fn check_odd_even_9x9_1_solve() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(1, 3, 8),
    FixedNumber::new(1, 4, 3),
    FixedNumber::new(2, 2, 2),
    FixedNumber::new(2, 6, 6),
    FixedNumber::new(2, 8, 1),
    FixedNumber::new(3, 3, 1),
    FixedNumber::new(3, 7, 4),
    FixedNumber::new(4, 1, 5),
    FixedNumber::new(4, 4, 7),
    FixedNumber::new(4, 7, 3),
    FixedNumber::new(5, 1, 4),
    FixedNumber::new(5, 5, 9),
    FixedNumber::new(6, 0, 1),
    FixedNumber::new(6, 2, 6),
    FixedNumber::new(6, 6, 2),
    FixedNumber::new(7, 4, 5),
    FixedNumber::new(7, 5, 8),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.even_cells = vec![
    CellPosition::new(0, 1), CellPosition::new(0, 7), CellPosition::new(1, 0),
    CellPosition::new(1, 8), CellPosition::new(2, 5), CellPosition::new(5, 2),
    CellPosition::new(7, 0), CellPosition::new(7, 8), CellPosition::new(8, 1),
    CellPosition::new(8, 7),
  ];
  constraints.odd_cells = vec![ CellPosition::new(3, 6), CellPosition::new(6, 3) ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 9, 6, 5, 2, 1, 7, 4, 8, 3 ],
    vec![ 4, 1, 7, 8, 3, 6, 5, 9, 2 ],
    vec![ 8, 3, 2, 5, 9, 4, 6, 7, 1 ],
    vec![ 3, 2, 9, 1, 8, 5, 7, 4, 6 ],
    vec![ 6, 5, 1, 4, 7, 2, 8, 3, 9 ],
    vec![ 7, 4, 8, 3, 6, 9, 1, 2, 5 ],
    vec![ 1, 9, 6, 7, 4, 3, 2, 5, 8 ],
    vec![ 2, 7, 3, 6, 5, 8, 9, 1, 4 ],
    vec![ 5, 8, 4, 9, 2, 1, 3, 6, 7 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
}

// https://gp.worldpuzzle.org/sites/default/files/Puzzles/2023/2023_SudokuRound1.pdf
#[test]
fn check_odd_even_9x9_2_solve() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 5),
    FixedNumber::new(1, 2, 3),
    FixedNumber::new(2, 1, 1),
    FixedNumber::new(2, 3, 8),
    FixedNumber::new(3, 2, 2),
    FixedNumber::new(3, 4, 9),
    FixedNumber::new(4, 3, 3),
    FixedNumber::new(4, 5, 1),
    FixedNumber::new(5, 4, 4),
    FixedNumber::new(5, 6, 2),
    FixedNumber::new(6, 5, 2),
    FixedNumber::new(6, 7, 3),
    FixedNumber::new(7, 6, 7),
    FixedNumber::new(8, 8, 4),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.even_cells = vec![
    CellPosition::new(0, 5), CellPosition::new(1, 6), CellPosition::new(2, 7),
    CellPosition::new(3, 8), CellPosition::new(3, 0), CellPosition::new(4, 1),
    CellPosition::new(5, 2), CellPosition::new(6, 3), CellPosition::new(7, 4),
    CellPosition::new(8, 5), CellPosition::new(7, 0), CellPosition::new(8, 1),
  ];
  constraints.odd_cells = vec![
    CellPosition::new(0, 3), CellPosition::new(1, 4), CellPosition::new(2, 5),
    CellPosition::new(3, 6), CellPosition::new(4, 7), CellPosition::new(5, 8),
    CellPosition::new(0, 7), CellPosition::new(1, 8), CellPosition::new(5, 0),
    CellPosition::new(6, 1), CellPosition::new(7, 2), CellPosition::new(8, 3),
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 5, 9, 8, 7, 6, 4, 3, 1, 2 ],
    vec![ 7, 6, 3, 2, 1, 5, 8, 4, 9 ],
    vec![ 2, 1, 4, 8, 3, 9, 5, 6, 7 ],
    vec![ 4, 3, 2, 6, 9, 7, 1, 5, 8 ],
    vec![ 9, 8, 5, 3, 2, 1, 4, 7, 6 ],
    vec![ 1, 7, 6, 5, 4, 8, 2, 9, 3 ],
    vec![ 8, 5, 9, 4, 7, 2, 6, 3, 1 ],
    vec![ 6, 4, 1, 9, 8, 3, 7, 2, 5 ],
    vec![ 3, 2, 7, 1, 5, 6, 9, 8, 4 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
}

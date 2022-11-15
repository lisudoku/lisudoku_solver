use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, SudokuGrid}, solver::Solver};

#[test]
fn check_6x6_thermo_solve() {
  let grid_size = 6;
  let fixed_numbers = vec![
    FixedNumber::new(1, 0, 4),
    FixedNumber::new(2, 0, 5),
    FixedNumber::new(4, 5, 2),
    FixedNumber::new(5, 4, 4),
    FixedNumber::new(5, 5, 3),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    vec![
      CellPosition { row: 0, col: 0 },
      CellPosition { row: 0, col: 1 },
      CellPosition { row: 0, col: 2 },
      CellPosition { row: 0, col: 3 },
      CellPosition { row: 0, col: 4 },
      CellPosition { row: 0, col: 5 },
    ],
    vec![
      CellPosition { row: 1, col: 4 },
      CellPosition { row: 2, col: 4 },
      CellPosition { row: 3, col: 4 },
    ],
    vec![
      CellPosition { row: 2, col: 2 },
      CellPosition { row: 3, col: 2 },
      CellPosition { row: 4, col: 2 },
      CellPosition { row: 4, col: 3 },
    ],
    vec![
      CellPosition { row: 3, col: 0 },
      CellPosition { row: 4, col: 0 },
      CellPosition { row: 5, col: 0 },
    ],
    vec![
      CellPosition { row: 3, col: 3 },
      CellPosition { row: 2, col: 3 },
      CellPosition { row: 1, col: 3 },
      CellPosition { row: 1, col: 2 },
    ],
  ];
  let solver = Solver::new(constraints, None);
  let solution = solver.intuitive_solve();
  assert_eq!(solution.solution_count, 1);
  assert_eq!(solution.solution, vec![
    vec![ 1, 2, 3, 4, 5, 6 ],
    vec![ 4, 5, 6, 3, 2, 1 ],
    vec![ 5, 6, 1, 2, 3, 4 ],
    vec![ 2, 3, 4, 1, 6, 5 ],
    vec![ 3, 4, 5, 6, 1, 2 ],
    vec![ 6, 1, 2, 5, 4, 3 ],
  ]);
  assert_eq!(solution.steps.len(), empty_cells);
}

#[test]
fn check_4x4_solve() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(1, 1, 4),
    FixedNumber::new(1, 3, 2),
    FixedNumber::new(2, 0, 1),
    FixedNumber::new(2, 2, 3),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let solver = Solver::new(constraints, None);
  let solution = solver.intuitive_solve();
  assert_eq!(solution.solution_count, 1);
  assert_eq!(solution.solution, vec![
    vec![ 2, 1, 4, 3 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 1, 2, 3, 4 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  assert_eq!(solution.steps.len(), empty_cells);
}

#[test]
fn check_6x6_solve() {
  let grid_size = 6;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 6),
    FixedNumber::new(1, 0, 1),
    FixedNumber::new(1, 1, 4),
    FixedNumber::new(2, 1, 1),
    FixedNumber::new(2, 2, 2),
    FixedNumber::new(2, 3, 5),
    FixedNumber::new(2, 5, 6),
    FixedNumber::new(3, 0, 5),
    FixedNumber::new(3, 2, 6),
    FixedNumber::new(3, 3, 2),
    FixedNumber::new(3, 4, 1),
    FixedNumber::new(4, 4, 2),
    FixedNumber::new(4, 5, 1),
    FixedNumber::new(5, 5, 3),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let solver = Solver::new(constraints, None);
  let solution = solver.intuitive_solve();
  assert_eq!(solution.solution_count, 1);
  assert_eq!(solution.solution, vec![
    vec![ 6, 2, 3, 1, 4, 5 ],
    vec![ 1, 4, 5, 3, 6, 2 ],
    vec![ 4, 1, 2, 5, 3, 6 ],
    vec![ 5, 3, 6, 2, 1, 4 ],
    vec![ 3, 5, 4, 6, 2, 1 ],
    vec![ 2, 6, 1, 4, 5, 3 ],
  ]);
  assert_eq!(solution.steps.len(), empty_cells);
}

#[test]
fn check_9x9_solve() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 8),
    FixedNumber::new(0, 5, 1),
    FixedNumber::new(0, 8, 4),
    FixedNumber::new(1, 0, 4),
    FixedNumber::new(1, 1, 5),
    FixedNumber::new(1, 7, 1),
    FixedNumber::new(1, 8, 7),
    FixedNumber::new(2, 1, 9),
    FixedNumber::new(2, 2, 1),
    FixedNumber::new(2, 4, 2),
    FixedNumber::new(2, 5, 4),
    FixedNumber::new(2, 6, 5),
    FixedNumber::new(2, 7, 6),
    FixedNumber::new(3, 1, 4),
    FixedNumber::new(3, 7, 2),
    FixedNumber::new(4, 2, 6),
    FixedNumber::new(4, 6, 3),
    FixedNumber::new(5, 0, 9),
    FixedNumber::new(5, 1, 3),
    FixedNumber::new(5, 7, 8),
    FixedNumber::new(5, 8, 1),
    FixedNumber::new(6, 1, 7),
    FixedNumber::new(6, 2, 3),
    FixedNumber::new(6, 4, 8),
    FixedNumber::new(6, 5, 6),
    FixedNumber::new(6, 6, 4),
    FixedNumber::new(6, 7, 5),
    FixedNumber::new(7, 0, 5),
    FixedNumber::new(7, 1, 8),
    FixedNumber::new(7, 7, 7),
    FixedNumber::new(7, 8, 6),
    FixedNumber::new(8, 0, 6),
    FixedNumber::new(8, 5, 5),
    FixedNumber::new(8, 8, 3),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let solver = Solver::new(constraints, None);
  let solution = solver.intuitive_solve();
  assert_eq!(solution.solution_count, 1);
  assert_eq!(solution.solution, vec![
    vec![ 8, 6, 7, 5, 9, 1, 2, 3, 4 ],
    vec![ 4, 5, 2, 6, 3, 8, 9, 1, 7 ],
    vec![ 3, 9, 1, 7, 2, 4, 5, 6, 8 ],
    vec![ 7, 4, 8, 3, 1, 9, 6, 2, 5 ],
    vec![ 2, 1, 6, 8, 5, 7, 3, 4, 9 ],
    vec![ 9, 3, 5, 4, 6, 2, 7, 8, 1 ],
    vec![ 1, 7, 3, 9, 8, 6, 4, 5, 2 ],
    vec![ 5, 8, 9, 2, 4, 3, 1, 7, 6 ],
    vec![ 6, 2, 4, 1, 7, 5, 8, 9, 3 ],
  ]);
  assert_eq!(solution.steps.len(), empty_cells);
}

#[test]
fn check_wrong_row() {
  let constraints = SudokuConstraints::new(4, vec![]);
  let grid = SudokuGrid {
    values: vec![
      vec![ 1, 1, 1, 1 ],
      vec![ 2, 2, 2, 2 ],
      vec![ 3, 3, 3, 3 ],
      vec![ 4, 4, 4, 4 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(solved, false);
}

#[test]
fn check_wrong_col() {
  let constraints = SudokuConstraints::new(4, vec![]);
  let grid = SudokuGrid {
    values: vec![
      vec![ 1, 2, 3, 4 ],
      vec![ 1, 2, 3, 4 ],
      vec![ 1, 2, 3, 4 ],
      vec![ 1, 2, 3, 4 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(solved, false);
}

#[test]
fn check_wrong_region() {
  let constraints = SudokuConstraints::new(4, vec![]);
  let grid = SudokuGrid {
    values: vec![
      vec![ 1, 2, 3, 4 ],
      vec![ 2, 1, 4, 3 ],
      vec![ 3, 4, 1, 2 ],
      vec![ 4, 3, 2, 1 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(solved, false);
}

#[test]
fn check_solved_grid() {
  let constraints = SudokuConstraints::new(4, vec![]);
  let grid = SudokuGrid {
    values: vec![
      vec![ 2, 1, 4, 3 ],
      vec![ 3, 4, 1, 2 ],
      vec![ 1, 2, 3, 4 ],
      vec![ 4, 3, 2, 1 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(solved, true);
}

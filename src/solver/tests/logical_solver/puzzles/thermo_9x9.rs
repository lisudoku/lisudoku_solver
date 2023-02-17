use crate::{types::{FixedNumber, SudokuConstraints, CellPosition, SolutionType}, solver::Solver};

#[test]
fn check_9x9_thermo_medium_solve() {
  // UK Sudoku Championship 2022 booklet - 9x9 thermo https://ukpuzzles.org/file_download.php?fileid=247&md5=c200e06d8822177932d906103919ceba
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(2, 2, 2),
    FixedNumber::new(2, 6, 4),
    FixedNumber::new(3, 4, 5),
    FixedNumber::new(5, 4, 1),
    FixedNumber::new(6, 2, 9),
    FixedNumber::new(6, 6, 5),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    vec![
      CellPosition::new(0, 6),
      CellPosition::new(0, 5),
      CellPosition::new(0, 4),
      CellPosition::new(0, 3),
      CellPosition::new(0, 2),
      CellPosition::new(0, 1),
      CellPosition::new(0, 0),
    ],
    vec![
      CellPosition::new(2, 0),
      CellPosition::new(3, 0),
      CellPosition::new(4, 0),
      CellPosition::new(5, 0),
      CellPosition::new(6, 0),
      CellPosition::new(7, 0),
      CellPosition::new(8, 0),
    ],
    vec![
      CellPosition::new(2, 5),
      CellPosition::new(2, 4),
      CellPosition::new(2, 3),
    ],
    vec![
      CellPosition::new(3, 2),
      CellPosition::new(4, 2),
      CellPosition::new(5, 2),
    ],
    vec![
      CellPosition::new(5, 6),
      CellPosition::new(4, 6),
      CellPosition::new(3, 6),
    ],
    vec![
      CellPosition::new(6, 3),
      CellPosition::new(6, 4),
      CellPosition::new(6, 5),
    ],
    vec![
      CellPosition::new(6, 8),
      CellPosition::new(5, 8),
      CellPosition::new(4, 8),
      CellPosition::new(3, 8),
      CellPosition::new(2, 8),
      CellPosition::new(1, 8),
      CellPosition::new(0, 8),
    ],
    vec![
      CellPosition::new(8, 2),
      CellPosition::new(8, 3),
      CellPosition::new(8, 4),
      CellPosition::new(8, 5),
      CellPosition::new(8, 6),
      CellPosition::new(8, 7),
      CellPosition::new(8, 8),
    ]
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 9, 8, 6, 5, 4, 2, 1, 3, 7 ],
    vec![ 3, 5, 4, 7, 8, 1, 2, 9, 6 ],
    vec![ 1, 7, 2, 9, 6, 3, 4, 8, 5 ],
    vec![ 2, 6, 3, 8, 5, 7, 9, 1, 4 ],
    vec![ 4, 1, 7, 6, 2, 9, 8, 5, 3 ],
    vec![ 5, 9, 8, 3, 1, 4, 7, 6, 2 ],
    vec![ 6, 3, 9, 4, 7, 8, 5, 2, 1 ],
    vec![ 7, 2, 5, 1, 9, 6, 3, 4, 8 ],
    vec![ 8, 4, 1, 2, 3, 5, 6, 7, 9 ],
  ]);
  assert!(result.steps.len() > empty_cells);
}

#[test]
fn check_9x9_thermo_no_solution() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(8, 1, 9),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    vec![
      CellPosition::new(0, 0),
      CellPosition::new(1, 0),
      CellPosition::new(2, 0),
      CellPosition::new(3, 0),
      CellPosition::new(4, 0),
      CellPosition::new(5, 0),
      CellPosition::new(6, 0),
      CellPosition::new(7, 0),
      CellPosition::new(8, 0),
    ],
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::None);
}

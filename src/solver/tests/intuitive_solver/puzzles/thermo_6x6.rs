use crate::{types::{FixedNumber, SudokuConstraints, CellPosition, SolutionType}, solver::Solver};

#[test]
fn check_thermo_6x6_1_solve() {
  // WSC booklet 6x6 thermo https://uploads-ssl.webflow.com/62793457876c001d28edf162/6348945a45b06acb414391b7_WSC_2022_IB_v2.1.pdf
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
      CellPosition::new(0, 0),
      CellPosition::new(0, 1),
      CellPosition::new(0, 2),
      CellPosition::new(0, 3),
      CellPosition::new(0, 4),
      CellPosition::new(0, 5),
    ],
    vec![
      CellPosition::new(1, 4),
      CellPosition::new(2, 4),
      CellPosition::new(3, 4),
    ],
    vec![
      CellPosition::new(2, 2),
      CellPosition::new(3, 2),
      CellPosition::new(4, 2),
      CellPosition::new(4, 3),
    ],
    vec![
      CellPosition::new(3, 0),
      CellPosition::new(4, 0),
      CellPosition::new(5, 0),
    ],
    vec![
      CellPosition::new(3, 3),
      CellPosition::new(2, 3),
      CellPosition::new(1, 3),
      CellPosition::new(1, 2),
    ],
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.intuitive_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 1, 2, 3, 4, 5, 6 ],
    vec![ 4, 5, 6, 3, 2, 1 ],
    vec![ 5, 6, 1, 2, 3, 4 ],
    vec![ 2, 3, 4, 1, 6, 5 ],
    vec![ 3, 4, 5, 6, 1, 2 ],
    vec![ 6, 1, 2, 5, 4, 3 ],
  ]);
  assert_eq!(result.steps.len(), empty_cells);
}

// https://github.com/lisudoku/lisudoku_solver/issues/4
#[test]
fn check_thermo_6x6_2_solve() {
  let grid_size = 6;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 5),
    FixedNumber::new(0, 5, 4),
    FixedNumber::new(2, 2, 1),
    FixedNumber::new(3, 3, 6),
    FixedNumber::new(5, 0, 6),
    FixedNumber::new(5, 3, 3),
    FixedNumber::new(5, 5, 2),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    vec![
      CellPosition::new(1, 1),
      CellPosition::new(2, 1),
      CellPosition::new(3, 1),
    ],
    vec![
      CellPosition::new(1, 4),
      CellPosition::new(1, 3),
      CellPosition::new(1, 2),
    ],
    vec![
      CellPosition::new(4, 1),
      CellPosition::new(4, 2),
      CellPosition::new(4, 3),
    ],
    vec![
      CellPosition::new(4, 4),
      CellPosition::new(3, 4),
      CellPosition::new(2, 4),
    ],
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.intuitive_solve();
  assert_eq!(result.solution_type, SolutionType::None);
}

// https://github.com/lisudoku/lisudoku_solver/issues/3
#[test]
fn check_thermo_6x6_3_solve() {
  let grid_size = 6;
  let fixed_numbers = vec![
    FixedNumber::new(1, 1, 6),
    FixedNumber::new(2, 2, 5),
    FixedNumber::new(3, 3, 4),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    vec![
      CellPosition::new(2, 0),
      CellPosition::new(1, 0),
      CellPosition::new(0, 0),
      CellPosition::new(0, 1),
      CellPosition::new(0, 2),
    ],
    vec![
      CellPosition::new(4, 1),
      CellPosition::new(3, 2),
      CellPosition::new(2, 3),
      CellPosition::new(1, 4),
    ],
    vec![
      CellPosition::new(3, 5),
      CellPosition::new(4, 5),
      CellPosition::new(5, 5),
      CellPosition::new(5, 4),
      CellPosition::new(5, 3),
    ],
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.intuitive_solve();
  assert_eq!(result.solution_type, SolutionType::None);
}

// https://github.com/lisudoku/lisudoku_solver/issues/7
#[test]
fn check_thermo_6x6_4_solve() {
  let grid_size = 6;
  let fixed_numbers = vec![
    FixedNumber::new(1, 1, 5),
    FixedNumber::new(4, 4, 6),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    vec![
      CellPosition::new(1, 4),
      CellPosition::new(2, 3),
      CellPosition::new(3, 2),
      CellPosition::new(4, 1),
    ],
    vec![
      CellPosition::new(2, 0),
      CellPosition::new(3, 1),
      CellPosition::new(4, 2),
      CellPosition::new(5, 3),
    ],
    vec![
      CellPosition::new(3, 5),
      CellPosition::new(2, 4),
      CellPosition::new(1, 3),
      CellPosition::new(0, 2),
    ],
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.intuitive_solve();
  assert_eq!(result.solution_type, SolutionType::None);
}

// https://github.com/lisudoku/lisudoku_solver/issues/8
#[test]
fn check_thermo_6x6_5_solve() {
  let grid_size = 6;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 1),
    FixedNumber::new(0, 5, 3),
    FixedNumber::new(5, 0, 2),
    FixedNumber::new(5, 5, 1),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    vec![
      CellPosition::new(1, 4),
      CellPosition::new(0, 3),
      CellPosition::new(0, 2),
      CellPosition::new(1, 1),
    ],
    vec![
      CellPosition::new(2, 1),
      CellPosition::new(3, 1),
      CellPosition::new(4, 1),
      CellPosition::new(5, 2),
    ],
    vec![
      CellPosition::new(5, 3),
      CellPosition::new(4, 4),
      CellPosition::new(3, 4),
      CellPosition::new(3, 3),
    ],
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.intuitive_solve();
  assert_eq!(result.solution_type, SolutionType::None);
}

// https://github.com/lisudoku/lisudoku_solver/issues/9
#[test]
fn check_thermo_6x6_6_solve() {
  let grid_size = 6;
  let fixed_numbers = vec![
    FixedNumber::new(1, 5, 3),
    FixedNumber::new(2, 5, 1),
    FixedNumber::new(5, 2, 1),
    FixedNumber::new(5, 3, 4),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    vec![
      CellPosition::new(0, 3),
      CellPosition::new(1, 2),
      CellPosition::new(2, 1),
      CellPosition::new(3, 0),
    ],
    vec![
      CellPosition::new(4, 4),
      CellPosition::new(3, 3),
      CellPosition::new(2, 2),
      CellPosition::new(1, 1),
    ],
    vec![
      CellPosition::new(5, 0),
      CellPosition::new(4, 1),
      CellPosition::new(3, 2),
      CellPosition::new(2, 3),
    ],
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.intuitive_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 4, 5, 3, 1, 2, 6 ],
    vec![ 1, 6, 2, 5, 4, 3 ],
    vec![ 2, 3, 4, 6, 5, 1 ],
    vec![ 6, 1, 5, 2, 3, 4 ],
    vec![ 5, 4, 6, 3, 1, 2 ],
    vec![ 3, 2, 1, 4, 6, 5 ],
  ]);
}

// https://github.com/lisudoku/lisudoku_solver/issues/10
#[test]
fn check_thermo_6x6_7_solve() {
  let grid_size = 6;
  let fixed_numbers = vec![];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    vec![
      CellPosition::new(2, 1),
      CellPosition::new(2, 2),
      CellPosition::new(1, 2),
      CellPosition::new(1, 3),
      CellPosition::new(1, 4),
    ],
    vec![
      CellPosition::new(3, 0),
      CellPosition::new(3, 1),
      CellPosition::new(3, 2),
      CellPosition::new(3, 3),
    ],
    vec![
      CellPosition::new(4, 4),
      CellPosition::new(4, 3),
      CellPosition::new(4, 2),
      CellPosition::new(4, 1),
    ],
    vec![
      CellPosition::new(5, 2),
      CellPosition::new(5, 3),
      CellPosition::new(5, 4),
      CellPosition::new(5, 5),
      CellPosition::new(4, 5),
    ],
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.intuitive_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 5, 2, 6, 1, 3, 4 ],
    vec![ 1, 3, 4, 5, 6, 2 ],
    vec![ 6, 1, 2, 4, 5, 3 ],
    vec![ 3, 4, 5, 6, 2, 1 ],
    vec![ 4, 5, 3, 2, 1, 6 ],
    vec![ 2, 6, 1, 3, 4, 5 ],
  ]);
}

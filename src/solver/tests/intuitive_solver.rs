use crate::{types::{SudokuConstraints, FixedNumber, CellPosition}, solver::Solver};

mod locked_candidates;
mod naked_singles;
mod grid_steps;

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
  let mut solver = Solver::new(constraints, None);
  let result = solver.intuitive_solve();
  assert_eq!(result.full_solution, true);
  assert_eq!(result.solution, vec![
    vec![ 2, 1, 4, 3 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 1, 2, 3, 4 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  assert_eq!(result.steps.len(), empty_cells);
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
  let mut solver = Solver::new(constraints, None);
  let result = solver.intuitive_solve();
  assert_eq!(result.full_solution, true);
  assert_eq!(result.solution, vec![
    vec![ 6, 2, 3, 1, 4, 5 ],
    vec![ 1, 4, 5, 3, 6, 2 ],
    vec![ 4, 1, 2, 5, 3, 6 ],
    vec![ 5, 3, 6, 2, 1, 4 ],
    vec![ 3, 5, 4, 6, 2, 1 ],
    vec![ 2, 6, 1, 4, 5, 3 ],
  ]);
  assert_eq!(result.steps.len(), empty_cells);
}

#[test]
fn check_9x9_easy_solve() {
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
  let mut solver = Solver::new(constraints, None);
  let result = solver.intuitive_solve();
  assert_eq!(result.full_solution, true);
  assert_eq!(result.solution, vec![
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
  assert_eq!(result.steps.len(), empty_cells);
}

#[test]
fn check_9x9_medium_solve() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 1, 1),
    FixedNumber::new(0, 2, 4),
    FixedNumber::new(1, 4, 6),
    FixedNumber::new(1, 5, 5),
    FixedNumber::new(2, 3, 9),
    FixedNumber::new(2, 4, 2),
    FixedNumber::new(2, 6, 1),
    FixedNumber::new(2, 7, 5),
    FixedNumber::new(3, 1, 5),
    FixedNumber::new(3, 4, 3),
    FixedNumber::new(3, 6, 4),
    FixedNumber::new(3, 7, 6),
    FixedNumber::new(4, 2, 3),
    FixedNumber::new(4, 4, 4),
    FixedNumber::new(5, 2, 2),
    FixedNumber::new(5, 3, 6),
    FixedNumber::new(5, 5, 1),
    FixedNumber::new(5, 6, 7),
    FixedNumber::new(5, 8, 5),
    FixedNumber::new(6, 1, 7),
    FixedNumber::new(6, 6, 5),
    FixedNumber::new(6, 7, 1),
    FixedNumber::new(6, 8, 3),
    FixedNumber::new(7, 0, 1),
    FixedNumber::new(7, 7, 9),
    FixedNumber::new(8, 3, 3),
    FixedNumber::new(8, 5, 6),
    FixedNumber::new(8, 7, 7),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  let result = solver.intuitive_solve();
  assert_eq!(result.full_solution, true);
  assert_eq!(result.solution, vec![
    vec![ 5, 1, 4, 8, 7, 3, 9, 2, 6 ],
    vec![ 8, 2, 9, 1, 6, 5, 3, 4, 7 ],
    vec![ 3, 6, 7, 9, 2, 4, 1, 5, 8 ],
    vec![ 7, 5, 1, 2, 3, 8, 4, 6, 9 ],
    vec![ 6, 9, 3, 5, 4, 7, 2, 8, 1 ],
    vec![ 4, 8, 2, 6, 9, 1, 7, 3, 5 ],
    vec![ 2, 7, 6, 4, 8, 9, 5, 1, 3 ],
    vec![ 1, 3, 8, 7, 5, 2, 6, 9, 4 ],
    vec![ 9, 4, 5, 3, 1, 6, 8, 7, 2 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
}

#[test]
fn check_6x6_thermo_solve() {
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
  let mut solver = Solver::new(constraints, None);
  let result = solver.intuitive_solve();
  assert_eq!(result.full_solution, true);
  assert_eq!(result.solution, vec![
    vec![ 1, 2, 3, 4, 5, 6 ],
    vec![ 4, 5, 6, 3, 2, 1 ],
    vec![ 5, 6, 1, 2, 3, 4 ],
    vec![ 2, 3, 4, 1, 6, 5 ],
    vec![ 3, 4, 5, 6, 1, 2 ],
    vec![ 6, 1, 2, 5, 4, 3 ],
  ]);
  assert_eq!(result.steps.len(), empty_cells);
}

// TODO: uncomment after implementing more rules
// #[test]
// fn check_9x9_thermo_solve() {
//   // UK Sudoku Championship 2022 booklet - 9x9 thermo https://ukpuzzles.org/file_download.php?fileid=247&md5=c200e06d8822177932d906103919ceba
//   // Note: added extra fixed numbers to make the solution unique
//   let grid_size = 9;
//   let fixed_numbers = vec![
//     FixedNumber::new(0, 0, 9), // extra!
//     FixedNumber::new(0, 1, 8), // extra!
//     FixedNumber::new(2, 2, 2),
//     FixedNumber::new(2, 6, 4),
//     FixedNumber::new(3, 4, 5),
//     FixedNumber::new(5, 4, 1),
//     FixedNumber::new(6, 2, 9),
//     FixedNumber::new(6, 6, 5),
//   ];
//   let empty_cells = grid_size * grid_size - fixed_numbers.len();
//   let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
//   constraints.thermos = vec![
//     vec![
//       CellPosition { row: 0, col: 6 },
//       CellPosition { row: 0, col: 5 },
//       CellPosition { row: 0, col: 4 },
//       CellPosition { row: 0, col: 3 },
//       CellPosition { row: 0, col: 2 },
//       CellPosition { row: 0, col: 1 },
//       CellPosition { row: 0, col: 0 },
//     ],
//     vec![
//       CellPosition { row: 2, col: 0 },
//       CellPosition { row: 3, col: 0 },
//       CellPosition { row: 4, col: 0 },
//       CellPosition { row: 5, col: 0 },
//       CellPosition { row: 6, col: 0 },
//       CellPosition { row: 7, col: 0 },
//       CellPosition { row: 8, col: 0 },
//     ],
//     vec![
//       CellPosition { row: 2, col: 5 },
//       CellPosition { row: 2, col: 4 },
//       CellPosition { row: 2, col: 3 },
//     ],
//     vec![
//       CellPosition { row: 3, col: 2 },
//       CellPosition { row: 4, col: 2 },
//       CellPosition { row: 5, col: 2 },
//     ],
//     vec![
//       CellPosition { row: 5, col: 6 },
//       CellPosition { row: 4, col: 6 },
//       CellPosition { row: 3, col: 6 },
//     ],
//     vec![
//       CellPosition { row: 6, col: 3 },
//       CellPosition { row: 6, col: 4 },
//       CellPosition { row: 6, col: 5 },
//     ],
//     vec![
//       CellPosition { row: 6, col: 8 },
//       CellPosition { row: 5, col: 8 },
//       CellPosition { row: 4, col: 8 },
//       CellPosition { row: 3, col: 8 },
//       CellPosition { row: 2, col: 8 },
//       CellPosition { row: 1, col: 8 },
//       CellPosition { row: 0, col: 8 },
//     ],
//     vec![
//       CellPosition { row: 8, col: 2 },
//       CellPosition { row: 8, col: 3 },
//       CellPosition { row: 8, col: 4 },
//       CellPosition { row: 8, col: 5 },
//       CellPosition { row: 8, col: 6 },
//       CellPosition { row: 8, col: 7 },
//       CellPosition { row: 8, col: 8 },
//     ]
//   ];
//   let mut solver = Solver::new(constraints, None);
//   let solution = solver.intuitive_solve();
//   assert_eq!(solution.solution_count, 1);
//   assert_eq!(solution.solution, vec![
//     vec![ 1, 2, 3, 4, 5, 6 ],
//     vec![ 4, 5, 6, 3, 2, 1 ],
//     vec![ 5, 6, 1, 2, 3, 4 ],
//     vec![ 2, 3, 4, 1, 6, 5 ],
//     vec![ 3, 4, 5, 6, 1, 2 ],
//     vec![ 6, 1, 2, 5, 4, 3 ],
//   ]);
//   assert_eq!(solution.steps.len(), empty_cells);
// }

#[test]
fn check_9x9_thermo_no_solution() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(8, 1, 9),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    vec![
      CellPosition { row: 0, col: 0 },
      CellPosition { row: 1, col: 0 },
      CellPosition { row: 2, col: 0 },
      CellPosition { row: 3, col: 0 },
      CellPosition { row: 4, col: 0 },
      CellPosition { row: 5, col: 0 },
      CellPosition { row: 6, col: 0 },
      CellPosition { row: 7, col: 0 },
      CellPosition { row: 8, col: 0 },
    ],
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.intuitive_solve();
  assert_eq!(result.full_solution, false);
  assert_eq!(result.no_solution, true);
}

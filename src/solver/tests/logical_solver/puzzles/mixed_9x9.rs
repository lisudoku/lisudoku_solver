use crate::{types::{FixedNumber, SudokuConstraints, CellPosition}, solver::Solver};

// https://youtu.be/LwkNChSO2yE
#[test]
fn check_mixed_9x9_1_hard_solve() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(1, 3, 9),
  ];
  let _empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers).with_anti_knight();
  constraints.thermos = vec![
    vec![
      CellPosition::new(1, 4),
      CellPosition::new(0, 3),
      CellPosition::new(1, 2),
      CellPosition::new(2, 3),
    ],
    vec![
      CellPosition::new(1, 6),
      CellPosition::new(2, 5),
      CellPosition::new(3, 4),
    ],
    vec![
      CellPosition::new(2, 4),
      CellPosition::new(1, 3),
    ],
    vec![
      CellPosition::new(2, 6),
      CellPosition::new(3, 7),
      CellPosition::new(4, 6),
      CellPosition::new(3, 5),
    ],
    vec![
      CellPosition::new(5, 6),
      CellPosition::new(6, 7),
      CellPosition::new(7, 6),
      CellPosition::new(6, 5),
      CellPosition::new(5, 4),
    ],
    vec![
      CellPosition::new(6, 6),
      CellPosition::new(7, 5),
      CellPosition::new(6, 4),
      CellPosition::new(5, 5),
    ],
  ];
  let mut _solver = Solver::new(constraints, None);
  // TODO: this requires more complex rules to solve
  // let result = solver.logical_solve();
  // assert_eq!(result.solution_type, SolutionType::Full);
  // assert_eq!(result.solution.unwrap(), vec![
  //   vec![ 9, 2, 6, 4, 7, 3, 8, 5, 1 ],
  //   vec![ 8, 3, 5, 9, 1, 2, 4, 7, 6 ],
  //   vec![ 4, 1, 7, 8, 6, 5, 2, 9, 3 ],
  //   vec![ 6, 4, 1, 2, 8, 7, 9, 3, 5 ],
  //   vec![ 5, 9, 2, 3, 4, 1, 6, 8, 7 ],
  //   vec![ 7, 8, 3, 5, 9, 6, 1, 4, 2 ],
  //   vec![ 1, 6, 4, 7, 5, 8, 3, 2, 9 ],
  //   vec![ 3, 5, 9, 1, 2, 4, 7, 6, 8 ],
  //   vec![ 2, 7, 8, 6, 3, 9, 5, 1, 4 ],
  // ]);
  // assert!(result.steps.len() >= empty_cells);
}

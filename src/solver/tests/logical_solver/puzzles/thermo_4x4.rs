use crate::{types::{FixedNumber, SudokuConstraints, CellPosition, SolutionType}, solver::Solver};

// https://github.com/lisudoku/lisudoku_solver/issues/6
#[test]
fn check_thermo_4x4_edge_1_solve() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(3, 3, 2),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    vec![
      CellPosition::new(1, 1),
      CellPosition::new(0, 1),
      CellPosition::new(0, 2),
    ],
    vec![
      CellPosition::new(1, 2),
      CellPosition::new(2, 2),
      CellPosition::new(2, 1),
    ],
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::None);
}

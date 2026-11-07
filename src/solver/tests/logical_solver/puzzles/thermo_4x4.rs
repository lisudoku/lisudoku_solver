use crate::{solver::logical::logical_solver::LogicalSolver, types::{CellPosition, FixedNumber, SolutionType, SudokuConstraints, Thermo}};

// https://github.com/lisudoku/lisudoku_solver/issues/6
#[test]
fn check_thermo_4x4_edge_1_solve() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      [FixedNumber::new(3, 3, 2)]
    )
    .with_thermos(
      [
        Thermo(vec![
          CellPosition::new(1, 1),
          CellPosition::new(0, 1),
          CellPosition::new(0, 2),
        ]),
        Thermo(vec![
          CellPosition::new(1, 2),
          CellPosition::new(2, 2),
          CellPosition::new(2, 1),
        ]),
      ]
    );
  let mut solver = LogicalSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_type, SolutionType::None);
}

use crate::{solver::Solver, types::{Area, CellPosition, InvalidStateReason, InvalidStateType, SudokuConstraints, SudokuGrid}};

#[test]
fn check_wrong_thermo() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.thermos = vec![
    vec![
      CellPosition::new(0, 0),
      CellPosition::new(1, 0),
      CellPosition::new(2, 0),
    ],
  ];
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
  assert_eq!(
    solved,
    (
      false,
      Some(InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Thermo(0),
        values: vec![3, 1],
      }),
    )
  );
}

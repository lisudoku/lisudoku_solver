use crate::{solver::{Solver, checker::SolvedState}, types::{Area, CellPosition, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints, Thermo}};

#[test]
fn check_wrong_thermo() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.thermos = vec![
    Thermo(vec![
      CellPosition::new(0, 0),
      CellPosition::new(1, 0),
      CellPosition::new(2, 0),
    ]),
  ];
  let grid = Grid(vec![
    vec![ 2, 1, 4, 3 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 1, 2, 3, 4 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Thermo(0),
        values: vec![3, 1],
      }
    )
  );
}

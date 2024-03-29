use crate::{solver::Solver, types::{Area, InvalidStateReason, InvalidStateType, SudokuConstraints, SudokuGrid}};

#[test]
fn check_anti_knight_correct() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.anti_knight = true;
  let grid = SudokuGrid {
    values: vec![
      vec![ 1, 2, 4, 3 ],
      vec![ 3, 4, 2, 1 ],
      vec![ 4, 3, 1, 2 ],
      vec![ 2, 1, 3, 4 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(solved, (true, None));
}

#[test]
fn check_anti_knight_wrong() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.anti_knight = true;
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
        state_type: InvalidStateType::CellInvalidValue,
        area: Area::Cell(0, 0),
        values: vec![2],
      }),
    )
  );
}

#[test]
fn check_anti_knight_invalid_region() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.anti_knight = true;
  let grid = SudokuGrid {
    values: vec![
      vec![ 1, 0, 0, 0 ],
      vec![ 0, 0, 0, 0 ],
      vec![ 0, 0, 0, 0 ],
      vec![ 0, 0, 0, 1 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_partially_solved();
  assert_eq!(
    solved,
    (
      false,
      Some(InvalidStateReason {
        state_type: InvalidStateType::AreaCandidates,
        area: Area::Row(1),
        values: vec![1],
      }),
    )
  );
}

use crate::{solver::Solver, types::{Area, InvalidStateReason, InvalidStateType, SudokuConstraints, SudokuGrid}};

#[test]
fn check_topbot_solved() {
  let constraints = SudokuConstraints::new(4, vec![]).with_top_bottom();
  let grid = SudokuGrid {
    values: vec![
      vec![ 1, 3, 2, 4 ],
      vec![ 2, 4, 1, 3 ],
      vec![ 3, 1, 4, 2 ],
      vec![ 4, 2, 3, 1 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(solved, (true, None));
}

#[test]
fn check_topbot_wrong_one_sequence() {
  let constraints = SudokuConstraints::new(4, vec![]).with_top_bottom();
  let grid = SudokuGrid {
    values: vec![
      vec![ 4, 1, 3, 2 ],
      vec![ 3, 2, 4, 1 ],
      vec![ 1, 3, 2, 4 ],
      vec![ 2, 4, 1, 3 ],
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
        area: Area::Grid,
        values: vec![],
      }),
    )
  );
}

#[test]
fn check_topbot_wrong_both() {
  let constraints = SudokuConstraints::new(4, vec![]).with_top_bottom();
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
        area: Area::Grid,
        values: vec![],
      }),
    )
  );
}

#[test]
fn check_topbot_empty_correct() {
  let constraints = SudokuConstraints::new(4, vec![]).with_top_bottom();
  let grid = SudokuGrid {
    values: vec![
      vec![ 1, 0, 0, 0 ],
      vec![ 0, 2, 0, 0 ],
      vec![ 0, 0, 0, 0 ],
      vec![ 0, 0, 0, 0 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let partially_solved = solver.check_partially_solved();
  assert_eq!(partially_solved, (true, None));
}

#[test]
fn check_topbot_empty_wrong() {
  let constraints = SudokuConstraints::new(4, vec![]).with_top_bottom();
  let grid = SudokuGrid {
    values: vec![
      vec![ 1, 0, 0, 0 ],
      vec![ 0, 0, 2, 0 ],
      vec![ 0, 0, 0, 0 ],
      vec![ 0, 0, 0, 0 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let partially_solved = solver.check_partially_solved();
  assert_eq!(
    partially_solved,
    (
      false,
      Some(InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Grid,
        values: vec![],
      }),
    )
  );
}

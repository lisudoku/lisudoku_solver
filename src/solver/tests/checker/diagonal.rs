use crate::{solver::Solver, types::{Area, InvalidStateReason, InvalidStateType, SudokuConstraints, SudokuGrid}};

#[test]
fn check_wrong_primary_diagonal() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.primary_diagonal = true;
  let grid = SudokuGrid {
    values: vec![
      vec![ 2, 1, 3, 4 ],
      vec![ 4, 3, 1, 2 ],
      vec![ 1, 4, 2, 3 ],
      vec![ 3, 2, 4, 1 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    (
      false,
      Some(InvalidStateReason {
        state_type: InvalidStateType::AreaValueConflict,
        area: Area::PrimaryDiagonal,
        values: vec![2],
      }),
    )
  );
}

#[test]
fn check_wrong_secondary_diagonal() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.secondary_diagonal = true;
  let grid = SudokuGrid {
    values: vec![
      vec![ 2, 1, 3, 4 ],
      vec![ 4, 3, 1, 2 ],
      vec![ 1, 4, 2, 3 ],
      vec![ 3, 2, 4, 1 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    (
      false,
      Some(InvalidStateReason {
        state_type: InvalidStateType::AreaValueConflict,
        area: Area::SecondaryDiagonal,
        values: vec![4],
      }),
    )
  );
}

#[test]
fn check_correct_both_diagonals() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.primary_diagonal = true;
  constraints.secondary_diagonal = true;
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
  assert_eq!(solved, (true, None));
}

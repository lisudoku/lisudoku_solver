use crate::{solver::{SolveState, SolvedState}, types::{Area, CellPosition, Grid, InvalidStateReason, InvalidStateType, Renban, SudokuConstraints}};

#[test]
fn check_renban_correct() {
  let constraints = SudokuConstraints::new(4)
    .with_renbans(
      vec![
        Renban(vec![CellPosition::new(1, 2), CellPosition::new(0, 2), CellPosition::new(0, 3)]),
      ]
    );
  let grid = Grid(vec![
    vec![ 1, 2, 4, 3 ],
    vec![ 3, 4, 2, 1 ],
    vec![ 4, 3, 1, 2 ],
    vec![ 2, 1, 3, 4 ],
  ]);
  let solve_state = SolveState::from_constraints_and_grid(constraints, grid);
  let solved = solve_state.check_solved();
  assert_eq!(solved, SolvedState::solved());
}

#[test]
fn check_renban_wrong_value_conflict() {
  let constraints = SudokuConstraints::new(4)
    .with_renbans(
      vec![
        Renban(vec![CellPosition::new(1, 3), CellPosition::new(2, 2)]),
      ]
    );
  let grid = Grid(vec![
    vec![ 1, 2, 4, 3 ],
    vec![ 3, 4, 2, 1 ],
    vec![ 4, 3, 1, 2 ],
    vec![ 2, 1, 3, 4 ],
  ]);
  let solve_state = SolveState::from_constraints_and_grid(constraints, grid);
  let solved = solve_state.check_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaValueConflict,
        area: Area::Renban(0),
        values: vec![1],
      }
    )
  );
}

#[test]
fn check_renban_wrong_value_conflict_2() {
  let constraints = SudokuConstraints::new(4)
    .with_renbans(
      vec![
        Renban(vec![
          CellPosition::new(0, 0), CellPosition::new(0, 1),
          CellPosition::new(1, 2), CellPosition::new(2, 2),
        ]),
      ]
    );
  let grid = Grid(vec![
    vec![ 2, 1, 4, 3 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 1, 2, 3, 4 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  let solve_state = SolveState::from_constraints_and_grid(constraints, grid);
  let solved = solve_state.check_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaValueConflict,
        area: Area::Renban(0),
        values: vec![1],
      }
    )
  );
}

#[test]
fn check_renban_wrong_not_consecutive() {
  let constraints = SudokuConstraints::new(4)
    .with_renbans(
      vec![
        Renban(vec![CellPosition::new(0, 0), CellPosition::new(1, 0), CellPosition::new(1, 1)]),
      ]
    );
  let grid = Grid(vec![
    vec![ 1, 2, 4, 3 ],
    vec![ 3, 4, 2, 1 ],
    vec![ 4, 3, 1, 2 ],
    vec![ 2, 1, 3, 4 ],
  ]);
  let solve_state = SolveState::from_constraints_and_grid(constraints, grid);
  let solved = solve_state.check_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Renban(0),
        values: vec![],
      }
    )
  );
}

#[test]
fn check_renban_wrong_partial_not_consecutive() {
  let constraints = SudokuConstraints::new(4)
    .with_renbans(
      vec![
        Renban(vec![CellPosition::new(0, 2), CellPosition::new(1, 3), CellPosition::new(0, 1)]),
      ]
    );
  let grid = Grid(vec![
    vec![ 0, 0, 4, 3 ],
    vec![ 3, 4, 2, 1 ],
    vec![ 4, 3, 1, 2 ],
    vec![ 2, 1, 3, 4 ],
  ]);
  let solve_state = SolveState::from_constraints_and_grid(constraints, grid);
  let solved = solve_state.check_partially_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Renban(0),
        values: vec![],
      }
    )
  );
}

#[test]
fn check_renban_correct_partial_with_fixed_value() {
  let constraints = SudokuConstraints::new(4)
    .with_renbans(
      vec![
        Renban(vec![
          CellPosition::new(1, 1), CellPosition::new(2, 1),
          CellPosition::new(3, 1), CellPosition::new(3, 0),
        ]),
      ]
    );
  let grid = Grid(vec![
    vec![ 1, 2, 4, 3 ],
    vec![ 3, 4, 2, 1 ],
    vec![ 4, 0, 1, 2 ],
    vec![ 0, 0, 3, 4 ],
  ]);
  let solve_state = SolveState::from_constraints_and_grid(constraints, grid);
  let solved = solve_state.check_partially_solved();
  assert_eq!(solved, SolvedState::solved());
}

#[test]
fn check_renban_correct_partial_with_no_value() {
  let constraints = SudokuConstraints::new(4)
    .with_renbans(
      vec![
        Renban(vec![
          CellPosition::new(1, 1), CellPosition::new(2, 1),
          CellPosition::new(3, 1), CellPosition::new(3, 0),
        ]),
      ]
    );
  let grid = Grid(vec![
    vec![ 1, 2, 4, 3 ],
    vec![ 3, 0, 2, 1 ],
    vec![ 4, 0, 1, 2 ],
    vec![ 0, 0, 3, 4 ],
  ]);
  let solve_state = SolveState::from_constraints_and_grid(constraints, grid);
  let solved = solve_state.check_partially_solved();
  assert_eq!(solved, SolvedState::solved());
}

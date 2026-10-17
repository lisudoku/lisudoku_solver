use crate::{solver::{SolveState, SolvedState}, types::{Area, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints}};

#[test]
fn check_top_bottom_solved() {
  let constraints = SudokuConstraints::new(4).with_top_bottom();
  let grid = Grid(vec![
    vec![ 1, 3, 2, 4 ],
    vec![ 2, 4, 1, 3 ],
    vec![ 3, 1, 4, 2 ],
    vec![ 4, 2, 3, 1 ],
  ]);
  let solve_state = SolveState::from_constraints_and_grid(constraints, grid);
  let solved = solve_state.check_solved();
  assert_eq!(solved, SolvedState::solved());
}

#[test]
fn check_top_bottom_wrong_one_sequence() {
  let constraints = SudokuConstraints::new(4).with_top_bottom();
  let grid = Grid(vec![
    vec![ 4, 1, 3, 2 ],
    vec![ 3, 2, 4, 1 ],
    vec![ 1, 3, 2, 4 ],
    vec![ 2, 4, 1, 3 ],
  ]);
  let solve_state = SolveState::from_constraints_and_grid(constraints, grid);
  let solved = solve_state.check_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Grid,
        values: vec![],
      }
    )
  );
}

#[test]
fn check_top_bottom_wrong_both() {
  let constraints = SudokuConstraints::new(4).with_top_bottom();
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
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Grid,
        values: vec![],
      }
    )
  );
}

#[test]
fn check_top_bottom_empty_correct() {
  let constraints = SudokuConstraints::new(4).with_top_bottom();
  let grid = Grid(vec![
    vec![ 1, 0, 0, 0 ],
    vec![ 0, 2, 0, 0 ],
    vec![ 0, 0, 0, 0 ],
    vec![ 0, 0, 0, 0 ],
  ]);
  let solve_state = SolveState::from_constraints_and_grid(constraints, grid);
  let solved = solve_state.check_partially_solved();
  assert_eq!(solved, SolvedState::solved());
}

#[test]
fn check_top_bottom_empty_wrong() {
  let constraints = SudokuConstraints::new(4).with_top_bottom();
  let grid = Grid(vec![
    vec![ 1, 0, 0, 0 ],
    vec![ 0, 0, 2, 0 ],
    vec![ 0, 0, 0, 0 ],
    vec![ 0, 0, 0, 0 ],
  ]);
  let solve_state = SolveState::from_constraints_and_grid(constraints, grid);
  let solved = solve_state.check_partially_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Grid,
        values: vec![],
      }
    )
  );
}

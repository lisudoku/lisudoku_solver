use crate::{solver::{SolveState, SolvedState}, types::{Area, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints}};

#[test]
fn check_anti_knight_correct() {
  let constraints = SudokuConstraints::new(4).with_anti_knight();
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
fn check_anti_knight_wrong() {
  let constraints = SudokuConstraints::new(4).with_anti_knight();
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
        state_type: InvalidStateType::CellInvalidValue,
        area: Area::Cell(0, 0),
        values: vec![2],
      }
    )
  );
}

#[test]
fn check_anti_knight_invalid_region() {
  let constraints = SudokuConstraints::new(4).with_anti_knight();
  let grid = Grid(vec![
    vec![ 1, 0, 0, 0 ],
    vec![ 0, 0, 0, 0 ],
    vec![ 0, 0, 0, 0 ],
    vec![ 0, 0, 0, 1 ],
  ]);
  let solve_state = SolveState::from_constraints_and_grid(constraints, grid);
  let solved = solve_state.check_partially_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaCandidates,
        area: Area::Region(1),
        values: vec![1],
      }
    )
  );
}

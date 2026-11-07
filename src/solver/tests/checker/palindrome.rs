use crate::{solver::{SolveState, GridCheckResult}, types::{Area, CellPosition, Grid, InvalidStateReason, InvalidStateType, Palindrome, SudokuConstraints}};

#[test]
fn check_palindrome_correct_even() {
  let constraints = SudokuConstraints::new(4)
    .with_palindromes(
      [
        Palindrome(vec![CellPosition::new(0, 1), CellPosition::new(1, 2)]),
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
  assert_eq!(solved, GridCheckResult::solved());
}

#[test]
fn check_palindrome_correct_odd() {
  let constraints = SudokuConstraints::new(4)
    .with_palindromes(
      [
        Palindrome(vec![CellPosition::new(0, 0), CellPosition::new(1, 1), CellPosition::new(2, 2)]),
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
  assert_eq!(solved, GridCheckResult::solved());
}

#[test]
fn check_palindrome_wrong_edge() {
  let constraints = SudokuConstraints::new(4)
    .with_palindromes(
      [
        Palindrome(vec![CellPosition::new(0, 0), CellPosition::new(1, 0), CellPosition::new(2, 0)]),
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
    GridCheckResult::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Palindrome(0),
        values: vec![0, 2],
      }
    )
  );
}

#[test]
fn check_palindrome_wrong_partial_edge() {
  let constraints = SudokuConstraints::new(4)
    .with_palindromes(
      [
        Palindrome(vec![
          CellPosition::new(1, 0), CellPosition::new(0, 0),
          CellPosition::new(0, 1), CellPosition::new(0, 2),
        ]),
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
    GridCheckResult::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Palindrome(0),
        values: vec![0, 3],
      }
    )
  );
}

#[test]
fn check_palindrome_correct_partial() {
  let constraints = SudokuConstraints::new(4)
    .with_palindromes(
      [
        Palindrome(vec![
          CellPosition::new(1, 0), CellPosition::new(2, 0), CellPosition::new(3, 0),
          CellPosition::new(2, 1), CellPosition::new(3, 1), CellPosition::new(3, 2),
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
  assert_eq!(solved, GridCheckResult::solved());
}

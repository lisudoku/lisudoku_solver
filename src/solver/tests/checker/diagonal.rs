use crate::{solver::{Solver, checker::SolvedState}, types::{Area, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints}};

#[test]
fn check_wrong_primary_diagonal() {
  let constraints = SudokuConstraints::new(4).with_primary_diagonal();
  let grid = Grid(vec![
    vec![ 2, 1, 3, 4 ],
    vec![ 4, 3, 1, 2 ],
    vec![ 1, 4, 2, 3 ],
    vec![ 3, 2, 4, 1 ],
  ]);
  let solver = Solver::new(constraints).with_grid(grid);
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaValueConflict,
        area: Area::PrimaryDiagonal,
        values: vec![2],
      }
    )
  );
}

#[test]
fn check_wrong_secondary_diagonal() {
  let constraints = SudokuConstraints::new(4).with_secondary_diagonal();
  let grid = Grid(vec![
    vec![ 2, 1, 3, 4 ],
    vec![ 4, 3, 1, 2 ],
    vec![ 1, 4, 2, 3 ],
    vec![ 3, 2, 4, 1 ],
  ]);
  let solver = Solver::new(constraints).with_grid(grid);
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaValueConflict,
        area: Area::SecondaryDiagonal,
        values: vec![4],
      }
    )
  );
}

#[test]
fn check_correct_both_diagonals() {
  let constraints = SudokuConstraints::new(4).with_diagonals();
  let grid = Grid(vec![
    vec![ 2, 1, 4, 3 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 1, 2, 3, 4 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  let solver = Solver::new(constraints).with_grid(grid);
  let solved = solver.check_solved();
  assert_eq!(solved, SolvedState::solved());
}

use crate::{solver::{Solver, checker::SolvedState}, types::{Area, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints}};

#[test]
fn check_wrong_row() {
  let constraints = SudokuConstraints::new(4);
  let grid = Grid(vec![
    vec![ 1, 1, 1, 1 ],
    vec![ 2, 2, 2, 2 ],
    vec![ 3, 3, 3, 3 ],
    vec![ 4, 4, 4, 4 ],
  ]);
  let solver = Solver::new(constraints).with_grid(grid);
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaValueConflict,
        area: Area::Row(0),
        values: vec![1],
      }
    )
  );
}

#[test]
fn check_wrong_col() {
  let constraints = SudokuConstraints::new(4);
  let grid = Grid(vec![
    vec![ 1, 2, 3, 4 ],
    vec![ 1, 2, 3, 4 ],
    vec![ 1, 2, 3, 4 ],
    vec![ 1, 2, 3, 4 ],
  ]);
  let solver = Solver::new(constraints).with_grid(grid);
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaValueConflict,
        area: Area::Column(0),
        values: vec![1],
      }
    )
  );
}

#[test]
fn check_wrong_region() {
  let constraints = SudokuConstraints::new(4);
  let grid = Grid(vec![
    vec![ 1, 2, 3, 4 ],
    vec![ 2, 1, 4, 3 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  let solver = Solver::new(constraints).with_grid(grid);
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaValueConflict,
        area: Area::Region(0),
        values: vec![2],
      }
    )
  );
}

#[test]
fn check_solved_grid() {
  let constraints = SudokuConstraints::new(4);
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

#[test]
fn check_partially_solved_no_solution_grid() {
  let constraints = SudokuConstraints::new(9);
  let grid = Grid(vec![
    vec![ 0, 0, 0, 0, 0, 0, 0, 1, 2 ],
    vec![ 0, 0, 0, 0, 1, 2, 0, 0, 0 ],
    vec![ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    vec![ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    vec![ 0, 2, 0, 0, 0, 0, 0, 0, 0 ],
    vec![ 0, 1, 0, 0, 0, 0, 0, 0, 0 ],
    vec![ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    vec![ 1, 0, 0, 0, 0, 0, 0, 0, 0 ],
    vec![ 2, 0, 0, 0, 0, 0, 0, 0, 0 ],
  ]);
  let solver = Solver::new(constraints).with_grid(grid);
  let solved = solver.check_partially_solved();
  // 1 and 2 both can only be put into the same cell
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaCandidates,
        area: Area::Row(2),
        values: vec![1, 2],
      }
    )
  );
}

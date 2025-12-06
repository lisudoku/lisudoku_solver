use crate::{solver::{Solver, checker::SolvedState}, types::{Area, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints}};

#[test]
fn check_anti_king_correct() {
  let constraints = SudokuConstraints::new(6).with_anti_king();
  let grid = Grid(vec![
    vec![ 4, 5, 1, 3, 2, 6 ],
    vec![ 2, 3, 6, 4, 1, 5 ],
    vec![ 1, 4, 5, 2, 6, 3 ],
    vec![ 6, 2, 3, 1, 5, 4 ],
    vec![ 5, 1, 4, 6, 3, 2 ],
    vec![ 3, 6, 2, 5, 4, 1 ],
  ]);
  let solver = Solver::new(constraints).with_grid(grid);
  let solved = solver.check_solved();
  assert_eq!(solved, SolvedState::solved());
}

#[test]
fn check_anti_king_wrong_1() {
  let constraints = SudokuConstraints::new(4).with_anti_king();
  let grid = Grid(vec![
    vec![ 1, 2, 4, 3 ],
    vec![ 3, 4, 2, 1 ],
    vec![ 4, 3, 1, 2 ],
    vec![ 2, 1, 3, 4 ],
  ]);
  let solver = Solver::new(constraints).with_grid(grid);
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::CellInvalidValue,
        area: Area::Cell(0, 1),
        values: vec![2],
      }
    )
  );
}

#[test]
fn check_anti_king_wrong_2() {
  let constraints = SudokuConstraints::new(4).with_anti_king();
  let grid = Grid(vec![
    vec![ 1, 2, 4, 3 ],
    vec![ 3, 0, 2, 1 ],
    vec![ 4, 0, 0, 2 ],
    vec![ 2, 1, 3, 4 ],
  ]);
  let solver = Solver::new(constraints).with_grid(grid);
  let partially_solved = solver.check_partially_solved();
  assert_eq!(
    partially_solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::CellNoCandidates,
        area: Area::Cell(1, 1),
        values: vec![],
      }
    )
  );
}

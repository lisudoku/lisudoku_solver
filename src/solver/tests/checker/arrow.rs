use crate::{solver::{SolveState, GridCheckResult}, types::{Area, Arrow, CellPosition, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints}};

#[test]
fn check_wrong_arrow() {
  let constraints = SudokuConstraints::new(4)
    .with_arrows(
      [
        Arrow {
          circle_cells: vec![ CellPosition::new(1, 0) ],
          arrow_cells: vec![
            CellPosition::new(0, 0), CellPosition::new(1, 1),
          ],
        },
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
    GridCheckResult::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Arrow(0),
        values: vec![],
      }
    )
  );
}

#[test]
fn check_correct_arrow() {
  let constraints = SudokuConstraints::new(4)
    .with_arrows(
      [
        Arrow {
          circle_cells: vec![ CellPosition::new(1, 0) ],
          arrow_cells: vec![
            CellPosition::new(0, 0), CellPosition::new(0, 1),
          ],
        },
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
  assert_eq!(solved, GridCheckResult::solved());
}

#[test]
fn check_wrong_oval_arrow() {
  let constraints = SudokuConstraints::new(4)
    .with_arrows(
      [
        Arrow {
          circle_cells: vec![
            CellPosition::new(1, 2), CellPosition::new(1, 3),
          ],
          arrow_cells: vec![
            CellPosition::new(2, 3), CellPosition::new(2, 2),
            CellPosition::new(2, 1), CellPosition::new(3, 0),
          ],
        },
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
    GridCheckResult::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Arrow(0),
        values: vec![],
      }
    )
  );
}

#[test]
fn check_correct_oval_arrow() {
  let constraints = SudokuConstraints::new(4)
    .with_arrows(
      [
        Arrow {
          circle_cells: vec![
            CellPosition::new(1, 2), CellPosition::new(1, 3),
          ],
          arrow_cells: vec![
            CellPosition::new(2, 3), CellPosition::new(2, 2),
            CellPosition::new(2, 1), CellPosition::new(3, 1),
          ],
        },
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
  assert_eq!(solved, GridCheckResult::solved());
}

#[test]
fn check_partial_arrow() {
  let constraints = SudokuConstraints::new(4)
    .with_arrows(
      [
        Arrow {
          circle_cells: vec![ CellPosition::new(3, 0) ],
          arrow_cells: vec![
            CellPosition::new(2, 1), CellPosition::new(1, 2), CellPosition::new(0, 1),
          ],
        },
      ]
    );
  let grid = Grid(vec![
    vec![ 0, 1, 0, 3 ],
    vec![ 0, 0, 0, 0 ],
    vec![ 1, 0, 0, 0 ],
    vec![ 0, 0, 0, 0 ],
  ]);
  let solve_state = SolveState::from_constraints_and_grid(constraints, grid);
  let solved = solve_state.check_partially_solved();
  assert_eq!(solved, GridCheckResult::solved());
}

#[test]
fn check_partial_wrong_arrows() {
  let constraints = SudokuConstraints::new(4)
    .with_arrows(
      [
        Arrow {
          circle_cells: vec![CellPosition::new(0, 0)],
          arrow_cells: vec![CellPosition::new(0, 1), CellPosition::new(0, 2)],
        },
        Arrow {
          circle_cells: vec![CellPosition::new(3, 3)],
          arrow_cells: vec![CellPosition::new(3, 2), CellPosition::new(3, 1)],
        },
      ]
    );
  let grid = Grid(vec![
    vec![ 3, 1, 2, 0 ],
    vec![ 0, 0, 0, 0 ],
    vec![ 0, 0, 0, 0 ],
    vec![ 0, 2, 4, 3 ],
  ]);
  let solve_state = SolveState::from_constraints_and_grid(constraints, grid);
  let solved = solve_state.check_partially_solved();
  assert_eq!(
    solved,
    GridCheckResult::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Arrow(1),
        values: vec![],
      }
    )
  );
}

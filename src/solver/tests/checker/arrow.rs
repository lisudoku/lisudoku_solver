use crate::{solver::{Solver, checker::SolvedState}, types::{Area, Arrow, CellPosition, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints}};

#[test]
fn check_wrong_arrow() {
  let constraints = SudokuConstraints::new(4)
    .with_arrows(
      vec![
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
  let solver = Solver::new(constraints).with_grid(grid);
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
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
      vec![
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
  let solver = Solver::new(constraints).with_grid(grid);
  let solved = solver.check_solved();
  assert_eq!(solved, SolvedState::solved());
}

#[test]
fn check_wrong_oval_arrow() {
  let constraints = SudokuConstraints::new(4)
    .with_arrows(
      vec![
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
  let solver = Solver::new(constraints).with_grid(grid);
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
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
      vec![
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
  let solver = Solver::new(constraints).with_grid(grid);
  let solved = solver.check_solved();
  assert_eq!(solved, SolvedState::solved());
}

#[test]
fn check_partial_arrow() {
  let constraints = SudokuConstraints::new(4)
    .with_arrows(
      vec![
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
  let solver = Solver::new(constraints).with_grid(grid);
  let solved = solver.check_partially_solved();
  assert_eq!(solved, SolvedState::solved());
}

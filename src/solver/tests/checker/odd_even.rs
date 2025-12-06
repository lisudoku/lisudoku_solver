use crate::{solver::{Solver, checker::SolvedState}, types::{Area, CellPosition, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints}};

#[test]
fn check_wrong_odd() {
  let constraints = SudokuConstraints::new(4)
    .with_even_cells(
      vec![
        CellPosition::new(0, 0), CellPosition::new(0, 2), CellPosition::new(3, 2),
      ]
    )
    .with_odd_cells(
      vec![
        CellPosition::new(0, 1), CellPosition::new(0, 3), CellPosition::new(3, 0),
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
        state_type: InvalidStateType::CellInvalidValue,
        area: Area::Cell(3, 0),
        values: vec![4],
      }
    )
  );
}

#[test]
fn check_wrong_even() {
  let constraints = SudokuConstraints::new(4)
    .with_even_cells(
      vec![
        CellPosition::new(0, 0), CellPosition::new(0, 2), CellPosition::new(3, 3),
      ]
    )
    .with_odd_cells(
      vec![
        CellPosition::new(0, 1), CellPosition::new(0, 3), CellPosition::new(3, 1),
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
        state_type: InvalidStateType::CellInvalidValue,
        area: Area::Cell(3, 3),
        values: vec![1],
      }
    )
  );
}

#[test]
fn check_solved_grid() {
  let constraints = SudokuConstraints::new(4)
    .with_even_cells(
      vec![
        CellPosition::new(0, 0), CellPosition::new(0, 2), CellPosition::new(3, 2),
      ]
    )
    .with_odd_cells(
      vec![
        CellPosition::new(0, 1), CellPosition::new(0, 3), CellPosition::new(3, 1),
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

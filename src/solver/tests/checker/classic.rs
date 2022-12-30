use crate::{types::{SudokuConstraints, SudokuGrid}, solver::Solver};

#[test]
fn check_wrong_row() {
  let constraints = SudokuConstraints::new(4, vec![]);
  let grid = SudokuGrid {
    values: vec![
      vec![ 1, 1, 1, 1 ],
      vec![ 2, 2, 2, 2 ],
      vec![ 3, 3, 3, 3 ],
      vec![ 4, 4, 4, 4 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(solved, false);
}

#[test]
fn check_wrong_col() {
  let constraints = SudokuConstraints::new(4, vec![]);
  let grid = SudokuGrid {
    values: vec![
      vec![ 1, 2, 3, 4 ],
      vec![ 1, 2, 3, 4 ],
      vec![ 1, 2, 3, 4 ],
      vec![ 1, 2, 3, 4 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(solved, false);
}

#[test]
fn check_wrong_region() {
  let constraints = SudokuConstraints::new(4, vec![]);
  let grid = SudokuGrid {
    values: vec![
      vec![ 1, 2, 3, 4 ],
      vec![ 2, 1, 4, 3 ],
      vec![ 3, 4, 1, 2 ],
      vec![ 4, 3, 2, 1 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(solved, false);
}

#[test]
fn check_solved_grid() {
  let constraints = SudokuConstraints::new(4, vec![]);
  let grid = SudokuGrid {
    values: vec![
      vec![ 2, 1, 4, 3 ],
      vec![ 3, 4, 1, 2 ],
      vec![ 1, 2, 3, 4 ],
      vec![ 4, 3, 2, 1 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(solved, true);
}

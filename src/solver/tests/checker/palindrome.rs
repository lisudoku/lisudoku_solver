use crate::{solver::{Solver, checker::SolvedState}, types::{Area, CellPosition, Grid, InvalidStateReason, InvalidStateType, Palindrome, SudokuConstraints}};

#[test]
fn check_palindrome_correct_even() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.palindromes = vec![
    Palindrome(vec![CellPosition::new(0, 1), CellPosition::new(1, 2)]),
  ];
  let grid = Grid(vec![
    vec![ 1, 2, 4, 3 ],
    vec![ 3, 4, 2, 1 ],
    vec![ 4, 3, 1, 2 ],
    vec![ 2, 1, 3, 4 ],
  ]);
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(solved, SolvedState::solved());
}

#[test]
fn check_palindrome_correct_odd() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.palindromes = vec![
    Palindrome(vec![CellPosition::new(0, 0), CellPosition::new(1, 1), CellPosition::new(2, 2)]),
  ];
  let grid = Grid(vec![
    vec![ 1, 2, 4, 3 ],
    vec![ 3, 4, 2, 1 ],
    vec![ 4, 3, 1, 2 ],
    vec![ 2, 1, 3, 4 ],
  ]);
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(solved, SolvedState::solved());
}

#[test]
fn check_palindrome_wrong_edge() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.palindromes = vec![
    Palindrome(vec![CellPosition::new(0, 0), CellPosition::new(1, 0), CellPosition::new(2, 0)]),
  ];
  let grid = Grid(vec![
    vec![ 1, 2, 4, 3 ],
    vec![ 3, 4, 2, 1 ],
    vec![ 4, 3, 1, 2 ],
    vec![ 2, 1, 3, 4 ],
  ]);
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
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
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.palindromes = vec![
    Palindrome(vec![
      CellPosition::new(1, 0), CellPosition::new(0, 0),
      CellPosition::new(0, 1), CellPosition::new(0, 2),
    ]),
  ];
  let grid = Grid(vec![
    vec![ 0, 0, 4, 3 ],
    vec![ 3, 4, 2, 1 ],
    vec![ 4, 3, 1, 2 ],
    vec![ 2, 1, 3, 4 ],
  ]);
  let solver = Solver::new(constraints, Some(grid));
  let partially_solved = solver.check_partially_solved();
  assert_eq!(
    partially_solved,
    SolvedState::unsolved(
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
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.palindromes = vec![
    Palindrome(vec![
      CellPosition::new(1, 0), CellPosition::new(2, 0), CellPosition::new(3, 0),
      CellPosition::new(2, 1), CellPosition::new(3, 1), CellPosition::new(3, 2),
    ]),
  ];
  let grid = Grid(vec![
    vec![ 1, 2, 4, 3 ],
    vec![ 3, 4, 2, 1 ],
    vec![ 4, 0, 1, 2 ],
    vec![ 0, 0, 3, 4 ],
  ]);
  let solver = Solver::new(constraints, Some(grid));
  let partially_solved = solver.check_partially_solved();
  assert_eq!(partially_solved, SolvedState::solved());
}

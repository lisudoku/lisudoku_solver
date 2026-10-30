use crate::{solver::brute::BruteSolver, types::{CellPosition, FixedNumber, Grid, Palindrome, SudokuConstraints}};
use ntest::timeout;

// https://github.com/lisudoku/lisudoku_solver/issues/65#issue-1926328789 WSC IB
fn palindrome_9x9_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 8), FixedNumber::new(0, 1, 2), FixedNumber::new(0, 2, 1),
        FixedNumber::new(0, 6, 9), FixedNumber::new(0, 7, 5), FixedNumber::new(0, 8, 3),
        FixedNumber::new(1, 3, 2), FixedNumber::new(1, 4, 3), FixedNumber::new(1, 5, 1),
        FixedNumber::new(4, 3, 1), FixedNumber::new(4, 5, 6),
        FixedNumber::new(6, 0, 5), FixedNumber::new(7, 0, 6), FixedNumber::new(8, 0, 1), FixedNumber::new(8, 1, 3),
        FixedNumber::new(6, 8, 4), FixedNumber::new(7, 8, 9), FixedNumber::new(8, 7, 8), FixedNumber::new(8, 8, 5),
      ]
    )
    .with_palindromes(
      vec![
        Palindrome(vec![
          CellPosition::new(3, 3), CellPosition::new(2, 2), CellPosition::new(1, 1),
          CellPosition::new(2, 0), CellPosition::new(3, 0), CellPosition::new(4, 0),
          CellPosition::new(5, 1), CellPosition::new(6, 2), CellPosition::new(7, 3),
          CellPosition::new(8, 4), CellPosition::new(7, 5), CellPosition::new(6, 6),
          CellPosition::new(5, 7),
        ]),
        Palindrome(vec![
          CellPosition::new(4, 8), CellPosition::new(3, 8), CellPosition::new(2, 8),
          CellPosition::new(1, 7), CellPosition::new(2, 6), CellPosition::new(3, 5),
          CellPosition::new(3, 4), CellPosition::new(4, 4), CellPosition::new(5, 4),
        ]),
      ]
    );
  let solution = Grid(vec![
    vec![ 8, 2, 1, 4, 6, 7, 9, 5, 3 ],
    vec![ 9, 5, 7, 2, 3, 1, 8, 4, 6 ],
    vec![ 4, 6, 3, 8, 5, 9, 1, 7, 2 ],
    vec![ 3, 1, 8, 9, 2, 4, 5, 6, 7 ],
    vec![ 2, 9, 5, 1, 7, 6, 4, 3, 8 ],
    vec![ 7, 4, 6, 5, 8, 3, 2, 9, 1 ],
    vec![ 5, 7, 2, 6, 9, 8, 3, 1, 4 ],
    vec![ 6, 8, 4, 3, 1, 5, 7, 2, 9 ],
    vec![ 1, 3, 9, 7, 4, 2, 6, 8, 5 ],
  ]);

  (constraints, solution)
}

#[test]
#[ignore="solver needs more optimizations"]
#[timeout(10_000)]
fn brute_solver_without_logic_finds_unique_solution_in_palindrome_9x9() {
  let (constraints, solution) = palindrome_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(500)]
fn brute_solver_with_logic_finds_unique_solution_in_palindrome_9x9() {
  let (constraints, solution) = palindrome_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

fn palindrome_4x4_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 1), FixedNumber::new(0, 2, 2),
        FixedNumber::new(1, 3, 1), FixedNumber::new(2, 1, 4),
      ]
    )
    .with_palindromes(
      vec![
        Palindrome(vec![
          CellPosition::new(1, 2), CellPosition::new(2, 2),
          CellPosition::new(3, 1), CellPosition::new(2, 0),
        ]),
      ]
    );
  let solution = Grid(vec![
    vec![ 1, 3, 2, 4 ],
    vec![ 4, 2, 3, 1 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 2, 1, 4, 3 ],
  ]);

  (constraints, solution)
}

#[test]
#[timeout(500)]
fn brute_solver_without_logic_finds_unique_solution_in_palindrome_4x4() {
  let (constraints, solution) = palindrome_4x4_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

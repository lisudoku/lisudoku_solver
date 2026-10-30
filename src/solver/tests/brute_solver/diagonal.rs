use crate::{solver::brute::BruteSolver, types::{FixedNumber, Grid, SudokuConstraints}};
use ntest::timeout;

// https://ukpuzzles.org/file_download.php?fileid=247&md5=c200e06d8822177932d906103919ceba
fn diagonal_9x9_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 5, 3),
        FixedNumber::new(0, 6, 8),
        FixedNumber::new(1, 4, 4),
        FixedNumber::new(1, 5, 7),
        FixedNumber::new(2, 0, 6),
        FixedNumber::new(2, 4, 2),
        FixedNumber::new(3, 0, 8),
        FixedNumber::new(3, 1, 9),
        FixedNumber::new(4, 1, 2),
        FixedNumber::new(4, 2, 3),
        FixedNumber::new(4, 6, 7),
        FixedNumber::new(4, 7, 8),
        FixedNumber::new(5, 7, 3),
        FixedNumber::new(5, 8, 6),
        FixedNumber::new(6, 4, 3),
        FixedNumber::new(6, 8, 4),
        FixedNumber::new(7, 3, 1),
        FixedNumber::new(7, 4, 6),
        FixedNumber::new(8, 2, 7),
        FixedNumber::new(8, 3, 8),
      ]
    )
    .with_diagonals();
  let solution = Grid(vec![
    vec![ 2, 5, 9, 6, 1, 3, 8, 4, 7 ],
    vec![ 3, 8, 1, 9, 4, 7, 2, 6, 5 ],
    vec![ 6, 7, 4, 5, 2, 8, 9, 1, 3 ],
    vec![ 8, 9, 6, 3, 7, 1, 4, 5, 2 ],
    vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9 ],
    vec![ 7, 4, 5, 2, 8, 9, 1, 3, 6 ],
    vec![ 5, 1, 8, 7, 3, 2, 6, 9, 4 ],
    vec![ 9, 3, 2, 1, 6, 4, 5, 7, 8 ],
    vec![ 4, 6, 7, 8, 9, 5, 3, 2, 1 ],
  ]);

  (constraints, solution)
}

#[test]
#[timeout(500)]
fn brute_solver_without_logic_finds_unique_solution_in_diagonal_9x9() {
  let (constraints, solution) = diagonal_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(500)]
fn brute_solver_with_logic_finds_unique_solution_in_diagonal_9x9() {
  let (constraints, solution) = diagonal_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

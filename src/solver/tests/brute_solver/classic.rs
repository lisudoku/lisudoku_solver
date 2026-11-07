use crate::{solver::brute::BruteSolver, types::{FixedNumber, Grid, SudokuConstraints}};
use ntest::timeout;

fn classic_4x4_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      [
        FixedNumber::new(1, 1, 4),
        FixedNumber::new(1, 3, 2),
        FixedNumber::new(2, 0, 1),
        FixedNumber::new(2, 2, 3),
      ]
    );
  let solution = Grid::from_string("
    2143
    3412
    1234
    4321
  ");
  (constraints, solution)
}

/*
  0000
  0402
  0000
  0000
*/
fn classic_4x4_multiple_solutions() -> SudokuConstraints {
  SudokuConstraints::new(4)
    .with_fixed_numbers(
      [
        FixedNumber::new(1, 1, 4),
        FixedNumber::new(1, 3, 2),
      ]
    )
}

fn classic_9x9_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      [
        FixedNumber::new(0, 0, 8),
        FixedNumber::new(0, 5, 1),
        FixedNumber::new(0, 8, 4),
        FixedNumber::new(1, 0, 4),
        FixedNumber::new(1, 1, 5),
        FixedNumber::new(1, 7, 1),
        FixedNumber::new(1, 8, 7),
        FixedNumber::new(2, 1, 9),
        FixedNumber::new(2, 2, 1),
        FixedNumber::new(2, 4, 2),
        FixedNumber::new(2, 5, 4),
        FixedNumber::new(2, 6, 5),
        FixedNumber::new(2, 7, 6),
        FixedNumber::new(3, 1, 4),
        FixedNumber::new(3, 7, 2),
        FixedNumber::new(4, 2, 6),
        FixedNumber::new(4, 6, 3),
        FixedNumber::new(5, 0, 9),
        FixedNumber::new(5, 1, 3),
        FixedNumber::new(5, 7, 8),
        FixedNumber::new(5, 8, 1),
        FixedNumber::new(6, 1, 7),
        FixedNumber::new(6, 2, 3),
        FixedNumber::new(6, 4, 8),
        FixedNumber::new(6, 5, 6),
        FixedNumber::new(6, 6, 4),
        FixedNumber::new(6, 7, 5),
        FixedNumber::new(7, 0, 5),
        FixedNumber::new(7, 1, 8),
        FixedNumber::new(7, 7, 7),
        FixedNumber::new(7, 8, 6),
        FixedNumber::new(8, 0, 6),
        FixedNumber::new(8, 5, 5),
        FixedNumber::new(8, 8, 3),
      ]
    );
  let solution = Grid(vec![
    vec![ 8, 6, 7, 5, 9, 1, 2, 3, 4 ],
    vec![ 4, 5, 2, 6, 3, 8, 9, 1, 7 ],
    vec![ 3, 9, 1, 7, 2, 4, 5, 6, 8 ],
    vec![ 7, 4, 8, 3, 1, 9, 6, 2, 5 ],
    vec![ 2, 1, 6, 8, 5, 7, 3, 4, 9 ],
    vec![ 9, 3, 5, 4, 6, 2, 7, 8, 1 ],
    vec![ 1, 7, 3, 9, 8, 6, 4, 5, 2 ],
    vec![ 5, 8, 9, 2, 4, 3, 1, 7, 6 ],
    vec![ 6, 2, 4, 1, 7, 5, 8, 9, 3 ],
  ]);
  (constraints, solution)
}

#[test]
#[timeout(500)]
fn brute_solver_without_logic_solves_classic_4x4() {
  let (constraints, solution) = classic_4x4_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(500)]
fn brute_solver_with_logic_solves_classic_4x4() {
  let (constraints, solution) = classic_4x4_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(500)]
fn brute_solver_without_logic_finds_multiple_solutions_in_classic_4x4() {
  let mut solver = BruteSolver::new(classic_4x4_multiple_solutions());
  let result = solver.solve();
  assert_eq!(result.solution_count, 2);
  assert_eq!(
    result.solution.unwrap(),
    Grid::from_string("
      2314
      1432
      3241
      4123
    "),
  );
}

#[test]
#[timeout(500)]
fn brute_solver_with_logic_finds_multiple_solutions_in_classic_4x4() {
  let mut solver = BruteSolver::new(classic_4x4_multiple_solutions()).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 2);
  // A bit weird to test this with logic because it can change the order of found solutions
  assert_eq!(
    result.solution.unwrap(),
    Grid::from_string("
      2314
      1432
      3241
      4123
    "),
  );
}

#[test]
#[timeout(1_000)]
fn brute_solver_without_logic_solves_classic_9x9() {
  let (constraints, solution) = classic_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(500)]
fn brute_solver_with_logic_solves_classic_9x9() {
  let (constraints, solution) = classic_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

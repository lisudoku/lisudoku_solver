use crate::{solver::brute::BruteSolver, types::{CellPosition, FixedNumber, Grid, Region, SudokuConstraints}};
use ntest::timeout;

fn extra_regions_9x9_multiple_solutions_constraints() -> (SudokuConstraints, u32) {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      [
        FixedNumber::new(8, 0, 9),
        FixedNumber::new(8, 1, 8),
        FixedNumber::new(8, 2, 7),
        FixedNumber::new(5, 3, 6),
        FixedNumber::new(5, 4, 5),
        FixedNumber::new(5, 5, 4),
        FixedNumber::new(2, 6, 3),
        FixedNumber::new(2, 7, 2),
        FixedNumber::new(2, 8, 1),
        FixedNumber::new(4, 4, 9),
        FixedNumber::new(7, 7, 1),
      ]
    )
    .with_extra_regions(
      [
        Region(vec![
          CellPosition::new(0, 0), CellPosition::new(0, 1), CellPosition::new(0, 2),
          CellPosition::new(0, 3), CellPosition::new(0, 4), CellPosition::new(1, 1),
          CellPosition::new(1, 3), CellPosition::new(1, 4), CellPosition::new(2, 3),
        ]),
        Region(vec![
          CellPosition::new(0, 5), CellPosition::new(0, 6), CellPosition::new(0, 7),
          CellPosition::new(0, 8), CellPosition::new(1, 5), CellPosition::new(1, 6),
          CellPosition::new(1, 7), CellPosition::new(1, 8), CellPosition::new(2, 6),
        ]),
        Region(vec![
          CellPosition::new(1, 0), CellPosition::new(1, 2), CellPosition::new(2, 0),
          CellPosition::new(2, 1), CellPosition::new(2, 2), CellPosition::new(3, 0),
          CellPosition::new(3, 1), CellPosition::new(3, 2), CellPosition::new(4, 1),
        ]),
        Region(vec![
          CellPosition::new(2, 4), CellPosition::new(2, 5), CellPosition::new(3, 3),
          CellPosition::new(3, 4), CellPosition::new(3, 5), CellPosition::new(4, 2),
          CellPosition::new(4, 3), CellPosition::new(4, 4), CellPosition::new(5, 3),
        ]),
      ]
    );
  (constraints, 52_199)
}

// Currently at 3.4s (debug) and 0.5s (release)
#[test]
#[timeout(9_000)]
fn brute_solver_without_logic_finds_many_solutions_in_extra_regions_9x9() {
  let (constraints, expected_solution_count) = extra_regions_9x9_multiple_solutions_constraints();
  let mut solver = BruteSolver::new(constraints).with_solution_limit(expected_solution_count + 1);
  let result = solver.solve();
  assert_eq!(result.solution_count, expected_solution_count);
}

// Currently at 10s (debug) and 2.5s (release)
#[test]
#[ignore="solver needs more optimizations"]
#[timeout(10_000)]
fn brute_solver_with_logic_finds_many_solutions_in_extra_regions_9x9() {
  let (constraints, expected_solution_count) = extra_regions_9x9_multiple_solutions_constraints();
  let mut solver = BruteSolver::new(constraints).with_logic().with_solution_limit(expected_solution_count + 1);
  let result = solver.solve();
  assert_eq!(result.solution_count, expected_solution_count);
}

fn extra_regions_9x9_no_solution_constraints() -> SudokuConstraints {
  SudokuConstraints::new(9)
    .with_extra_regions(
      [
        Region(vec![
          CellPosition::new(0, 0), CellPosition::new(0, 1), CellPosition::new(0, 2),
          CellPosition::new(0, 3), CellPosition::new(0, 4), CellPosition::new(1, 1),
          CellPosition::new(1, 3), CellPosition::new(1, 4), CellPosition::new(2, 3),
        ]),
        Region(vec![
          CellPosition::new(0, 5), CellPosition::new(0, 6), CellPosition::new(0, 7),
          CellPosition::new(0, 8), CellPosition::new(1, 5), CellPosition::new(1, 6),
          CellPosition::new(1, 7), CellPosition::new(1, 8), CellPosition::new(2, 6),
        ]),
        Region(vec![
          CellPosition::new(1, 0), CellPosition::new(1, 2), CellPosition::new(2, 0),
          CellPosition::new(2, 1), CellPosition::new(2, 2), CellPosition::new(3, 0),
          CellPosition::new(3, 1), CellPosition::new(3, 2), CellPosition::new(4, 1),
        ]),
        Region(vec![
          CellPosition::new(2, 4), CellPosition::new(2, 5), CellPosition::new(3, 3),
          CellPosition::new(3, 4), CellPosition::new(3, 5), CellPosition::new(4, 2),
          CellPosition::new(4, 3), CellPosition::new(4, 4), CellPosition::new(5, 3),
        ]),
        Region(vec![
          CellPosition::new(2, 7), CellPosition::new(2, 8), CellPosition::new(3, 6),
          CellPosition::new(3, 7), CellPosition::new(3, 8), CellPosition::new(4, 5),
          CellPosition::new(4, 6), CellPosition::new(5, 6), CellPosition::new(5, 7),
        ]),
      ]
    )
}

#[test]
#[ignore="solver needs more optimizations"]
#[timeout(10_000)]
fn brute_solver_without_logic_finds_no_solution_in_extra_regions_9x9() {
  let mut solver = BruteSolver::new(extra_regions_9x9_no_solution_constraints()).with_solution_limit(1);
  let result = solver.solve();
  assert_eq!(result.solution, None);
  assert_eq!(result.solution_count, 0);
}

#[test]
#[ignore="solver needs more optimizations"]
#[timeout(10_000)]
fn brute_solver_with_logic_finds_no_solution_in_extra_regions_9x9() {
  let mut solver = BruteSolver::new(extra_regions_9x9_no_solution_constraints()).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution, None);
  assert_eq!(result.solution_count, 0);
}

// https://logicmastersindia.com/live/?contest=SM202301
fn extra_regions_9x9_unique_solution_constraints() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      [
        FixedNumber::new(0, 0, 1),
        FixedNumber::new(0, 2, 8),
        FixedNumber::new(0, 3, 3),
        FixedNumber::new(0, 5, 2),
        FixedNumber::new(0, 6, 5),
        FixedNumber::new(0, 8, 6),
        FixedNumber::new(2, 2, 3),
        FixedNumber::new(2, 6, 8),
        FixedNumber::new(4, 0, 4),
        FixedNumber::new(4, 2, 2),
        FixedNumber::new(4, 4, 5),
        FixedNumber::new(4, 6, 7),
        FixedNumber::new(4, 8, 8),
        FixedNumber::new(6, 2, 9),
        FixedNumber::new(6, 6, 4),
        FixedNumber::new(8, 0, 7),
        FixedNumber::new(8, 2, 5),
        FixedNumber::new(8, 3, 4),
        FixedNumber::new(8, 5, 9),
        FixedNumber::new(8, 6, 6),
        FixedNumber::new(8, 8, 2),
      ]
    )
    .with_extra_regions(
      [
        Region(vec![
          CellPosition::new(1, 0), CellPosition::new(1, 1), CellPosition::new(1, 2),
          CellPosition::new(1, 3), CellPosition::new(2, 0), CellPosition::new(2, 3),
          CellPosition::new(3, 0), CellPosition::new(3, 1), CellPosition::new(3, 2),
        ]),
        Region(vec![
          CellPosition::new(1, 5), CellPosition::new(1, 6), CellPosition::new(1, 7),
          CellPosition::new(1, 8), CellPosition::new(2, 5), CellPosition::new(2, 8),
          CellPosition::new(3, 6), CellPosition::new(3, 7), CellPosition::new(3, 8),
        ]),
        Region(vec![
          CellPosition::new(5, 0), CellPosition::new(5, 1), CellPosition::new(5, 2),
          CellPosition::new(6, 0), CellPosition::new(6, 3), CellPosition::new(7, 0),
          CellPosition::new(7, 1), CellPosition::new(7, 2), CellPosition::new(7, 3),
        ]),
        Region(vec![
          CellPosition::new(5, 6), CellPosition::new(5, 7), CellPosition::new(5, 8),
          CellPosition::new(6, 5), CellPosition::new(6, 8), CellPosition::new(7, 5),
          CellPosition::new(7, 6), CellPosition::new(7, 7), CellPosition::new(7, 8),
        ]),
      ]
    );
  let solution = Grid(vec![
    vec![ 1, 7, 8, 3, 9, 2, 5, 4, 6 ],
    vec![ 9, 4, 6, 5, 1, 8, 2, 7, 3 ],
    vec![ 2, 5, 3, 7, 6, 4, 8, 9, 1 ],
    vec![ 8, 3, 1, 2, 4, 7, 9, 6, 5 ],
    vec![ 4, 6, 2, 9, 5, 1, 7, 3, 8 ],
    vec![ 5, 9, 7, 6, 8, 3, 1, 2, 4 ],
    vec![ 3, 1, 9, 8, 2, 6, 4, 5, 7 ],
    vec![ 6, 2, 4, 1, 7, 5, 3, 8, 9 ],
    vec![ 7, 8, 5, 4, 3, 9, 6, 1, 2 ],
  ]);
  (constraints, solution)
}

#[test]
#[timeout(500)]
fn brute_solver_without_logic_finds_unique_solution_in_extra_regions_9x9() {
  let (constraints, solution) = extra_regions_9x9_unique_solution_constraints();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(500)]
fn brute_solver_with_logic_finds_unique_solution_in_extra_regions_9x9() {
  let (constraints, solution) = extra_regions_9x9_unique_solution_constraints();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

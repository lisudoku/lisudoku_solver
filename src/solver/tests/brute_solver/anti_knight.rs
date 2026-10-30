use crate::{solver::brute::BruteSolver, types::{FixedNumber, Grid, SudokuConstraints}};
use ntest::timeout;

fn anti_knight_9x9_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 4, 9),
        FixedNumber::new(0, 8, 7),
        FixedNumber::new(1, 4, 6),
        FixedNumber::new(2, 0, 6),
        FixedNumber::new(2, 5, 4),
        FixedNumber::new(3, 5, 9),
        FixedNumber::new(3, 8, 5),
        FixedNumber::new(4, 3, 4),
        FixedNumber::new(4, 8, 1),
        FixedNumber::new(5, 3, 7),
        FixedNumber::new(6, 7, 3),
        FixedNumber::new(7, 4, 4),
        FixedNumber::new(8, 0, 3),
        FixedNumber::new(8, 2, 2),
      ]
    )
    .with_anti_knight();
  let solution = Grid::from_string("
    524193867
    183567294
    697284153
    261839475
    738456921
    459721386
    845612739
    976348512
    312975648
  ");

  (constraints, solution)
}

#[test]
#[timeout(3_000)]
fn brute_solver_without_logic_finds_unique_solution_in_anti_knight_9x9() {
  let (constraints, solution) = anti_knight_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(2_000)]
fn brute_solver_with_logic_finds_unique_solution_in_anti_knight_9x9() {
  let (constraints, solution) = anti_knight_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

fn anti_knight_9x9_multiple_solutions() -> (SudokuConstraints, u32) {
  let (mut constraints, _) = anti_knight_9x9_unique_solution();
  constraints.fixed_numbers.as_mut().unwrap().pop();
  constraints.fixed_numbers.as_mut().unwrap().pop();
  (constraints, 954)
}

// Currently at 2.8s (debug) and 0.35s (release)
#[test]
#[timeout(7_000)]
fn brute_solver_without_logic_finds_multiple_solutions_in_anti_knight_9x9() {
  let (constraints, expected_solution_count) = anti_knight_9x9_multiple_solutions();
  let mut solver = BruteSolver::new(constraints).with_solution_limit(expected_solution_count + 1);
  let result = solver.solve();
  assert_eq!(result.solution_count, expected_solution_count);
}

// Currently at 8.3s (debug) and 1.4s (release)
#[test]
#[timeout(12_000)]
fn brute_solver_with_logic_finds_multiple_solutions_in_anti_knight_9x9() {
  let (constraints, expected_solution_count) = anti_knight_9x9_multiple_solutions();
  let mut solver = BruteSolver::new(constraints).with_logic().with_solution_limit(expected_solution_count + 1);
  let result = solver.solve();
  assert_eq!(result.solution_count, expected_solution_count);
}

fn anti_knight_9x9_no_solutions() -> SudokuConstraints {
  let (mut constraints, _) = anti_knight_9x9_unique_solution();
  constraints.fixed_numbers.as_mut().unwrap().push(FixedNumber::new(8, 8, 6));
  constraints
}

#[test]
#[timeout(2_000)]
fn brute_solver_without_logic_finds_no_solutions_in_anti_knight_9x9() {
  let constraints = anti_knight_9x9_no_solutions();
  let mut solver = BruteSolver::new(constraints).with_solution_limit(1);
  let result = solver.solve();
  assert_eq!(result.solution_count, 0);
}

#[test]
#[timeout(2_000)]
fn brute_solver_with_logic_finds_no_solutions_in_anti_knight_9x9() {
  let constraints = anti_knight_9x9_no_solutions();
  let mut solver = BruteSolver::new(constraints).with_logic().with_solution_limit(1);
  let result = solver.solve();
  assert_eq!(result.solution_count, 0);
}

use crate::{solver::brute::BruteSolver, types::{Arrow, CellPosition, FixedNumber, Grid, SudokuConstraints}};
use ntest::timeout;

// https://gp.worldpuzzle.org/sites/default/files/Puzzles/2023/2023_SudokuRound7.pdf puzzle 8
// https://lisudoku.xyz/p/9q70tmwBmkpjvXIBck8j
fn arrow_9x9_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      [ FixedNumber::new(0, 3, 1), FixedNumber::new(5, 8, 1) ]
    )
    .with_arrows(
      [
        Arrow {
          circle_cells: vec![ CellPosition::new(0, 0) ],
          arrow_cells: vec![
            CellPosition::new(1, 1), CellPosition::new(2, 2), CellPosition::new(3, 3),
            CellPosition::new(4, 4), CellPosition::new(5, 5),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(1, 2) ],
          arrow_cells: vec![ CellPosition::new(2, 3) ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(1, 7) ],
          arrow_cells: vec![
            CellPosition::new(2, 8), CellPosition::new(2, 7), CellPosition::new(2, 6),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(2, 4) ],
          arrow_cells: vec![ CellPosition::new(3, 4), CellPosition::new(4, 5) ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(3, 2), CellPosition::new(4, 2) ],
          arrow_cells: vec![
            CellPosition::new(4, 3), CellPosition::new(5, 3), CellPosition::new(5, 4),
            CellPosition::new(6, 5),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(5, 1) ],
          arrow_cells: vec![ CellPosition::new(6, 2) ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(5, 6) ],
          arrow_cells: vec![ CellPosition::new(5, 7), CellPosition::new(4, 8) ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(6, 0) ],
          arrow_cells: vec![
            CellPosition::new(5, 0), CellPosition::new(4, 0), CellPosition::new(3, 0),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(7, 5) ],
          arrow_cells: vec![ CellPosition::new(6, 6) ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(7, 6) ],
          arrow_cells: vec![ CellPosition::new(6, 7), CellPosition::new(7, 8) ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(7, 7) ],
          arrow_cells: vec![
            CellPosition::new(8, 8), CellPosition::new(8, 7), CellPosition::new(8, 6),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(8, 2) ],
          arrow_cells: vec![
            CellPosition::new(8, 3), CellPosition::new(7, 2), CellPosition::new(7, 1),
            CellPosition::new(7, 0),
          ],
        },
      ]
    );
  let solution = Grid(vec![
    vec![ 9, 4, 3, 1, 2, 7, 6, 5, 8 ],
    vec![ 5, 2, 8, 4, 6, 3, 1, 9, 7 ],
    vec![ 7, 6, 1, 8, 9, 5, 2, 4, 3 ],
    vec![ 1, 8, 2, 3, 5, 6, 4, 7, 9 ],
    vec![ 3, 9, 5, 7, 1, 4, 8, 6, 2 ],
    vec![ 4, 7, 6, 9, 8, 2, 5, 3, 1 ],
    vec![ 8, 3, 7, 5, 4, 1, 9, 2, 6 ],
    vec![ 2, 1, 4, 6, 3, 9, 7, 8, 5 ],
    vec![ 6, 5, 9, 2, 7, 8, 3, 1, 4 ],
  ]);

  (constraints, solution)
}

#[test]
#[ignore="solver needs more optimizations"]
#[timeout(10_000)]
fn brute_solver_without_logic_solves_arrow_9x9() {
  let (constraints, solution) = arrow_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(2_000)]
fn brute_solver_with_logic_solves_arrow_9x9() {
  let (constraints, solution) = arrow_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

// https://www.youtube.com/watch?v=vKEmgKgYg_U
// Reaaally long arrow with 2 circle cells and 12 arrow cells
// N4IghgThD2DuDOIBcBtUkawMIFMA2eiqoAxtHsgGwA0ImyADAL7WnnICst9SzAurRIBLCCTw5cBImhBkKSGnTjIAjEz4t0UOJMLIZczt2VI1rWeyQAWY7FWaL8gMy37A2SLET8e4o6qupuoOGDo%2B0mzyDIFODoamMXGWAEyJ5vEuSnbW6oKe4roR%2FryByUnyKqXB5qHY4fqRyDZZyLHplpk8bY1IqS057fJ9PBy5HqIF9X4ZVYNNVRo12nVSDcVc%2Fd3rgVblRv2j7sIT3qvTlhs8ZYtamIVr8QAcsz3P%2FWpH%2Bae%2BBpaKV3skAB2BYhZb3c7yS4mSiA6HZIFjY5eCG%2FeT%2FEyHMF3KZo5Ag%2FqwuZIN48RHE0kmR6Agk8anEjHZek9eHIZnFZp0wGdKmA4a84mVfrs%2BLRYWAsVkpFfVE9WmYwGU7JYpY4s54hIHQFCni7T4nWXFSUw6q3MLqnr87JEno862Azkm%2FUo3E9HUmRE3cDg13FO34vmBT15A2%2B%2BKOhHBdwAcwgQgAJgBlIQALxwyAAnEwgA%3D
fn arrow_9x9_2_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(9)
    .with_arrows(
      [
        Arrow {
          circle_cells: vec![ CellPosition::new(1, 6) ],
          arrow_cells: vec![ CellPosition::new(0, 6), CellPosition::new(0, 5) ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(1, 6) ],
          arrow_cells: vec![
            CellPosition::new(1, 5), CellPosition::new(1, 4), CellPosition::new(1, 3),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(2, 0), CellPosition::new(2, 1) ],
          arrow_cells: vec![
            CellPosition::new(3, 0), CellPosition::new(3, 1), CellPosition::new(3, 2),
            CellPosition::new(4, 3),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(2, 3), CellPosition::new(2, 4) ],
          arrow_cells: vec![
            CellPosition::new(3, 4), CellPosition::new(3, 3), CellPosition::new(4, 2),
            CellPosition::new(5, 2),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(2, 5) ],
          arrow_cells: vec![
            CellPosition::new(3, 5), CellPosition::new(4, 5), CellPosition::new(5, 5),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(2, 6), CellPosition::new(2, 7) ],
          arrow_cells: vec![ CellPosition::new(2, 8), CellPosition::new(1, 8) ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(5, 6) ],
          arrow_cells: vec![ CellPosition::new(6, 5), CellPosition::new(7, 5) ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(5, 7), CellPosition::new(5, 8) ],
          arrow_cells: vec![
            CellPosition::new(6, 7), CellPosition::new(7, 8), CellPosition::new(8, 8),
            CellPosition::new(8, 7), CellPosition::new(8, 6), CellPosition::new(8, 5),
            CellPosition::new(8, 4), CellPosition::new(8, 3), CellPosition::new(8, 2),
            CellPosition::new(8, 1), CellPosition::new(8, 0), CellPosition::new(7, 0),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(6, 0) ],
          arrow_cells: vec![ CellPosition::new(5, 1), CellPosition::new(4, 1) ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(7, 1) ],
          arrow_cells: vec![
            CellPosition::new(6, 2), CellPosition::new(6, 3), CellPosition::new(6, 4),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(7, 4) ],
          arrow_cells: vec![ CellPosition::new(7, 3), CellPosition::new(7, 2) ],
        },
      ]
    );
  let solution = Grid(vec![
    vec![ 8, 1, 4, 9, 7, 6, 2, 5, 3 ],
    vec![ 3, 9, 6, 2, 1, 5, 8, 4, 7 ],
    vec![ 2, 5, 7, 3, 4, 8, 1, 6, 9 ],
    vec![ 6, 7, 5, 8, 9, 1, 3, 2, 4 ],
    vec![ 1, 3, 9, 7, 2, 4, 6, 8, 5 ],
    vec![ 4, 2, 8, 5, 6, 3, 9, 7, 1 ],
    vec![ 5, 8, 2, 1, 3, 7, 4, 9, 6 ],
    vec![ 9, 6, 1, 4, 5, 2, 7, 3, 8 ],
    vec![ 7, 4, 3, 6, 8, 9, 5, 1, 2 ],
  ]);

  (constraints, solution)
}

#[test]
#[ignore="solver needs more optimizations"]
#[timeout(10_000)]
fn brute_solver_without_logic_solves_arrow_9x9_2() {
  let (constraints, solution) = arrow_9x9_2_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(13_000)]
fn brute_solver_with_logic_solves_arrow_9x9_2() {
  let (constraints, solution) = arrow_9x9_2_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

fn arrow_4x4_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      [
        FixedNumber::new(0, 3, 3), FixedNumber::new(2, 0, 1),
      ]
    )
    .with_arrows(
      [
        Arrow {
          circle_cells: vec![ CellPosition::new(3, 0) ],
          arrow_cells: vec![
            CellPosition::new(2, 1), CellPosition::new(1, 2), CellPosition::new(0, 1),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(3, 1) ],
          arrow_cells: vec![ CellPosition::new(3, 2), CellPosition::new(3, 3) ],
        },
      ]
    );
  let solution = Grid(vec![
    vec![ 2, 1, 4, 3 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 1, 2, 3, 4 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  (constraints, solution)
}

#[test]
#[timeout(500)]
fn brute_solver_without_logic_solves_arrow_4x4() {
  let (constraints, solution) = arrow_4x4_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

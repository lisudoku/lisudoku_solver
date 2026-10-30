use crate::{solver::brute::BruteSolver, types::{CellPosition, FixedNumber, Grid, Renban, SudokuConstraints}};
use ntest::timeout;

// https://gp.worldpuzzle.org/sites/default/files/Puzzles/2024/2024_SudokuRound4.pdf
fn renban_9x9_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 1, 4), FixedNumber::new(0, 5, 8), FixedNumber::new(2, 6, 7),
        FixedNumber::new(3, 8, 6), FixedNumber::new(5, 0, 5), FixedNumber::new(6, 2, 2),
        FixedNumber::new(8, 3, 1), FixedNumber::new(8, 7, 3),
      ]
    )
    .with_renbans(
      vec![
        Renban(vec![
          CellPosition::new(0, 7), CellPosition::new(0, 8), CellPosition::new(1, 8),
        ]),
        Renban(vec![
          CellPosition::new(1, 0), CellPosition::new(1, 1),
        ]),
        Renban(vec![
          CellPosition::new(1, 2), CellPosition::new(1, 3), CellPosition::new(2, 3),
        ]),
        Renban(vec![
          CellPosition::new(1, 5), CellPosition::new(1, 6), CellPosition::new(1, 7),
        ]),
        Renban(vec![
          CellPosition::new(2, 0), CellPosition::new(3, 0), CellPosition::new(3, 1),
          CellPosition::new(4, 1),
        ]),
        Renban(vec![
          CellPosition::new(3, 3), CellPosition::new(3, 4), CellPosition::new(3, 5),
          CellPosition::new(3, 6),
        ]),
        Renban(vec![
          CellPosition::new(4, 7), CellPosition::new(5, 7), CellPosition::new(5, 8),
          CellPosition::new(6, 8),
        ]),
        Renban(vec![
          CellPosition::new(5, 2), CellPosition::new(5, 3), CellPosition::new(5, 4),
          CellPosition::new(5, 5),
        ]),
        Renban(vec![
          CellPosition::new(6, 5), CellPosition::new(7, 5), CellPosition::new(7, 6),
        ]),
        Renban(vec![
          CellPosition::new(7, 0), CellPosition::new(8, 0), CellPosition::new(8, 1),
        ]),
        Renban(vec![
          CellPosition::new(7, 1), CellPosition::new(7, 2), CellPosition::new(7, 3),
        ]),
        Renban(vec![
          CellPosition::new(7, 7), CellPosition::new(7, 8),
        ]),
      ]
    );
  let solution = Grid(vec![
    vec![ 1, 4, 7, 9, 2, 8, 3, 6, 5 ],
    vec![ 2, 3, 6, 5, 1, 7, 8, 9, 4 ],
    vec![ 8, 5, 9, 4, 3, 6, 7, 1, 2 ],
    vec![ 9, 7, 1, 3, 4, 2, 5, 8, 6 ],
    vec![ 3, 6, 4, 8, 5, 1, 9, 2, 7 ],
    vec![ 5, 2, 8, 6, 7, 9, 1, 4, 3 ],
    vec![ 4, 9, 2, 7, 8, 3, 6, 5, 1 ],
    vec![ 6, 1, 3, 2, 9, 5, 4, 7, 8 ],
    vec![ 7, 8, 5, 1, 6, 4, 2, 3, 9 ],
  ]);

  (constraints, solution)
}

#[test]
#[ignore="solver needs more optimizations"]
#[timeout(10_000)]
fn brute_solver_without_logic_finds_unique_solution_in_renban_9x9() {
  let (constraints, solution) = renban_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(1_000)]
fn brute_solver_with_logic_finds_unique_solution_in_renban_9x9() {
  let (constraints, solution) = renban_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

// // https://logicmastersindia.com/live/?contest=SM202403
fn renban_6x6_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 5, 1),
        FixedNumber::new(1, 4, 6),
        FixedNumber::new(3, 4, 2),
        FixedNumber::new(5, 0, 4),
      ]
    )
    .with_renbans(
      vec![
        Renban(vec![
          CellPosition::new(0, 0),
          CellPosition::new(1, 1),
          CellPosition::new(2, 1),
        ]),
        Renban(vec![
          CellPosition::new(0, 1),
          CellPosition::new(1, 2),
        ]),
        Renban(vec![
          CellPosition::new(0, 4),
          CellPosition::new(1, 3),
        ]),
        Renban(vec![
          CellPosition::new(2, 0),
          CellPosition::new(3, 0),
        ]),
        Renban(vec![
          CellPosition::new(3, 3),
          CellPosition::new(2, 2),
          CellPosition::new(3, 2),
          CellPosition::new(4, 3),
          CellPosition::new(5, 4),
          CellPosition::new(5, 3),
        ]),
        Renban(vec![
          CellPosition::new(3, 5),
          CellPosition::new(2, 5),
          CellPosition::new(2, 4),
          CellPosition::new(2, 3),
          CellPosition::new(3, 4),
        ]),
        Renban(vec![
          CellPosition::new(4, 0),
          CellPosition::new(3, 1),
          CellPosition::new(4, 2),
          CellPosition::new(5, 1),
        ]),
        Renban(vec![
          CellPosition::new(4, 4),
          CellPosition::new(5, 5),
          CellPosition::new(4, 5),
        ]),
      ]
    );
  let solution = Grid(vec![
    vec![ 3, 5, 6, 2, 4, 1 ],
    vec![ 1, 2, 4, 3, 6, 5 ],
    vec![ 6, 1, 2, 5, 3, 4 ],
    vec![ 5, 4, 3, 1, 2, 6 ],
    vec![ 2, 6, 5, 4, 1, 3 ],
    vec![ 4, 3, 1, 6, 5, 2 ],
  ]);

  (constraints, solution)
}

#[test]
#[timeout(1_000)]
fn brute_solver_without_logic_finds_unique_solution_in_renban_6x6() {
  let (constraints, solution) = renban_6x6_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

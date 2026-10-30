use crate::{solver::brute::BruteSolver, types::{CellPosition, FixedNumber, Grid, KillerCage, Region, SudokuConstraints}};
use ntest::timeout;

// https://ukpuzzles.org/file_download.php?fileid=247&md5=c200e06d8822177932d906103919ceba
fn killer_9x9_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(9)
    .with_killer_cages(
      vec![
        KillerCage {
          sum: Some(20),
          region: Region(vec![
            CellPosition::new(0, 0), CellPosition::new(0, 1),
            CellPosition::new(1, 0), CellPosition::new(1, 1),
          ]),
        },
        KillerCage {
          sum: Some(9),
          region: Region(vec![
            CellPosition::new(0, 2), CellPosition::new(0, 3),
          ]),
        },
        KillerCage {
          sum: Some(22),
          region: Region(vec![
            CellPosition::new(1, 2), CellPosition::new(2, 2), CellPosition::new(2, 1),
          ]),
        },
        KillerCage {
          sum: Some(17),
          region: Region(vec![
            CellPosition::new(0, 4), CellPosition::new(1, 4), CellPosition::new(2, 4),
          ]),
        },
        KillerCage {
          sum: Some(9),
          region: Region(vec![
            CellPosition::new(0, 5), CellPosition::new(0, 6),
          ]),
        },
        KillerCage {
          sum: Some(18),
          region: Region(vec![
            CellPosition::new(1, 6), CellPosition::new(2, 6), CellPosition::new(2, 7),
          ]),
        },
        KillerCage {
          sum: Some(20),
          region: Region(vec![
            CellPosition::new(0, 7), CellPosition::new(0, 8),
            CellPosition::new(1, 7), CellPosition::new(1, 8),
          ]),
        },
        KillerCage {
          sum: Some(8),
          region: Region(vec![
            CellPosition::new(2, 0), CellPosition::new(3, 0),
          ]),
        },
        KillerCage {
          sum: Some(16),
          region: Region(vec![
            CellPosition::new(1, 3), CellPosition::new(2, 3), CellPosition::new(3, 3),
            CellPosition::new(3, 2), CellPosition::new(3, 1),
          ]),
        },
        KillerCage {
          sum: Some(27),
          region: Region(vec![
            CellPosition::new(1, 5), CellPosition::new(2, 5), CellPosition::new(3, 5),
            CellPosition::new(3, 6), CellPosition::new(3, 7),
          ]),
        },
        KillerCage {
          sum: Some(11),
          region: Region(vec![
            CellPosition::new(2, 8), CellPosition::new(3, 8),
          ]),
        },
        KillerCage {
          sum: Some(12),
          region: Region(vec![
            CellPosition::new(4, 0), CellPosition::new(4, 1), CellPosition::new(4, 2),
          ]),
        },
        KillerCage {
          sum: Some(22),
          region: Region(vec![
            CellPosition::new(4, 6), CellPosition::new(4, 7), CellPosition::new(4, 8),
          ]),
        },
        KillerCage {
          sum: Some(15),
          region: Region(vec![
            CellPosition::new(3, 4), CellPosition::new(4, 3), CellPosition::new(4, 4),
            CellPosition::new(4, 5), CellPosition::new(5, 4),
          ]),
        },
        KillerCage {
          sum: Some(8),
          region: Region(vec![
            CellPosition::new(5, 0), CellPosition::new(6, 0),
          ]),
        },
        KillerCage {
          sum: Some(30),
          region: Region(vec![
            CellPosition::new(5, 1), CellPosition::new(5, 2), CellPosition::new(5, 3),
            CellPosition::new(6, 3), CellPosition::new(7, 3),
          ]),
        },
        KillerCage {
          sum: Some(17),
          region: Region(vec![
            CellPosition::new(5, 7), CellPosition::new(5, 6), CellPosition::new(5, 5),
            CellPosition::new(6, 5), CellPosition::new(7, 5),
          ]),
        },
        KillerCage {
          sum: Some(11),
          region: Region(vec![
            CellPosition::new(5, 8), CellPosition::new(6, 8),
          ]),
        },
        KillerCage {
          sum: Some(11),
          region: Region(vec![
            CellPosition::new(6, 1), CellPosition::new(6, 2), CellPosition::new(7, 2),
          ]),
        },
        KillerCage {
          sum: Some(24),
          region: Region(vec![
            CellPosition::new(7, 0), CellPosition::new(7, 1),
            CellPosition::new(8, 0), CellPosition::new(8, 1),
          ]),
        },
        KillerCage {
          sum: Some(16),
          region: Region(vec![
            CellPosition::new(8, 2), CellPosition::new(8, 3),
          ]),
        },
        KillerCage {
          sum: Some(19),
          region: Region(vec![
            CellPosition::new(6, 4), CellPosition::new(7, 4), CellPosition::new(8, 4),
          ]),
        },
        KillerCage {
          sum: Some(10),
          region: Region(vec![
            CellPosition::new(8, 5), CellPosition::new(8, 6),
          ]),
        },
        KillerCage {
          sum: Some(16),
          region: Region(vec![
            CellPosition::new(6, 7), CellPosition::new(6, 6), CellPosition::new(7, 6),
          ]),
        },
        KillerCage {
          sum: Some(17),
          region: Region(vec![
            CellPosition::new(7, 7), CellPosition::new(7, 8),
            CellPosition::new(8, 7), CellPosition::new(8, 8),
          ]),
        },
      ]
    );
  let solution = Grid(vec![
    vec![ 6, 3, 2, 7, 9, 8, 1, 5, 4 ],
    vec![ 4, 7, 8, 1, 6, 5, 3, 9, 2 ],
    vec![ 1, 9, 5, 3, 2, 4, 7, 8, 6 ],
    vec![ 7, 2, 4, 6, 3, 9, 8, 1, 5 ],
    vec![ 8, 1, 3, 4, 5, 2, 6, 7, 9 ],
    vec![ 5, 6, 9, 8, 1, 7, 2, 4, 3 ],
    vec![ 3, 4, 6, 5, 7, 1, 9, 2, 8 ],
    vec![ 9, 8, 1, 2, 4, 3, 5, 6, 7 ],
    vec![ 2, 5, 7, 9, 8, 6, 4, 3, 1 ],
  ]);

  (constraints, solution)
}

// Currently at 60+s
#[test]
#[ignore="solver needs more optimizations"]
#[timeout(10_000)]
fn brute_solver_without_logic_finds_unique_solution_in_killer_9x9() {
  let (constraints, solution) = killer_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(2_000)]
fn brute_solver_with_logic_finds_unique_solution_in_killer_9x9() {
  let (constraints, solution) = killer_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

// https://uploads-ssl.webflow.com/62793457876c001d28edf162/6348945a45b06acb414391b7_WSC_2022_IB_v2.1.pdf
fn killer_6x6_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 1),
        FixedNumber::new(0, 2, 3),
        FixedNumber::new(5, 3, 3),
        FixedNumber::new(5, 5, 2),
      ]
    )
    .with_killer_cages(
      vec![
        KillerCage {
          sum: Some(5),
          region: Region(vec![
            CellPosition::new(0, 0), CellPosition::new(1, 0),
          ]),
        },
        KillerCage {
          sum: Some(15),
          region: Region(vec![
            CellPosition::new(0, 1), CellPosition::new(0, 2),
            CellPosition::new(0, 3), CellPosition::new(0, 4),
            CellPosition::new(1, 3),
          ]),
        },
        KillerCage {
          sum: Some(5),
          region: Region(vec![ CellPosition::new(1, 1) ]),
        },
        KillerCage {
          sum: Some(5),
          region: Region(vec![
            CellPosition::new(1, 4), CellPosition::new(1, 5),
          ]),
        },
        KillerCage {
          sum: Some(5),
          region: Region(vec![
            CellPosition::new(2, 0), CellPosition::new(2, 1),
          ]),
        },
        KillerCage {
          sum: Some(5),
          region: Region(vec![
            CellPosition::new(2, 2), CellPosition::new(3, 2),
          ]),
        },
        KillerCage {
          sum: Some(5),
          region: Region(vec![ CellPosition::new(3, 0) ]),
        },
        KillerCage {
          sum: Some(15),
          region: Region(vec![
            CellPosition::new(2, 3), CellPosition::new(2, 4), CellPosition::new(2, 5),
          ]),
        },
        KillerCage {
          sum: Some(5),
          region: Region(vec![
            CellPosition::new(3, 3), CellPosition::new(3, 4),
          ]),
        },
        KillerCage {
          sum: Some(15),
          region: Region(vec![
            CellPosition::new(4, 0), CellPosition::new(4, 1), CellPosition::new(4, 2),
            CellPosition::new(5, 0),
          ]),
        },
        KillerCage {
          sum: Some(5),
          region: Region(vec![ CellPosition::new(4, 5) ]),
        },
        KillerCage {
          sum: Some(15),
          region: Region(vec![
            CellPosition::new(5, 1), CellPosition::new(5, 2), CellPosition::new(5, 3),
            CellPosition::new(5, 4), CellPosition::new(5, 5),
          ]),
        },
      ]
    );
  let solution = Grid(vec![
    vec![ 1, 2, 3, 4, 5, 6 ],
    vec![ 4, 5, 6, 1, 2, 3 ],
    vec![ 2, 3, 1, 5, 6, 4 ],
    vec![ 5, 6, 4, 2, 3, 1 ],
    vec![ 3, 4, 2, 6, 1, 5 ],
    vec![ 6, 1, 5, 3, 4, 2 ],
  ]);

  (constraints, solution)
}

#[test]
#[timeout(500)]
fn brute_solver_without_logic_finds_unique_solution_in_killer_6x6() {
  let (constraints, solution) = killer_6x6_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

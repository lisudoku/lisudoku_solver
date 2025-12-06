use crate::{solver::Solver, types::{CellPosition, FixedNumber, Grid, Renban, Rule, SolutionType, SudokuConstraints}};

// https://logicmastersindia.com/live/?contest=SM202403
#[test]
fn check_renban_9x9_1_solve() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 5), FixedNumber::new(2, 1, 6), FixedNumber::new(0, 4, 3),
        FixedNumber::new(0, 5, 6), FixedNumber::new(1, 3, 2), FixedNumber::new(3, 1, 3),
        FixedNumber::new(3, 0, 7), FixedNumber::new(4, 0, 8), FixedNumber::new(3, 5, 1),
        FixedNumber::new(5, 3, 4), FixedNumber::new(3, 7, 2), FixedNumber::new(4, 8, 1),
        FixedNumber::new(5, 8, 7), FixedNumber::new(5, 7, 9), FixedNumber::new(7, 5, 8),
        FixedNumber::new(8, 3, 3), FixedNumber::new(8, 4, 2), FixedNumber::new(6, 7, 7),
      ]
    )
    .with_renbans(
      vec![
        Renban(vec![
          CellPosition::new(1, 0), CellPosition::new(0, 0), CellPosition::new(0, 1),
          CellPosition::new(0, 2), CellPosition::new(1, 2),
        ]),
        Renban(vec![
          CellPosition::new(1, 1), CellPosition::new(2, 0),
        ]),
        Renban(vec![
          CellPosition::new(1, 5), CellPosition::new(0, 6), CellPosition::new(0, 7),
          CellPosition::new(1, 8),
        ]),
        Renban(vec![
          CellPosition::new(1, 6), CellPosition::new(2, 6), CellPosition::new(2, 7),
          CellPosition::new(2, 8), CellPosition::new(1, 7),
        ]),
        Renban(vec![
          CellPosition::new(2, 4), CellPosition::new(2, 3), CellPosition::new(2, 2),
          CellPosition::new(3, 2), CellPosition::new(4, 2),
        ]),
        Renban(vec![
          CellPosition::new(2, 5), CellPosition::new(3, 6), CellPosition::new(4, 6),
          CellPosition::new(3, 7), CellPosition::new(4, 7),
        ]),
        Renban(vec![
          CellPosition::new(4, 3), CellPosition::new(3, 3), CellPosition::new(3, 4),
          CellPosition::new(4, 4), CellPosition::new(5, 4),
        ]),
        Renban(vec![
          CellPosition::new(4, 5), CellPosition::new(5, 5), CellPosition::new(6, 5),
          CellPosition::new(6, 4), CellPosition::new(7, 4),
        ]),
        Renban(vec![
          CellPosition::new(5, 2), CellPosition::new(5, 1), CellPosition::new(5, 0),
          CellPosition::new(6, 0), CellPosition::new(7, 0),
        ]),
        Renban(vec![
          CellPosition::new(7, 1), CellPosition::new(8, 0), CellPosition::new(8, 1),
          CellPosition::new(8, 2), CellPosition::new(7, 2),
        ]),
        Renban(vec![
          CellPosition::new(7, 6), CellPosition::new(8, 5),
        ]),
        Renban(vec![
          CellPosition::new(7, 7), CellPosition::new(7, 8), CellPosition::new(8, 8),
          CellPosition::new(8, 7), CellPosition::new(8, 6),
        ]),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 5, 2, 4, 1, 3, 6, 7, 8, 9 ],
      vec![ 1, 8, 3, 2, 7, 9, 4, 5, 6 ],
      vec![ 9, 6, 7, 5, 8, 4, 2, 1, 3 ],
      vec![ 7, 3, 6, 8, 9, 1, 5, 2, 4 ],
      vec![ 8, 4, 9, 7, 5, 2, 3, 6, 1 ],
      vec![ 2, 5, 1, 4, 6, 3, 8, 9, 7 ],
      vec![ 3, 1, 2, 6, 4, 5, 9, 7, 8 ],
      vec![ 4, 7, 5, 9, 1, 8, 6, 3, 2 ],
      vec![ 6, 9, 8, 3, 2, 7, 1, 4, 5 ],
    ])
  );
  let rules: Vec<_> = result.steps.iter().map(|step| step.rule).collect();
  assert!(rules.contains(&Rule::RenbanCandidates));
  assert!(!rules.contains(&Rule::NishioForcingChains));
  insta::assert_yaml_snapshot!(result.steps);
}

// https://logicmastersindia.com/live/main?contest=SM202403 IB puzzle 16
#[test]
fn check_renban_9x9_2_solve() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(1, 1, 1), FixedNumber::new(1, 2, 2), FixedNumber::new(1, 3, 3),
        FixedNumber::new(2, 1, 4), FixedNumber::new(2, 2, 5), FixedNumber::new(2, 3, 6),
        FixedNumber::new(3, 1, 7), FixedNumber::new(3, 2, 8), FixedNumber::new(3, 3, 9),
        FixedNumber::new(1, 6, 4), FixedNumber::new(1, 7, 5), FixedNumber::new(2, 6, 3),
        FixedNumber::new(2, 7, 2), FixedNumber::new(5, 5, 1), FixedNumber::new(5, 6, 2),
        FixedNumber::new(5, 7, 3), FixedNumber::new(6, 5, 4), FixedNumber::new(6, 6, 5),
        FixedNumber::new(6, 7, 6), FixedNumber::new(7, 5, 7), FixedNumber::new(7, 6, 8),
        FixedNumber::new(7, 7, 9), FixedNumber::new(6, 1, 2), FixedNumber::new(6, 2, 3),
        FixedNumber::new(7, 1, 5), FixedNumber::new(7, 2, 4),
      ]
    )
    .with_renbans(
      vec![
        Renban(vec![
          CellPosition::new(0, 7), CellPosition::new(0, 8),
          CellPosition::new(1, 8), CellPosition::new(2, 8),
        ]),
        Renban(vec![
          CellPosition::new(1, 4), CellPosition::new(2, 5),
        ]),
        Renban(vec![
          CellPosition::new(4, 3), CellPosition::new(5, 3), CellPosition::new(5, 4),
          CellPosition::new(4, 4),
        ]),
        Renban(vec![
          CellPosition::new(6, 0), CellPosition::new(7, 0), CellPosition::new(8, 1),
          CellPosition::new(8, 2),
        ]),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 3, 6, 7, 4, 2, 5, 1, 8, 9 ],
      vec![ 8, 1, 2, 3, 7, 9, 4, 5, 6 ],
      vec![ 9, 4, 5, 6, 1, 8, 3, 2, 7 ],
      vec![ 2, 7, 8, 9, 4, 3, 6, 1, 5 ],
      vec![ 4, 3, 1, 5, 6, 2, 9, 7, 8 ],
      vec![ 5, 9, 6, 7, 8, 1, 2, 3, 4 ],
      vec![ 7, 2, 3, 8, 9, 4, 5, 6, 1 ],
      vec![ 6, 5, 4, 1, 3, 7, 8, 9, 2 ],
      vec![ 1, 8, 9, 2, 5, 6, 7, 4, 3 ],
    ])
  );
  let rules: Vec<_> = result.steps.iter().map(|step| step.rule).collect();
  assert!(rules.contains(&Rule::RenbanCandidates));
  assert!(!rules.contains(&Rule::NishioForcingChains));
  insta::assert_yaml_snapshot!(result.steps);
}

// https://gp.worldpuzzle.org/sites/default/files/Puzzles/2024/2024_SudokuRound4.pdf
#[test]
fn check_renban_9x9_3_solve() {
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
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 1, 4, 7, 9, 2, 8, 3, 6, 5 ],
      vec![ 2, 3, 6, 5, 1, 7, 8, 9, 4 ],
      vec![ 8, 5, 9, 4, 3, 6, 7, 1, 2 ],
      vec![ 9, 7, 1, 3, 4, 2, 5, 8, 6 ],
      vec![ 3, 6, 4, 8, 5, 1, 9, 2, 7 ],
      vec![ 5, 2, 8, 6, 7, 9, 1, 4, 3 ],
      vec![ 4, 9, 2, 7, 8, 3, 6, 5, 1 ],
      vec![ 6, 1, 3, 2, 9, 5, 4, 7, 8 ],
      vec![ 7, 8, 5, 1, 6, 4, 2, 3, 9 ],
    ])
  );
  let rules: Vec<_> = result.steps.iter().map(|step| step.rule).collect();
  assert!(rules.contains(&Rule::RenbanCandidates));
  assert!(!rules.contains(&Rule::NishioForcingChains));
  insta::assert_yaml_snapshot!(result.steps);
}

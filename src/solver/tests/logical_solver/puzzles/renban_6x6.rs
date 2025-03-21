use crate::{solver::Solver, types::{CellPosition, FixedNumber, Rule, SolutionType, SudokuConstraints}};

// https://logicmastersindia.com/live/?contest=SM202403
#[test]
fn check_renban_6x6_1_solve() {
  let grid_size = 6;
  let fixed_numbers = vec![
    FixedNumber::new(0, 5, 1),
    FixedNumber::new(1, 4, 6),
    FixedNumber::new(3, 4, 2),
    FixedNumber::new(5, 0, 4),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.renbans = vec![
    vec![
      CellPosition::new(0, 0),
      CellPosition::new(1, 1),
      CellPosition::new(2, 1),
    ],
    vec![
      CellPosition::new(0, 1),
      CellPosition::new(1, 2),
    ],
    vec![
      CellPosition::new(0, 4),
      CellPosition::new(1, 3),
    ],
    vec![
      CellPosition::new(2, 0),
      CellPosition::new(3, 0),
    ],
    vec![
      CellPosition::new(3, 3),
      CellPosition::new(2, 2),
      CellPosition::new(3, 2),
      CellPosition::new(4, 3),
      CellPosition::new(5, 4),
      CellPosition::new(5, 3),
    ],
    vec![
      CellPosition::new(3, 5),
      CellPosition::new(2, 5),
      CellPosition::new(2, 4),
      CellPosition::new(2, 3),
      CellPosition::new(3, 4),
    ],
    vec![
      CellPosition::new(4, 0),
      CellPosition::new(3, 1),
      CellPosition::new(4, 2),
      CellPosition::new(5, 1),
    ],
    vec![
      CellPosition::new(4, 4),
      CellPosition::new(5, 5),
      CellPosition::new(4, 5),
    ],
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  dbg!(&result);
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 3, 5, 6, 2, 4, 1 ],
    vec![ 1, 2, 4, 3, 6, 5 ],
    vec![ 6, 1, 2, 5, 3, 4 ],
    vec![ 5, 4, 3, 1, 2, 6 ],
    vec![ 2, 6, 5, 4, 1, 3 ],
    vec![ 4, 3, 1, 6, 5, 2 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
  let rules: Vec<_> = result.steps.iter().map(|step| step.rule).collect();
  assert!(rules.contains(&Rule::RenbanCandidates));
  assert!(!rules.contains(&Rule::NishioForcingChains));
  insta::assert_yaml_snapshot!(result.steps);
}

// https://logicmastersindia.com/live/main?contest=SM202403 IB puzzle 15
#[test]
fn check_renban_6x6_2_solve() {
  let grid_size = 6;
  let fixed_numbers = vec![
    FixedNumber::new(0, 1, 1), FixedNumber::new(0, 4, 2), FixedNumber::new(1, 0, 5),
    FixedNumber::new(1, 3, 1), FixedNumber::new(1, 5, 4), FixedNumber::new(2, 4, 5),
    FixedNumber::new(3, 1, 5), FixedNumber::new(4, 0, 3), FixedNumber::new(4, 2, 5),
    // Note: commenting these to make original puzzle harder to actually need renban technique
    // FixedNumber::new(4, 5, 1), FixedNumber::new(5, 1, 4), FixedNumber::new(5, 4, 3),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.renbans = vec![
    vec![
      CellPosition::new(0, 2), CellPosition::new(1, 1), CellPosition::new(2, 0),
    ],
    vec![
      CellPosition::new(0, 5), CellPosition::new(1, 4), CellPosition::new(2, 3),
      CellPosition::new(3, 2), CellPosition::new(4, 1), CellPosition::new(5, 0),
    ],
    vec![
      CellPosition::new(3, 5), CellPosition::new(4, 4), CellPosition::new(5, 3),
    ],
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 6, 1, 4, 3, 2, 5 ],
    vec![ 5, 3, 2, 1, 6, 4 ],
    vec![ 2, 6, 1, 4, 5, 3 ],
    vec![ 4, 5, 3, 2, 1, 6 ],
    vec![ 3, 2, 5, 6, 4, 1 ],
    vec![ 1, 4, 6, 5, 3, 2 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
  let rules: Vec<_> = result.steps.iter().map(|step| step.rule).collect();
  assert!(rules.contains(&Rule::RenbanCandidates));
  assert!(!rules.contains(&Rule::NishioForcingChains));
  insta::assert_yaml_snapshot!(result.steps);
}

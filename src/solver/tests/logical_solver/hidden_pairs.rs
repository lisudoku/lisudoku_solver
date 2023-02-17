use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule}, solver::{Solver, logical_solver::{candidates::Candidates, technique::Technique, hidden_set::HiddenSet}}};

#[test]
fn check_hidden_pairs_with_affected_cells() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 3, 1),
    FixedNumber::new(0, 4, 2),
    FixedNumber::new(3, 1, 1),
    FixedNumber::new(4, 1, 2),
    FixedNumber::new(6, 2, 1),
    FixedNumber::new(7, 2, 2),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = HiddenSet::new(2).run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::HiddenPairs);
  assert_eq!(step.cells, vec![ CellPosition::new(1, 0), CellPosition::new(2, 0) ]);
  assert_eq!(step.values, vec![ 1, 2 ]);
  assert_eq!(solver.candidates[1][0].len(), 9);
  assert_eq!(solver.candidates[2][0].len(), 9);

  solver.apply_rule(&mut step);
  assert_eq!(solver.candidates[2][0].len(), 2);
  assert_eq!(solver.candidates[2][0].len(), 2);
}

#[test]
fn check_hidden_pairs_no_affected_cells() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 3, 1),
    FixedNumber::new(0, 4, 2),
    FixedNumber::new(3, 1, 1),
    FixedNumber::new(4, 1, 2),
    FixedNumber::new(6, 2, 1),
    FixedNumber::new(7, 2, 2),
    FixedNumber::new(0, 0, 3),
    FixedNumber::new(3, 0, 4),
    FixedNumber::new(4, 0, 5),
    FixedNumber::new(5, 0, 6),
    FixedNumber::new(6, 0, 7),
    FixedNumber::new(7, 0, 8),
    FixedNumber::new(8, 0, 9),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = HiddenSet::new(2).run(&solver);
  assert!(steps.is_empty());
}

use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule}, solver::{Solver, intuitive_solver::{candidates::Candidates, technique::Technique, naked_set::NakedSet}}};
use itertools::Itertools;

#[test]
fn check_naked_pairs_with_affected_cells() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 1),
    FixedNumber::new(0, 1, 2),
    FixedNumber::new(0, 2, 3),
    FixedNumber::new(2, 0, 7),
    FixedNumber::new(2, 1, 8),
    FixedNumber::new(2, 2, 9),
    FixedNumber::new(5, 0, 6),
    FixedNumber::new(6, 2, 6),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = NakedSet::new(2).run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::NakedPairs);
  assert_eq!(step.cells, vec![ CellPosition::new(1, 0), CellPosition::new(1, 2) ]);
  assert_eq!(step.values.iter().cloned().sorted().collect::<Vec<u32>>(), vec![ 4, 5 ]);
  assert!(step.affected_cells.len() == 7);
  let initial_candidates = solver.candidates[1][8].clone();

  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[1][8];
  assert_eq!(final_candidates.len(), initial_candidates.len() - 2);
}

#[test]
fn check_naked_pairs_no_affected_cells() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 1),
    FixedNumber::new(0, 1, 2),
    FixedNumber::new(0, 2, 3),
    FixedNumber::new(0, 7, 4),
    FixedNumber::new(0, 8, 5),
    FixedNumber::new(1, 0, 6),
    FixedNumber::new(2, 0, 7),
    FixedNumber::new(2, 1, 8),
    FixedNumber::new(2, 2, 9),
    FixedNumber::new(2, 4, 4),
    FixedNumber::new(2, 5, 5),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = NakedSet::new(2).run(&solver);
  assert!(steps.is_empty());
}

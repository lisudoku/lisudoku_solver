use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule}, solver::{Solver, logical_solver::{candidates::Candidates, technique::Technique, naked_set::NakedSet}}};

#[test]
fn check_naked_triples() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(2, 2, 9),
        FixedNumber::new(3, 2, 3),
        FixedNumber::new(4, 2, 4),
        FixedNumber::new(5, 2, 5),
        FixedNumber::new(6, 2, 6),
        FixedNumber::new(7, 2, 7),
        FixedNumber::new(8, 2, 8),
        FixedNumber::new(1, 3, 5),
        FixedNumber::new(1, 4, 6),
        FixedNumber::new(1, 5, 9),
        FixedNumber::new(2, 3, 8),
        FixedNumber::new(2, 4, 7),
        FixedNumber::new(4, 3, 1),
        FixedNumber::new(5, 3, 4),
        FixedNumber::new(7, 4, 2),
        FixedNumber::new(8, 4, 4),
      ]
    );
  let mut solver = Solver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = NakedSet::new(3).run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::NakedTriples);
  assert_eq!(step.cells, vec![ CellPosition::new(0, 2), CellPosition::new(0, 3), CellPosition::new(0, 4) ]);
  assert_eq!(step.values.iter().cloned().collect::<Vec<u32>>(), vec![ 1, 2, 3 ]);
  assert!(step.affected_cells.len() == 6);
  let initial_candidates = solver.candidates[0][8].clone();

  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[0][8];
  assert_eq!(final_candidates.len(), initial_candidates.len() - 3);
}

#[test]
fn check_naked_triples_no_affected_cells() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 8),
        FixedNumber::new(0, 1, 6),
        FixedNumber::new(0, 2, 7),
        FixedNumber::new(0, 3, 5),
        FixedNumber::new(0, 4, 9),
        FixedNumber::new(0, 5, 1),
        FixedNumber::new(0, 6, 2),
        FixedNumber::new(0, 7, 3),
        FixedNumber::new(0, 8, 4),
        FixedNumber::new(1, 3, 6),
        FixedNumber::new(1, 4, 3),
        FixedNumber::new(1, 5, 8),
        FixedNumber::new(1, 6, 9),
        FixedNumber::new(1, 7, 1),
        FixedNumber::new(1, 8, 7),
        FixedNumber::new(2, 0, 3),
        FixedNumber::new(2, 1, 9),
        FixedNumber::new(2, 2, 1),
        FixedNumber::new(2, 3, 7),
        FixedNumber::new(2, 4, 2),
        FixedNumber::new(2, 5, 4),
        FixedNumber::new(2, 6, 5),
        FixedNumber::new(2, 7, 6),
        FixedNumber::new(2, 8, 8),
      ]
    );
  let mut solver = Solver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = NakedSet::new(3).run(&solver);
  assert!(steps.is_empty());
}

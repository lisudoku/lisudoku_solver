use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule, Area}, solver::{Solver, logical_solver::{candidates::Candidates, technique::Technique, locked_candidates::LockedCandidates}}};

#[test]
fn check_locked_candidates_triples() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 4),
    FixedNumber::new(0, 1, 5),
    FixedNumber::new(0, 2, 6),
    FixedNumber::new(1, 3, 1),
    FixedNumber::new(8, 8, 1),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = LockedCandidates::new(3).run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::LockedCandidatesTriples);
  assert_eq!(step.areas, vec![ Area::Region(0), Area::Row(2) ]);
  assert_eq!(step.cells, vec![
    CellPosition::new(2, 0), CellPosition::new(2, 1), CellPosition::new(2, 2),
  ]);
  assert_eq!(step.values, vec![ 1 ]);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(2, 6), CellPosition::new(2, 7),
  ]);
  let initial_candidates = solver.candidates[2][6].clone();
  assert!(initial_candidates.contains(&1));
  assert_eq!(initial_candidates.len(), 9);

  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[2][6];
  assert!(!final_candidates.contains(&1));
  assert_eq!(final_candidates.len(), 8);
}

#[test]
fn check_locked_candidates_triples_no_affected_cells() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 4),
    FixedNumber::new(0, 1, 5),
    FixedNumber::new(0, 2, 6),
    FixedNumber::new(0, 6, 1),
    FixedNumber::new(1, 3, 1),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = LockedCandidates::new(3).run(&solver);
  assert!(steps.is_empty());
}

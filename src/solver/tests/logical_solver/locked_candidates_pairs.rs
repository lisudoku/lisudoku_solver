use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule, Area}, solver::{Solver, logical_solver::{candidates::Candidates, technique::Technique, locked_candidates::LockedCandidates}}};

#[test]
fn check_locked_candidates_pairs() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 4),
    FixedNumber::new(0, 1, 5),
    FixedNumber::new(0, 2, 6),
    FixedNumber::new(1, 3, 1),
    FixedNumber::new(3, 0, 1),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = LockedCandidates::new(2).run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::LockedCandidatesPairs);
  assert_eq!(step.areas, vec![ Area::Region(0), Area::Row(2) ]);
  assert_eq!(step.cells, vec![ CellPosition::new(2, 1), CellPosition::new(2, 2) ]);
  assert_eq!(step.values, vec![ 1 ]);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(2, 6), CellPosition::new(2, 7), CellPosition::new(2, 8),
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
fn check_locked_candidates_pairs_no_affected_cells() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 4),
    FixedNumber::new(0, 1, 5),
    FixedNumber::new(0, 2, 6),
    FixedNumber::new(0, 6, 1),
    FixedNumber::new(1, 3, 1),
    FixedNumber::new(3, 0, 1),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = LockedCandidates::new(2).run(&solver);
  assert!(steps.is_empty());
}

#[test]
fn check_locked_candidates_pairs_on_primary_diagonal() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 1, 2),
    FixedNumber::new(0, 2, 4),
    FixedNumber::new(1, 2, 5),
    FixedNumber::new(2, 1, 3),
    FixedNumber::new(3, 0, 1),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.primary_diagonal = true;
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = LockedCandidates::new(2).run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::LockedCandidatesPairs);
  assert_eq!(step.areas, vec![ Area::Region(0), Area::PrimaryDiagonal ]);
  assert_eq!(step.cells, vec![ CellPosition::new(1, 1), CellPosition::new(2, 2) ]);
  assert_eq!(step.values, vec![ 1 ]);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(4, 4), CellPosition::new(5, 5),
    CellPosition::new(6, 6), CellPosition::new(7, 7), CellPosition::new(8, 8),
  ]);
  let initial_candidates_1 = solver.candidates[4][4].clone();
  assert!(initial_candidates_1.contains(&1));
  assert_eq!(initial_candidates_1.len(), 9);
  let initial_candidates_2 = solver.candidates[8][8].clone();
  assert!(initial_candidates_2.contains(&1));
  assert_eq!(initial_candidates_2.len(), 9);

  solver.apply_rule(&mut step);
  let final_candidates_1 = &solver.candidates[4][4];
  assert!(!final_candidates_1.contains(&1));
  assert_eq!(final_candidates_1.len(), 8);
  let final_candidates_2 = &solver.candidates[8][8];
  assert!(!final_candidates_2.contains(&1));
  assert_eq!(final_candidates_2.len(), 8);
}

#[test]
fn check_locked_candidates_pairs_on_secondary_diagonal() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 6, 4),
    FixedNumber::new(0, 7, 2),
    FixedNumber::new(1, 6, 5),
    FixedNumber::new(2, 7, 3),
    FixedNumber::new(3, 8, 1),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.secondary_diagonal = true;
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = LockedCandidates::new(2).run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::LockedCandidatesPairs);
  assert_eq!(step.areas, vec![ Area::Region(2), Area::SecondaryDiagonal ]);
  assert_eq!(step.cells, vec![ CellPosition::new(1, 7), CellPosition::new(2, 6) ]);
  assert_eq!(step.values, vec![ 1 ]);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(4, 4), CellPosition::new(5, 3),
    CellPosition::new(6, 2), CellPosition::new(7, 1), CellPosition::new(8, 0),
  ]);
  let initial_candidates_1 = solver.candidates[4][4].clone();
  assert!(initial_candidates_1.contains(&1));
  assert_eq!(initial_candidates_1.len(), 9);
  let initial_candidates_2 = solver.candidates[8][0].clone();
  assert!(initial_candidates_2.contains(&1));
  assert_eq!(initial_candidates_2.len(), 9);

  solver.apply_rule(&mut step);
  let final_candidates_1 = &solver.candidates[4][4];
  assert!(!final_candidates_1.contains(&1));
  assert_eq!(final_candidates_1.len(), 8);
  let final_candidates_2 = &solver.candidates[8][0];
  assert!(!final_candidates_2.contains(&1));
  assert_eq!(final_candidates_2.len(), 8);
}

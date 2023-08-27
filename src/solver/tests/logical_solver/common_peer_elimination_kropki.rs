use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule, Area, KropkiDot}, solver::{Solver, logical_solver::{kropki_chain_candidates::KropkiChainCandidates, technique::Technique, candidates::Candidates, common_peer_elimination_kropki::CommonPeerEliminationKropki}}};

#[test]
fn check_kropki_common_peer_elimination_1() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 2, 5),
    FixedNumber::new(2, 2, 6),
    FixedNumber::new(1, 2, 7),
    FixedNumber::new(1, 3, 2),
    FixedNumber::new(1, 4, 3),
    FixedNumber::new(0, 1, 4),
    FixedNumber::new(1, 6, 8),
    FixedNumber::new(6, 1, 7),
    FixedNumber::new(7, 2, 9),
    FixedNumber::new(8, 3, 4),
    FixedNumber::new(8, 4, 5),
    FixedNumber::new(8, 6, 6),
    FixedNumber::new(8, 7, 8),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(1, 0), CellPosition::new(2, 0)),
    KropkiDot::consecutive(CellPosition::new(2, 0), CellPosition::new(3, 0)),
  ];
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = KropkiChainCandidates::new(false).run(&solver);
  for mut step in steps {
    solver.apply_rule(&mut step);
  }

  let steps = CommonPeerEliminationKropki.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::CommonPeerEliminationKropki);
  assert_eq!(step.values, vec![ 1, 2, 3 ]);
  assert_eq!(step.areas, vec![ Area::Column(0), Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(1, 0), CellPosition::new(2, 0), CellPosition::new(3, 0),
  ]);
  assert_eq!(step.cells, vec![ CellPosition::new(8, 0) ]);
  assert!(solver.candidates[3][0].contains(&3));

  solver.apply_rule(&mut step);
  assert!(!solver.candidates[3][0].contains(&3));
}

#[test]
fn check_kropki_common_peer_elimination_2() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 2, 5),
    FixedNumber::new(2, 2, 6),
    FixedNumber::new(1, 2, 7),
    FixedNumber::new(1, 3, 2),
    FixedNumber::new(1, 4, 3),
    FixedNumber::new(0, 1, 4),
    FixedNumber::new(1, 6, 8),
    FixedNumber::new(6, 1, 3),
    FixedNumber::new(7, 2, 1),
    FixedNumber::new(8, 3, 4),
    FixedNumber::new(8, 4, 5),
    FixedNumber::new(8, 6, 6),
    FixedNumber::new(8, 7, 2),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(1, 0), CellPosition::new(2, 0)),
    KropkiDot::consecutive(CellPosition::new(2, 0), CellPosition::new(3, 0)),
  ];
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = KropkiChainCandidates::new(false).run(&solver);
  for mut step in steps {
    solver.apply_rule(&mut step);
  }

  let steps = CommonPeerEliminationKropki.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::CommonPeerEliminationKropki);
  assert_eq!(step.values, vec![ 9, 8, 7 ]);
  assert_eq!(step.areas, vec![ Area::Column(0), Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(1, 0), CellPosition::new(2, 0), CellPosition::new(3, 0),
  ]);
  assert_eq!(step.cells, vec![ CellPosition::new(8, 0) ]);
  assert!(solver.candidates[2][0].contains(&8));

  solver.apply_rule(&mut step);
  assert!(!solver.candidates[2][0].contains(&8));
}

#[test]
fn check_kropki_common_peer_elimination_3() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(1, 5, 3),
    FixedNumber::new(1, 6, 4),
    FixedNumber::new(1, 7, 5),
    FixedNumber::new(1, 8, 6),
    FixedNumber::new(5, 0, 9),
    FixedNumber::new(6, 0, 8),
    FixedNumber::new(7, 0, 7),
    FixedNumber::new(8, 5, 1),
    FixedNumber::new(8, 6, 5),
    FixedNumber::new(8, 7, 6),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(1, 0), CellPosition::new(2, 0)),
    KropkiDot::consecutive(CellPosition::new(2, 0), CellPosition::new(3, 0)),
  ];
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = KropkiChainCandidates::new(false).run(&solver);
  for mut step in steps {
    solver.apply_rule(&mut step);
  }

  let steps = CommonPeerEliminationKropki.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::CommonPeerEliminationKropki);
  assert_eq!(step.values, vec![ 2, 3, 4 ]);
  assert_eq!(step.areas, vec![ Area::Column(0), Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(1, 0), CellPosition::new(2, 0), CellPosition::new(3, 0),
  ]);
  assert_eq!(step.cells, vec![ CellPosition::new(8, 0) ]);
  assert!(solver.candidates[2][0].contains(&3));

  solver.apply_rule(&mut step);
  assert!(!solver.candidates[2][0].contains(&3));
}

#[test]
fn check_kropki_common_peer_elimination_4() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(1, 3, 2),
    FixedNumber::new(1, 4, 3),
    FixedNumber::new(1, 5, 4),
    FixedNumber::new(1, 6, 6),
    FixedNumber::new(1, 7, 7),
    FixedNumber::new(6, 0, 8),
    FixedNumber::new(7, 0, 9),
    FixedNumber::new(8, 5, 6),
    FixedNumber::new(8, 6, 7),
    FixedNumber::new(8, 7, 2),
    FixedNumber::new(8, 8, 1),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(1, 0), CellPosition::new(2, 0)),
    KropkiDot::consecutive(CellPosition::new(2, 0), CellPosition::new(3, 0)),
  ];
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = KropkiChainCandidates::new(false).run(&solver);
  for mut step in steps {
    solver.apply_rule(&mut step);
  }

  let steps = CommonPeerEliminationKropki.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::CommonPeerEliminationKropki);
  assert_eq!(step.values, vec![ 4 ]);
  assert_eq!(step.areas, vec![ Area::Column(0), Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(2, 0) ]);
  assert_eq!(step.cells, vec![ CellPosition::new(8, 0) ]);
  assert!(solver.candidates[2][0].contains(&4));

  solver.apply_rule(&mut step);
  assert!(!solver.candidates[2][0].contains(&4));
}

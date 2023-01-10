use crate::{types::{SudokuConstraints, CellPosition, Rule, KillerCage, FixedNumber, Area}, solver::Solver};
use itertools::Itertools;

#[test]
fn check_killer_candidates_single_unfixed() {
  let grid_size = 9;
  let mut constraints = SudokuConstraints::new(grid_size, vec![]);
  constraints.killer_cages = vec![
    KillerCage {
      sum: Some(7),
      region: vec![ CellPosition::new(8, 8) ],
    },
  ];
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut solver.find_candidates_step().unwrap());
  let steps = solver.find_killer_candidate_updates();
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();

  assert_eq!(step.rule, Rule::KillerCandidates);
  assert_eq!(step.affected_cells[0], CellPosition::new(8, 8));
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![ 1, 2, 3, 4, 5, 6 ]);
  assert_eq!(step.areas, vec![ Area::KillerCage(0) ]);
  assert!(solver.candidates[8][8].contains(&1));
  solver.apply_rule(&step);
  let final_candidates = &solver.candidates[8][8];
  assert_eq!(final_candidates.len(), 1);
  assert!(!final_candidates.contains(&4));
}

#[test]
fn check_killer_candidates_single_fixed() {
  let grid_size = 9;
  let fixed_numbers = vec![ FixedNumber::new(0, 0, 5) ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.killer_cages = vec![
    KillerCage {
      sum: Some(5),
      region: vec![ CellPosition::new(0, 0) ],
    },
  ];
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut solver.find_candidates_step().unwrap());
  let steps = solver.find_killer_candidate_updates();
  assert!(steps.is_empty());
}

#[test]
fn check_killer_candidates_pair_1() {
  let grid_size = 9;
  let mut constraints = SudokuConstraints::new(grid_size, vec![]);
  constraints.killer_cages = vec![
    KillerCage {
      sum: Some(4),
      region: vec![ CellPosition::new(0, 0), CellPosition::new(0, 1) ],
    },
  ];
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut solver.find_candidates_step().unwrap());
  let steps = solver.find_killer_candidate_updates();
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();

  assert_eq!(step.rule, Rule::KillerCandidates);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 0) ]);
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![ 2, 4 ]);
  assert_eq!(step.areas, vec![ Area::KillerCage(0) ]);
  assert!(solver.candidates[0][0].contains(&4));
  solver.apply_rule(&step);
  let final_candidates = &solver.candidates[0][0];
  assert_eq!(final_candidates.len(), 2);
  assert!(!final_candidates.contains(&4));

  let steps = solver.find_killer_candidate_updates();
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();
  assert_eq!(step.rule, Rule::KillerCandidates);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 1) ]);
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![ 2, 4 ]);
}

#[test]
fn check_killer_candidates_pair_2() {
  let grid_size = 9;
  let fixed_numbers = vec![ FixedNumber::new(0, 0, 1) ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.killer_cages = vec![
    KillerCage {
      sum: Some(4),
      region: vec![ CellPosition::new(8, 0), CellPosition::new(8, 1) ],
    },
  ];
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut solver.find_candidates_step().unwrap());
  let steps = solver.find_killer_candidate_updates();
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();

  assert_eq!(step.rule, Rule::KillerCandidates);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(8, 0) ]);
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![ 2, 4 ]);
  assert_eq!(step.areas, vec![ Area::KillerCage(0) ]);
  assert!(solver.candidates[8][0].contains(&2));
  solver.apply_rule(&step);
  let final_candidates = &solver.candidates[8][0];
  assert_eq!(final_candidates.len(), 1);
  assert!(!final_candidates.contains(&2));

  let steps = solver.find_killer_candidate_updates();
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();
  assert_eq!(step.rule, Rule::KillerCandidates);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(8, 1) ]);
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![ 2, 3, 4 ]);
  assert_eq!(step.areas, vec![ Area::KillerCage(0) ]);
}

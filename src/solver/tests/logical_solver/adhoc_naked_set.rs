use std::{collections::HashSet, vec};
use ntest::timeout;
use crate::{solver::{logical_solver::{adhoc_naked_set::AdhocNakedSet, candidates::Candidates, technique::Technique, thermo_candidates::ThermoCandidates}, Solver}, types::{Area, CellPosition, FixedNumber, Rule, SudokuConstraints, SudokuGrid}};

#[test]
fn check_adhoc_naked_sets_antiknight_4x4() {
  let fixed_numbers: Vec<FixedNumber> = vec![
    FixedNumber::new(2, 0, 1), FixedNumber::new(2, 1, 2), FixedNumber::new(3, 2, 2),
  ];
  let constraints = SudokuConstraints::new(4, fixed_numbers).with_anti_knight();
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = AdhocNakedSet.run(&solver);
  assert_eq!(steps.len(), 1);
  
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::AdhocNakedSet);
  assert_eq!(step.areas, vec![Area::Adhoc(vec![
    CellPosition::new(0, 0), CellPosition::new(0, 2), CellPosition::new(0, 3),
    CellPosition::new(1, 2),
  ])]);
  assert_eq!(step.cells, vec![ CellPosition::new(0, 0), CellPosition::new(1, 2) ]);
  assert_eq!(step.values, vec![ 3, 4 ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 2), CellPosition::new(0, 3) ]);

  let initial_candidates = solver.candidates[0][3].clone();
  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[0][3];
  assert_eq!(final_candidates.len(), initial_candidates.len() - 2);
}

#[test]
fn check_adhoc_naked_sets_diagonal_full_4x4() {
  let fixed_numbers = SudokuGrid::from_string(String::from("\
    0312\
    2034\
    1203\
    3420\
  ")).to_fixed_numbers();
  let constraints = SudokuConstraints::new(4, fixed_numbers).with_diagonals();
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = AdhocNakedSet.run(&solver);
  assert!(steps.is_empty());
}

#[test]
fn check_adhoc_naked_sets_thermo_4x4_1() {
  let fixed_numbers: Vec<FixedNumber> = vec![
    FixedNumber::new(0, 1, 2), FixedNumber::new(1, 1, 1), FixedNumber::new(1, 3, 2),
  ];
  let mut constraints = SudokuConstraints::new(4, fixed_numbers);
  constraints.thermos = vec![
    vec![
      CellPosition { row: 2, col: 2 },
      CellPosition { row: 2, col: 1 },
      CellPosition { row: 1, col: 2 },
    ],
  ];
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = AdhocNakedSet.run(&solver);
  assert_eq!(steps.len(), 1);
  
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::AdhocNakedSet);
  assert_eq!(step.areas, vec![Area::Adhoc(vec![
    CellPosition::new(1, 2), CellPosition::new(2, 1), CellPosition::new(2, 2),
  ])]);
  assert_eq!(step.cells, vec![ CellPosition::new(1, 2), CellPosition::new(2, 1) ]);
  assert_eq!(step.values, vec![ 3, 4 ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(2, 2) ]);

  let initial_candidates = solver.candidates[2][2].clone();
  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[2][2];
  assert_eq!(final_candidates.len(), initial_candidates.len() - 2);
}

#[test]
fn check_adhoc_naked_sets_diagonal_6x6_1() {
  let fixed_numbers: Vec<FixedNumber> = vec![
    FixedNumber::new(5, 0, 1), FixedNumber::new(4, 1, 2), FixedNumber::new(3, 2, 3),
    FixedNumber::new(5, 5, 3), FixedNumber::new(3, 5, 1), FixedNumber::new(3, 3, 2),
    FixedNumber::new(0, 3, 1), FixedNumber::new(1, 3, 3), FixedNumber::new(1, 4, 4),
  ];
  let constraints = SudokuConstraints::new(6, fixed_numbers).with_diagonals();
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = AdhocNakedSet.run(&solver);
  assert_eq!(steps.len(), 1);
  
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::AdhocNakedSet);
  assert_eq!(step.areas, vec![Area::Adhoc(vec![
    CellPosition::new(0, 5), CellPosition::new(2, 3), CellPosition::new(2, 5),
  ])]);
  assert_eq!(step.cells, vec![ CellPosition::new(0, 5), CellPosition::new(2, 3) ]);
  assert_eq!(step.values, vec![ 5, 6 ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(2, 5) ]);

  let initial_candidates = solver.candidates[2][5].clone();
  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[2][5];
  assert_eq!(final_candidates.len(), initial_candidates.len() - 2);
}

#[test]
fn check_adhoc_naked_sets_thermo_6x6_peers() {
  let fixed_numbers: Vec<FixedNumber> = vec![
    FixedNumber::new(1, 0, 3), FixedNumber::new(1, 4, 2), FixedNumber::new(1, 5, 1),
  ];
  let mut constraints = SudokuConstraints::new(6, fixed_numbers);
  constraints.thermos = vec![
    vec![
      CellPosition { row: 2, col: 3 },
      CellPosition { row: 1, col: 2 },
      CellPosition { row: 0, col: 3 },
    ],
  ];
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  solver.apply_rules(&mut ThermoCandidates.run(&solver));

  let steps = AdhocNakedSet.run(&solver);
  assert_eq!(steps.len(), 1);
  
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::AdhocNakedSet);
  assert_eq!(step.areas, vec![Area::Adhoc(vec![
    CellPosition::new(0, 3), CellPosition::new(1, 2), CellPosition::new(1, 3),
    CellPosition::new(2, 3),
  ])]);
  assert_eq!(step.cells, vec![
    CellPosition::new(0, 3), CellPosition::new(1, 2), CellPosition::new(1, 3),
  ]);
  assert_eq!(step.values, vec![ 4, 5, 6 ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(2, 3) ]);

  let initial_candidates = solver.candidates[2][3].clone();
  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[2][3];
  assert_eq!(final_candidates.len(), initial_candidates.len() - 1);
}

#[test]
fn check_adhoc_naked_sets_antiknight_9x9() {
  let fixed_numbers: Vec<FixedNumber> = vec![
    FixedNumber::new(7, 0, 9), FixedNumber::new(7, 1, 8), FixedNumber::new(7, 2, 7),
    FixedNumber::new(7, 3, 6), FixedNumber::new(7, 7, 5), FixedNumber::new(7, 8, 4),
    FixedNumber::new(6, 7, 3), FixedNumber::new(5, 5, 6), FixedNumber::new(5, 6, 7),
  ];
  let constraints = SudokuConstraints::new(9, fixed_numbers).with_anti_knight();
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = AdhocNakedSet.run(&solver);
  assert_eq!(steps.len(), 1);

  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::AdhocNakedSet);
  assert_eq!(step.areas, vec![Area::Adhoc(vec![
    CellPosition::new(6, 4), CellPosition::new(7, 4), CellPosition::new(7, 5),
    CellPosition::new(7, 6), CellPosition::new(8, 4),
  ])]);
  assert_eq!(step.cells, vec![ CellPosition::new(7, 5), CellPosition::new(7, 6) ]);
  assert_eq!(step.values, vec![ 1, 2 ]);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(6, 4), CellPosition::new(7, 4), CellPosition::new(8, 4),
  ]);

  let initial_candidates = solver.candidates[8][4].clone();
  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[8][4];
  assert_eq!(final_candidates.len(), initial_candidates.len() - 2);
}

#[test]
#[timeout(1000)]
fn check_adhoc_naked_sets_empty_antiknight_9x9() {
  let constraints = SudokuConstraints::new(9, vec![]).with_anti_knight();
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = AdhocNakedSet.run(&solver);
  assert!(steps.is_empty());
}

// https://logicmastersindia.com/live/?contest=M202501S puzzle 7
// https://lisudoku.xyz/p/X2vLSy6lGcGaNTVlQ9iY
#[test]
fn check_adhoc_naked_sets_contest_puzzle_antiknight_9x9() {
  let fixed_numbers = SudokuGrid::from_string(String::from("\
    005024010\
    024016005\
    000935624\
    000693502\
    096582070\
    502471900\
    053240000\
    040368250\
    200150000\
  ")).to_fixed_numbers();
  let constraints = SudokuConstraints::new(9, fixed_numbers).with_anti_knight();
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  // Exclude Common Peer Eliminations
  solver.candidates[0][0].remove(&7);
  solver.candidates[0][0].remove(&8);
  solver.candidates[0][1].remove(&7);
  solver.candidates[0][1].remove(&8);

  let steps = AdhocNakedSet.run(&solver);
  assert_eq!(steps.len(), 1);

  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::AdhocNakedSet);
  assert_eq!(step.areas, vec![Area::Adhoc(vec![
    CellPosition::new(2, 0), CellPosition::new(2, 2), CellPosition::new(3, 0),
    CellPosition::new(3, 2),
  ])]);
  assert_eq!(step.cells, vec![
    CellPosition::new(2, 0), CellPosition::new(2, 2), CellPosition::new(3, 2),
  ]);
  assert_eq!(step.values, vec![ 1, 7, 8 ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(3, 0) ]);

  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[3][0];
  assert_eq!(final_candidates, &HashSet::from([4]));
}

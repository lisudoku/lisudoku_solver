use crate::{types::{SudokuConstraints, CellPosition, Rule, KillerCage, Area, FixedNumber}, solver::{Solver, logical_solver::{candidates::Candidates, technique::Technique, killer45::Killer45}}};
use itertools::Itertools;

#[test]
fn check_killer45_row() {
  let grid_size = 6;
  let mut constraints = SudokuConstraints::new(grid_size, vec![]);
  constraints.killer_cages = vec![
    KillerCage {
      sum: Some(4),
      region: vec![ CellPosition::new(0, 0), CellPosition::new(0, 1) ],
    },
    KillerCage {
      sum: Some(15),
      region: vec![ CellPosition::new(0, 2), CellPosition::new(0, 3), CellPosition::new(0, 4) ],
    },
  ];
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  let steps = Killer45.run(&solver);
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();

  assert_eq!(step.rule, Rule::Killer45);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 5) ]);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 1, 3, 4, 5, 6 ]);
  assert_eq!(step.areas, vec![ Area::Row(0) ]);
  assert!(solver.candidates[0][5].contains(&1));
  solver.apply_rule(&step);
  let final_candidates = &solver.candidates[0][5];
  assert_eq!(final_candidates.len(), 1);
  assert!(!final_candidates.contains(&1));
}

#[test]
fn check_killer45_region() {
  let grid_size = 9;
  let mut constraints = SudokuConstraints::new(grid_size, vec![]);
  constraints.killer_cages = vec![
    KillerCage {
      sum: Some(30),
      region: vec![
        CellPosition::new(3, 3), CellPosition::new(3, 4), CellPosition::new(3, 5),
        CellPosition::new(4, 3), CellPosition::new(5, 3),
      ],
    },
    KillerCage {
      sum: Some(11),
      region: vec![ CellPosition::new(4, 5), CellPosition::new(5, 5) ],
    },
  ];
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  let steps = Killer45.run(&solver);
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();

  assert_eq!(step.rule, Rule::Killer45);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(4, 4) ]);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 2, 4, 5, 6, 7, 8, 9 ]);
  assert_eq!(step.areas, vec![ Area::Region(4) ]);
  assert!(solver.candidates[4][4].contains(&2));
  solver.apply_rule(&step);
  let final_candidates = &solver.candidates[4][4];
  assert_eq!(final_candidates.len(), 2);
  assert!(!final_candidates.contains(&2));
}

#[test]
fn check_killer45_col_with_fixed_digits() {
  let grid_size = 6;
  let fixed_numbers = vec![ FixedNumber::new(0, 1, 3), FixedNumber::new(5, 1, 2) ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.killer_cages = vec![
    KillerCage {
      sum: Some(12),
      region: vec![ CellPosition::new(1, 1), CellPosition::new(2, 1), CellPosition::new(3, 1) ],
    },
  ];
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  let steps = Killer45.run(&solver);
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();

  assert_eq!(step.rule, Rule::Killer45);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(4, 1) ]);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 1, 5, 6 ]);
  assert_eq!(step.areas, vec![ Area::Column(1) ]);
  assert!(solver.candidates[4][1].contains(&1));
  solver.apply_rule(&step);
  let final_candidates = &solver.candidates[4][1];
  assert_eq!(final_candidates.len(), 1);
  assert!(!final_candidates.contains(&1));
}

#[test]
fn check_killer45_col_partial_cages() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 4, 6), FixedNumber::new(0, 5, 5),
    FixedNumber::new(1, 2, 6), FixedNumber::new(2, 2, 5), FixedNumber::new(3, 4, 2),
    FixedNumber::new(4, 4, 1),
    FixedNumber::new(4, 2, 2),
    FixedNumber::new(6, 2, 1), FixedNumber::new(7, 2, 3), FixedNumber::new(6, 3, 7),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.killer_cages = vec![
    KillerCage {
      sum: Some(21),
      region: vec![
        CellPosition::new(1, 2), CellPosition::new(2, 2), CellPosition::new(2, 3),
        CellPosition::new(3, 3), CellPosition::new(3, 4), CellPosition::new(4, 4),
      ],
    },
    KillerCage {
      sum: Some(13),
      region: vec![
        CellPosition::new(4, 2), CellPosition::new(4, 3), CellPosition::new(5, 3),
      ],
    },
    KillerCage {
      sum: Some(12),
      region: vec![ CellPosition::new(0, 3), CellPosition::new(0, 4), CellPosition::new(0, 5) ],
    },
    KillerCage {
      sum: Some(28),
      region: vec![
        CellPosition::new(6, 3), CellPosition::new(6, 2), CellPosition::new(7, 2),
        CellPosition::new(7, 3), CellPosition::new(8, 3),
      ],
    },
  ];
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  let steps = Killer45.run(&solver);
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();

  assert_eq!(step.rule, Rule::Killer45);
  assert_eq!(step.areas, vec![ Area::Column(3) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(1, 3) ]);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 1, 3, 4, 8, 9 ]);
  assert!(solver.candidates[1][3].contains(&1));
  solver.apply_rule(&step);
  let final_candidates = &solver.candidates[1][3];
  assert_eq!(final_candidates.len(), 1);
  assert!(!final_candidates.contains(&1));
}

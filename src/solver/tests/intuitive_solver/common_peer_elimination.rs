use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule, Area}, solver::Solver};

#[test]
fn check_anti_knight_common_peer_elimination_1() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(3, 3, 1),
    FixedNumber::new(3, 5, 2),
    FixedNumber::new(5, 5, 3),
    FixedNumber::new(6, 1, 5),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.anti_knight = true;
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut solver.find_candidates_step().unwrap());

  let step = solver.find_common_peer_elimination();
  assert!(step.is_some());
  let mut step = step.unwrap();
  assert_eq!(step.rule, Rule::CommonPeerElimination);
  assert_eq!(step.areas, vec![ Area::Region(4) ]);
  assert_eq!(step.cells, vec![
    CellPosition::new(3, 4), CellPosition::new(4, 3), CellPosition::new(4, 4),
    CellPosition::new(4, 5), CellPosition::new(5, 4),
  ]);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(2, 4), CellPosition::new(4, 6),
  ]);
  assert_eq!(step.values, vec![ 5 ]);
  assert!(solver.candidates[4][6].contains(&5));

  solver.apply_rule(&mut step);
  assert!(!solver.candidates[4][6].contains(&5));
}

#[test]
fn check_anti_knight_common_peer_elimination_2() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(3, 3, 1),
    FixedNumber::new(3, 4, 2),
    FixedNumber::new(3, 5, 3),
    FixedNumber::new(4, 3, 4),
    FixedNumber::new(4, 4, 5),
    FixedNumber::new(5, 3, 6),
    FixedNumber::new(5, 5, 7),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.anti_knight = true;
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut solver.find_candidates_step().unwrap());

  let step = solver.find_common_peer_elimination();
  assert!(step.is_some());
  let mut step = step.unwrap();
  assert_eq!(step.rule, Rule::CommonPeerElimination);
  assert_eq!(step.values, vec![8]);
  assert_eq!(step.areas, vec![ Area::Region(4) ]);
  assert_eq!(step.cells, vec![
    CellPosition::new(4, 5), CellPosition::new(5, 4),
  ]);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(2, 4), CellPosition::new(4, 2), CellPosition::new(4, 6),
    CellPosition::new(5, 7), CellPosition::new(6, 4), CellPosition::new(6, 6),
    CellPosition::new(7, 5),
  ]);
  assert!(solver.candidates[6][6].contains(&8));

  solver.apply_rule(&mut step);
  assert!(!solver.candidates[6][6].contains(&8));
}

#[test]
fn check_common_peer_elimination_overlapping_thermos_1() {
  let grid_size = 6;
  let fixed_numbers = vec![
    FixedNumber::new(5, 0, 1),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    vec![
      CellPosition::new(3, 2),
      CellPosition::new(4, 3),
      CellPosition::new(4, 4),
      CellPosition::new(4, 5),
    ],
    vec![
      CellPosition::new(4, 5),
      CellPosition::new(3, 5),
    ],
  ];
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut solver.find_candidates_step().unwrap());

  let step = solver.find_common_peer_elimination();
  assert!(step.is_some());
  let mut step = step.unwrap();
  assert_eq!(step.rule, Rule::CommonPeerElimination);
  assert_eq!(step.values, vec![1]);
  assert_eq!(step.areas, vec![ Area::Row(4) ]);
  assert_eq!(step.cells, vec![
    CellPosition::new(4, 3), CellPosition::new(4, 4), CellPosition::new(4, 5),
  ]);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(3, 2),
  ]);
  assert!(solver.candidates[3][2].contains(&1));

  solver.apply_rule(&mut step);
  assert!(!solver.candidates[3][2].contains(&1));
}

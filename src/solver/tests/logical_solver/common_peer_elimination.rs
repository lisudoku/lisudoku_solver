use crate::{solver::{Solver, logical_solver::{candidates::Candidates, common_peer_elimination::CommonPeerElimination, technique::Technique}}, types::{Area, CellPosition, FixedNumber, Rule, SudokuConstraints, Thermo}};

#[test]
fn check_anti_knight_common_peer_elimination_1() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(3, 3, 1),
        FixedNumber::new(3, 5, 2),
        FixedNumber::new(5, 5, 3),
        FixedNumber::new(6, 1, 5),
      ]
    )
    .with_anti_knight();
  let mut solver = Solver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = CommonPeerElimination.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
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
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(3, 3, 1),
        FixedNumber::new(3, 4, 2),
        FixedNumber::new(3, 5, 3),
        FixedNumber::new(4, 3, 4),
        FixedNumber::new(4, 4, 5),
        FixedNumber::new(5, 3, 6),
        FixedNumber::new(5, 5, 7),
      ]
    )
    .with_anti_knight();
  let mut solver = Solver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = CommonPeerElimination.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
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
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![FixedNumber::new(5, 0, 1)]
    )
    .with_thermos(
      vec![
        Thermo(vec![
          CellPosition::new(3, 2),
          CellPosition::new(4, 3),
          CellPosition::new(4, 4),
          CellPosition::new(4, 5),
        ]),
        Thermo(vec![
          CellPosition::new(4, 5),
          CellPosition::new(3, 5),
        ]),
      ]
    );
  let mut solver = Solver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = CommonPeerElimination.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
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

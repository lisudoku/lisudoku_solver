use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule, Area, KropkiDot}, solver::{Solver, logical_solver::{kropki_chain_candidates::KropkiChainCandidates, technique::Technique, candidates::Candidates, kropki_advanced_candidates::KropkiAdvancedCandidates}}};

#[test]
fn check_kropki_advanced_candidates_1() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
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
      ]
    )
    .with_kropki_dots(
      vec![
        KropkiDot::consecutive(CellPosition::new(1, 0), CellPosition::new(2, 0)),
        KropkiDot::consecutive(CellPosition::new(2, 0), CellPosition::new(3, 0)),
      ]
    );
  let mut solver = Solver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = KropkiChainCandidates::new(false).run(&solver);
  solver.apply_rules(&steps);

  let steps = KropkiAdvancedCandidates.run(&solver);
  assert_eq!(steps.len(), 3);

  let mut step = &steps[0];
  assert_eq!(step.rule, Rule::KropkiAdvancedCandidates);
  assert_eq!(step.values, vec![ 1 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(1, 0) ]);
  assert!(solver.candidates[1][0].contains(&1));
  solver.apply_rule(&mut step);
  assert!(!solver.candidates[1][0].contains(&1));

  let mut step = &steps[1];
  assert_eq!(step.rule, Rule::KropkiAdvancedCandidates);
  assert_eq!(step.values, vec![ 2 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(2, 0) ]);
  assert!(solver.candidates[2][0].contains(&2));
  solver.apply_rule(&mut step);
  assert!(!solver.candidates[2][0].contains(&2));

  let mut step = &steps[2];
  assert_eq!(step.rule, Rule::KropkiAdvancedCandidates);
  assert_eq!(step.values, vec![ 3 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(3, 0) ]);
  assert!(solver.candidates[3][0].contains(&3));
  solver.apply_rule(&mut step);
  assert!(!solver.candidates[3][0].contains(&3));
}

#[test]
fn check_kropki_advanced_candidates_2() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
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
      ]
    )
    .with_kropki_dots(
      vec![
        KropkiDot::consecutive(CellPosition::new(1, 0), CellPosition::new(2, 0)),
        KropkiDot::consecutive(CellPosition::new(2, 0), CellPosition::new(3, 0)),
      ]
    );
  let mut solver = Solver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = KropkiChainCandidates::new(false).run(&solver);
  solver.apply_rules(&steps);

  let steps = KropkiAdvancedCandidates.run(&solver);
  assert_eq!(steps.len(), 3);

  let mut step = &steps[0];
  assert_eq!(step.rule, Rule::KropkiAdvancedCandidates);
  assert_eq!(step.values, vec![ 9 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(1, 0) ]);
  assert!(solver.candidates[1][0].contains(&9));
  solver.apply_rule(&mut step);
  assert!(!solver.candidates[1][0].contains(&9));

  let mut step = &steps[1];
  assert_eq!(step.rule, Rule::KropkiAdvancedCandidates);
  assert_eq!(step.values, vec![ 8 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(2, 0) ]);
  assert!(solver.candidates[2][0].contains(&8));
  solver.apply_rule(&mut step);
  assert!(!solver.candidates[2][0].contains(&8));

  let mut step = &steps[2];
  assert_eq!(step.rule, Rule::KropkiAdvancedCandidates);
  assert_eq!(step.values, vec![ 7 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(3, 0) ]);
  assert!(solver.candidates[3][0].contains(&7));
  solver.apply_rule(&mut step);
  assert!(!solver.candidates[3][0].contains(&7));
}

#[test]
fn check_kropki_advanced_candidates_3() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
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
      ]
    )
    .with_kropki_dots(
      vec![
        KropkiDot::consecutive(CellPosition::new(1, 0), CellPosition::new(2, 0)),
        KropkiDot::consecutive(CellPosition::new(2, 0), CellPosition::new(3, 0)),
      ]
    );
  let mut solver = Solver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = KropkiChainCandidates::new(false).run(&solver);
  solver.apply_rules(&steps);

  let steps = KropkiAdvancedCandidates.run(&solver);
  assert_eq!(steps.len(), 3);

  let mut step = &steps[0];
  assert_eq!(step.rule, Rule::KropkiAdvancedCandidates);
  assert_eq!(step.values, vec![ 2 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(1, 0) ]);
  assert!(solver.candidates[1][0].contains(&2));
  solver.apply_rule(&mut step);
  assert!(!solver.candidates[1][0].contains(&2));

  let mut step = &steps[1];
  assert_eq!(step.rule, Rule::KropkiAdvancedCandidates);
  assert_eq!(step.values, vec![ 3 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(2, 0) ]);
  assert!(solver.candidates[2][0].contains(&3));
  solver.apply_rule(&mut step);
  assert!(!solver.candidates[2][0].contains(&3));

  let mut step = &steps[2];
  assert_eq!(step.rule, Rule::KropkiAdvancedCandidates);
  assert_eq!(step.values, vec![ 4 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(3, 0) ]);
  assert!(solver.candidates[3][0].contains(&4));
  solver.apply_rule(&mut step);
  assert!(!solver.candidates[3][0].contains(&4));
}

#[test]
fn check_kropki_advanced_candidates_4() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
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
      ]
    )
    .with_kropki_dots(
      vec![
        KropkiDot::consecutive(CellPosition::new(1, 0), CellPosition::new(2, 0)),
        KropkiDot::consecutive(CellPosition::new(2, 0), CellPosition::new(3, 0)),
      ]
    );
  let mut solver = Solver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = KropkiChainCandidates::new(false).run(&solver);
  solver.apply_rules(&steps);

  let steps = KropkiAdvancedCandidates.run(&solver);
  assert_eq!(steps.len(), 1);
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::KropkiAdvancedCandidates);
  assert_eq!(step.values, vec![ 4 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(2, 0) ]);
  assert!(solver.candidates[2][0].contains(&4));
  solver.apply_rule(&mut step);
  assert!(!solver.candidates[2][0].contains(&4));
}

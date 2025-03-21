use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule, Area, KropkiDot}, solver::{Solver, logical_solver::{kropki_chain_candidates::KropkiChainCandidates, technique::Technique, candidates::Candidates, common_peer_elimination_kropki::CommonPeerEliminationKropki}}};
use itertools::Itertools;

#[test]
fn check_common_peer_elimination_kropki_1() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(4, 2, 7), FixedNumber::new(5, 2, 5),
    FixedNumber::new(5, 4, 2), FixedNumber::new(5, 5, 3), FixedNumber::new(4, 6, 2),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(3, 0), CellPosition::new(3, 1)),
    KropkiDot::consecutive(CellPosition::new(3, 0), CellPosition::new(4, 0)),
  ];
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = KropkiChainCandidates::new(false).run(&solver);
  solver.apply_rules(&steps);

  let steps = CommonPeerEliminationKropki.run(&solver);
  assert_eq!(steps.len(), 2);

  let mut step = &steps[0];
  assert_eq!(step.rule, Rule::CommonPeerEliminationKropki);
  assert_eq!(step.values, vec![ 2 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(3, 2) ]);
  assert!(solver.candidates[3][2].contains(&2));
  solver.apply_rule(&mut step);
  assert!(!solver.candidates[3][2].contains(&2));

  let mut step = &steps[1];
  assert_eq!(step.rule, Rule::CommonPeerEliminationKropki);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 2, 3, 3 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells.iter().copied().collect_vec(), vec![
    CellPosition::new(3, 2), CellPosition::new(3, 2), CellPosition::new(4, 1),
  ]);
  assert!(solver.candidates[4][1].contains(&3));
  solver.apply_rule(&mut step);
  assert!(!solver.candidates[4][1].contains(&3));
}

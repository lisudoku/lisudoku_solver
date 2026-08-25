use crate::{solver::{Solver, logical_solver::{candidates::Candidates, common_peer_elimination_kropki::CommonPeerEliminationKropki, kropki_chain_candidates::KropkiChainCandidates, technique::Technique}}, types::{Area, CellPosition, FixedNumber, KropkiDot, Rule, SolutionStep, SudokuConstraints}};

#[test]
fn check_common_peer_elimination_kropki_1() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(4, 2, 7), FixedNumber::new(5, 2, 5),
        FixedNumber::new(5, 4, 2), FixedNumber::new(5, 5, 3), FixedNumber::new(4, 6, 2),
      ]
    )
    .with_kropki_dots(
      vec![
        KropkiDot::consecutive(CellPosition::new(3, 0), CellPosition::new(3, 1)),
        KropkiDot::consecutive(CellPosition::new(3, 0), CellPosition::new(4, 0)),
      ]
    );
  let mut solver = Solver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = KropkiChainCandidates::new(false).run(&solver);
  solver.apply_rules(&steps);

  let steps = CommonPeerEliminationKropki.run(&solver);
  assert_eq!(steps.len(), 2);

  let mut step = &steps[0];
  assert_eq!(
    *step,
    SolutionStep::new(
      Rule::CommonPeerEliminationKropki,
      vec![],
      vec![2, 3, 3],
      vec![Area::KropkiDot(0), Area::KropkiDot(1)],
      vec![CellPosition::new(3, 2), CellPosition::new(3, 2), CellPosition::new(4, 1)],
    )
  );
  assert!(solver.candidates[4][1].contains(&3));

  solver.apply_rule(&mut step);
  assert!(!solver.candidates[4][1].contains(&3));

  let step = &steps[1];
  assert_eq!(
    *step,
    SolutionStep::new(
      Rule::CommonPeerEliminationKropki,
      vec![],
      vec![2],
      vec![Area::KropkiDot(0)],
      vec![CellPosition::new(3, 2)],
    ),
  );
}

use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule, Area, Arrow}, solver::{Solver, logical_solver::{technique::Technique, candidates::Candidates, arrow_candidates::ArrowCandidates, common_peer_elimination_arrow::CommonPeerEliminationArrow}}};

#[test]
fn check_arrow_common_peer_elimination() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 5, 1), FixedNumber::new(1, 5, 9),
        FixedNumber::new(5, 1, 1), FixedNumber::new(6, 0, 1),
      ]
    )
    .with_arrows(
      vec![
        Arrow {
          circle_cells: vec![ CellPosition::new(1, 7) ],
          arrow_cells: vec![
            CellPosition::new(2, 8), CellPosition::new(2, 7), CellPosition::new(2, 6),
          ],
        },
      ]
    );
  let mut solver = Solver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = ArrowCandidates.run(&solver);
  solver.apply_rules(&steps);

  let steps = CommonPeerEliminationArrow.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::CommonPeerEliminationArrow);
  assert_eq!(step.areas, vec![ Area::Arrow(0) ]);
  assert_eq!(step.affected_cells.iter().copied().collect::<Vec<CellPosition>>(), vec![
    CellPosition::new(1, 6), CellPosition::new(1, 8), CellPosition::new(2, 2),
  ]);
  assert_eq!(step.values, vec![ 1, 1, 1 ]);
  assert!(solver.candidates[2][2].contains(&1));

  solver.apply_rule(&mut step);
  assert!(!solver.candidates[2][2].contains(&1));
}

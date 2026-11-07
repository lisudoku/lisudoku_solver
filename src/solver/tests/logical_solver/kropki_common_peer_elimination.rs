use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{TechniqueDefinition, TechniqueKind, candidate_initialization::CandidateInitialization, kropki_common_peer_elimination::KropkiCommonPeerElimination, kropki_chain_candidate_update::KropkiChainCandidateUpdate}}, types::{Area, CellPosition, FixedNumber, KropkiDot, SolutionStep, SudokuConstraints}};

// N4IgZglgHgpgJgOQK4FsBGMBOBnEAuAbVAAcB7bCAFwlIDt9QBjUgG3wCYAaETUgd3wBWAL7cAbgEMWSGENElyVGvTxNW%2BAGzdeAvABZRISdNl528kGQrU6DEMzZnt%2FfAfFSZ%2BAOwWrS26r26nhaPC54Iu4m%2BADMwgC63ADmmBBwAMoQAF6mAJzcANa8xAUQACKklLiETDAsLACMdg74AAzOunHcjHUs7M3BDR2xhnCVACoAnsSmIADCdNgwjEjUYrIWPfVNgS147WGdhlt9A44HOq6jE9OzC7RLK2sb8cJAA%3D%3D%3D
#[test]
fn check_common_peer_elimination_kropki_1() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      [
        FixedNumber::new(4, 2, 7), FixedNumber::new(5, 2, 5),
        FixedNumber::new(4, 6, 2), FixedNumber::new(5, 6, 3),
      ]
    )
    .with_kropki_dots(
      [
        KropkiDot::consecutive(CellPosition::new(3, 0), CellPosition::new(3, 1)),
        KropkiDot::consecutive(CellPosition::new(3, 0), CellPosition::new(4, 0)),
      ]
    );
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  let steps = KropkiChainCandidateUpdate::new(false).run(&solver);
  solver.apply_steps(&steps);

  let steps = KropkiCommonPeerElimination.run(&solver);
  assert_eq!(steps.len(), 1);

  let mut step = &steps[0];
  assert_eq!(
    *step,
    SolutionStep::new(
      TechniqueKind::KropkiCommonPeerElimination,
      vec![],
      vec![2, 3, 2, 2, 2, 3, 2, 2],
      vec![Area::KropkiDot(0), Area::KropkiDot(1)],
      vec![
        CellPosition::new(3, 2), CellPosition::new(3, 2), CellPosition::new(3, 3),
        CellPosition::new(3, 4), CellPosition::new(3, 5), CellPosition::new(4, 1),
        CellPosition::new(5, 0), CellPosition::new(5, 1),
      ],
    )
  );
  let initial_count = solver.state.candidates[3][3].len();
  assert!(solver.state.candidates[3][3].contains(&2));

  solver.apply_step(&mut step);
  assert!(!solver.state.candidates[3][3].contains(&2));
  assert_eq!(solver.state.candidates[3][3].len(), initial_count - 1);
}

use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{TechniqueDefinition, TechniqueKind, candidate_initialization::CandidateInitialization, kropki_chain_candidate_update::KropkiChainCandidateUpdate}}, types::{Area, CellPosition, FixedNumber, KropkiDot, SolutionStep, SudokuConstraints}};
use itertools::Itertools;

#[test]
fn check_kropki_chain_row_consecutive_unfixed() {
  let constraints = SudokuConstraints::new(4)
    .with_kropki_dots(
      vec![
        KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(0, 1)),
        KropkiDot::consecutive(CellPosition::new(0, 1), CellPosition::new(0, 2)),
        KropkiDot::consecutive(CellPosition::new(0, 2), CellPosition::new(0, 3)),
      ]
    );
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  let steps = KropkiChainCandidateUpdate::new(false).run(&solver);
  assert_eq!(steps.len(), 4);

  let step = &steps[0];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiChainCandidateUpdate);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 0) ]);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 2, 3 ]);
  assert_eq!(step.areas, vec![ Area::Row(0), Area::KropkiDot(0), Area::KropkiDot(1), Area::KropkiDot(2) ]);
  assert!(solver.state.candidates[0][0].contains(&2));
  solver.apply_step(&step);
  let final_candidates = &solver.state.candidates[0][0];
  assert_eq!(final_candidates.len(), 2);
  assert!(!final_candidates.contains(&2));

  let step = &steps[1];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiChainCandidateUpdate);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 1) ]);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 1, 4 ]);
  assert_eq!(step.areas, vec![ Area::Row(0), Area::KropkiDot(0), Area::KropkiDot(1), Area::KropkiDot(2) ]);
  solver.apply_step(&step);
  assert_eq!(solver.state.candidates[0][1].len(), 2);

  let step = &steps[2];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiChainCandidateUpdate);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 2) ]);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 1, 4 ]);
  assert_eq!(step.areas, vec![ Area::Row(0), Area::KropkiDot(0), Area::KropkiDot(1), Area::KropkiDot(2) ]);
  solver.apply_step(&step);
  assert_eq!(solver.state.candidates[0][2].len(), 2);

  let step = &steps[3];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiChainCandidateUpdate);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 3) ]);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 2, 3 ]);
  assert_eq!(step.areas, vec![ Area::Row(0), Area::KropkiDot(0), Area::KropkiDot(1), Area::KropkiDot(2) ]);
  solver.apply_step(&step);
  assert_eq!(solver.state.candidates[0][3].len(), 2);
}

#[test]
fn check_kropki_chain_region_double_unfixed() {
  let constraints = SudokuConstraints::new(9)
    .with_kropki_dots(
      vec![
        KropkiDot::double(CellPosition::new(3, 8), CellPosition::new(4, 8)),
        KropkiDot::double(CellPosition::new(4, 7), CellPosition::new(4, 8)),
        KropkiDot::double(CellPosition::new(4, 7), CellPosition::new(5, 7)),
      ]
    );
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  let steps = KropkiChainCandidateUpdate::new(false).run(&solver);
  assert_eq!(steps.len(), 4);

  let step = &steps[0];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiChainCandidateUpdate);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(3, 8) ]);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 2, 3, 4, 5, 6, 7, 9 ]);
  assert_eq!(step.areas, vec![ Area::Region(5), Area::KropkiDot(0), Area::KropkiDot(1), Area::KropkiDot(2) ]);
  assert!(solver.state.candidates[3][8].contains(&2));
  solver.apply_step(&step);
  let final_candidates = &solver.state.candidates[3][8];
  assert_eq!(final_candidates.len(), 2);
  assert!(!final_candidates.contains(&2));

  let step = &steps[1];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiChainCandidateUpdate);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(4, 7) ]);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 1, 3, 5, 6, 7, 8, 9 ]);
  assert_eq!(step.areas, vec![ Area::Region(5), Area::KropkiDot(0), Area::KropkiDot(1), Area::KropkiDot(2) ]);
  solver.apply_step(&step);
  assert_eq!(solver.state.candidates[4][7].len(), 2);

  let step = &steps[2];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiChainCandidateUpdate);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(4, 8) ]);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 1, 3, 5, 6, 7, 8, 9 ]);
  assert_eq!(step.areas, vec![ Area::Region(5), Area::KropkiDot(0), Area::KropkiDot(1), Area::KropkiDot(2) ]);
  solver.apply_step(&step);
  assert_eq!(solver.state.candidates[4][8].len(), 2);

  let step = &steps[3];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiChainCandidateUpdate);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(5, 7) ]);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 2, 3, 4, 5, 6, 7, 9 ]);
  assert_eq!(step.areas, vec![ Area::Region(5), Area::KropkiDot(0), Area::KropkiDot(1), Area::KropkiDot(2) ]);
  solver.apply_step(&step);
  assert_eq!(solver.state.candidates[5][7].len(), 2);
}

#[test]
fn check_kropki_chain_row_consecutive_fixed() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(vec![ FixedNumber::new(0, 0, 3) ])
    .with_kropki_dots(
      vec![
        KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(0, 1)),
        KropkiDot::consecutive(CellPosition::new(0, 1), CellPosition::new(0, 2)),
      ]
    );
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  let steps = KropkiChainCandidateUpdate::new(false).run(&solver);
  assert_eq!(steps.len(), 2);

  let step = &steps[0];
  assert_eq!(
    *step,
    SolutionStep::new(
      TechniqueKind::KropkiChainCandidateUpdate,
      vec![],
      vec![1, 5, 6],
      vec![Area::Region(0), Area::KropkiDot(0), Area::KropkiDot(1)],
      vec![CellPosition::new(0, 1)],
    ),
  );
  assert!(solver.state.candidates[0][1].contains(&1));
  solver.apply_step(&step);
  let final_candidates = &solver.state.candidates[0][1];
  assert_eq!(final_candidates.len(), 2);
  assert!(!final_candidates.contains(&1));

  let step = &steps[1];
  assert_eq!(
    *step,
    SolutionStep::new(
      TechniqueKind::KropkiChainCandidateUpdate,
      vec![],
      vec![2, 4, 6],
      vec![Area::Region(0), Area::KropkiDot(0), Area::KropkiDot(1)],
      vec![CellPosition::new(0, 2)],
    ),
  );
  solver.apply_step(&step);
  assert_eq!(solver.state.candidates[0][2].len(), 2);
}

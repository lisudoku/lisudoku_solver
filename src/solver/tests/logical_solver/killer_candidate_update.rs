use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{TechniqueDefinition, TechniqueKind, candidate_initialization::CandidateInitialization, killer_candidate_update::KillerCandidateUpdate}}, types::{Area, CellPosition, FixedNumber, KillerCage, Region, SudokuConstraints}};
use itertools::Itertools;

#[test]
fn check_killer_candidates_single_unfixed() {
  let constraints = SudokuConstraints::new(9)
    .with_killer_cages(
      [
        KillerCage {
          sum: Some(7),
          region: Region(vec![ CellPosition::new(8, 8) ]),
        },
      ]
    );
  let mut solver = LogicalSolver::new(constraints);

  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());
  let steps = KillerCandidateUpdate.run(&solver);
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();

  assert_eq!(step.technique_kind, TechniqueKind::KillerCandidateUpdate);
  assert_eq!(step.affected_cells[0], CellPosition::new(8, 8));
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 1, 2, 3, 4, 5, 6 ]);
  assert_eq!(step.areas, vec![ Area::KillerCage(0) ]);
  assert!(solver.state.candidates[8][8].contains(&1));
  solver.apply_step(&step);
  let final_candidates = &solver.state.candidates[8][8];
  assert_eq!(final_candidates.len(), 1);
  assert!(!final_candidates.contains(&4));
}

#[test]
fn check_killer_candidates_single_fixed() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers([ FixedNumber::new(0, 0, 5) ])
    .with_killer_cages(
      [
        KillerCage {
          sum: Some(5),
          region: Region(vec![ CellPosition::new(0, 0) ]),
        },
      ]
    );
  let mut solver = LogicalSolver::new(constraints);

  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());
  let steps = KillerCandidateUpdate.run(&solver);
  assert!(steps.is_empty());
}

#[test]
fn check_killer_candidates_pair_1() {
  let constraints = SudokuConstraints::new(9)
    .with_killer_cages(
      [
        KillerCage {
          sum: Some(4),
          region: Region(vec![ CellPosition::new(0, 0), CellPosition::new(0, 1) ]),
        },
      ]
    );
  let mut solver = LogicalSolver::new(constraints);

  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());
  let steps = KillerCandidateUpdate.run(&solver);
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();

  assert_eq!(step.technique_kind, TechniqueKind::KillerCandidateUpdate);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 0) ]);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 2, 4 ]);
  assert_eq!(step.areas, vec![ Area::KillerCage(0) ]);
  assert!(solver.state.candidates[0][0].contains(&4));
  solver.apply_step(&step);
  let final_candidates = &solver.state.candidates[0][0];
  assert_eq!(final_candidates.len(), 2);
  assert!(!final_candidates.contains(&4));

  let steps = KillerCandidateUpdate.run(&solver);
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();
  assert_eq!(step.technique_kind, TechniqueKind::KillerCandidateUpdate);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 1) ]);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 2, 4 ]);
}

#[test]
fn check_killer_candidates_pair_2() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers([ FixedNumber::new(0, 0, 1) ])
    .with_killer_cages(
      [
        KillerCage {
          sum: Some(4),
          region: Region(vec![ CellPosition::new(8, 0), CellPosition::new(8, 1) ]),
        },
      ]
    );
  let mut solver = LogicalSolver::new(constraints);

  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());
  let steps = KillerCandidateUpdate.run(&solver);
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();

  assert_eq!(step.technique_kind, TechniqueKind::KillerCandidateUpdate);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(8, 0) ]);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 2, 4 ]);
  assert_eq!(step.areas, vec![ Area::KillerCage(0) ]);
  assert!(solver.state.candidates[8][0].contains(&2));
  solver.apply_step(&step);
  let final_candidates = &solver.state.candidates[8][0];
  assert_eq!(final_candidates.len(), 1);
  assert!(!final_candidates.contains(&2));

  let steps = KillerCandidateUpdate.run(&solver);
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();
  assert_eq!(step.technique_kind, TechniqueKind::KillerCandidateUpdate);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(8, 1) ]);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 2, 3, 4 ]);
  assert_eq!(step.areas, vec![ Area::KillerCage(0) ]);
}

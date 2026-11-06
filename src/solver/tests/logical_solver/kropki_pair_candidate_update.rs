use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{TechniqueDefinition, TechniqueKind, candidate_initialization::CandidateInitialization, kropki_chain_candidate_update::KropkiChainCandidateUpdate}}, types::{Area, CellPosition, FixedNumber, KropkiDot, SudokuConstraints}};
use itertools::Itertools;

#[test]
fn check_kropki_negative_row_consecutive() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 1),
        FixedNumber::new(1, 0, 3),
        FixedNumber::new(1, 1, 2),
        FixedNumber::new(1, 2, 4),
      ]
    )
    .with_kropki_dots(
      vec![
        KropkiDot::consecutive(CellPosition::new(0, 1), CellPosition::new(0, 2)),
        KropkiDot::consecutive(CellPosition::new(1, 0), CellPosition::new(1, 1)),
        KropkiDot::double(CellPosition::new(1, 1), CellPosition::new(1, 2)),
      ]
    )
    .with_kropki_negative();
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  let steps = KropkiChainCandidateUpdate::new(true).run(&solver);
  assert_eq!(steps.len(), 8);
  let step = steps.into_iter().next().unwrap();
  assert_eq!(step.technique_kind, TechniqueKind::KropkiPairCandidateUpdate);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 2) ]);
  assert_eq!(step.values.iter().copied().collect_vec(), vec![ 5 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(6) ]);
  assert!(solver.state.candidates[0][2].contains(&5));
  solver.apply_step(&step);
  assert!(!solver.state.candidates[0][2].contains(&5));
}

// We only handle negative dot with pairs for now

// #[test]
// fn check_kropki_row_double_unfixed() {
//   let grid_size = 9;
//   let mut constraints = SudokuConstraints::new(grid_size);
//   constraints.kropki_dots = vec![
//     KropkiDot::double(CellPosition::new(0, 0), CellPosition::new(0, 1)),
//   ];
//   let mut solver = LogicalSolver::new(constraints);
//   solver.apply_step(&mut solver.find_candidates_step().unwrap());

//   let steps = solver.find_kropki_pair_candidate_updates();
//   assert_eq!(steps.len(), 2);
//   let step = steps.into_iter().next().unwrap();

//   assert_eq!(step.technique_kind, TechniqueKind::Kropki);
//   assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 0) ]);
//   assert_eq!(step.values.iter().copied().collect_vec(), vec![ 5, 7, 9 ]);
//   assert_eq!(step.areas, vec![ Area::KropkiDot(0) ]);
//   assert!(solver.state.candidates[0][0].contains(&5));
//   solver.apply_step(&step);
//   let final_candidates = &solver.state.candidates[0][0];
//   assert_eq!(final_candidates.len(), 9 - 3);
//   assert!(!final_candidates.contains(&5));
// }

// #[test]
// fn check_kropki_row_double_fixed() {
//   let grid_size = 9;
//   let fixed_numbers = vec![ FixedNumber::new(0, 1, 4) ];
//   let mut constraints = SudokuConstraints::new(grid_size).with_fixed_numbers(fixed_numbers);
//   constraints.kropki_dots = vec![
//     KropkiDot::double(CellPosition::new(0, 0), CellPosition::new(0, 1))
//   ];
//   let mut solver = LogicalSolver::new(constraints);
//   solver.apply_step(&mut solver.find_candidates_step().unwrap());

//   let steps = solver.find_kropki_pair_candidate_updates();
//   assert_eq!(steps.len(), 1);
//   let step = steps.into_iter().next().unwrap();

//   assert_eq!(step.technique_kind, TechniqueKind::Kropki);
//   assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 0) ]);
//   assert_eq!(step.values.iter().copied().collect_vec(), vec![ 1, 3, 5, 6, 7, 9 ]);
//   assert_eq!(step.areas, vec![ Area::KropkiDot(0) ]);
//   assert!(solver.state.candidates[0][0].contains(&3));
//   solver.apply_step(&step);
//   let final_candidates = &solver.state.candidates[0][0];
//   assert_eq!(final_candidates.len(), 2);
//   assert!(!final_candidates.contains(&3));
// }

// #[test]
// fn check_kropki_row_consecutive_unfixed() {
//   let grid_size = 9;
//   let mut constraints = SudokuConstraints::new(grid_size);
//   constraints.kropki_dots = vec![
//     KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(0, 1))
//   ];
//   let mut solver = LogicalSolver::new(constraints);
//   solver.apply_step(&mut solver.find_candidates_step().unwrap());

//   let steps = solver.find_kropki_pair_candidate_updates();
//   assert!(steps.is_empty());
// }

// #[test]
// fn check_kropki_row_consecutive_fixed() {
//   let grid_size = 9;
//   let fixed_numbers = vec![ FixedNumber::new(0, 1, 4) ];
//   let mut constraints = SudokuConstraints::new(grid_size).with_fixed_numbers(fixed_numbers);
//   constraints.kropki_dots = vec![
//     KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(0, 1))
//   ];
//   let mut solver = LogicalSolver::new(constraints);
//   solver.apply_step(&mut solver.find_candidates_step().unwrap());

//   let steps = solver.find_kropki_pair_candidate_updates();
//   assert_eq!(steps.len(), 1);
//   let step = steps.into_iter().next().unwrap();

//   assert_eq!(step.technique_kind, TechniqueKind::Kropki);
//   assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 0) ]);
//   assert_eq!(step.values.iter().copied().collect_vec(), vec![ 1, 2, 6, 7, 8, 9 ]);
//   assert_eq!(step.areas, vec![ Area::KropkiDot(0) ]);
//   assert!(solver.state.candidates[0][0].contains(&2));
//   solver.apply_step(&step);
//   let final_candidates = &solver.state.candidates[0][0];
//   assert_eq!(final_candidates.len(), 2);
//   assert!(!final_candidates.contains(&2));
// }

// #[test]
// fn check_kropki_row_double_with_2_dots() {
//   let grid_size = 9;
//   let mut constraints = SudokuConstraints::new(grid_size);
//   constraints.kropki_dots = vec![
//     KropkiDot::double(CellPosition::new(0, 0), CellPosition::new(0, 1)),
//     KropkiDot::double(CellPosition::new(0, 1), CellPosition::new(0, 2)),
//   ];
//   let mut solver = LogicalSolver::new(constraints);
//   solver.apply_step(&mut solver.find_candidates_step().unwrap());

//   let steps = solver.find_kropki_pair_candidate_updates();
//   assert_eq!(steps.len(), 3);

//   let step = &steps[0];
//   assert_eq!(step.technique_kind, TechniqueKind::Kropki);
//   assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 0) ]);
//   assert_eq!(step.values.iter().copied().collect_vec(), vec![ 5, 7, 9 ]);
//   assert_eq!(step.areas, vec![ Area::KropkiDot(0) ]);
//   assert!(solver.state.candidates[0][0].contains(&5));
//   solver.apply_step(&step);
//   let final_candidates = &solver.state.candidates[0][0];
//   assert_eq!(final_candidates.len(), 9 - 3);
//   assert!(!final_candidates.contains(&5));

//   let step = &steps[1];
//   assert_eq!(step.technique_kind, TechniqueKind::Kropki);
//   assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 1) ]);
//   assert_eq!(step.values.iter().copied().collect_vec(), vec![ 5, 7, 9 ]);
//   assert_eq!(step.areas, vec![ Area::KropkiDot(0) ]);
//   solver.apply_step(&step);
//   assert_eq!(solver.state.candidates[0][1].len(), 9 - 3);

//   let step = &steps[2];
//   assert_eq!(step.technique_kind, TechniqueKind::Kropki);
//   assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 2) ]);
//   assert_eq!(step.values.iter().copied().collect_vec(), vec![ 5, 7, 9 ]);
//   assert_eq!(step.areas, vec![ Area::KropkiDot(1) ]);
//   solver.apply_step(&step);
//   assert_eq!(solver.state.candidates[0][2].len(), 9 - 3);
// }

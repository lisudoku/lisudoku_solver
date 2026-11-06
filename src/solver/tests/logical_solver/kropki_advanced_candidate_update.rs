use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{TechniqueDefinition, TechniqueKind, candidate_initialization::CandidateInitialization, kropki_advanced_candidate_update::KropkiAdvancedCandidateUpdate, kropki_chain_candidate_update::KropkiChainCandidateUpdate}}, types::{Area, CellPosition, FixedNumber, KropkiDot, SudokuConstraints}};

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
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  let steps = KropkiChainCandidateUpdate::new(false).run(&solver);
  solver.apply_steps(&steps);

  let steps = KropkiAdvancedCandidateUpdate.run(&solver);
  assert_eq!(steps.len(), 3);

  let mut step = &steps[0];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiAdvancedCandidateUpdate);
  assert_eq!(step.values, vec![ 1 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(1, 0) ]);
  assert!(solver.state.candidates[1][0].contains(&1));
  solver.apply_step(&mut step);
  assert!(!solver.state.candidates[1][0].contains(&1));

  let mut step = &steps[1];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiAdvancedCandidateUpdate);
  assert_eq!(step.values, vec![ 2 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(2, 0) ]);
  assert!(solver.state.candidates[2][0].contains(&2));
  solver.apply_step(&mut step);
  assert!(!solver.state.candidates[2][0].contains(&2));

  let mut step = &steps[2];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiAdvancedCandidateUpdate);
  assert_eq!(step.values, vec![ 3 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(3, 0) ]);
  assert!(solver.state.candidates[3][0].contains(&3));
  solver.apply_step(&mut step);
  assert!(!solver.state.candidates[3][0].contains(&3));
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
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  let steps = KropkiChainCandidateUpdate::new(false).run(&solver);
  solver.apply_steps(&steps);

  let steps = KropkiAdvancedCandidateUpdate.run(&solver);
  assert_eq!(steps.len(), 3);

  let mut step = &steps[0];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiAdvancedCandidateUpdate);
  assert_eq!(step.values, vec![ 9 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(1, 0) ]);
  assert!(solver.state.candidates[1][0].contains(&9));
  solver.apply_step(&mut step);
  assert!(!solver.state.candidates[1][0].contains(&9));

  let mut step = &steps[1];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiAdvancedCandidateUpdate);
  assert_eq!(step.values, vec![ 8 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(2, 0) ]);
  assert!(solver.state.candidates[2][0].contains(&8));
  solver.apply_step(&mut step);
  assert!(!solver.state.candidates[2][0].contains(&8));

  let mut step = &steps[2];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiAdvancedCandidateUpdate);
  assert_eq!(step.values, vec![ 7 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(3, 0) ]);
  assert!(solver.state.candidates[3][0].contains(&7));
  solver.apply_step(&mut step);
  assert!(!solver.state.candidates[3][0].contains(&7));
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
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  let steps = KropkiChainCandidateUpdate::new(false).run(&solver);
  solver.apply_steps(&steps);

  let steps = KropkiAdvancedCandidateUpdate.run(&solver);
  assert_eq!(steps.len(), 3);

  let mut step = &steps[0];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiAdvancedCandidateUpdate);
  assert_eq!(step.values, vec![ 2 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(1, 0) ]);
  assert!(solver.state.candidates[1][0].contains(&2));
  solver.apply_step(&mut step);
  assert!(!solver.state.candidates[1][0].contains(&2));

  let mut step = &steps[1];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiAdvancedCandidateUpdate);
  assert_eq!(step.values, vec![ 3 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(2, 0) ]);
  assert!(solver.state.candidates[2][0].contains(&3));
  solver.apply_step(&mut step);
  assert!(!solver.state.candidates[2][0].contains(&3));

  let mut step = &steps[2];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiAdvancedCandidateUpdate);
  assert_eq!(step.values, vec![ 4 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(3, 0) ]);
  assert!(solver.state.candidates[3][0].contains(&4));
  solver.apply_step(&mut step);
  assert!(!solver.state.candidates[3][0].contains(&4));
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
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  let steps = KropkiChainCandidateUpdate::new(false).run(&solver);
  solver.apply_steps(&steps);

  let steps = KropkiAdvancedCandidateUpdate.run(&solver);
  assert_eq!(steps.len(), 1);
  let mut step = steps.first().unwrap();
  assert_eq!(step.technique_kind, TechniqueKind::KropkiAdvancedCandidateUpdate);
  assert_eq!(step.values, vec![ 4 ]);
  assert_eq!(step.areas, vec![ Area::KropkiDot(0), Area::KropkiDot(1) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(2, 0) ]);
  assert!(solver.state.candidates[2][0].contains(&4));
  solver.apply_step(&mut step);
  assert!(!solver.state.candidates[2][0].contains(&4));
}

// N4IgZglgHgpgJgOQK4FsBGMBOBnEAuAbVAAcB7bCAFwlIDt9QBjUgG3wCYAaETUgdw4BfbgDcAhiyQx8AFmElyVGvTxNW%2BAGzdeAvO2Ehxk6XgDs8kGQrU6DEMzZnt%2FIaIlT8ADgtWlt1fbqeJ7OuvpuxvgAnD6KNipqjiE8LnhyER54AMyx1sp2Dl6h%2BACsBkaZ6Qp5%2FolFKboa5e4mZZzVfgmBScVmzZF4TQC63ADmmBBwAMoQAF4mUdwA1rzESxAAIqSUuIRMMCwsAIwFQQAMvWcGjAcs7KeOR5cGcNsAKgCexCYgAMJ02BgjCQ1BE0gsN0OJwChTwFwa%2BCu3EhdweiN6Rxe7y%2BPy2SDQLHBQ0EQA
#[test]
fn kropki_advanced_candidates_no_duplicate_eliminations() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(2, 2, 4),
        FixedNumber::new(2, 6, 7),
        FixedNumber::new(2, 7, 8),
        FixedNumber::new(2, 8, 9),
        FixedNumber::new(4, 8, 3),
        FixedNumber::new(5, 8, 4),
        FixedNumber::new(6, 8, 5),
        FixedNumber::new(7, 8, 6),
      ]
    )
    .with_kropki_dots(
      vec![
        KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(0, 1)),
        KropkiDot::double(CellPosition::new(0, 0), CellPosition::new(1, 0)),
      ]
    );
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  // (0, 0) and (1, 0)
  let steps = KropkiChainCandidateUpdate::new(false).run(&solver);
  solver.apply_steps(&steps);

  // (0, 1)
  let steps2 = KropkiChainCandidateUpdate::new(false).run(&solver);
  solver.apply_steps(&steps2);

  let steps = KropkiAdvancedCandidateUpdate.run(&solver);
  assert_eq!(steps.len(), 2);

  let step = &steps[0];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiAdvancedCandidateUpdate);
  assert_eq!(step.values, vec![1]);
  assert_eq!(step.areas, vec![Area::KropkiDot(0)]);
  assert_eq!(step.affected_cells, vec![CellPosition::new(0, 0)]);

  let step = &steps[1];
  assert_eq!(step.technique_kind, TechniqueKind::KropkiAdvancedCandidateUpdate);
  assert_eq!(step.values, vec![1]);
  assert_eq!(step.areas, vec![Area::KropkiDot(0)]);
  assert_eq!(step.affected_cells, vec![CellPosition::new(0, 1)]);
}

use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{TechniqueDefinition, TechniqueKind, candidate_initialization::CandidateInitialization, nishio_forcing_chain::NishioForcingChain, thermo_candidate_update::ThermoCandidateUpdate}}, types::{Area, CellPosition, FixedNumber, InvalidStateType, Palindrome, SudokuConstraints, Thermo}};

#[test]
fn check_nishio_forcing_chain_valid() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      [
        FixedNumber::new(2, 0, 1),
        FixedNumber::new(2, 1, 2),
        FixedNumber::new(2, 2, 3),
      ]
    )
    .with_thermos(
      [
        Thermo(vec![
          CellPosition::new(0, 6), CellPosition::new(0, 7), CellPosition::new(0, 8),
          CellPosition::new(1, 8), CellPosition::new(1, 7), CellPosition::new(1, 6),
        ]),
      ]
    );
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());
  solver.apply_steps(&ThermoCandidateUpdate.run(&solver));

  let mut steps = NishioForcingChain.run(&solver);
  assert_eq!(steps.len(), 3);

  let initial_candidates = solver.state.candidates[0][6].clone();
  assert!(initial_candidates.contains(&2));
  assert_eq!(initial_candidates.len(), 4);

  for step in &mut steps {
    assert_eq!(step.technique_kind, TechniqueKind::NishioForcingChain);
    let reason = step.invalid_state_reason.as_ref().unwrap();
    assert_eq!(reason.state_type, InvalidStateType::AreaCandidates);
    assert_eq!(reason.area, Area::Region(2));
    assert!(reason.values.contains(&1));
    assert_eq!(step.affected_cells, vec![
      CellPosition::new(0, 6),
    ]);
    solver.apply_step(step);
  }

  assert_eq!(steps.iter().flat_map(|step| step.values.to_vec()).collect::<Vec<_>>(), vec![2, 3, 4]);

  let final_candidates = &solver.state.candidates[0][6];
  assert!(!final_candidates.contains(&2));
  assert_eq!(final_candidates.len(), 1);
}

#[test]
fn check_nishio_forcing_chain_contradiction() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      [
        FixedNumber::new(0, 1, 2), FixedNumber::new(0, 2, 3), FixedNumber::new(2, 0, 4),
      ]
    )
    .with_palindromes(
      [
        Palindrome(vec![
          CellPosition::new(0, 0), CellPosition::new(1, 1), CellPosition::new(1, 2),
          CellPosition::new(0, 3),
        ]),
      ]
    );
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  let steps = NishioForcingChain.run(&solver);
  assert!(!steps.is_empty());

  // should not panic because of finding a grid step that each sets a different cell value
}

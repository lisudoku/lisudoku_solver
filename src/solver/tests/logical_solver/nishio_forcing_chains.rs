use crate::{solver::{logical_solver::{candidates::Candidates, nishio_forcing_chains::NishioForcingChains, technique::Technique, thermo_candidates::ThermoCandidates}, Solver}, types::{Area, CellPosition, FixedNumber, InvalidStateType, Rule, SudokuConstraints}};

#[test]
fn check_nishio_forcing_chain() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(2, 0, 1),
    FixedNumber::new(2, 1, 2),
    FixedNumber::new(2, 2, 3),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    vec![
      CellPosition::new(0, 6), CellPosition::new(0, 7), CellPosition::new(0, 8),
      CellPosition::new(1, 8), CellPosition::new(1, 7), CellPosition::new(1, 6),
    ]
  ];
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  solver.apply_rules(&ThermoCandidates.run(&solver));

  let mut steps = NishioForcingChains.run(&solver);
  assert_eq!(steps.len(), 3);

  let initial_candidates = solver.candidates[0][6].clone();
  assert!(initial_candidates.contains(&2));
  assert_eq!(initial_candidates.len(), 4);

  for step in &mut steps {
    assert_eq!(step.rule, Rule::NishioForcingChains);
    let reason = step.invalid_state_reason.as_ref().unwrap();
    assert_eq!(reason.state_type, InvalidStateType::AreaCandidates);
    assert_eq!(reason.area, Area::Region(2));
    assert!(reason.values.contains(&1));
    assert_eq!(step.affected_cells, vec![
      CellPosition::new(0, 6),
    ]);
    solver.apply_rule(step);
  }

  assert_eq!(steps.iter().flat_map(|step| step.values.to_vec()).collect::<Vec<_>>(), vec![2, 3, 4]);

  let final_candidates = &solver.candidates[0][6];
  assert!(!final_candidates.contains(&2));
  assert_eq!(final_candidates.len(), 1);
}

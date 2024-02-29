use crate::{solver::{logical_solver::{candidates::Candidates, nishio_forcing_chains::NishioForcingChains, technique::Technique, thermo_candidates::ThermoCandidates}, Solver}, types::{CellPosition, FixedNumber, Rule, SudokuConstraints}};

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
  solver.apply_rule(&mut ThermoCandidates.run(&solver).first().unwrap());

  let steps = NishioForcingChains.run(&solver);
  assert_eq!(steps.len(), 1);

  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::NishioForcingChains);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(0, 6),
  ]);
  assert_eq!(step.values, vec![2, 3, 4]);
  let initial_candidates = solver.candidates[0][6].clone();
  assert!(initial_candidates.contains(&2));
  assert_eq!(initial_candidates.len(), 4);
  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[0][6];
  assert!(!final_candidates.contains(&2));
  assert_eq!(final_candidates.len(), 1);
}

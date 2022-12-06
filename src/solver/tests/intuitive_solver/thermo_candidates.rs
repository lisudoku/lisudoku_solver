use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule}, solver::Solver};

#[test]
fn check_thermo_candidates_update_hidden_single() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 1, 5),
    FixedNumber::new(8, 0, 5),
    FixedNumber::new(4, 2, 1),
    FixedNumber::new(5, 2, 2),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    vec![ CellPosition::new(2, 3), CellPosition::new(3, 3), CellPosition::new(3, 2) ]
  ];
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut solver.find_candidates_step().unwrap());
  let step = solver.find_grid_step();
  assert!(step.is_some());
  let step = step.unwrap();
  assert_eq!(step.rule, Rule::HiddenSingle);
  assert_eq!(step.cells[0], CellPosition::new(3, 2));
  assert_eq!(step.values[0], 5);
  let initial_candidates = solver.candidates[2][3].clone();
  assert!(initial_candidates.contains(&7));
  solver.apply_rule(&step);

  let step = solver.find_thermo_candidate_updates();
  assert!(step.is_some());
  let step = step.unwrap();
  solver.apply_rule(&step);

  let final_candidates = &solver.candidates[2][3];
  assert_eq!(final_candidates.len(), 3);
  assert!(!final_candidates.contains(&7));
}

#[test]
fn check_thermo_candidates_update_locked_candidates() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 2),
    FixedNumber::new(4, 0, 7),
    FixedNumber::new(4, 1, 8),
    FixedNumber::new(4, 2, 9),
    FixedNumber::new(5, 0, 4),
    FixedNumber::new(5, 1, 5),
    FixedNumber::new(5, 2, 6),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    vec![ CellPosition::new(2, 5), CellPosition::new(3, 5), CellPosition::new(4, 5) ]
  ];
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut solver.find_candidates_step().unwrap());
  for _ in 0..3 {
    let step = solver.find_thermo_candidate_updates();
    assert!(step.is_some());
    let step = step.unwrap();
    solver.apply_rule(&step);
  }

  let step = solver.find_locked_candidates_pairs();
  assert!(step.is_some());
  let step = step.unwrap();
  assert_eq!(step.cells, vec![ CellPosition::new(3, 1), CellPosition::new(3, 2) ]);
  assert_eq!(step.values, vec![ 2 ]);
  let initial_candidates = solver.candidates[4][5].clone();
  assert_eq!(initial_candidates.len(), 4);
  assert!(initial_candidates.contains(&3));
  solver.apply_rule(&step);

  let step = solver.find_thermo_candidate_updates();
  assert!(step.is_some());
  let step = step.unwrap();
  solver.apply_rule(&step);

  let final_candidates = &solver.candidates[4][5];
  assert_eq!(final_candidates.len(), 3);
  assert!(!final_candidates.contains(&3));
}

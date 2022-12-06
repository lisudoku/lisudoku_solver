use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule}, solver::Solver};

#[test]
fn check_hidden_triples() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(3, 1, 1),
    FixedNumber::new(4, 1, 2),
    FixedNumber::new(5, 1, 3),
    FixedNumber::new(6, 2, 1),
    FixedNumber::new(7, 2, 2),
    FixedNumber::new(8, 2, 3),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut solver.find_candidates_step().unwrap());

  let step = solver.find_hidden_triples();
  assert!(step.is_some());
  let mut step = step.unwrap();
  assert_eq!(step.rule, Rule::HiddenTriples);
  assert_eq!(step.cells, vec![ CellPosition::new(0, 0), CellPosition::new(1, 0), CellPosition::new(2, 0) ]);
  assert_eq!(step.values, vec![ 1, 2, 3 ]);
  let initial_candidates = solver.candidates[2][0].clone();
  assert_eq!(initial_candidates.len(), 9);

  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[2][0];
  assert_eq!(final_candidates.len(), 3);
}

#[test]
fn check_hidden_triples_no_affected_cells() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 3, 5),
    FixedNumber::new(0, 4, 4),
    FixedNumber::new(0, 5, 2),
    FixedNumber::new(0, 6, 1),
    FixedNumber::new(0, 7, 3),
    FixedNumber::new(0, 8, 7),
    FixedNumber::new(1, 0, 3),
    FixedNumber::new(1, 1, 5),
    FixedNumber::new(1, 2, 4),
    FixedNumber::new(1, 3, 7),
    FixedNumber::new(1, 4, 8),
    FixedNumber::new(1, 5, 1),
    FixedNumber::new(1, 6, 2),
    FixedNumber::new(1, 7, 9),
    FixedNumber::new(1, 8, 6),
    FixedNumber::new(2, 0, 1),
    FixedNumber::new(2, 1, 7),
    FixedNumber::new(2, 2, 2),
    FixedNumber::new(2, 3, 9),
    FixedNumber::new(2, 4, 6),
    FixedNumber::new(2, 5, 3),
    FixedNumber::new(2, 6, 4),
    FixedNumber::new(2, 7, 8),
    FixedNumber::new(2, 8, 5),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut solver.find_candidates_step().unwrap());

  let step = solver.find_hidden_triples();
  dbg!(&step);
  assert!(step.is_none());
}

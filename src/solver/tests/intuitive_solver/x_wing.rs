use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule, Area}, solver::{Solver, intuitive_solver::{candidates::Candidates, technique::Technique, x_wing::XWing}}};

#[test]
fn check_x_wing_on_row() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(1, 0, 3),
    FixedNumber::new(1, 2, 4),
    FixedNumber::new(1, 3, 5),
    FixedNumber::new(1, 4, 6),
    FixedNumber::new(1, 5, 7),
    FixedNumber::new(1, 6, 8),
    FixedNumber::new(1, 7, 9),
    FixedNumber::new(6, 0, 4),
    FixedNumber::new(6, 2, 5),
    FixedNumber::new(6, 3, 6),
    FixedNumber::new(6, 4, 7),
    FixedNumber::new(6, 5, 8),
    FixedNumber::new(6, 6, 9),
    FixedNumber::new(6, 7, 1),
    FixedNumber::new(2, 4, 2),
    FixedNumber::new(5, 5, 2),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = XWing.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::XWing);
  assert_eq!(step.areas, vec![
    Area::Row(1), Area::Row(6), Area::Column(1), Area::Column(8)
  ]);
  assert_eq!(step.cells, vec![
    CellPosition::new(1, 1),
    CellPosition::new(1, 8),
    CellPosition::new(6, 1),
    CellPosition::new(6, 8),
  ]);
  assert_eq!(step.values, vec![ 2 ]);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(0, 1), CellPosition::new(3, 1), CellPosition::new(4, 1),
    CellPosition::new(7, 1), CellPosition::new(8, 1), CellPosition::new(0, 8),
    CellPosition::new(3, 8), CellPosition::new(4, 8), CellPosition::new(7, 8),
    CellPosition::new(8, 8),
  ]);
  let initial_candidates = solver.candidates[8][8].clone();
  assert!(initial_candidates.contains(&2));
  assert_eq!(initial_candidates.len(), 7);

  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[8][8];
  assert!(!final_candidates.contains(&2));
  assert_eq!(final_candidates.len(), 6);
}

#[test]
fn check_x_wing_on_row_3_possible_cells() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(1, 0, 3),
    FixedNumber::new(1, 2, 4),
    FixedNumber::new(1, 3, 5),
    FixedNumber::new(1, 4, 6),
    FixedNumber::new(1, 5, 7),
    FixedNumber::new(1, 6, 8),
    FixedNumber::new(1, 7, 9),
    FixedNumber::new(6, 0, 4),
    FixedNumber::new(6, 2, 5),
    FixedNumber::new(6, 3, 6),
    FixedNumber::new(6, 4, 7),
    FixedNumber::new(6, 5, 8),
    FixedNumber::new(6, 6, 9),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = XWing.run(&solver);
  assert!(steps.is_empty());
}

#[test]
fn check_x_wing_on_col() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 2, 1),
    FixedNumber::new(1, 2, 2),
    FixedNumber::new(2, 2, 3),
    FixedNumber::new(3, 2, 4),
    FixedNumber::new(5, 2, 5),
    FixedNumber::new(7, 2, 6),
    FixedNumber::new(8, 2, 7),
    FixedNumber::new(0, 6, 2),
    FixedNumber::new(1, 6, 3),
    FixedNumber::new(2, 6, 4),
    FixedNumber::new(3, 6, 5),
    FixedNumber::new(5, 6, 6),
    FixedNumber::new(7, 6, 7),
    FixedNumber::new(8, 6, 8),
    FixedNumber::new(3, 4, 9),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = XWing.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::XWing);
  assert_eq!(step.areas, vec![
    Area::Column(2), Area::Column(6), Area::Row(4), Area::Row(6)
  ]);
  assert_eq!(step.cells, vec![
    CellPosition::new(4, 2),
    CellPosition::new(6, 2),
    CellPosition::new(4, 6),
    CellPosition::new(6, 6),
  ]);
  assert_eq!(step.values, vec![ 9 ]);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(4, 0), CellPosition::new(4, 1),
    CellPosition::new(4, 7), CellPosition::new(4, 8),
    CellPosition::new(6, 0), CellPosition::new(6, 1), CellPosition::new(6, 3),
    CellPosition::new(6, 5), CellPosition::new(6, 7), CellPosition::new(6, 8),
  ]);
  let initial_candidates = solver.candidates[6][8].clone();
  assert!(initial_candidates.contains(&9));
  assert_eq!(initial_candidates.len(), 7);

  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[6][8];
  assert!(!final_candidates.contains(&9));
  assert_eq!(final_candidates.len(), 6);
}

#[test]
fn check_x_wing_no_affected_cells() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(1, 0, 3),
    FixedNumber::new(1, 2, 4),
    FixedNumber::new(1, 3, 5),
    FixedNumber::new(1, 4, 6),
    FixedNumber::new(1, 5, 7),
    FixedNumber::new(1, 6, 8),
    FixedNumber::new(1, 7, 9),
    FixedNumber::new(6, 0, 4),
    FixedNumber::new(6, 2, 5),
    FixedNumber::new(6, 3, 6),
    FixedNumber::new(6, 4, 7),
    FixedNumber::new(6, 5, 8),
    FixedNumber::new(6, 6, 9),
    FixedNumber::new(6, 7, 1),
    FixedNumber::new(0, 1, 8),
    FixedNumber::new(2, 1, 9),
    FixedNumber::new(3, 2, 2),
    FixedNumber::new(7, 1, 6),
    FixedNumber::new(8, 1, 7),
    FixedNumber::new(0, 8, 4),
    FixedNumber::new(2, 8, 5),
    FixedNumber::new(4, 7, 2),
    FixedNumber::new(7, 8, 7),
    FixedNumber::new(8, 8, 6),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = XWing.run(&solver);
  assert!(steps.is_empty());
}

#[test]
fn check_x_wing_same_box() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(2, 3, 3),
    FixedNumber::new(2, 4, 4),
    FixedNumber::new(3, 2, 3),
    FixedNumber::new(4, 2, 4),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = XWing.run(&solver);
  assert!(steps.is_empty());
}

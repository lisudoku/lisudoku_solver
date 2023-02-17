use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule}, solver::{Solver, logical_solver::{candidates::Candidates, technique::Technique, xy_wing::XYWing}}};

#[test]
fn check_xy_wing() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 2, 9),
    FixedNumber::new(1, 0, 5),
    FixedNumber::new(1, 1, 4),
    FixedNumber::new(1, 2, 3),
    FixedNumber::new(2, 0, 6),
    FixedNumber::new(2, 1, 7),
    FixedNumber::new(2, 2, 8),
    FixedNumber::new(3, 0, 4),
    FixedNumber::new(4, 0, 7),
    FixedNumber::new(5, 0, 8),
    FixedNumber::new(6, 0, 9),
    FixedNumber::new(7, 3, 1),
    FixedNumber::new(1, 7, 7),
    FixedNumber::new(3, 7, 5),
    FixedNumber::new(4, 7, 4),
    FixedNumber::new(5, 7, 6),
    FixedNumber::new(6, 7, 2),
    FixedNumber::new(8, 7, 8),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = XYWing.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::XYWing);
  assert_eq!(step.areas, vec![]);
  assert_eq!(step.cells, vec![
    CellPosition::new(0, 0), // XY
    CellPosition::new(0, 7), // XZ
    CellPosition::new(7, 0), // YZ
  ]);
  assert_eq!(step.values, vec![ 1, 2, 3 ]); // XYZ
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(7, 7), // sees both XZ and YZ and it has Z as a candidate
  ]);
  let initial_candidates = solver.candidates[7][7].clone();
  assert!(initial_candidates.contains(&3));
  assert_eq!(initial_candidates.len(), 2);

  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[7][7];
  assert!(!final_candidates.contains(&3));
  assert_eq!(final_candidates.len(), 1);
}

#[test]
fn check_xy_wing_no_affected_cells() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 2, 9),
    FixedNumber::new(1, 0, 5),
    FixedNumber::new(1, 1, 4),
    FixedNumber::new(1, 2, 3),
    FixedNumber::new(2, 0, 6),
    FixedNumber::new(2, 1, 7),
    FixedNumber::new(2, 2, 8),
    FixedNumber::new(3, 0, 4),
    FixedNumber::new(4, 0, 7),
    FixedNumber::new(5, 0, 8),
    FixedNumber::new(6, 0, 9),
    FixedNumber::new(7, 3, 1),
    FixedNumber::new(1, 7, 7),
    FixedNumber::new(3, 7, 5),
    FixedNumber::new(4, 7, 4),
    FixedNumber::new(5, 7, 6),
    FixedNumber::new(6, 7, 2),
    FixedNumber::new(8, 7, 8),
    FixedNumber::new(8, 8, 3), // this eliminates the affected cell
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = XYWing.run(&solver);
  assert!(steps.is_empty());
}

#[test]
fn check_xy_wing_on_row_no_affected_cells() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 3, 4),
    FixedNumber::new(0, 4, 5),
    FixedNumber::new(0, 5, 6),
    FixedNumber::new(1, 3, 1),
    FixedNumber::new(1, 4, 2),
    FixedNumber::new(1, 5, 3),
    FixedNumber::new(0, 6, 7),
    FixedNumber::new(0, 7, 8),
    FixedNumber::new(0, 8, 9),
    FixedNumber::new(2, 6, 1),
    FixedNumber::new(2, 7, 2),
    FixedNumber::new(2, 8, 3),
    FixedNumber::new(3, 0, 3),
    FixedNumber::new(3, 1, 2),
    FixedNumber::new(3, 2, 1),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = XYWing.run(&solver);
  assert!(steps.is_empty());
}

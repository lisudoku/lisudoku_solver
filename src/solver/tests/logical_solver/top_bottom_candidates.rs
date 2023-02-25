use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule, Area}, solver::{Solver, logical_solver::{candidates::Candidates, technique::Technique, top_bottom_candidates::TopBottomCandidates}}};

#[test]
fn check_top_bottom_candidates_ascending_row1() {
  let grid_size = 4;
  let fixed_numbers = vec![ FixedNumber::new(1, 3, 2) ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers).with_top_bottom();
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  let steps = TopBottomCandidates::new(false).run(&solver);
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();
  assert_eq!(step.rule, Rule::TopBottomCandidates);
  assert_eq!(step.values, vec![1]);
  assert_eq!(step.areas, vec![Area::Row(0), Area::Row(1)]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 0), CellPosition::new(0, 1) ]);

  assert!(solver.candidates[0][0].contains(&1));
  solver.apply_rule(&step);
  assert!(!solver.candidates[0][0].contains(&1));
  assert_eq!(solver.candidates[0][0].len(), 3);
}

#[test]
fn check_top_bottom_candidates_ascending_row4() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(0, 2, 1), FixedNumber::new(1, 2, 2), FixedNumber::new(2, 2, 3),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers).with_top_bottom();
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  let steps = TopBottomCandidates::new(false).run(&solver);
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();
  assert_eq!(step.rule, Rule::TopBottomCandidates);
  assert_eq!(step.values, vec![4]);
  assert_eq!(step.areas, vec![Area::Row(3), Area::Row(2)]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(3, 0) ]);

  assert!(solver.candidates[3][0].contains(&4));
  solver.apply_rule(&step);
  assert!(!solver.candidates[3][0].contains(&4));
  assert_eq!(solver.candidates[3][0].len(), 3);
}

#[test]
fn check_top_bottom_candidates_descending_row3() {
  let grid_size = 4;
  let fixed_numbers = vec![ FixedNumber::new(3, 3, 1) ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers).with_top_bottom();
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  let steps = TopBottomCandidates::new(false).run(&solver);
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();
  assert_eq!(step.rule, Rule::TopBottomCandidates);
  assert_eq!(step.values, vec![2]);
  assert_eq!(step.areas, vec![Area::Row(2), Area::Row(3)]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(2, 0), CellPosition::new(2, 1) ]);

  assert!(solver.candidates[2][0].contains(&2));
  solver.apply_rule(&step);
  assert!(!solver.candidates[2][0].contains(&2));
  assert_eq!(solver.candidates[2][0].len(), 3);
}

#[test]
fn check_top_bottom_candidates_descending_row1() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(1, 0, 3), FixedNumber::new(2, 0, 2), FixedNumber::new(3, 0, 1),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers).with_top_bottom();
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  let steps = TopBottomCandidates::new(false).run(&solver);
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();
  assert_eq!(step.rule, Rule::TopBottomCandidates);
  assert_eq!(step.values, vec![4]);
  assert_eq!(step.areas, vec![Area::Row(0), Area::Row(1)]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 2), CellPosition::new(0, 3) ]);

  assert!(solver.candidates[0][2].contains(&4));
  solver.apply_rule(&step);
  assert!(!solver.candidates[0][2].contains(&4));
  assert_eq!(solver.candidates[0][2].len(), 3);
}

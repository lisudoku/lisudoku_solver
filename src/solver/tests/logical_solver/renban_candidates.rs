use crate::{solver::{logical_solver::{candidates::Candidates, renban_candidates::RenbanCandidates, technique::Technique}, Solver}, types::{Area, CellPosition, FixedNumber, Rule, SudokuConstraints}};
use itertools::Itertools;

#[test]
fn check_renban_candidates_in_area() {
  let grid_size = 9;
  let fixed_numbers = vec![FixedNumber::new(1, 1, 4)];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let renban = vec![
    CellPosition::new(0, 0), CellPosition::new(0, 1),
    CellPosition::new(0, 2), CellPosition::new(1, 2),
  ];
  constraints.renbans = vec![renban.to_vec()];
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  let steps = RenbanCandidates.run(&solver);
  assert_eq!(steps.len(), 4);

  for (index, step) in steps.into_iter().enumerate() {
    assert_eq!(step.rule, Rule::RenbanCandidates);
    let cell = renban[index];
    assert_eq!(step.affected_cells, vec![cell]);
    assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![1, 2, 3]);
    assert_eq!(step.areas, vec![ Area::Renban(0) ]);
    assert!(solver.candidates[cell.row][cell.col].contains(&3));
    solver.apply_rule(&step);
    assert!(!solver.candidates[cell.row][cell.col].contains(&3));
    assert_eq!(solver.candidates[cell.row][cell.col].len(), 5);
  }
}

#[test]
fn check_renban_candidates_fixed_value() {
  let grid_size = 9;
  let fixed_numbers = vec![FixedNumber::new(0, 0, 5)];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let renban = vec![
    CellPosition::new(0, 0), CellPosition::new(0, 1),
    CellPosition::new(0, 2), CellPosition::new(1, 2),
  ];
  constraints.renbans = vec![renban.to_vec()];
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  let steps = RenbanCandidates.run(&solver);
  assert_eq!(steps.len(), 3);

  for (index, step) in steps.into_iter().enumerate() {
    assert_eq!(step.rule, Rule::RenbanCandidates);
    let cell = renban[index + 1];
    assert_eq!(step.affected_cells, vec![cell]);
    assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![1, 9]);
    assert_eq!(step.areas, vec![ Area::Renban(0) ]);
    assert!(solver.candidates[cell.row][cell.col].contains(&9));
    solver.apply_rule(&step);
    assert!(!solver.candidates[cell.row][cell.col].contains(&9));
    assert_eq!(solver.candidates[cell.row][cell.col].len(), 6);
  }
}

use crate::{solver::{Solver, logical_solver::{candidates::Candidates, renban_candidates::RenbanCandidates, technique::Technique}}, types::{Area, CellPosition, FixedNumber, Renban, Rule, SudokuConstraints}};
use itertools::Itertools;

#[test]
fn check_renban_candidates_in_area() {
  let renbans = vec![
    Renban(vec![
      CellPosition::new(0, 0), CellPosition::new(0, 1),
      CellPosition::new(0, 2), CellPosition::new(1, 2),
    ])
  ];
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![FixedNumber::new(1, 1, 4)]
    )
    .with_renbans(renbans.clone());
  let mut solver = Solver::new(constraints);

  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  let steps = RenbanCandidates.run(&solver);
  assert_eq!(steps.len(), 4);

  for (index, step) in steps.into_iter().enumerate() {
    assert_eq!(step.rule, Rule::RenbanCandidates);
    let cell = renbans[0][index];
    assert_eq!(step.affected_cells, vec![cell]);
    assert_eq!(step.values.iter().copied().collect_vec(), vec![1, 2, 3]);
    assert_eq!(step.areas, vec![ Area::Renban(0) ]);
    assert!(solver.candidates[cell.row][cell.col].contains(&3));
    solver.apply_rule(&step);
    assert!(!solver.candidates[cell.row][cell.col].contains(&3));
    assert_eq!(solver.candidates[cell.row][cell.col].len(), 5);
  }
}

#[test]
fn check_renban_candidates_fixed_value() {
  let renbans = vec![
    Renban(vec![
      CellPosition::new(0, 0), CellPosition::new(0, 1),
      CellPosition::new(0, 2), CellPosition::new(1, 2),
    ])
  ];
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![FixedNumber::new(0, 0, 5)]
    )
    .with_renbans(renbans.clone());
  let mut solver = Solver::new(constraints);

  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  let steps = RenbanCandidates.run(&solver);
  assert_eq!(steps.len(), 3);

  for (index, step) in steps.into_iter().enumerate() {
    assert_eq!(step.rule, Rule::RenbanCandidates);
    let cell = renbans[0][index + 1];
    assert_eq!(step.affected_cells, vec![cell]);
    assert_eq!(step.values.iter().copied().collect_vec(), vec![1, 9]);
    assert_eq!(step.areas, vec![ Area::Renban(0) ]);
    assert!(solver.candidates[cell.row][cell.col].contains(&9));
    solver.apply_rule(&step);
    assert!(!solver.candidates[cell.row][cell.col].contains(&9));
    assert_eq!(solver.candidates[cell.row][cell.col].len(), 6);
  }
}

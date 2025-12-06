use std::rc::Rc;

use itertools::Itertools;

use crate::{solver::{logical_solver::{candidates::Candidates, phistomefel_ring::PhistomefelRing, technique::Technique}, Solver}, types::{CellPosition, FixedNumber, Rule, SolutionStep, SudokuConstraints}};

#[test]
fn check_phistomefel_ring_set_a_full() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 1),
        FixedNumber::new(0, 1, 2),
        FixedNumber::new(1, 0, 3),
        FixedNumber::new(1, 1, 4),
        FixedNumber::new(0, 7, 4),
        FixedNumber::new(0, 8, 3),
        FixedNumber::new(1, 7, 5),
        FixedNumber::new(1, 8, 2),
        FixedNumber::new(7, 0, 2),
        FixedNumber::new(7, 1, 1),
        FixedNumber::new(8, 0, 5),
        FixedNumber::new(8, 1, 3),
        FixedNumber::new(7, 7, 3),
        FixedNumber::new(7, 8, 4),
        FixedNumber::new(8, 7, 2),
        FixedNumber::new(8, 8, 1),
      ]
    );
  let mut solver = Solver::new(constraints).with_techniques(vec![Rc::new(Candidates), Rc::new(PhistomefelRing)]);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = PhistomefelRing.run(&solver);
  assert_eq!(steps.len(), 16);

  let corner_cells = vec![
    CellPosition::new(2, 2), CellPosition::new(2, 6),
    CellPosition::new(6, 2), CellPosition::new(6, 6),
  ];
  let corner_steps: Vec<&SolutionStep> = steps
    .iter()
    .filter(|step| corner_cells.contains(&step.affected_cells[0]))
    .collect();

  let mut step = corner_steps[0];
  assert_eq!(step.rule, Rule::PhistomefelRing);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(2, 2),
  ]);
  assert_eq!(step.values, vec![6, 7, 8, 9]);
  let initial_candidates = solver.candidates[2][2].clone();
  assert!(initial_candidates.contains(&6));
  assert_eq!(initial_candidates.len(), 5);
  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[2][2];
  assert!(!final_candidates.contains(&6));
  assert_eq!(final_candidates.len(), 1);

  let step = corner_steps[1];
  assert_eq!(step.rule, Rule::PhistomefelRing);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(2, 6),
  ]);
  assert_eq!(step.values, vec![6, 7, 8, 9]);

  let step = corner_steps[2];
  assert_eq!(step.rule, Rule::PhistomefelRing);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(6, 2),
  ]);
  assert_eq!(step.values, vec![6, 7, 8, 9]);

  let step = corner_steps[3];
  assert_eq!(step.rule, Rule::PhistomefelRing);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(6, 6),
  ]);
  assert_eq!(step.values, vec![6, 7, 8, 9]);

  let edge_steps: Vec<&SolutionStep> = steps
    .iter()
    .filter(|step| !corner_cells.contains(&step.affected_cells[0]))
    .collect();
  assert_eq!(edge_steps.iter().flat_map(|step| step.affected_cells.to_vec()).collect_vec(), vec![
    CellPosition::new(2, 3), CellPosition::new(2, 4), CellPosition::new(2, 5),
    CellPosition::new(3, 2), CellPosition::new(3, 6),
    CellPosition::new(4, 2), CellPosition::new(4, 6),
    CellPosition::new(5, 2), CellPosition::new(5, 6),
    CellPosition::new(6, 3), CellPosition::new(6, 4), CellPosition::new(6, 5),
  ]);
  for mut step in edge_steps {
    assert_eq!(step.rule, Rule::PhistomefelRing);
    assert_eq!(step.affected_cells.len(), 1);
    let CellPosition { row, col } = step.affected_cells[0];
    assert_eq!(step.values, vec![6, 7, 8, 9]); // maybe 5?!
    let initial_candidates = solver.candidates[row][col].clone();
    assert!(initial_candidates.contains(&8));
    assert_eq!(initial_candidates.len(), 9);
    solver.apply_rule(&mut step);
    let final_candidates = &solver.candidates[row][col];
    assert!(!final_candidates.contains(&8));
    assert_eq!(final_candidates.len(), 5); // maybe 4?!
  }
}

#[test]
fn check_phistomefel_ring_set_b_full() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(2, 2, 1),
        FixedNumber::new(2, 3, 2),
        FixedNumber::new(2, 4, 3),
        FixedNumber::new(2, 5, 4),
        FixedNumber::new(2, 6, 5),
        FixedNumber::new(3, 6, 1),
        FixedNumber::new(4, 6, 2),
        FixedNumber::new(5, 6, 3),
        FixedNumber::new(6, 6, 4),
        FixedNumber::new(6, 5, 5),
        FixedNumber::new(6, 4, 1),
        FixedNumber::new(6, 3, 3),
        FixedNumber::new(6, 2, 2),
        FixedNumber::new(5, 2, 4),
        FixedNumber::new(4, 2, 5),
        FixedNumber::new(3, 2, 3),
      ]
    );
  let mut solver = Solver::new(constraints).with_techniques(vec![Rc::new(Candidates), Rc::new(PhistomefelRing)]);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = PhistomefelRing.run(&solver);
  assert_eq!(steps.len(), 16);

  for mut step in &steps {
    assert_eq!(step.rule, Rule::PhistomefelRing);
    assert_eq!(step.affected_cells.len(), 1);
    let CellPosition { row, col } = step.affected_cells[0];
    assert_eq!(step.values, vec![6, 7, 8, 9]);
    let initial_candidates = solver.candidates[row][col].clone();
    assert!(initial_candidates.contains(&8));
    assert_eq!(initial_candidates.len(), 8);
    solver.apply_rule(&mut step);
    let final_candidates = &solver.candidates[row][col];
    assert!(!final_candidates.contains(&8));
    assert_eq!(final_candidates.len(), 4);
  }
}

// https://youtu.be/yT3Fqt8MQUc?si=kTYxpIQcahWM9EVW&t=1274
#[test]
fn check_phistomefel_ring_misc() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(2, 2, 9),
        FixedNumber::new(2, 6, 6),
        FixedNumber::new(3, 6, 1),
        FixedNumber::new(4, 6, 3),
        FixedNumber::new(5, 2, 6),
        FixedNumber::new(5, 6, 2),
        FixedNumber::new(6, 2, 8),
        FixedNumber::new(1, 8, 7),
        FixedNumber::new(0, 5, 7),
        FixedNumber::new(8, 3, 7),
        FixedNumber::new(4, 0, 7),
      ]
    );
  let mut solver = Solver::new(constraints).with_techniques(vec![Rc::new(Candidates), Rc::new(PhistomefelRing)]);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = PhistomefelRing.run(&solver);
  assert_eq!(steps.len(), 1);

  let mut step = &steps[0];
  assert_eq!(step.rule, Rule::PhistomefelRing);
  assert_eq!(step.affected_cells.len(), 1);
  let CellPosition { row, col } = step.affected_cells[0];
  assert_eq!(step.values, vec![4, 5, 9]);
  let initial_candidates = solver.candidates[row][col].clone();
  assert!(initial_candidates.contains(&9));
  assert_eq!(initial_candidates.len(), 4);
  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[row][col];
  assert!(!final_candidates.contains(&9));
  assert_eq!(final_candidates.len(), 1);
}

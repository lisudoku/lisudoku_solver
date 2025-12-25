use crate::{solver::{Solver, logical_solver::nishio_forcing_chains::NishioForcingChains}, types::{FixedNumber, Grid, Rule, SolutionType, SudokuConstraints}};
use std::rc::Rc;

#[test]
fn check_classic_9x9_easy_solve() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 8),
        FixedNumber::new(0, 5, 1),
        FixedNumber::new(0, 8, 4),
        FixedNumber::new(1, 0, 4),
        FixedNumber::new(1, 1, 5),
        FixedNumber::new(1, 7, 1),
        FixedNumber::new(1, 8, 7),
        FixedNumber::new(2, 1, 9),
        FixedNumber::new(2, 2, 1),
        FixedNumber::new(2, 4, 2),
        FixedNumber::new(2, 5, 4),
        FixedNumber::new(2, 6, 5),
        FixedNumber::new(2, 7, 6),
        FixedNumber::new(3, 1, 4),
        FixedNumber::new(3, 7, 2),
        FixedNumber::new(4, 2, 6),
        FixedNumber::new(4, 6, 3),
        FixedNumber::new(5, 0, 9),
        FixedNumber::new(5, 1, 3),
        FixedNumber::new(5, 7, 8),
        FixedNumber::new(5, 8, 1),
        FixedNumber::new(6, 1, 7),
        FixedNumber::new(6, 2, 3),
        FixedNumber::new(6, 4, 8),
        FixedNumber::new(6, 5, 6),
        FixedNumber::new(6, 6, 4),
        FixedNumber::new(6, 7, 5),
        FixedNumber::new(7, 0, 5),
        FixedNumber::new(7, 1, 8),
        FixedNumber::new(7, 7, 7),
        FixedNumber::new(7, 8, 6),
        FixedNumber::new(8, 0, 6),
        FixedNumber::new(8, 5, 5),
        FixedNumber::new(8, 8, 3),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 8, 6, 7, 5, 9, 1, 2, 3, 4 ],
      vec![ 4, 5, 2, 6, 3, 8, 9, 1, 7 ],
      vec![ 3, 9, 1, 7, 2, 4, 5, 6, 8 ],
      vec![ 7, 4, 8, 3, 1, 9, 6, 2, 5 ],
      vec![ 2, 1, 6, 8, 5, 7, 3, 4, 9 ],
      vec![ 9, 3, 5, 4, 6, 2, 7, 8, 1 ],
      vec![ 1, 7, 3, 9, 8, 6, 4, 5, 2 ],
      vec![ 5, 8, 9, 2, 4, 3, 1, 7, 6 ],
      vec![ 6, 2, 4, 1, 7, 5, 8, 9, 3 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}

#[test]
fn check_classic_9x9_medium_solve() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 1, 1),
        FixedNumber::new(0, 2, 4),
        FixedNumber::new(1, 4, 6),
        FixedNumber::new(1, 5, 5),
        FixedNumber::new(2, 3, 9),
        FixedNumber::new(2, 4, 2),
        FixedNumber::new(2, 6, 1),
        FixedNumber::new(2, 7, 5),
        FixedNumber::new(3, 1, 5),
        FixedNumber::new(3, 4, 3),
        FixedNumber::new(3, 6, 4),
        FixedNumber::new(3, 7, 6),
        FixedNumber::new(4, 2, 3),
        FixedNumber::new(4, 4, 4),
        FixedNumber::new(5, 2, 2),
        FixedNumber::new(5, 3, 6),
        FixedNumber::new(5, 5, 1),
        FixedNumber::new(5, 6, 7),
        FixedNumber::new(5, 8, 5),
        FixedNumber::new(6, 1, 7),
        FixedNumber::new(6, 6, 5),
        FixedNumber::new(6, 7, 1),
        FixedNumber::new(6, 8, 3),
        FixedNumber::new(7, 0, 1),
        FixedNumber::new(7, 7, 9),
        FixedNumber::new(8, 3, 3),
        FixedNumber::new(8, 5, 6),
        FixedNumber::new(8, 7, 7),
      ]
    );
  let mut solver = Solver::new(constraints).with_enriched_steps(true);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.clone().unwrap(),
    Grid(vec![
      vec![ 5, 1, 4, 8, 7, 3, 9, 2, 6 ],
      vec![ 8, 2, 9, 1, 6, 5, 3, 4, 7 ],
      vec![ 3, 6, 7, 9, 2, 4, 1, 5, 8 ],
      vec![ 7, 5, 1, 2, 3, 8, 4, 6, 9 ],
      vec![ 6, 9, 3, 5, 4, 7, 2, 8, 1 ],
      vec![ 4, 8, 2, 6, 9, 1, 7, 3, 5 ],
      vec![ 2, 7, 6, 4, 8, 9, 5, 1, 3 ],
      vec![ 1, 3, 8, 7, 5, 2, 6, 9, 4 ],
      vec![ 9, 4, 5, 3, 1, 6, 8, 7, 2 ],
    ])
  );

  assert!(result.steps.iter().all(|step| step.grid.is_some()));
  assert_eq!(result.solution.unwrap(), result.steps.last().unwrap().grid.clone().unwrap());

  let candidates_step_index = result.steps.iter().position(|step| step.rule == Rule::Candidates).unwrap();
  assert!(result.steps[..candidates_step_index].iter().all(|step| step.candidates.is_none()));
  assert!(result.steps[candidates_step_index..].iter().all(|step| step.candidates.is_some()));
  assert!(result.steps.last().unwrap().candidates.iter().all(|row| row.iter().all(|cell| cell.is_sorted())));

  insta::assert_yaml_snapshot!(result.steps);
}

#[test]
fn check_classic_9x9_hard_solve() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 8),
        FixedNumber::new(0, 1, 7),
        FixedNumber::new(1, 3, 9),
        FixedNumber::new(1, 8, 4),
        FixedNumber::new(2, 1, 2),
        FixedNumber::new(2, 3, 7),
        FixedNumber::new(2, 6, 1),
        FixedNumber::new(2, 8, 5),
        FixedNumber::new(3, 2, 9),
        FixedNumber::new(3, 3, 6),
        FixedNumber::new(3, 7, 3),
        FixedNumber::new(4, 8, 9),
        FixedNumber::new(5, 2, 6),
        FixedNumber::new(5, 3, 5),
        FixedNumber::new(5, 4, 4),
        FixedNumber::new(6, 0, 6),
        FixedNumber::new(6, 1, 9),
        FixedNumber::new(6, 6, 7),
        FixedNumber::new(7, 0, 2),
        FixedNumber::new(7, 5, 7),
        FixedNumber::new(7, 6, 4),
        FixedNumber::new(8, 3, 3),
        FixedNumber::new(8, 7, 1),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 8, 7, 3, 4, 1, 5, 9, 6, 2 ],
      vec![ 1, 6, 5, 9, 2, 8, 3, 7, 4 ],
      vec![ 9, 2, 4, 7, 6, 3, 1, 8, 5 ],
      vec![ 4, 8, 9, 6, 7, 2, 5, 3, 1 ],
      vec![ 7, 5, 2, 8, 3, 1, 6, 4, 9 ],
      vec![ 3, 1, 6, 5, 4, 9, 8, 2, 7 ],
      vec![ 6, 9, 1, 2, 8, 4, 7, 5, 3 ],
      vec![ 2, 3, 8, 1, 5, 7, 4, 9, 6 ],
      vec![ 5, 4, 7, 3, 9, 6, 2, 1, 8 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}

#[test]
fn check_classic_9x9_hard_xy_wing_solve() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 2, 1),
        FixedNumber::new(0, 6, 9),
        FixedNumber::new(1, 1, 7),
        FixedNumber::new(1, 5, 8),
        FixedNumber::new(1, 6, 4),
        FixedNumber::new(1, 7, 3),
        FixedNumber::new(2, 0, 8),
        FixedNumber::new(2, 3, 6),
        FixedNumber::new(3, 2, 2),
        FixedNumber::new(3, 4, 1),
        FixedNumber::new(4, 1, 4),
        FixedNumber::new(4, 5, 6),
        FixedNumber::new(4, 6, 8),
        FixedNumber::new(4, 7, 7),
        FixedNumber::new(5, 8, 5),
        FixedNumber::new(6, 2, 4),
        FixedNumber::new(6, 3, 2),
        FixedNumber::new(6, 6, 3),
        FixedNumber::new(6, 7, 5),
        FixedNumber::new(7, 1, 5),
        FixedNumber::new(7, 8, 6),
        FixedNumber::new(8, 5, 3),
        FixedNumber::new(8, 8, 9),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 4, 2, 1, 3, 5, 7, 9, 6, 8 ],
      vec![ 5, 7, 6, 1, 9, 8, 4, 3, 2 ],
      vec![ 8, 3, 9, 6, 4, 2, 5, 1, 7 ],
      vec![ 3, 8, 2, 7, 1, 5, 6, 9, 4 ],
      vec![ 1, 4, 5, 9, 2, 6, 8, 7, 3 ],
      vec![ 6, 9, 7, 8, 3, 4, 1, 2, 5 ],
      vec![ 7, 6, 4, 2, 8, 9, 3, 5, 1 ],
      vec![ 9, 5, 3, 4, 7, 1, 2, 8, 6 ],
      vec![ 2, 1, 8, 5, 6, 3, 7, 4, 9 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}

#[test]
fn check_classic_9x9_hard_x_wing_solve() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 6),
        FixedNumber::new(0, 4, 9),
        FixedNumber::new(0, 8, 7),
        FixedNumber::new(1, 1, 4),
        FixedNumber::new(1, 5, 7),
        FixedNumber::new(1, 6, 1),
        FixedNumber::new(2, 2, 2),
        FixedNumber::new(2, 3, 8),
        FixedNumber::new(2, 7, 5),
        FixedNumber::new(3, 0, 8),
        FixedNumber::new(3, 7, 9),
        FixedNumber::new(4, 4, 7),
        FixedNumber::new(5, 1, 3),
        FixedNumber::new(5, 8, 8),
        FixedNumber::new(6, 1, 5),
        FixedNumber::new(6, 5, 2),
        FixedNumber::new(6, 6, 3),
        FixedNumber::new(7, 2, 4),
        FixedNumber::new(7, 3, 5),
        FixedNumber::new(7, 7, 2),
        FixedNumber::new(8, 0, 9),
        FixedNumber::new(8, 4, 3),
        FixedNumber::new(8, 8, 4),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 6, 8, 3, 1, 9, 5, 2, 4, 7 ],
      vec![ 5, 4, 9, 6, 2, 7, 1, 8, 3 ],
      vec![ 7, 1, 2, 8, 4, 3, 9, 5, 6 ],
      vec![ 8, 6, 5, 3, 1, 4, 7, 9, 2 ],
      vec![ 4, 9, 1, 2, 7, 8, 6, 3, 5 ],
      vec![ 2, 3, 7, 9, 5, 6, 4, 1, 8 ],
      vec![ 1, 5, 6, 4, 8, 2, 3, 7, 9 ],
      vec![ 3, 7, 4, 5, 6, 9, 8, 2, 1 ],
      vec![ 9, 2, 8, 7, 3, 1, 5, 6, 4 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}

#[test]
fn check_classic_9x9_hard_solve_with_solution() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 1, 3),
        FixedNumber::new(0, 3, 5),
        FixedNumber::new(1, 0, 1),
        FixedNumber::new(1, 3, 8),
        FixedNumber::new(1, 5, 2),
        FixedNumber::new(1, 7, 9),
        FixedNumber::new(2, 2, 9),
        FixedNumber::new(2, 6, 4),
        FixedNumber::new(3, 0, 8),
        FixedNumber::new(3, 3, 9),
        FixedNumber::new(3, 5, 1),
        FixedNumber::new(3, 7, 4),
        FixedNumber::new(4, 4, 7),
        FixedNumber::new(5, 1, 6),
        FixedNumber::new(5, 8, 3),
        FixedNumber::new(6, 0, 7),
        FixedNumber::new(6, 4, 4),
        FixedNumber::new(7, 1, 8),
        FixedNumber::new(7, 3, 2),
        FixedNumber::new(7, 5, 7),
        FixedNumber::new(7, 6, 6),
        FixedNumber::new(8, 7, 2),
      ]
    );
  let mut solver = Solver::new(constraints).without_techniques(vec![Rc::new(NishioForcingChains)]);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Partial);
  // TODO: why no full?
  insta::assert_yaml_snapshot!(result.steps);
}

// https://github.com/lisudoku/lisudoku_solver/issues/5
#[test]
fn check_classic_9x9_1_solve() {
  let fixed_numbers = vec![
    FixedNumber::new(0, 4, 5),
    FixedNumber::new(0, 6, 6),
    FixedNumber::new(0, 8, 3),
    FixedNumber::new(1, 2, 3),
    FixedNumber::new(1, 3, 6),
    FixedNumber::new(1, 6, 9),
    FixedNumber::new(2, 0, 8),
    FixedNumber::new(2, 4, 1),
    FixedNumber::new(2, 6, 4),
    FixedNumber::new(2, 7, 2),
    FixedNumber::new(3, 0, 6),
    FixedNumber::new(3, 1, 2),
    FixedNumber::new(3, 2, 9),
    FixedNumber::new(3, 5, 5),
    FixedNumber::new(3, 6, 8),
    FixedNumber::new(4, 1, 5),
    FixedNumber::new(4, 4, 2),
    FixedNumber::new(4, 7, 4),
    FixedNumber::new(4, 8, 9),
    FixedNumber::new(5, 1, 7),
    FixedNumber::new(5, 6, 2),
    FixedNumber::new(5, 7, 5),
    FixedNumber::new(6, 3, 4),
    FixedNumber::new(6, 8, 2),
    FixedNumber::new(7, 0, 4),
    FixedNumber::new(7, 2, 7),
    FixedNumber::new(7, 6, 5),
    FixedNumber::new(8, 1, 1),
    FixedNumber::new(8, 6, 7),
  ];
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(fixed_numbers.iter().copied().collect());
  let mut solver = Solver::new(constraints).without_techniques(vec![Rc::new(NishioForcingChains)]);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 7, 9, 1, 2, 5, 4, 6, 8, 3 ],
      vec![ 2, 4, 3, 6, 8, 7, 9, 1, 5 ],
      vec![ 8, 6, 5, 9, 1, 3, 4, 2, 7 ],
      vec![ 6, 2, 9, 3, 4, 5, 8, 7, 1 ],
      vec![ 1, 5, 8, 7, 2, 6, 3, 4, 9 ],
      vec![ 3, 7, 4, 8, 9, 1, 2, 5, 6 ],
      vec![ 5, 8, 6, 4, 7, 9, 1, 3, 2 ],
      vec![ 4, 3, 7, 1, 6, 2, 5, 9, 8 ],
      vec![ 9, 1, 2, 5, 3, 8, 7, 6, 4 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);

  let fixed_numbers: Vec<FixedNumber> = fixed_numbers.iter().copied().filter(|fixed_number| *fixed_number != FixedNumber::new(4, 1, 5)).collect();
  let constraints = SudokuConstraints::new(9).with_fixed_numbers(fixed_numbers);
  let mut solver = Solver::new(constraints).without_techniques(vec![Rc::new(NishioForcingChains)]);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Partial);
}

// https://github.com/lisudoku/lisudoku_solver/issues/2
#[test]
fn check_classic_9x9_2_conflicts_solve() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 3),
        FixedNumber::new(0, 1, 3),
        FixedNumber::new(0, 7, 8),
        FixedNumber::new(0, 8, 1),
        FixedNumber::new(1, 1, 2),
        FixedNumber::new(1, 5, 3),
        FixedNumber::new(2, 4, 1),
        FixedNumber::new(2, 5, 7),
        FixedNumber::new(2, 6, 4),
        FixedNumber::new(2, 7, 3),
        FixedNumber::new(3, 1, 9),
        FixedNumber::new(3, 2, 6),
        FixedNumber::new(3, 3, 4),
        FixedNumber::new(3, 6, 5),
        FixedNumber::new(3, 7, 7),
        FixedNumber::new(4, 3, 7),
        FixedNumber::new(4, 5, 2),
        FixedNumber::new(5, 1, 8),
        FixedNumber::new(5, 6, 6),
        FixedNumber::new(6, 1, 6),
        FixedNumber::new(6, 4, 2),
        FixedNumber::new(7, 0, 3),
        FixedNumber::new(7, 2, 9),
        FixedNumber::new(7, 7, 6),
        FixedNumber::new(8, 8, 9),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::None);
}

#[test]
fn check_classic_9x9_hard_empty_rectangles_solve() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers_grid(
      Grid(vec![
        vec![ 0, 0, 0, 9, 0, 0, 6, 0, 0 ],
        vec![ 4, 0, 9, 0, 0, 0, 8, 0, 0 ],
        vec![ 2, 0, 7, 6, 0, 0, 5, 0, 4 ],
        vec![ 1, 0, 8, 7, 0, 0, 3, 0, 0 ],
        vec![ 0, 0, 0, 0, 0, 3, 0, 0, 0 ],
        vec![ 0, 0, 0, 0, 0, 0, 0, 4, 0 ],
        vec![ 0, 9, 0, 0, 0, 7, 0, 0, 1 ],
        vec![ 0, 0, 5, 0, 0, 2, 0, 0, 0 ],
        vec![ 0, 3, 0, 5, 0, 6, 0, 0, 0 ],
      ])
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 3, 5, 1, 9, 4, 8, 6, 2, 7 ],
      vec![ 4, 6, 9, 2, 7, 5, 8, 1, 3 ],
      vec![ 2, 8, 7, 6, 3, 1, 5, 9, 4 ],
      vec![ 1, 2, 8, 7, 5, 4, 3, 6, 9 ],
      vec![ 9, 4, 6, 8, 2, 3, 1, 7, 5 ],
      vec![ 5, 7, 3, 1, 6, 9, 2, 4, 8 ],
      vec![ 6, 9, 2, 3, 8, 7, 4, 5, 1 ],
      vec![ 8, 1, 5, 4, 9, 2, 7, 3, 6 ],
      vec![ 7, 3, 4, 5, 1, 6, 9, 8, 2 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}

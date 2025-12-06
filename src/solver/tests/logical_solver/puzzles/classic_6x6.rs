use crate::{solver::Solver, types::{FixedNumber, Grid, SolutionType, SudokuConstraints}};

#[test]
fn check_classic_6x6_1_solve() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 6),
        FixedNumber::new(1, 0, 1),
        FixedNumber::new(1, 1, 4),
        FixedNumber::new(2, 1, 1),
        FixedNumber::new(2, 2, 2),
        FixedNumber::new(2, 3, 5),
        FixedNumber::new(2, 5, 6),
        FixedNumber::new(3, 0, 5),
        FixedNumber::new(3, 2, 6),
        FixedNumber::new(3, 3, 2),
        FixedNumber::new(3, 4, 1),
        FixedNumber::new(4, 4, 2),
        FixedNumber::new(4, 5, 1),
        FixedNumber::new(5, 5, 3),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 6, 2, 3, 1, 4, 5 ],
      vec![ 1, 4, 5, 3, 6, 2 ],
      vec![ 4, 1, 2, 5, 3, 6 ],
      vec![ 5, 3, 6, 2, 1, 4 ],
      vec![ 3, 5, 4, 6, 2, 1 ],
      vec![ 2, 6, 1, 4, 5, 3 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}

// https://sudoku-puzzles.net/sudoku-6x6-hard
#[test]
fn check_classic_6x6_2_hard_solve() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 2, 5),
        FixedNumber::new(0, 5, 2),
        FixedNumber::new(1, 0, 6),
        FixedNumber::new(2, 0, 4),
        FixedNumber::new(2, 5, 5),
        FixedNumber::new(3, 0, 5),
        FixedNumber::new(3, 4, 4),
        FixedNumber::new(4, 2, 1),
        FixedNumber::new(4, 3, 2),
        FixedNumber::new(5, 5, 1),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 1, 3, 5, 4, 6, 2 ],
      vec![ 6, 4, 2, 5, 1, 3 ],
      vec![ 4, 1, 6, 3, 2, 5 ],
      vec![ 5, 2, 3, 1, 4, 6 ],
      vec![ 3, 6, 1, 2, 5, 4 ],
      vec![ 2, 5, 4, 6, 3, 1 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}

// SM 2023 Round 2 - puzzle 1
// https://logicmastersindia.com/live/score?contest=SM202302
#[test]
fn check_classic_6x6_3_hard_solve() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 2, 2),
        FixedNumber::new(0, 3, 1),
        FixedNumber::new(1, 5, 6),
        FixedNumber::new(2, 0, 1),
        FixedNumber::new(2, 2, 3),
        FixedNumber::new(2, 5, 4),
        FixedNumber::new(3, 0, 5),
        FixedNumber::new(3, 3, 2),
        FixedNumber::new(3, 5, 1),
        FixedNumber::new(4, 0, 2),
        FixedNumber::new(5, 2, 1),
        FixedNumber::new(5, 3, 6),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 6, 3, 2, 1, 4, 5 ],
      vec![ 4, 1, 5, 3, 2, 6 ],
      vec![ 1, 2, 3, 5, 6, 4 ],
      vec![ 5, 6, 4, 2, 3, 1 ],
      vec![ 2, 5, 6, 4, 1, 3 ],
      vec![ 3, 4, 1, 6, 5, 2 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}

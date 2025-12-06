use crate::{solver::Solver, types::{CellPosition, FixedNumber, Grid, SolutionType, SudokuConstraints, Thermo}};

#[test]
fn check_thermo_6x6_1_solve() {
  // WSC booklet 6x6 thermo https://uploads-ssl.webflow.com/62793457876c001d28edf162/6348945a45b06acb414391b7_WSC_2022_IB_v2.1.pdf
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(1, 0, 4),
        FixedNumber::new(2, 0, 5),
        FixedNumber::new(4, 5, 2),
        FixedNumber::new(5, 4, 4),
        FixedNumber::new(5, 5, 3),
      ]
    )
    .with_thermos(
      vec![
        Thermo(vec![
          CellPosition::new(0, 0),
          CellPosition::new(0, 1),
          CellPosition::new(0, 2),
          CellPosition::new(0, 3),
          CellPosition::new(0, 4),
          CellPosition::new(0, 5),
        ]),
        Thermo(vec![
          CellPosition::new(1, 4),
          CellPosition::new(2, 4),
          CellPosition::new(3, 4),
        ]),
        Thermo(vec![
          CellPosition::new(2, 2),
          CellPosition::new(3, 2),
          CellPosition::new(4, 2),
          CellPosition::new(4, 3),
        ]),
        Thermo(vec![
          CellPosition::new(3, 0),
          CellPosition::new(4, 0),
          CellPosition::new(5, 0),
        ]),
        Thermo(vec![
          CellPosition::new(3, 3),
          CellPosition::new(2, 3),
          CellPosition::new(1, 3),
          CellPosition::new(1, 2),
        ]),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 1, 2, 3, 4, 5, 6 ],
      vec![ 4, 5, 6, 3, 2, 1 ],
      vec![ 5, 6, 1, 2, 3, 4 ],
      vec![ 2, 3, 4, 1, 6, 5 ],
      vec![ 3, 4, 5, 6, 1, 2 ],
      vec![ 6, 1, 2, 5, 4, 3 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}

// https://github.com/lisudoku/lisudoku_solver/issues/4
#[test]
fn check_thermo_6x6_2_solve() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 5),
        FixedNumber::new(0, 5, 4),
        FixedNumber::new(2, 2, 1),
        FixedNumber::new(3, 3, 6),
        FixedNumber::new(5, 0, 6),
        FixedNumber::new(5, 3, 3),
        FixedNumber::new(5, 5, 2),
      ]
    )
    .with_thermos(
      vec![
        Thermo(vec![
          CellPosition::new(1, 1),
          CellPosition::new(2, 1),
          CellPosition::new(3, 1),
        ]),
        Thermo(vec![
          CellPosition::new(1, 4),
          CellPosition::new(1, 3),
          CellPosition::new(1, 2),
        ]),
        Thermo(vec![
          CellPosition::new(4, 1),
          CellPosition::new(4, 2),
          CellPosition::new(4, 3),
        ]),
        Thermo(vec![
          CellPosition::new(4, 4),
          CellPosition::new(3, 4),
          CellPosition::new(2, 4),
        ]),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::None);
}

// https://github.com/lisudoku/lisudoku_solver/issues/3
#[test]
fn check_thermo_6x6_3_solve() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(1, 1, 6),
        FixedNumber::new(2, 2, 5),
        FixedNumber::new(3, 3, 4),
      ]
    )
    .with_thermos(
      vec![
        Thermo(vec![
          CellPosition::new(2, 0),
          CellPosition::new(1, 0),
          CellPosition::new(0, 0),
          CellPosition::new(0, 1),
          CellPosition::new(0, 2),
        ]),
        Thermo(vec![
          CellPosition::new(4, 1),
          CellPosition::new(3, 2),
          CellPosition::new(2, 3),
          CellPosition::new(1, 4),
        ]),
        Thermo(vec![
          CellPosition::new(3, 5),
          CellPosition::new(4, 5),
          CellPosition::new(5, 5),
          CellPosition::new(5, 4),
          CellPosition::new(5, 3),
        ]),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::None);
}

// https://github.com/lisudoku/lisudoku_solver/issues/7
#[test]
fn check_thermo_6x6_4_solve() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(1, 1, 5),
        FixedNumber::new(4, 4, 6),
      ]
    )
    .with_thermos(
      vec![
        Thermo(vec![
          CellPosition::new(1, 4),
          CellPosition::new(2, 3),
          CellPosition::new(3, 2),
          CellPosition::new(4, 1),
        ]),
        Thermo(vec![
          CellPosition::new(2, 0),
          CellPosition::new(3, 1),
          CellPosition::new(4, 2),
          CellPosition::new(5, 3),
        ]),
        Thermo(vec![
          CellPosition::new(3, 5),
          CellPosition::new(2, 4),
          CellPosition::new(1, 3),
          CellPosition::new(0, 2),
        ]),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::None);
}

// https://github.com/lisudoku/lisudoku_solver/issues/8
#[test]
fn check_thermo_6x6_5_solve() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 1),
        FixedNumber::new(0, 5, 3),
        FixedNumber::new(5, 0, 2),
        FixedNumber::new(5, 5, 1),
      ]
    )
    .with_thermos(
      vec![
        Thermo(vec![
          CellPosition::new(1, 4),
          CellPosition::new(0, 3),
          CellPosition::new(0, 2),
          CellPosition::new(1, 1),
        ]),
        Thermo(vec![
          CellPosition::new(2, 1),
          CellPosition::new(3, 1),
          CellPosition::new(4, 1),
          CellPosition::new(5, 2),
        ]),
        Thermo(vec![
          CellPosition::new(5, 3),
          CellPosition::new(4, 4),
          CellPosition::new(3, 4),
          CellPosition::new(3, 3),
        ]),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::None);
}

// https://github.com/lisudoku/lisudoku_solver/issues/9
#[test]
fn check_thermo_6x6_6_solve() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(1, 5, 3),
        FixedNumber::new(2, 5, 1),
        FixedNumber::new(5, 2, 1),
        FixedNumber::new(5, 3, 4),
      ]
    )
    .with_thermos(
      vec![
        Thermo(vec![
          CellPosition::new(0, 3),
          CellPosition::new(1, 2),
          CellPosition::new(2, 1),
          CellPosition::new(3, 0),
        ]),
        Thermo(vec![
          CellPosition::new(4, 4),
          CellPosition::new(3, 3),
          CellPosition::new(2, 2),
          CellPosition::new(1, 1),
        ]),
        Thermo(vec![
          CellPosition::new(5, 0),
          CellPosition::new(4, 1),
          CellPosition::new(3, 2),
          CellPosition::new(2, 3),
        ]),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 4, 5, 3, 1, 2, 6 ],
      vec![ 1, 6, 2, 5, 4, 3 ],
      vec![ 2, 3, 4, 6, 5, 1 ],
      vec![ 6, 1, 5, 2, 3, 4 ],
      vec![ 5, 4, 6, 3, 1, 2 ],
      vec![ 3, 2, 1, 4, 6, 5 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}

// https://github.com/lisudoku/lisudoku_solver/issues/10
#[test]
fn check_thermo_6x6_7_solve() {
  let constraints = SudokuConstraints::new(6)
    .with_thermos(
      vec![
        Thermo(vec![
          CellPosition::new(2, 1),
          CellPosition::new(2, 2),
          CellPosition::new(1, 2),
          CellPosition::new(1, 3),
          CellPosition::new(1, 4),
        ]),
        Thermo(vec![
          CellPosition::new(3, 0),
          CellPosition::new(3, 1),
          CellPosition::new(3, 2),
          CellPosition::new(3, 3),
        ]),
        Thermo(vec![
          CellPosition::new(4, 4),
          CellPosition::new(4, 3),
          CellPosition::new(4, 2),
          CellPosition::new(4, 1),
        ]),
        Thermo(vec![
          CellPosition::new(5, 2),
          CellPosition::new(5, 3),
          CellPosition::new(5, 4),
          CellPosition::new(5, 5),
          CellPosition::new(4, 5),
        ]),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 5, 2, 6, 1, 3, 4 ],
      vec![ 1, 3, 4, 5, 6, 2 ],
      vec![ 6, 1, 2, 4, 5, 3 ],
      vec![ 3, 4, 5, 6, 2, 1 ],
      vec![ 4, 5, 3, 2, 1, 6 ],
      vec![ 2, 6, 1, 3, 4, 5 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}

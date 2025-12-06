use crate::{solver::Solver, types::{CellPosition, FixedNumber, Grid, SolutionType, SudokuConstraints}};

// https://logicmastersindia.com/live/?contest=SM202301
#[test]
fn check_odd_even_6x6_1_solve() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 1),
        FixedNumber::new(0, 5, 3),
        FixedNumber::new(2, 0, 3),
        FixedNumber::new(2, 5, 5),
        FixedNumber::new(3, 0, 2),
        FixedNumber::new(3, 5, 4),
        FixedNumber::new(5, 0, 4),
        FixedNumber::new(5, 5, 2),
      ]
    )
    .with_even_cells(
      vec![
        CellPosition::new(0, 1), CellPosition::new(0, 3), CellPosition::new(1, 2),
        CellPosition::new(2, 1), CellPosition::new(2, 3),
      ]
    )
    .with_odd_cells(
      vec![
        CellPosition::new(3, 2), CellPosition::new(3, 4), CellPosition::new(4, 3),
        CellPosition::new(5, 2), CellPosition::new(5, 4),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 1, 2, 5, 4, 6, 3 ],
      vec![ 6, 3, 4, 5, 2, 1 ],
      vec![ 3, 4, 6, 2, 1, 5 ],
      vec![ 2, 5, 1, 6, 3, 4 ],
      vec![ 5, 1, 2, 3, 4, 6 ],
      vec![ 4, 6, 3, 1, 5, 2 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}

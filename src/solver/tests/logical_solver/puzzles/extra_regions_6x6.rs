use crate::{solver::Solver, types::{CellPosition, FixedNumber, Grid, Region, SolutionType, SudokuConstraints}};

// https://logicmastersindia.com/live/?contest=SM202301
#[test]
fn check_extra_regions_6x6_1_solve() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 5),
        FixedNumber::new(0, 1, 1),
        FixedNumber::new(0, 2, 4),
        FixedNumber::new(0, 3, 2),
        FixedNumber::new(2, 2, 2),
        FixedNumber::new(3, 3, 3),
        FixedNumber::new(5, 2, 3),
        FixedNumber::new(5, 3, 5),
        FixedNumber::new(5, 4, 4),
        FixedNumber::new(5, 5, 1),
      ]
    )
    .with_extra_regions(
      vec![
        Region(vec![
          CellPosition::new(1, 0), CellPosition::new(2, 0), CellPosition::new(2, 1),
          CellPosition::new(3, 0), CellPosition::new(3, 1), CellPosition::new(4, 0),
        ]),
        Region(vec![
          CellPosition::new(1, 5), CellPosition::new(2, 4), CellPosition::new(2, 5),
          CellPosition::new(3, 4), CellPosition::new(3, 5), CellPosition::new(4, 5),
        ]),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 5, 1, 4, 2, 3, 6 ],
      vec![ 2, 3, 6, 1, 5, 4 ],
      vec![ 3, 6, 2, 4, 1, 5 ],
      vec![ 4, 5, 1, 3, 6, 2 ],
      vec![ 1, 4, 5, 6, 2, 3 ],
      vec![ 6, 2, 3, 5, 4, 1 ],
    ]),
  );
  insta::assert_yaml_snapshot!(result.steps);
}

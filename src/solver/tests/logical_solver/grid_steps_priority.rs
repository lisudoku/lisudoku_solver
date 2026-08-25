use crate::{solver::Solver, types::{Area, CellPosition, Grid, Rule, SolutionStep, SudokuConstraints}};

#[test]
fn check_grid_steps_priority() {
  struct TestCase<'a> {
    grid: &'a str,
    expected: SolutionStep,
  }
  let cases = vec![
    // We have naked single on (1, 0), but prefer the hidden single on (0, 1).
    TestCase {
      grid: "
        0000
        0402
        1030
        0000
      ",
      expected: SolutionStep::new(
        Rule::HiddenSingle,
        vec![CellPosition::new(0, 1), CellPosition::new(2, 0)],
        vec![1],
        vec![Area::Region(0)],
        vec![],
      ),
    },
    
    // We have the same hidden single on (0, 1), but the one on (1, 2) is simpler.
    TestCase {
      grid: "
        0000
        3402
        1030
        0000
      ",
      expected: SolutionStep::new(
        Rule::HiddenSingle,
        vec![CellPosition::new(1, 2)],
        vec![1],
        vec![Area::Row(1)],
        vec![],
      ),
    },

    // Same hidden single on (0, 1) :-)
    TestCase {
      grid: "
        0000
        3412
        1030
        0000
      ",
      expected: SolutionStep::new(
        Rule::HiddenSingle,
        vec![CellPosition::new(0, 1), CellPosition::new(2, 0)],
        vec![1],
        vec![Area::Region(0)],
        vec![],
      ),
    },

    // (2, 3) technically could be naked single, but I prefer it as hidden single
    TestCase {
      grid: "
        0000
        3412
        1230
        0000
      ",
      expected: SolutionStep::new(
        Rule::HiddenSingle,
        vec![CellPosition::new(2, 3)],
        vec![4],
        vec![Area::Row(2)],
        vec![],
      ),
    },

    TestCase {
      grid: "
        0000
        3412
        1234
        0000
      ",
      expected: SolutionStep::new(
        Rule::HiddenSingle,
        vec![CellPosition::new(0, 1), CellPosition::new(2, 0)],
        vec![1],
        vec![Area::Region(0)],
        vec![],
      ),
    },

    TestCase {
      grid: "
        2000
        3412
        1234
        0000
      ",
      expected: SolutionStep::new(
        Rule::HiddenSingle,
        vec![CellPosition::new(0, 1)],
        vec![1],
        vec![Area::Region(0)],
        vec![],
      ),
    },

    TestCase {
      grid: "
        0000
        0412
        1030
        0000
      ",
      expected: SolutionStep::new(
        Rule::HiddenSingle,
        vec![CellPosition::new(1, 0)],
        vec![3],
        vec![Area::Row(1)],
        vec![],
      ),
    },
  ];

  for case in cases {
    let constraints = SudokuConstraints::new(4)
      .with_fixed_numbers_grid(
        Grid::from_string(case.grid)
      );
    let mut solver = Solver::new(constraints);

    let result = solver.logical_solve();
    assert!(!result.steps.is_empty(), "didn't find solution for grid {}", case.grid);
    let step = result.steps.first().unwrap();
    assert_eq!(
      *step,
      case.expected,
      "wrong step for grid {}",
      case.grid,
    );
  }
}

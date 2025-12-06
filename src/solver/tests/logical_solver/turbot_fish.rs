use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule, Area}, solver::{Solver, logical_solver::{candidates::Candidates, technique::Technique, turbot_fish::TurbotFish}}};

// https://www.sudopedia.org/wiki/2-String_Kite
#[test]
fn check_turbot_fish_2_string_kite() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 1),
        FixedNumber::new(0, 1, 2),
        FixedNumber::new(1, 0, 3),
        FixedNumber::new(1, 1, 4),
        FixedNumber::new(1, 2, 5),
        FixedNumber::new(2, 1, 7),
        FixedNumber::new(2, 2, 6),
        FixedNumber::new(0, 3, 7),
        FixedNumber::new(0, 4, 3),
        FixedNumber::new(0, 5, 4),
        FixedNumber::new(0, 7, 5),
        FixedNumber::new(0, 8, 6),
        FixedNumber::new(3, 0, 2),
        FixedNumber::new(4, 0, 4),
        FixedNumber::new(5, 0, 5),
        FixedNumber::new(7, 0, 6),
        FixedNumber::new(6, 8, 9),
        FixedNumber::new(8, 5, 8),
      ]
    );
  let mut solver = Solver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = TurbotFish.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::TurbotFish);
  assert_eq!(step.areas, vec![ Area::Row(0), Area::Column(0) ]);
  assert_eq!(step.cells, vec![
    CellPosition::new(0, 2),
    CellPosition::new(0, 6),
    CellPosition::new(2, 0),
    CellPosition::new(6, 0),
  ]);
  assert_eq!(step.values, vec![ 8 ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(6, 6) ]);
  let initial_candidates = solver.candidates[6][6].clone();
  assert!(initial_candidates.contains(&8));
  assert_eq!(initial_candidates.len(), 8);

  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[6][6];
  assert!(!final_candidates.contains(&8));
  assert_eq!(final_candidates.len(), 7);
}

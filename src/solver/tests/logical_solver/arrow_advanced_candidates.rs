use crate::{types::{FixedNumber, SudokuConstraints, CellPosition, Rule, Area, Arrow}, solver::{logical_solver::{arrow_candidates::ArrowCandidates, candidates::Candidates, arrow_advanced_candidates::ArrowAdvancedCandidates, technique::Technique}, Solver}};

#[test]
fn check_arrow_advanced_candidates() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 1), FixedNumber::new(1, 0, 4), FixedNumber::new(2, 0, 5),
        FixedNumber::new(3, 0, 6), FixedNumber::new(4, 0, 7), FixedNumber::new(5, 0, 8),
        FixedNumber::new(6, 0, 9), FixedNumber::new(6, 7, 5), FixedNumber::new(6, 8, 6),
      ]
    )
    .with_arrows(
      vec![
        Arrow {
          circle_cells: vec![ CellPosition::new(7, 7) ],
          arrow_cells: vec![
            CellPosition::new(8, 8), CellPosition::new(8, 7), CellPosition::new(8, 6),
          ],
        },
      ]
    );
  let mut solver = Solver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = ArrowCandidates.run(&solver);
  solver.apply_rules(&steps);

  let steps = ArrowAdvancedCandidates.run(&solver);
  assert_eq!(steps.len(), 1);
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::ArrowAdvancedCandidates);
  assert_eq!(step.values, vec![9]);
  assert_eq!(step.areas, vec![ Area::Arrow(0) ]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(7, 7) ]);
  assert!(solver.candidates[7][7].contains(&9));

  solver.apply_rule(&mut step);
  assert!(!solver.candidates[7][7].contains(&9));
}

use crate::{types::{SudokuConstraints, CellPosition, Rule, Area, Arrow, FixedNumber}, solver::{Solver, logical_solver::{candidates::Candidates, technique::Technique, arrow_candidates::ArrowCandidates}}};
use itertools::Itertools;

#[test]
fn check_arrow_candidates_simple() {
  let grid_size = 9;
  let mut constraints = SudokuConstraints::new(grid_size, vec![]);
  constraints.arrows = vec![
    Arrow {
      arrow_cells: vec![ CellPosition::new(0, 0), CellPosition::new(0, 1) ],
      circle_cells: vec![ CellPosition::new(0, 2) ],
    },
  ];
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  let steps = ArrowCandidates.run(&solver);
  assert_eq!(steps.len(), 3);

  let step = &steps[0];
  assert_eq!(step.rule, Rule::ArrowCandidates);
  assert_eq!(step.affected_cells[0], CellPosition::new(0, 0));
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![9]);
  assert_eq!(step.areas, vec![ Area::Arrow(0) ]);
  assert!(solver.candidates[0][0].contains(&9));
  solver.apply_rule(&step);
  assert!(!solver.candidates[0][0].contains(&9));
  assert_eq!(solver.candidates[0][0].len(), 8);

  let step = &steps[1];
  assert_eq!(step.rule, Rule::ArrowCandidates);
  assert_eq!(step.affected_cells[0], CellPosition::new(0, 1));
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![9]);
  assert_eq!(step.areas, vec![ Area::Arrow(0) ]);
  assert!(solver.candidates[0][1].contains(&9));
  solver.apply_rule(&step);
  assert!(!solver.candidates[0][1].contains(&9));
  assert_eq!(solver.candidates[0][1].len(), 8);

  let step = &steps[2];
  assert_eq!(step.rule, Rule::ArrowCandidates);
  assert_eq!(step.affected_cells[0], CellPosition::new(0, 2));
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![1, 2]);
  assert_eq!(step.areas, vec![ Area::Arrow(0) ]);
  assert!(solver.candidates[0][2].contains(&1));
  solver.apply_rule(&step);
  assert!(!solver.candidates[0][2].contains(&1));
  assert_eq!(solver.candidates[0][2].len(), 7);
}

#[test]
fn check_arrow_candidates_independent_arrow_cells_fixed_circle() {
  let grid_size = 6;
  let fixed_numbers = vec![ FixedNumber::new(1, 4, 2) ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.arrows = vec![
    Arrow {
      arrow_cells: vec![ CellPosition::new(1, 2), CellPosition::new(2, 3) ],
      circle_cells: vec![ CellPosition::new(1, 4) ],
    },
  ];
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  let steps = ArrowCandidates.run(&solver);
  assert_eq!(steps.len(), 2);

  let step = &steps[0];
  assert_eq!(step.rule, Rule::ArrowCandidates);
  assert_eq!(step.affected_cells[0], CellPosition::new(1, 2));
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![3, 4, 5, 6]);
  assert_eq!(step.areas, vec![ Area::Arrow(0) ]);
  assert!(solver.candidates[1][2].contains(&3));
  solver.apply_rule(&step);
  assert!(!solver.candidates[1][2].contains(&3));
  assert_eq!(solver.candidates[1][2].len(), 1);

  let step = &steps[1];
  assert_eq!(step.rule, Rule::ArrowCandidates);
  assert_eq!(step.affected_cells[0], CellPosition::new(2, 3));
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![2, 3, 4, 5, 6]);
  assert_eq!(step.areas, vec![ Area::Arrow(0) ]);
  assert!(solver.candidates[2][3].contains(&2));
  solver.apply_rule(&step);
  assert!(!solver.candidates[2][3].contains(&2));
  assert_eq!(solver.candidates[2][3].len(), 1);
}

#[test]
fn check_arrow_candidates_2_digit_circle_horizontal() {
  let grid_size = 9;
  let fixed_numbers = vec![ FixedNumber::new(6, 6, 5), FixedNumber::new(7, 7, 9) ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.arrows = vec![
    Arrow {
      arrow_cells: vec![
        CellPosition::new(2, 2), CellPosition::new(3, 3), CellPosition::new(4, 4),
        CellPosition::new(5, 5), CellPosition::new(6, 6), CellPosition::new(7, 7),
      ],
      circle_cells: vec![ CellPosition::new(1, 1), CellPosition::new(1, 0) ], // reversed!
    },
  ];
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  let steps = ArrowCandidates.run(&solver);
  assert_eq!(steps.len(), 1);

  let step = &steps[0];
  assert_eq!(step.rule, Rule::ArrowCandidates);
  assert_eq!(step.affected_cells[0], CellPosition::new(1, 0));
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![1, 5, 6, 7, 8, 9]);
  assert_eq!(step.areas, vec![ Area::Arrow(0) ]);
  assert!(solver.candidates[1][0].contains(&5));
  solver.apply_rule(&step);
  assert!(!solver.candidates[1][0].contains(&5));
  assert_eq!(solver.candidates[1][0].len(), 3);
}

#[test]
fn check_arrow_candidates_2_digit_circle_vertical() {
  let grid_size = 9;
  let fixed_numbers = vec![ FixedNumber::new(3, 0, 1) ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.arrows = vec![
    Arrow {
      circle_cells: vec![ CellPosition::new(3, 2), CellPosition::new(4, 2) ],
      arrow_cells: vec![
        CellPosition::new(4, 3), CellPosition::new(5, 3), CellPosition::new(5, 4),
        CellPosition::new(6, 5),
      ],
    },
  ];
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  let steps = ArrowCandidates.run(&solver);
  assert_eq!(steps.len(), 1);

  let step = &steps[0];
  assert_eq!(step.rule, Rule::ArrowCandidates);
  assert_eq!(step.affected_cells[0], CellPosition::new(3, 2));
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![4, 5, 6, 7, 8, 9]);
  assert_eq!(step.areas, vec![ Area::Arrow(0) ]);
  assert!(solver.candidates[3][2].contains(&4));
  solver.apply_rule(&step);
  assert!(!solver.candidates[3][2].contains(&4));
  assert_eq!(solver.candidates[3][2].len(), 2);
}

#[test]
fn check_arrow_candidates_long() {
  let grid_size = 9;
  let fixed_numbers = vec![ FixedNumber::new(0, 3, 1), FixedNumber::new(5, 8, 1) ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.arrows = vec![
    Arrow {
      arrow_cells: vec![
        CellPosition::new(1, 1), CellPosition::new(2, 2), CellPosition::new(3, 3),
        CellPosition::new(4, 4), CellPosition::new(5, 5),
      ],
      circle_cells: vec![ CellPosition::new(0, 0) ],
    },
  ];
  let mut solver = Solver::new(constraints, None);

  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());
  let steps = ArrowCandidates.run(&solver);
  assert_eq!(steps.len(), 6);

  let step = &steps[0];
  assert_eq!(step.rule, Rule::ArrowCandidates);
  assert_eq!(step.affected_cells[0], CellPosition::new(1, 1));
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![3, 4, 5, 6, 7, 8, 9]);
  assert_eq!(step.areas, vec![ Area::Arrow(0) ]);
  assert!(solver.candidates[1][1].contains(&3));
  solver.apply_rule(&step);
  assert!(!solver.candidates[1][1].contains(&3));
  assert_eq!(solver.candidates[1][1].len(), 2);

  let step = &steps[1];
  assert_eq!(step.rule, Rule::ArrowCandidates);
  assert_eq!(step.affected_cells[0], CellPosition::new(2, 2));
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![3, 4, 5, 6, 7, 8, 9]);
  assert_eq!(step.areas, vec![ Area::Arrow(0) ]);
  assert!(solver.candidates[2][2].contains(&3));
  solver.apply_rule(&step);
  assert!(!solver.candidates[2][2].contains(&3));
  assert_eq!(solver.candidates[2][2].len(), 2);

  let step = &steps[2];
  assert_eq!(step.rule, Rule::ArrowCandidates);
  assert_eq!(step.affected_cells[0], CellPosition::new(3, 3));
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![4, 5, 6, 7, 8, 9]);
  assert_eq!(step.areas, vec![ Area::Arrow(0) ]);
  assert!(solver.candidates[3][3].contains(&4));
  solver.apply_rule(&step);
  assert!(!solver.candidates[3][3].contains(&4));
  assert_eq!(solver.candidates[3][3].len(), 2);
  
  let step = &steps[3];
  assert_eq!(step.rule, Rule::ArrowCandidates);
  assert_eq!(step.affected_cells[0], CellPosition::new(4, 4));
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![2, 3, 4, 5, 6, 7, 8, 9]);
  assert_eq!(step.areas, vec![ Area::Arrow(0) ]);
  assert!(solver.candidates[4][4].contains(&2));
  solver.apply_rule(&step);
  assert!(!solver.candidates[4][4].contains(&2));
  assert_eq!(solver.candidates[4][4].len(), 1);

  let step = &steps[4];
  assert_eq!(step.rule, Rule::ArrowCandidates);
  assert_eq!(step.affected_cells[0], CellPosition::new(5, 5));
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![4, 5, 6, 7, 8, 9]);
  assert_eq!(step.areas, vec![ Area::Arrow(0) ]);
  assert!(solver.candidates[5][5].contains(&4));
  solver.apply_rule(&step);
  assert!(!solver.candidates[5][5].contains(&4));
  assert_eq!(solver.candidates[5][5].len(), 2);

  let step = &steps[5];
  assert_eq!(step.rule, Rule::ArrowCandidates);
  assert_eq!(step.affected_cells[0], CellPosition::new(0, 0));
  assert_eq!(step.values.iter().sorted().copied().collect_vec(), vec![2, 3, 4, 5, 6, 7, 8]);
  assert_eq!(step.areas, vec![ Area::Arrow(0) ]);
  assert!(solver.candidates[0][0].contains(&8));
  solver.apply_rule(&step);
  assert!(!solver.candidates[0][0].contains(&8));
  assert_eq!(solver.candidates[0][0].len(), 1);
}

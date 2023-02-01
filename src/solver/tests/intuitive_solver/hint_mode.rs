use crate::{types::{SudokuConstraints, CellPosition, SolutionType, KropkiDot}, solver::Solver};

#[test]
fn check_hint_mode() {
  let grid_size = 4;
  let mut constraints = SudokuConstraints::new(grid_size, vec![]);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(0, 1)),
    KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(1, 0)),
    KropkiDot::consecutive(CellPosition::new(3, 2), CellPosition::new(3, 3)),
    KropkiDot::double(CellPosition::new(1, 1), CellPosition::new(1, 2)),
    KropkiDot::double(CellPosition::new(1, 2), CellPosition::new(1, 3)),
    KropkiDot::double(CellPosition::new(2, 0), CellPosition::new(2, 1)),
    KropkiDot::double(CellPosition::new(2, 1), CellPosition::new(2, 2)),
  ];

  let mut solver = Solver::new(constraints, None).with_hint_mode();
  let result = solver.intuitive_solve();

  assert_eq!(result.solution_type, SolutionType::Partial);
  // at most 5 steps (1 + 3 kropki + single)
  assert!(result.steps.len() <= 5);

  let last_step = result.steps.last().unwrap();
  assert!(last_step.is_grid_step());
  let cell = last_step.cells[0];

  let previous_step = &result.steps[result.steps.len() - 2];
  assert!(previous_step.affected_cells.contains(&cell));
}

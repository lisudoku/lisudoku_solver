use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{TechniqueDefinition, thermo_naked_single::ThermoNakedSingle}}, types::{CellPosition, FixedNumber, SudokuConstraints}};

#[test]
fn check_thermo_steps() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      [
        FixedNumber::new(4, 3, 7),
        FixedNumber::new(6, 3, 3),
      ]
    )
    .with_thermos(
      [
        crate::types::Thermo(vec![
          CellPosition::new(8, 0),
          CellPosition::new(7, 0),
          CellPosition::new(6, 0),
          CellPosition::new(5, 0),
          CellPosition::new(4, 0),
          CellPosition::new(3, 0),
          CellPosition::new(2, 0),
        ]),
      ]
    );
  let mut solver = LogicalSolver::new(constraints);

  let steps = ThermoNakedSingle.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.cells, vec![ CellPosition::new(6, 0) ]);
  assert_eq!(step.values, vec![4]);
  let cell = step.cells[0];
  let step_value = step.values[0];
  let initial_value = solver.state.value_at(&cell);
  assert!(initial_value == 0);

  solver.apply_step(&mut step);

  let final_value = solver.state.value_at(&cell);
  assert!(final_value == step_value);
}

#[test]
fn check_full_thermo() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      [
        FixedNumber::new(0, 0, 1),
        FixedNumber::new(1, 0, 2),
        FixedNumber::new(2, 0, 3),
        FixedNumber::new(3, 0, 4),
      ]
    )
    .with_thermos(
      [
        crate::types::Thermo(vec![
          CellPosition::new(0, 0),
          CellPosition::new(1, 0),
          CellPosition::new(2, 0),
          CellPosition::new(3, 0),
        ]),
      ]
    );
  let solver = LogicalSolver::new(constraints);

  let steps = ThermoNakedSingle.run(&solver);
  assert!(steps.is_empty());
}

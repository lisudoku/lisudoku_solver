use crate::{solver::constraints::{Constraint, ConstraintBuilder, odd_even::{OddEvenBaseConstraint, OddEvenBaseLogic}}, types::{CellPosition, SudokuConstraints}};
use std::slice::Iter;

#[derive(Clone, Default, Debug)]
pub(crate) struct EvenConstraintLogic {
  even_cells: Vec<CellPosition>,
  even_cells_by_cell: Vec<Vec<bool>>,
}

impl OddEvenBaseLogic for EvenConstraintLogic {
  fn cells(&self) -> Iter<'_, CellPosition> { self.even_cells.iter() }
  
  fn is_value_valid(&self, value: u32) -> bool { value % 2 == 0 }

  fn is_constraint_in_cell(&self, cell: &CellPosition) -> bool { self.even_cells_by_cell[cell.row][cell.col] }
}

pub(crate) struct EvenConstraintBuilder;

impl ConstraintBuilder for EvenConstraintBuilder {
  fn from_input(&self, constraints: &SudokuConstraints) -> Option<Constraint> {
    let even_cells = constraints.even_cells.clone().unwrap_or_default();

    if even_cells.is_empty() {
      return None
    }

    let mut even_cells_by_cell = vec![ vec![ false; constraints.grid_size ]; constraints.grid_size ];
    for cell in &even_cells {
      even_cells_by_cell[cell.row][cell.col] = true;
    }

    Some(Constraint::Even(OddEvenBaseConstraint(
      EvenConstraintLogic{
        even_cells,
        even_cells_by_cell,
      }
    )))
  }
}

pub(crate) static EVEN_CONSTRAINT_BUILDER: EvenConstraintBuilder = EvenConstraintBuilder;

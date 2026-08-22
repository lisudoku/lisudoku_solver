use crate::{solver::constraints::{Constraint, ConstraintBuilder, odd_even::{OddEvenBaseConstraint, OddEvenBaseLogic}}, types::{CellPosition, SudokuConstraints}};
use std::slice::Iter;

#[derive(Clone, Default, Debug)]
pub(crate) struct OddConstraintLogic {
  odd_cells: Vec<CellPosition>,
  odd_cells_by_cell: Vec<Vec<bool>>,
}

impl OddEvenBaseLogic for OddConstraintLogic {
  fn cells(&self) -> Iter<'_, CellPosition> { self.odd_cells.iter() }
  
  fn is_value_valid(&self, value: u32) -> bool { value % 2 == 1 }

  fn is_constraint_in_cell(&self, cell: &CellPosition) -> bool { self.odd_cells_by_cell[cell.row][cell.col] }
}

pub(crate) struct OddConstraintBuilder;

impl ConstraintBuilder for OddConstraintBuilder {
  fn from_input(&self, constraints: &SudokuConstraints) -> Option<Constraint> {
    let odd_cells = constraints.odd_cells.clone().unwrap_or_default();

    if odd_cells.is_empty() {
      return None
    }

    let mut odd_cells_by_cell = vec![ vec![ false; constraints.grid_size ]; constraints.grid_size ];
    for cell in &odd_cells {
      odd_cells_by_cell[cell.row][cell.col] = true;
    }

    Some(Constraint::Odd(OddEvenBaseConstraint(
      OddConstraintLogic{
        odd_cells,
        odd_cells_by_cell,
      }
    )))
  }
}

pub(crate) static ODD_CONSTRAINT_BUILDER: OddConstraintBuilder = OddConstraintBuilder;

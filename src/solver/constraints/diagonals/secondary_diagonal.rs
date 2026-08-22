use crate::{solver::constraints::{Constraint, ConstraintBuilder, diagonals::{DiagonalBaseConstraint, DiagonalBaseLogic}}, types::{Area, CellPosition, SudokuConstraints}};

#[derive(Clone, Default, Debug)]
pub struct SecondaryDiagonalLogic {
  grid_size: usize,
}

impl DiagonalBaseLogic for SecondaryDiagonalLogic {
  const DIAGONAL_AREA: Area = Area::SecondaryDiagonal;

  fn grid_size(&self) -> usize { self.grid_size }

  fn diagonal_cell_at_index(&self, index: usize) -> CellPosition {
    CellPosition::new(index,  self.grid_size() - 1 - index)
  }

  fn is_cell_on_diagonal(&self, cell: &CellPosition) -> bool {
    cell.row == self.grid_size() - 1 - cell.col
  }
}

pub struct SecondaryDiagonalConstraintBuilder;

impl ConstraintBuilder for SecondaryDiagonalConstraintBuilder {
  fn from_input(&self, constraints: &SudokuConstraints) -> Option<Constraint> {
    if !constraints.secondary_diagonal.unwrap_or_default() {
      return None
    }

    Some(Constraint::SecondaryDiagonal(DiagonalBaseConstraint(SecondaryDiagonalLogic{
      grid_size: constraints.grid_size,
    })))
  }
}

pub static SECONDARY_DIAGONAL_CONSTRAINT_BUILDER: SecondaryDiagonalConstraintBuilder = SecondaryDiagonalConstraintBuilder;

use crate::{solver::constraints::{Constraint, ConstraintBuilder, diagonals::{DiagonalBaseConstraint, DiagonalBaseLogic}}, types::{Area, CellPosition, SudokuConstraints}};

#[derive(Clone, Default, Debug)]
pub struct PrimaryDiagonalLogic {
  grid_size: usize,
}

impl DiagonalBaseLogic for PrimaryDiagonalLogic {
  const DIAGONAL_AREA: Area = Area::PrimaryDiagonal;

  fn grid_size(&self) -> usize { self.grid_size }

  fn diagonal_cell_at_index(&self, index: usize) -> CellPosition {
    CellPosition::new(index, index)
  }

  fn is_cell_on_diagonal(&self, cell: &CellPosition) -> bool {
    cell.row == cell.col
  }
}

pub struct PrimaryDiagonalConstraintBuilder;

impl ConstraintBuilder for PrimaryDiagonalConstraintBuilder {
  fn from_input(&self, constraints: &SudokuConstraints) -> Option<Constraint> {
    if !constraints.primary_diagonal.unwrap_or_default() {
      return None
    }

    Some(Constraint::PrimaryDiagonal(DiagonalBaseConstraint(PrimaryDiagonalLogic{
      grid_size: constraints.grid_size,
    })))
  }
}

pub static PRIMARY_DIAGONAL_CONSTRAINT_BUILDER: PrimaryDiagonalConstraintBuilder = PrimaryDiagonalConstraintBuilder;

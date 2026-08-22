use crate::{solver::{checker::SolvedState, constraints::{CandidateMode, ConstraintDefinition, ConstraintDefinitionExt, PeerMode, context::{ConstraintCellPeersContext}, diagonals::{primary_diagonal::PrimaryDiagonalLogic, secondary_diagonal::SecondaryDiagonalLogic}}, types::CandidateSet}, types::{Area, CellPosition, Grid}};
use derive_more::Deref;

mod primary_diagonal;
mod secondary_diagonal;

pub(crate) use primary_diagonal::PRIMARY_DIAGONAL_CONSTRAINT_BUILDER;
pub(crate) type PrimaryDiagonalConstraint = DiagonalBaseConstraint<PrimaryDiagonalLogic>;
pub(crate) use secondary_diagonal::SECONDARY_DIAGONAL_CONSTRAINT_BUILDER;
pub(crate) type SecondaryDiagonalConstraint = DiagonalBaseConstraint<SecondaryDiagonalLogic>;

pub(crate) trait DiagonalBaseLogic {
  const DIAGONAL_AREA: Area;

  fn grid_size(&self) -> usize;

  fn diagonal_cell_at_index(&self, index: usize) -> CellPosition;

  fn is_cell_on_diagonal(&self, cell: &CellPosition) -> bool;
}

#[derive(Clone, Debug, Deref)]
pub(crate) struct DiagonalBaseConstraint<L: DiagonalBaseLogic>(L);

impl<L: DiagonalBaseLogic> ConstraintDefinition for DiagonalBaseConstraint<L> {
  fn check_grid_special_validity(&self, _grid: &Grid, _candidates: &Vec<Vec<CandidateSet>>) -> SolvedState {
    SolvedState::solved()
  }

  fn region_areas(&self) -> Vec<Area> {
    vec![L::DIAGONAL_AREA]
  }

  fn area_cells(&self, area: &Area) -> Vec<CellPosition> {
    if area != &L::DIAGONAL_AREA {
      return vec![]
    }

    (0..self.grid_size()).map(|index| self.diagonal_cell_at_index(index)).collect()
  }

  fn cell_region_areas(&self, cell: &CellPosition, peer_mode: PeerMode, _candidate_mode: CandidateMode) -> Vec<Area> {
    if peer_mode == PeerMode::Classic {
      return vec![]
    }

    if self.is_cell_on_diagonal(cell) {
      return self.region_areas()
    }

    vec![]
  }

  fn restrict_cell_special_candidates(&self, _cell: &CellPosition, _grid: &Grid, _candidates: &mut CandidateSet) {}

  fn cell_peers(&self, cell: &CellPosition, ctx: &ConstraintCellPeersContext) -> Vec<CellPosition> {
    if ctx.peer_mode == PeerMode::Classic {
      return vec![]
    }

    self.default_cell_peers(cell, ctx.peer_mode, ctx.candidate_mode)
  }
}

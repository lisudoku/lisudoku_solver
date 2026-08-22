use crate::{solver::{checker::SolvedState, constraints::{CandidateMode, ConstraintDefinition, PeerMode, context::{ConstraintCellPeersContext}, odd_even::{even::EvenConstraintLogic, odd::OddConstraintLogic}}, types::CandidateSet}, types::{Area, CellPosition, Grid, InvalidStateReason, InvalidStateType}};
use std::slice::Iter;
use derive_more::Deref;

mod odd;
mod even;

pub(crate) use odd::ODD_CONSTRAINT_BUILDER;
pub(crate) type OddConstraint = OddEvenBaseConstraint<OddConstraintLogic>;
pub(crate) use even::EVEN_CONSTRAINT_BUILDER;
pub(crate) type EvenConstraint = OddEvenBaseConstraint<EvenConstraintLogic>;

pub(crate) trait OddEvenBaseLogic {
  fn cells(&self) -> Iter<'_, CellPosition>;
  
  fn is_value_valid(&self, value: u32) -> bool;

  fn is_constraint_in_cell(&self, cell: &CellPosition) -> bool;
}

#[derive(Clone, Debug, Deref)]
pub(crate) struct OddEvenBaseConstraint<L: OddEvenBaseLogic>(L);

impl<L: OddEvenBaseLogic> ConstraintDefinition for OddEvenBaseConstraint<L> {
  fn check_grid_special_validity(&self, grid: &Grid, _candidates: &Vec<Vec<CandidateSet>>) -> SolvedState {
    for cell in self.cells() {
      let value = grid[cell.row][cell.col];
      if value != 0 && !self.is_value_valid(value) {
        return SolvedState::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::CellInvalidValue,
            area: Area::Cell(cell.row, cell.col),
            values: vec![value],
          }
        )
      }
    }

    SolvedState::solved()
  }

  fn region_areas(&self) -> Vec<Area> { vec![] }

  fn area_cells(&self, _area: &Area) -> Vec<CellPosition> { vec![] }

  fn cell_region_areas(&self, _cell: &CellPosition, _peer_mode: PeerMode, _candidate_mode: CandidateMode) -> Vec<Area> { vec![] }

  fn restrict_cell_special_candidates(&self, cell: &CellPosition, _grid: &Grid, candidates: &mut CandidateSet) {
    if self.is_constraint_in_cell(cell) {
      candidates.retain(|value| self.is_value_valid(*value));
    }
  }

  fn cell_peers(&self, _cell: &CellPosition, _ctx: &ConstraintCellPeersContext) -> Vec<CellPosition> { vec![] }
}

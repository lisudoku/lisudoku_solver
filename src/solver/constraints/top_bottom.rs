use crate::{solver::{checker::SolvedState, constraints::{CandidateMode, Constraint, ConstraintBuilder, ConstraintDefinition, PeerMode, context::{ConstraintCellPeersContext}}, logical_solver::{top_bottom_candidates::TopBottomCandidates}, types::CandidateSet}, types::{Area, CellPosition, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints}};

#[derive(Clone, Default, Debug)]
pub struct TopBottomConstraint;

impl ConstraintDefinition for TopBottomConstraint {
  fn check_grid_special_validity(&self, grid: &Grid, candidates: &Vec<Vec<CandidateSet>>) -> SolvedState {
    let valid = TopBottomCandidates::new(true).run_with_data(grid, candidates).is_empty();

    if valid {
      return SolvedState::solved()
    }

    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Grid,
        values: vec![],
      }
    )
  }

  fn region_areas(&self) -> Vec<Area> { vec![] }

  fn area_cells(&self, _area: &Area) -> Vec<CellPosition> { vec![] }

  fn cell_region_areas(&self, _cell: &CellPosition, _peer_mode: PeerMode, _candidate_mode: CandidateMode) -> Vec<Area> { vec![] }

  fn restrict_cell_special_candidates(&self, _cell: &CellPosition, _grid: &Grid, _candidates: &mut CandidateSet) {
    // Do not enforce candidates directly, use an explicit rule for that
  }

  fn cell_peers(&self, _cell: &CellPosition, _ctx: &ConstraintCellPeersContext) -> Vec<CellPosition> {
    vec![]
  }
}

pub struct TopBottomConstraintBuilder;

impl ConstraintBuilder for TopBottomConstraintBuilder {
  fn from_input(&self, constraints: &SudokuConstraints) -> Option<Constraint> {
    if !constraints.top_bottom.unwrap_or_default() {
      return None
    }

    Some(Constraint::TopBottom(TopBottomConstraint{}))
  }
}

pub static TOP_BOTTOM_CONSTRAINT_BUILDER: TopBottomConstraintBuilder = TopBottomConstraintBuilder;

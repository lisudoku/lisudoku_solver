use crate::{solver::{GridCheckResult, constraints::{CandidateMode, Constraint, ConstraintBuilder, ConstraintDefinition, PeerMode, context::{ConstraintCellPeersContext}}, logical::techniques::{top_bottom_candidate_update::TopBottomCandidateUpdate}, types::{CandidateGrid, CandidateSet}}, types::{Area, CellPosition, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints}};

#[derive(Clone, Default, Debug)]
pub(crate) struct TopBottomConstraint;

impl ConstraintDefinition for TopBottomConstraint {
  fn check_grid_special_validity(&self, grid: &Grid, candidates: &CandidateGrid) -> GridCheckResult {
    let valid = TopBottomCandidateUpdate::new(true).run_with_data(grid, candidates).is_empty();

    if valid {
      return GridCheckResult::solved()
    }

    GridCheckResult::unsolved(
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
    // Do not enforce candidates directly, use an explicit technique for that
  }

  fn cell_peers(&self, _cell: &CellPosition, _ctx: &ConstraintCellPeersContext) -> Vec<CellPosition> {
    vec![]
  }
}

pub(crate) struct TopBottomConstraintBuilder;

impl ConstraintBuilder for TopBottomConstraintBuilder {
  fn from_input(&self, constraints: &SudokuConstraints) -> Option<Constraint> {
    if !constraints.top_bottom.unwrap_or_default() {
      return None
    }

    Some(TopBottomConstraint{}.into())
  }
}

pub(crate) static TOP_BOTTOM_CONSTRAINT_BUILDER: TopBottomConstraintBuilder = TopBottomConstraintBuilder;

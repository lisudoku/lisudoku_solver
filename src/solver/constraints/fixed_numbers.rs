use crate::{solver::{checker::SolvedState, constraints::{CandidateMode, Constraint, ConstraintBuilder, ConstraintDefinition, PeerMode, context::{ConstraintCellPeersContext}}, types::CandidateSet}, types::{Area, CellPosition, FixedNumber, Grid, SudokuConstraints}};

#[derive(Clone, Default, Debug)]
pub struct FixedNumbersConstraint {
  #[allow(dead_code)]
  pub fixed_numbers: Vec<FixedNumber>,
}

impl ConstraintDefinition for FixedNumbersConstraint {
  fn check_grid_special_validity(&self, _grid: &Grid, _candidates: &Vec<Vec<CandidateSet>>) -> SolvedState {
    SolvedState::solved()
  }

  fn region_areas(&self) -> Vec<Area> { vec![] }

  fn area_cells(&self, _area: &Area) -> Vec<CellPosition> { vec![] }

  fn cell_region_areas(&self, _cell: &CellPosition, _peer_mode: PeerMode, _candidate_mode: CandidateMode) -> Vec<Area> { vec![] }

  fn restrict_cell_special_candidates(&self, _cell: &CellPosition, _grid: &Grid, _candidates: &mut CandidateSet) {}

  fn cell_peers(&self, _cell: &CellPosition, _ctx: &ConstraintCellPeersContext) -> Vec<CellPosition> { vec![] }
}

impl FixedNumbersConstraint {
  pub fn build_initial_grid_from_fixed_numbers(fixed_numbers: &Vec<FixedNumber>, grid_size: usize) -> Grid {
    let mut initial_grid = vec![ vec![ 0; grid_size ]; grid_size ];
    for fixed_number in fixed_numbers.iter() {
      initial_grid[fixed_number.position.row][fixed_number.position.col] = fixed_number.value;
    }
    Grid(initial_grid)
  }
}

pub struct FixedNumbersConstraintBuilder;

impl ConstraintBuilder for FixedNumbersConstraintBuilder {
  fn from_input(&self, constraints: &SudokuConstraints) -> Option<Constraint> {
    if constraints.fixed_numbers.is_none() {
      return None
    }

    let fixed_numbers = constraints.fixed_numbers.clone().unwrap();
    Some(Constraint::FixedNumbers(FixedNumbersConstraint{
      fixed_numbers,
    }))
  }
}

pub static FIXED_NUMBERS_CONSTRAINT_BUILDER: FixedNumbersConstraintBuilder = FixedNumbersConstraintBuilder;

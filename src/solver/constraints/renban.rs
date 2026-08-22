use crate::{solver::{checker::SolvedState, constraints::{CandidateMode, Constraint, ConstraintBuilder, ConstraintDefinition, ConstraintDefinitionExt, PeerMode, context::{ConstraintCellPeersContext}}, types::CandidateSet}, types::{Area, CellPosition, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints, Renban}};

#[derive(Clone, Default, Debug)]
pub struct RenbanConstraint {
  pub renbans: Vec<Renban>,
  renbans_by_cell: Vec<Vec<Vec<usize>>>,
}

impl ConstraintDefinition for RenbanConstraint {
  fn check_grid_special_validity(&self, grid: &Grid, _candidates: &Vec<Vec<CandidateSet>>) -> SolvedState {
    for renban_index in 0..self.renbans.len() {
      let mut min_value: u32 = u32::MAX;
      let mut max_value: u32 = 0;
      let mut any_zero = false;
      let area = Area::Renban(renban_index);
      let cells = self.area_cells(&area);
      for cell in &cells {
        let value = grid[cell.row][cell.col];
        if value == 0 {
          any_zero = true;
        }
        if value > 0 && value < min_value {
          min_value = value;
        }
        if value > max_value {
          max_value = value;
        }
      }

      if max_value == 0 {
        // All zeroes is valid (well, not necessarily, but it's not the checker's job)
        continue
      }

      if (!any_zero && max_value - min_value + 1 != cells.len() as u32) ||
          (any_zero && max_value - min_value + 1 > cells.len() as u32) {
        return SolvedState::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::AreaConstraint,
            area: area.clone(),
            values: vec![],
          }
        )
      }
    }

    SolvedState::solved()
  }

  fn region_areas(&self) -> Vec<Area> {
    (0..self.renbans.len()).map(|renban_index| Area::Renban(renban_index)).collect()
  }

  fn area_cells(&self, area: &Area) -> Vec<CellPosition> {
    let Area::Renban(renban_index) = area else { return vec![] };

    self.renbans[*renban_index].0.clone()
  }

  fn cell_region_areas(&self, cell: &CellPosition, peer_mode: PeerMode, _candidate_mode: CandidateMode) -> Vec<Area> {
    if peer_mode == PeerMode::Classic {
      return vec![]
    }

    self.renbans_by_cell[cell.row][cell.col].iter().map(|&renban_index|
      Area::Renban(renban_index)
    ).collect()
  }

  fn restrict_cell_special_candidates(&self, _cell: &CellPosition, _grid: &Grid, _candidates: &mut CandidateSet) {}

  fn cell_peers(&self, cell: &CellPosition, ctx: &ConstraintCellPeersContext) -> Vec<CellPosition> {
    if ctx.peer_mode == PeerMode::Classic {
      return vec![]
    }

    self.default_cell_peers(cell, ctx.peer_mode, ctx.candidate_mode)
  }
}

pub struct RenbanConstraintBuilder;

impl ConstraintBuilder for RenbanConstraintBuilder {
  fn from_input(&self, constraints: &SudokuConstraints) -> Option<Constraint> {
    let renbans = constraints.renbans.clone().unwrap_or_default();

    if renbans.is_empty() {
      return None
    }

    let mut renbans_by_cell = vec![ vec![ vec![]; constraints.grid_size ]; constraints.grid_size ];
    for (index, renban) in renbans.iter().enumerate() {
      for cell in renban.iter() {
        renbans_by_cell[cell.row][cell.col].push(index);
      }
    }

    Some(Constraint::Renban(RenbanConstraint{
      renbans,
      renbans_by_cell,
    }))
  }
}

pub static RENBAN_CONSTRAINT_BUILDER: RenbanConstraintBuilder = RenbanConstraintBuilder;

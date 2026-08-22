use crate::{solver::{checker::SolvedState, constraints::{CandidateMode, Constraint, ConstraintBuilder, ConstraintDefinition, ConstraintDefinitionExt, PeerMode, context::{ConstraintCellPeersContext}}, types::CandidateSet}, types::{Area, CellPosition, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints, Thermo}};
use std::cmp::{min, max};

#[derive(Clone, Default, Debug)]
pub struct ThermoConstraint {
  pub thermos: Vec<Thermo>,
  thermos_by_cell: Vec<Vec<Vec<usize>>>,
}

impl ConstraintDefinition for ThermoConstraint {
  // TODO: maybe check candidates
  fn check_grid_special_validity(&self, grid: &Grid, _candidates: &Vec<Vec<CandidateSet>>) -> SolvedState {
    for thermo_index in 0..self.thermos.len() {
      let mut crt_max_value: u32 = 0;

      let thermo_area = Area::Thermo(thermo_index);
      let cells = self.area_cells(&thermo_area);
      for CellPosition { row, col } in cells {
        let value = grid[row][col];
        if value == 0 {
          continue
        }
        if value <= crt_max_value {
          return SolvedState::unsolved(
            InvalidStateReason {
              state_type: InvalidStateType::AreaConstraint,
              area: thermo_area,
              values: vec![crt_max_value, value],
            }
          )
        }
        crt_max_value = value
      }
    }

    SolvedState::solved()
  }

  fn region_areas(&self) -> Vec<Area> { vec![] }

  fn area_cells(&self, area: &Area) -> Vec<CellPosition> {
    let Area::Thermo(thermo_index) = area else { return vec![] };

    self.thermos[*thermo_index].0.clone()
  }

  fn cell_region_areas(&self, cell: &CellPosition, peer_mode: PeerMode, candidate_mode: CandidateMode) -> Vec<Area> {
    if candidate_mode == CandidateMode::Simple || peer_mode == PeerMode::Classic {
      return vec![]
    }

    self.thermos_by_cell[cell.row][cell.col].iter().map(|&thermo_index|
      Area::Thermo(thermo_index)
    ).collect()
  }

  fn restrict_cell_special_candidates(&self, cell: &CellPosition, grid: &Grid, candidates: &mut CandidateSet) {
    for area in self.cell_region_areas(cell, PeerMode::Full, CandidateMode::Full) {
      let cells = self.area_cells(&area);

      let mut after = false;
      let mut max_before = 0;
      let mut min_after = grid.len() as u32 + 1;

      for cell in cells {
        if cell.row == cell.row && cell.col == cell.col {
          after = true;
          continue
        }
        let value = grid[cell.row][cell.col];
        if value == 0 {
          continue
        }

        if after {
          min_after = min(min_after, value);
        } else {
          max_before = max(max_before, value);
        }
      }

      candidates.retain(|value| *value > max_before && *value < min_after);
    }
  }

  fn cell_peers(&self, cell: &CellPosition, ctx: &ConstraintCellPeersContext) -> Vec<CellPosition> {
    if ctx.candidate_mode == CandidateMode::Simple || ctx.peer_mode == PeerMode::Classic {
      return vec![]
    }

    self.default_cell_peers(cell, ctx.peer_mode, ctx.candidate_mode)
  }
}

pub struct ThermoConstraintBuilder;

impl ConstraintBuilder for ThermoConstraintBuilder {
  fn from_input(&self, constraints: &SudokuConstraints) -> Option<Constraint> {
    let thermos = constraints.thermos.clone().unwrap_or_default();

    if thermos.is_empty() {
      return None
    }

    let mut thermos_by_cell = vec![ vec![ vec![]; constraints.grid_size ]; constraints.grid_size ];
    for (index, thermo) in thermos.iter().enumerate() {
      for cell in thermo.iter() {
        thermos_by_cell[cell.row][cell.col].push(index);
      }
    }

    Some(Constraint::Thermo(ThermoConstraint{
      thermos,
      thermos_by_cell,
    }))
  }
}

pub static THERMO_CONSTRAINT_BUILDER: ThermoConstraintBuilder = ThermoConstraintBuilder;

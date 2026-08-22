use crate::{solver::{checker::SolvedState, constraints::{CandidateMode, Constraint, ConstraintBuilder, ConstraintDefinition, ConstraintDefinitionExt, PeerMode, context::{ConstraintCellPeersContext}}, types::CandidateSet}, types::{Area, CellPosition, Grid, InvalidStateReason, InvalidStateType, KillerCage, SudokuConstraints}};

#[derive(Clone, Default, Debug)]
pub struct KillerConstraint {
  pub killer_cages: Vec<KillerCage>,
  killer_cages_by_cell: Vec<Vec<Vec<usize>>>,
}

impl ConstraintDefinition for KillerConstraint {
  fn check_grid_special_validity(&self, grid: &Grid, _candidates: &Vec<Vec<CandidateSet>>) -> SolvedState {
    for killer_cage_index in 0..self.killer_cages.len() {
      let killer_cage_area = Area::KillerCage(killer_cage_index);
      let cells = self.area_cells(&killer_cage_area);

      let mut sum: u32 = 0;
      let mut any_zero = false;
      for cell in cells {
        let value = grid[cell.row][cell.col];
        if value == 0 {
          any_zero = true;
        }
        sum += value;
      }

      let killer_cage = &self.killer_cages[killer_cage_index];
      let Some(killer_sum) = killer_cage.sum else { continue };

      if sum != killer_sum && !any_zero || sum > killer_sum {
        return SolvedState::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::AreaConstraint,
            area: killer_cage_area.clone(),
            values: vec![],
          }
        )
      }
    }

    SolvedState::solved()
  }

  fn region_areas(&self) -> Vec<Area> {
    (0..self.killer_cages.len()).map(|killer_cage_index| Area::KillerCage(killer_cage_index)).collect()
  }

  fn area_cells(&self, area: &Area) -> Vec<CellPosition> {
    let Area::KillerCage(killer_cage_index) = area else { return vec![] };

    self.killer_cages[*killer_cage_index].region.0.clone()
  }

  fn cell_region_areas(&self, cell: &CellPosition, peer_mode: PeerMode, _candidate_mode: CandidateMode) -> Vec<Area> {
    if peer_mode == PeerMode::Classic {
      return vec![]
    }

    self.killer_cages_by_cell[cell.row][cell.col].iter().map(|&killer_cage_index|
      Area::KillerCage(killer_cage_index)
    ).collect()
  }

  fn restrict_cell_special_candidates(&self, cell: &CellPosition, _grid: &Grid, candidates: &mut CandidateSet) {
    for area in self.cell_region_areas(cell, PeerMode::Full, CandidateMode::Full) {
      let Area::KillerCage(killer_cage_index) = area else { return };
      let killer_cage = &self.killer_cages[killer_cage_index];

      let Some(sum) = killer_cage.sum else { continue; };

      candidates.retain(|value| *value <= sum);
    }
  }

  fn cell_peers(&self, cell: &CellPosition, ctx: &ConstraintCellPeersContext) -> Vec<CellPosition> {
    if ctx.peer_mode == PeerMode::Classic {
      return vec![]
    }

    self.default_cell_peers(cell, ctx.peer_mode, ctx.candidate_mode)
  }
}

pub struct KillerConstraintBuilder;

impl ConstraintBuilder for KillerConstraintBuilder {
  fn from_input(&self, constraints: &SudokuConstraints) -> Option<Constraint> {
    if constraints.killer_cages.is_none() {
      return None
    }

    let killer_cages = constraints.killer_cages.clone().unwrap();
    let mut killer_cages_by_cell = vec![ vec![ vec![]; constraints.grid_size ]; constraints.grid_size ];
    for (index, killer_cage) in killer_cages.iter().enumerate() {
      for cell in killer_cage.region.iter() {
        killer_cages_by_cell[cell.row][cell.col].push(index);
      }
    }

    Some(Constraint::Killer(KillerConstraint{
      killer_cages,
      killer_cages_by_cell,
    }))
  }
}

pub static KILLER_CONSTRAINT_BUILDER: KillerConstraintBuilder = KillerConstraintBuilder;

use crate::{solver::{checker::SolvedState, constraints::{CandidateMode, Constraint, ConstraintBuilder, ConstraintDefinition, PeerMode, context::{ConstraintCellPeersContext}}, types::CandidateSet}, types::{Area, Arrow, CellPosition, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints}};

#[derive(Clone, Default, Debug)]
pub struct ArrowConstraint {
  pub arrows: Vec<Arrow>,
}

impl ArrowConstraint {
  fn arrow_arrow_sum(arrow: &Arrow, grid: &Grid) -> (u32, bool) {
    let mut sum: u32 = 0;
    let mut full = true;
    for &CellPosition { row, col } in &arrow.arrow_cells {
      sum += grid[row][col];
      if grid[row][col] == 0 {
        full = false;
      }
    }
    (sum, full)
  }

  fn arrow_circle_number(arrow: &Arrow, grid: &Grid) -> (u32, bool) {
    let mut value: u32 = 0;
    let mut full = true;
    for &CellPosition { row, col } in &arrow.circle_cells {
      value = 10 * value + grid[row][col];
      if grid[row][col] == 0 {
        full = false;
      }
    }
    (value, full)
  }
}

impl ConstraintDefinition for ArrowConstraint {
  fn check_grid_special_validity(&self, grid: &Grid, _candidates: &Vec<Vec<CandidateSet>>) -> SolvedState {
    for (arrow_index, arrow) in self.arrows.iter().enumerate() {
      let (arrow_sum, arrow_full) = Self::arrow_arrow_sum(arrow, grid);
      let (circle_number, circle_full) = Self::arrow_circle_number(arrow, grid);

      if arrow_full && circle_full {
        if arrow_sum == circle_number {
          return SolvedState::solved()
        }
        return SolvedState::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::AreaConstraint,
            area: Area::Arrow(arrow_index),
            values: vec![],
          }
        )
      }

      if circle_full {
        if arrow_sum <= circle_number {
          return SolvedState::solved()
        }

        return SolvedState::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::AreaConstraint,
            area: Area::Arrow(arrow_index),
            values: vec![],
          }
        )
      }
    }

    SolvedState::solved()
  }

  fn region_areas(&self) -> Vec<Area> { vec![] }

  fn area_cells(&self, area: &Area) -> Vec<CellPosition> {
    let Area::Arrow(arrow_index) = area else { return vec![] };

    let arrow = &self.arrows[*arrow_index];
    arrow.circle_cells.iter().chain(arrow.arrow_cells.iter()).copied().collect()
  }

  fn cell_region_areas(&self, _cell: &CellPosition, _peer_mode: PeerMode, _candidate_mode: CandidateMode) -> Vec<Area> {
    // Arrows are not regions
    vec![]
  }

  fn restrict_cell_special_candidates(&self, _cell: &CellPosition, _grid: &Grid, _candidates: &mut CandidateSet) {}

  fn cell_peers(&self, _cell: &CellPosition, _ctx: &ConstraintCellPeersContext) -> Vec<CellPosition> { vec![] }
}

pub struct ArrowConstraintBuilder;

impl ConstraintBuilder for ArrowConstraintBuilder {
  fn from_input(&self, constraints: &SudokuConstraints) -> Option<Constraint> {
    if constraints.arrows.is_none() {
      return None
    }

    let arrows = constraints.arrows
      .clone()
      .unwrap()
      .iter()
      .map(|arrow| {
        let mut arrow = arrow.clone();
        arrow.circle_cells.sort_by_key(|cell| *cell);
        arrow
      })
      .collect();

    Some(Constraint::Arrow(ArrowConstraint{
      arrows,
    }))
  }
}

pub static ARROW_CONSTRAINT_BUILDER: ArrowConstraintBuilder = ArrowConstraintBuilder;

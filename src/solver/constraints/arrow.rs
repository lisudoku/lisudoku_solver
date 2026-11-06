use crate::{solver::{GridCheckResult, constraints::{CandidateMode, Constraint, ConstraintBuilder, ConstraintDefinition, PeerMode, context::{ConstraintCellPeersContext}}, types::{CandidateGrid, CandidateSet}}, types::{Area, Arrow, CellPosition, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints}};

#[derive(Clone, Default, Debug)]
pub(crate) struct ArrowConstraint {
  pub(crate) arrows: Vec<Arrow>,
}

impl ArrowConstraint {
  fn arrow_sum(arrow: &Arrow, grid: &Grid) -> (u32, bool, usize) {
    let mut sum: u32 = 0;
    let mut used_cell_count = 0;
    for &CellPosition { row, col } in &arrow.arrow_cells {
      sum += grid[row][col];
      if grid[row][col] != 0 {
        used_cell_count += 1;
      }
    }
    (sum, used_cell_count == arrow.arrow_cells.len(), used_cell_count)
  }

  fn circle_number(arrow: &Arrow, grid: &Grid) -> (u32, bool, usize) {
    let mut value: u32 = 0;
    let mut used_cell_count = 0;
    for &CellPosition { row, col } in &arrow.circle_cells {
      value = 10 * value + grid[row][col];
      if grid[row][col] != 0 {
        used_cell_count += 1;
      }
    }
    (value, used_cell_count == arrow.circle_cells.len(), used_cell_count)
  }
}

impl ConstraintDefinition for ArrowConstraint {
  fn check_grid_special_validity(&self, grid: &Grid, _candidates: &CandidateGrid) -> GridCheckResult {
    for (arrow_index, arrow) in self.arrows.iter().enumerate() {
      let (arrow_sum, arrow_full, _) = Self::arrow_sum(arrow, grid);
      let (circle_number, circle_full, _) = Self::circle_number(arrow, grid);

      if arrow_full && circle_full {
        if arrow_sum == circle_number {
          return GridCheckResult::solved()
        }
        return GridCheckResult::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::AreaConstraint,
            area: Area::Arrow(arrow_index),
            values: vec![],
          }
        )
      }

      if circle_full {
        if arrow_sum <= circle_number {
          return GridCheckResult::solved()
        }

        return GridCheckResult::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::AreaConstraint,
            area: Area::Arrow(arrow_index),
            values: vec![],
          }
        )
      }
    }

    GridCheckResult::solved()
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

pub(crate) struct ArrowConstraintBuilder;

impl ConstraintBuilder for ArrowConstraintBuilder {
  fn from_input(&self, constraints: &SudokuConstraints) -> Option<Constraint> {
    if constraints.arrows.is_none() {
      return None
    }

    let arrows: Vec<Arrow> = constraints.arrows
      .clone()
      .unwrap()
      .iter()
      .map(|arrow| {
        let mut arrow = arrow.clone();
        arrow.circle_cells.sort_by_key(|cell| *cell);
        arrow
      })
      .collect();

    Some(ArrowConstraint{
      arrows,
    }.into())
  }
}

pub(crate) static ARROW_CONSTRAINT_BUILDER: ArrowConstraintBuilder = ArrowConstraintBuilder;

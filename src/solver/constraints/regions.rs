use crate::{solver::{checker::SolvedState, constraints::{CandidateMode, Constraint, ConstraintBuilder, ConstraintDefinition, ConstraintDefinitionExt, PeerMode, context::{ConstraintCellPeersContext}}, types::CandidateSet}, types::{Area, CellPosition, Grid, Region, SudokuConstraints}};

#[derive(Clone, Default, Debug)]
pub struct RegionsConstraint {
  regions: Vec<Region>,
  regions_by_cell: Vec<Vec<Vec<usize>>>,
  row_regions: Vec<Region>,
  col_regions: Vec<Region>,
}

impl ConstraintDefinition for RegionsConstraint {
  fn check_grid_special_validity(&self, _grid: &Grid, _candidates: &Vec<Vec<CandidateSet>>) -> SolvedState {
    SolvedState::solved()
  }

  // TODO: maybe it's more intuitive to check regions first?
  fn region_areas(&self) -> Vec<Area> {
    (0..self.row_regions.len()).map(|row| Area::Row(row))
      .chain((0..self.col_regions.len()).map(|col| Area::Column(col)))
      .chain((0..self.regions.len()).map(|region_index| Area::Region(region_index)))
      .collect()
  }

  fn area_cells(&self, area: &Area) -> Vec<CellPosition> {
    let cells = match area {
      &Area::Region(region_index) => &self.regions[region_index],
      &Area::Row(row_index) => &self.row_regions[row_index],
      &Area::Column(col_index) => &self.col_regions[col_index],
      _ => &Region(vec![])
    };

    cells.0.clone()
  }

  // TODO: maybe it's more intuitive to check regions first?
  fn cell_region_areas(&self, cell: &CellPosition, peer_mode: PeerMode, _candidate_mode: CandidateMode) -> Vec<Area> {
    let mut areas: Vec<Area> = vec![];

    if peer_mode != PeerMode::Special {
      areas.extend([Area::Row(cell.row), Area::Column(cell.col)]);
    }

    let grid_size = self.row_regions.len();
    areas.extend(
      self.regions_by_cell[cell.row][cell.col].iter().filter_map(|&region_index| {
        // Note: the first `grid_size` regions are the regular ones, after that they're extra regions
        if peer_mode != PeerMode::Classic && region_index >= grid_size ||
           peer_mode != PeerMode::Special && region_index < grid_size {
          Some(Area::Region(region_index))
        } else {
          None
        }
      })
    );

    areas
  }

  fn restrict_cell_special_candidates(&self, _cell: &CellPosition, _grid: &Grid, _candidates: &mut CandidateSet) {}

  fn cell_peers(&self, cell: &CellPosition, ctx: &ConstraintCellPeersContext) -> Vec<CellPosition> {
    // Note: `ctx.peer_mode` behaviour is implemented in `cell_region_areas` so it works here too
    self.default_cell_peers(cell, ctx.peer_mode, ctx.candidate_mode)
  }
}

impl RegionsConstraint {
  pub(crate) fn original_region_areas(&self) -> Vec<Area> {
    (0..self.regions.len()).map(|region_index| Area::Region(region_index)).collect()
  }

  pub(crate) fn row_areas(&self) -> Vec<Area> {
    (0..self.row_regions.len()).map(|row| Area::Row(row)).collect()
  }

  pub(crate) fn col_areas(&self) -> Vec<Area> {
    (0..self.col_regions.len()).map(|col| Area::Column(col)).collect()
  }
}

pub struct RegionsConstraintBuilder;

impl ConstraintBuilder for RegionsConstraintBuilder {
  fn from_input(&self, constraints: &SudokuConstraints) -> Option<Constraint> {
    let regions: Vec<Region> = constraints.regions.clone().unwrap_or(SudokuConstraints::default_regions(constraints.grid_size)).into_iter().chain(constraints.extra_regions.clone().unwrap_or_default()).collect();
    let row_regions: Vec<Region> = (0..constraints.grid_size).map(|row|
      Region((0..constraints.grid_size).map(|col| CellPosition::new(row, col)).collect())
    ).collect();
    let col_regions: Vec<Region> = (0..constraints.grid_size).map(|col|
      Region((0..constraints.grid_size).map(|row| CellPosition::new(row, col)).collect())
    ).collect();

    let mut regions_by_cell = vec![ vec![ vec![]; constraints.grid_size ]; constraints.grid_size ];
    for (index, regions) in regions.iter().enumerate() {
      for cell in regions.iter() {
        regions_by_cell[cell.row][cell.col].push(index);
      }
    }

    Some(Constraint::Regions(RegionsConstraint{
      regions,
      regions_by_cell,
      row_regions,
      col_regions,
    }))
  }
}

pub static REGIONS_CONSTRAINT_BUILDER: RegionsConstraintBuilder = RegionsConstraintBuilder;

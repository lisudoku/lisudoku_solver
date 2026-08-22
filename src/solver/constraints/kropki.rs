use crate::{solver::{checker::SolvedState, constraints::{CandidateMode, Constraint, ConstraintBuilder, ConstraintDefinition, PeerMode, context::{ConstraintCellPeersContext}}, types::CandidateSet}, types::{Area, CellPosition, Grid, InvalidStateReason, InvalidStateType, KropkiDot, KropkiDotType, SudokuConstraints}};
use itertools::Itertools;
use std::{collections::HashSet, mem::swap};

#[derive(Clone, Default, Debug)]
pub(crate) struct KropkiConstraint {
  pub kropki_dots: Vec<KropkiDot>,
  kropki_dots_by_cell: Vec<Vec<Vec<usize>>>,
}

impl ConstraintDefinition for KropkiConstraint {
  fn check_grid_special_validity(&self, grid: &Grid, _candidates: &Vec<Vec<CandidateSet>>) -> SolvedState {
    for kropki_dot_index in 0..self.kropki_dots.len() {
      let kropki_dot = &self.kropki_dots[kropki_dot_index];
      let KropkiDot { dot_type: _, cell_1, cell_2 } = kropki_dot;
      let mut value1 = grid[cell_1.row][cell_1.col];
      let mut value2 = grid[cell_2.row][cell_2.col];
      if value1 > value2 {
        swap(&mut value1, &mut value2);
      }

      if value1 != 0 && !kropki_dot.check_values(value1, value2) {
        return SolvedState::unsolved(
          InvalidStateReason {
            state_type: InvalidStateType::AreaConstraint,
            area: Area::KropkiDot(kropki_dot_index),
            values: vec![],
          }
        )
      }
    }

    SolvedState::solved()
  }

  fn region_areas(&self) -> Vec<Area> {
    (0..self.kropki_dots.len()).map(|kropki_dot_index| Area::KropkiDot(kropki_dot_index)).collect()
  }

  fn area_cells(&self, area: &Area) -> Vec<CellPosition> {
    let Area::KropkiDot(kropki_dot_index) = area else { return vec![] };

    let kropki_dot = &self.kropki_dots[*kropki_dot_index];
    vec![kropki_dot.cell_1, kropki_dot.cell_2]
  }

  fn cell_region_areas(&self, cell: &CellPosition, peer_mode: PeerMode, _candidate_mode: CandidateMode) -> Vec<Area> {
    if peer_mode == PeerMode::Classic {
      return vec![]
    }

    self.kropki_dots_by_cell[cell.row][cell.col].iter().map(|&kropki_dot_index|
      Area::KropkiDot(kropki_dot_index)
    ).collect()
  }

  fn restrict_cell_special_candidates(&self, _cell: &CellPosition, _grid: &Grid, _candidates: &mut CandidateSet) {
    // Do not enforce candidates directly, use an explicit rule for that
  }

  fn cell_peers(&self, _cell: &CellPosition, _ctx: &ConstraintCellPeersContext) -> Vec<CellPosition> { vec![] }
}

pub struct KropkiConstraintBuilder;

impl ConstraintBuilder for KropkiConstraintBuilder {
  fn from_input(&self, constraints: &SudokuConstraints) -> Option<Constraint> {
    if constraints.kropki_dots.is_none() && !constraints.kropki_negative.unwrap_or_default() {
      return None
    }

    let mut kropki_dots = constraints.kropki_dots.clone().unwrap_or_default();
    let mut kropki_dots_by_cell = vec![ vec![ vec![]; constraints.grid_size ]; constraints.grid_size ];
    for (index, kropki_dot) in kropki_dots.iter().enumerate() {
      let KropkiDot { dot_type: _, cell_1, cell_2 } = kropki_dot;
      kropki_dots_by_cell[cell_1.row][cell_1.col].push(index);
      kropki_dots_by_cell[cell_2.row][cell_2.col].push(index);
    }

    // If the kropki negative flag is set, explicitly build the negative kropki dots
    if constraints.kropki_negative.unwrap_or_default() {
      for cell in CellPosition::all_cells_in_grid_size(constraints.grid_size) {
        let adjacent_cells: HashSet<CellPosition> = cell.otrhogonal_neighbours(constraints.grid_size).into_iter().collect();
        let dot_cells: HashSet<CellPosition> = kropki_dots_by_cell[cell.row][cell.col].iter()
          .map(|&kropki_dot_index| {
            let kropki_dot = &kropki_dots[kropki_dot_index];
            let other_cell = kropki_dot.other_cell(&cell);
            other_cell
          })
          .collect();
        let negative_cells: HashSet<CellPosition> = adjacent_cells.difference(&dot_cells).copied().collect();
        for negative_cell in negative_cells.into_iter().sorted() {
          let kropki_dot_index = kropki_dots.len();
          kropki_dots_by_cell[cell.row][cell.col].push(kropki_dot_index);
          kropki_dots_by_cell[negative_cell.row][negative_cell.col].push(kropki_dot_index);
          kropki_dots.push(KropkiDot {
            dot_type: KropkiDotType::Negative,
            cell_1: cell,
            cell_2: negative_cell,
          })
        }
      }
    }

    Some(Constraint::Kropki(KropkiConstraint{
      kropki_dots,
      kropki_dots_by_cell,
    }))
  }
}

pub static KROPKI_CONSTRAINT_BUILDER: KropkiConstraintBuilder = KropkiConstraintBuilder;
